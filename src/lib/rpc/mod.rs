mod api;
mod backend;

pub use api::{RpcApiClient, RpcApiServer};
pub use backend::RpcBackend;
use jsonrpsee::types::ErrorObjectOwned;

pub fn kv_error(message: String) -> ErrorObjectOwned {
    ErrorObjectOwned::owned(0, message, None as Option<bool>)
}
