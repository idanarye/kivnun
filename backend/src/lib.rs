#[cfg(feature = "server")]
pub mod wsjrpc;

use dioxus::prelude::*;

/// Echo the user input on the server.
#[server(EchoServer)]
pub async fn echo_server(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}
