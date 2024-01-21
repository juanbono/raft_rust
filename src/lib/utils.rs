use jsonrpsee::types::ErrorObjectOwned;

pub fn kv_error(message: String) -> ErrorObjectOwned {
    ErrorObjectOwned::owned(0, message, None as Option<bool>)
}
