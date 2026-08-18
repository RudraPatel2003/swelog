use std::{
    fs,
    io,
    path::{
        Path,
        PathBuf,
    },
    time::Duration,
};

use async_trait::async_trait;
use miette::{
    Result,
    miette,
};
use rmcp::transport::{
    AuthError,
    AuthorizationManager,
    AuthorizationRequest,
    AuthorizationSession,
    CredentialStore,
    StoredCredentials,
};
use tokio::{
    io::{
        AsyncReadExt,
        AsyncWriteExt,
    },
    net::TcpListener,
    time::timeout,
};

use crate::errors::{
    LinearAuthorizationFailed,
    LinearAuthorizationTimedOut,
    LinearCredentialStoreFailed,
};

const LINEAR_MCP_URL: &str = "https://mcp.linear.app/mcp/readonly";
const AUTHORIZATION_TIMEOUT: Duration = Duration::from_secs(300);
const MAX_CALLBACK_REQUEST_SIZE: usize = 16 * 1024;

#[derive(Clone)]
struct FileCredentialStore {
    path: PathBuf,
}

impl FileCredentialStore {
    fn new(path: PathBuf) -> Self {
        Self { path }
    }

    fn storage_error(&self) -> AuthError {
        AuthError::InternalError(
            LinearCredentialStoreFailed { path: self.path.clone() }.to_string(),
        )
    }
}

#[async_trait]
impl CredentialStore for FileCredentialStore {
    async fn load(&self) -> Result<Option<StoredCredentials>, AuthError> {
        if !self.path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&self.path).map_err(|_| self.storage_error())?;
        let credentials = serde_json::from_str(&content).map_err(|_| self.storage_error())?;

        Ok(Some(credentials))
    }

    async fn save(&self, credentials: StoredCredentials) -> Result<(), AuthError> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(|_| self.storage_error())?;
        }

        let content =
            serde_json::to_string_pretty(&credentials).map_err(|_| self.storage_error())?;
        let temporary_path = self.path.with_extension("json.tmp");

        fs::write(&temporary_path, format!("{content}\n")).map_err(|_| self.storage_error())?;
        set_owner_only_permissions(&temporary_path).map_err(|_| self.storage_error())?;
        replace_credentials_file(&temporary_path, &self.path).map_err(|_| self.storage_error())?;

        Ok(())
    }

    async fn clear(&self) -> Result<(), AuthError> {
        match fs::remove_file(&self.path) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
            Err(_) => Err(self.storage_error()),
        }
    }
}

#[cfg(unix)]
fn set_owner_only_permissions(path: &Path) -> io::Result<()> {
    use std::os::unix::fs::PermissionsExt;

    fs::set_permissions(path, fs::Permissions::from_mode(0o600))
}

#[cfg(not(unix))]
fn set_owner_only_permissions(_path: &Path) -> io::Result<()> {
    Ok(())
}

#[cfg(not(windows))]
fn replace_credentials_file(source: &Path, destination: &Path) -> io::Result<()> {
    fs::rename(source, destination)
}

#[cfg(windows)]
fn replace_credentials_file(source: &Path, destination: &Path) -> io::Result<()> {
    match fs::remove_file(destination) {
        Ok(()) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }

    fs::rename(source, destination)
}

pub async fn get_authorization_manager(credentials_file: &Path) -> Result<AuthorizationManager> {
    let mut authorization_manager =
        AuthorizationManager::new(LINEAR_MCP_URL).await.map_err(map_authorization_error)?;

    authorization_manager
        .set_credential_store(FileCredentialStore::new(credentials_file.to_path_buf()));

    if authorization_manager.initialize_from_store().await.map_err(map_authorization_error)? {
        return Ok(authorization_manager);
    }

    authorize_interactively(authorization_manager).await
}

