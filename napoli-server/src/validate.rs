use time::{OffsetDateTime, UtcOffset};

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

pub fn cutoff_time_not_in_past(cutoff_time: OffsetDateTime) -> Result<(), tonic::Status> {
    let current_date = OffsetDateTime::now_utc();
    if cutoff_time < current_date {}
    unimplemented!()
}
