use crate::core;
use thiserror::Error;
use tower_lsp::lsp_types;

/// Runtime errors for the LSP server.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum Error {
    /// Error that occurs when [`core::Session.client`] is accessed and is `None`.
    //#[error("ClientNotInitialzed")]
    //ClientNotInitialized,
    /// Error that occurs when a session resource is requested and does not exist.
    #[error("core::SessionResourceNotFound: kind={kind:?}, uri={uri:?}")]
    SessionResourceNotFound {
        /// The kind of the requested session resource.
        kind: core::session::SessionResourceKind,
        /// The URL of the requested session resource.
        uri: lsp_types::Url,
    },

    #[error("I/O error")]
    IoError(#[from] std::io::Error),

    #[error("UTF8 conversion error")]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error("ParseInt error")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("Language error")]
    LanguageError(#[from] tree_sitter::LanguageError),

    #[error("Cannot convert URI to file path")]
    UriToPathConversion,
}

/// Wrapper struct for converting [`anyhow::Error`] into [`tower_lsp::jsonrpc::Error`].
pub struct IntoJsonRpcError(pub anyhow::Error);

impl From<IntoJsonRpcError> for tower_lsp::jsonrpc::Error {
    fn from(error: IntoJsonRpcError) -> Self {
        let mut rpc_error = tower_lsp::jsonrpc::Error::internal_error();
        rpc_error.data = Some(serde_json::to_value(format!("{}", error.0)).unwrap());
        rpc_error
    }
}
