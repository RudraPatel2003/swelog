set positional-arguments

# Windows runners reach the recipes through Git Bash, so they behave the same as every other platform.
set windows-shell := ['bash', '-cu']

import 'just/rust.just'
import 'just/docs.just'
import 'just/release.just'
