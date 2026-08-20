use miette::{
    Result,
    miette,
};
use tokio::{
    io::{
        AsyncReadExt,
        AsyncWriteExt,
    },
    net::{
        TcpListener,
        TcpStream,
    },
};

const CALLBACK_PATH: &str = "/callback";
const MAX_CALLBACK_REQUEST_SIZE: usize = 16 * 1024;
const RESPONSE_BODY: &str =
    "Linear authorization complete. You can close this window and return to swelog.";

/// A single-use loopback HTTP server that receives the Linear OAuth redirect.
pub struct CallbackServer {
    listener: TcpListener,
    redirect_uri: String,
}

impl CallbackServer {
    pub async fn bind() -> Result<Self> {
        let listener = TcpListener::bind(("127.0.0.1", 0))
            .await
            .map_err(|error| miette!("failed to start Linear OAuth callback server: {error}"))?;

        let address = listener.local_addr().map_err(|error| {
            miette!("failed to determine Linear OAuth callback address: {error}")
        })?;

        let redirect_uri = format!("http://127.0.0.1:{}{CALLBACK_PATH}", address.port());

        Ok(Self { listener, redirect_uri })
    }

    pub fn redirect_uri(&self) -> &str {
        &self.redirect_uri
    }

    /// Waits for the browser redirect and returns the full callback URL,
    /// including the authorization code and state query parameters.
    pub async fn receive_callback_url(self) -> Result<String> {
        let (mut stream, _) = self
            .listener
            .accept()
            .await
            .map_err(|error| miette!("failed to accept Linear OAuth callback: {error}"))?;

        let request = read_request(&mut stream).await?;
        let request_target = parse_request_target(&request)?;

        let callback_origin = self
            .redirect_uri
            .strip_suffix(CALLBACK_PATH)
            .ok_or_else(|| miette!("invalid Linear OAuth redirect URI"))?;

        let callback_url = format!("{callback_origin}{request_target}");

        write_response(&mut stream).await?;

        Ok(callback_url)
    }
}

async fn read_request(stream: &mut TcpStream) -> Result<String> {
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

        let chunk = buffer
            .get(..bytes_read)
            .ok_or_else(|| miette!("Linear OAuth callback read exceeded the read buffer"))?;

        request.extend_from_slice(chunk);

        if request.windows(4).any(|window| window == b"\r\n\r\n") {
            break;
        }
    }

    String::from_utf8(request)
        .map_err(|error| miette!("Linear OAuth callback was not valid UTF-8: {error}"))
}

fn parse_request_target(request: &str) -> Result<&str> {
    request
        .lines()
        .next()
        .and_then(|request_line| request_line.split_whitespace().nth(1))
        .ok_or_else(|| miette!("Linear OAuth callback did not contain a request target"))
}

async fn write_response(stream: &mut TcpStream) -> Result<()> {
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{RESPONSE_BODY}",
        RESPONSE_BODY.len()
    );

    stream
        .write_all(response.as_bytes())
        .await
        .map_err(|error| miette!("failed to respond to Linear OAuth callback: {error}"))?;

    Ok(())
}
