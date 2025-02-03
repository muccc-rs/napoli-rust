pub fn length(name: &'static str, string: &str) -> Result<(), tonic::Status> {
    if string.len() > napoli_lib::limits::MAX_STR_LEN {
        return Err(tonic::Status::invalid_argument(format!(
            "{} exceeds the maximum limit {}",
            name,
            napoli_lib::limits::MAX_STR_LEN,
        )));
    }
    Ok(())
}
