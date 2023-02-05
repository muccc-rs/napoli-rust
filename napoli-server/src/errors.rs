pub fn map_to_status<T>(err: T) -> tonic::Status
where
    T: std::fmt::Display,
{
    tonic::Status::internal(err.to_string())
}

pub fn grpc_check_err<T>(res: anyhow::Result<T>) -> std::result::Result<T, tonic::Status> {
    match res {
        Ok(t) => Ok(t),
        Err(e) => Err(tonic::Status::internal(e.to_string())),
    }
}