async fn authorize_interactively(
    mut authorization_manager: AuthorizationManager,
) -> Result<AuthorizationManager> {
    let callback_listener = TcpListener::bind(("127.0.0.1", 0))
        .await
        .map_err(|error| miette!("failed to start Linear OAuth callback server: {error}"))?;
    let callback_address = callback_listener
        .local_addr()
        .map_err(|error| miette!("failed to determine Linear OAuth callback address: {error}"))?;
    let redirect_uri = format!("http://127.0.0.1:{}/callback", callback_address.port());

    let metadata =
        authorization_manager.resolve_metadata().await.map_err(map_authorization_error)?.metadata;
    authorization_manager.set_metadata(metadata);

    let authorization_request = AuthorizationRequest::new(&redirect_uri).with_client_name("swelog");
    let authorization_session =
        AuthorizationSession::new(authorization_manager, authorization_request)
            .await
            .map_err(|(_, error)| map_authorization_error(error))?;

    let authorization_url = authorization_session.get_authorization_url();

    println!("Authorize swelog with Linear using this URL:\n{authorization_url}\n");

    if webbrowser::open(authorization_url).is_err() {
        println!("Unable to open a browser automatically. Open the URL above manually.");
    }

    let callback_url =
        timeout(AUTHORIZATION_TIMEOUT, receive_callback_url(callback_listener, &redirect_uri))
            .await
            .map_err(|_| LinearAuthorizationTimedOut)??;

    authorization_session
        .handle_callback_url(&callback_url)
        .await
        .map_err(map_authorization_error)?;

    Ok(authorization_session.auth_manager)
}

async fn receive_callback_url(
    callback_listener: TcpListener,
    redirect_uri: &str,
) -> Result<String> {
    let (mut stream, _) = callback_listener
        .accept()
        .await
        .map_err(|error| miette!("failed to accept Linear OAuth callback: {error}"))?;

    let mut request = Vec::new();
    let mut buffer = [0_u8; 1024];

    while request.len() < MAX_CALLBACK_REQUEST_SIZE {
        let bytes_read = stream
            .read(&mut buffer)
            .await
            .map_err(|error| miette!("failed to read Linear OAuth callback: {error}"))?;

        if bytes_read == 0 {
            break;
        }

        request.extend_from_slice(&buffer[..bytes_read]);

        if request.windows(4).any(|window| window == b"\r\n\r\n") {
            break;
        }
    }

    let request = String::from_utf8(request)
        .map_err(|error| miette!("Linear OAuth callback was not valid UTF-8: {error}"))?;
    let request_target = request
        .lines()
        .next()
        .and_then(|request_line| request_line.split_whitespace().nth(1))
        .ok_or_else(|| miette!("Linear OAuth callback did not contain a request target"))?;
    let callback_origin = redirect_uri
        .strip_suffix("/callback")
        .ok_or_else(|| miette!("invalid Linear OAuth redirect URI"))?;
    let callback_url = format!("{callback_origin}{request_target}");

    let response_body =
        "Linear authorization complete. You can close this window and return to swelog.";
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{response_body}",
        response_body.len()
    );

    stream
        .write_all(response.as_bytes())
        .await
        .map_err(|error| miette!("failed to respond to Linear OAuth callback: {error}"))?;

    Ok(callback_url)
}

fn map_authorization_error(error: AuthError) -> miette::Report {
    LinearAuthorizationFailed { message: error.to_string() }.into()
}

pub fn linear_mcp_url() -> &'static str {
    LINEAR_MCP_URL
}

pub fn clear_linear_authorization(credentials_file: &Path) -> Result<()> {
    match fs::remove_file(credentials_file) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(_) => Err(LinearCredentialStoreFailed { path: credentials_file.to_path_buf() }.into()),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::clear_linear_authorization;

    #[test]
    fn clear_linear_authorization_removes_credentials_and_is_idempotent() {
        let temporary_directory = tempdir().expect("temporary directory should be created");
        let credentials_file = temporary_directory.path().join("linear-oauth.json");

        fs::write(&credentials_file, "credentials").expect("credentials file should be written");

        clear_linear_authorization(&credentials_file).expect("credentials file should be removed");
        clear_linear_authorization(&credentials_file)
            .expect("missing credentials file should be accepted");

        assert!(!credentials_file.exists());
    }
}
