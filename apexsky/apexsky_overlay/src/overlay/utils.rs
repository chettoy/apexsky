use instant::SystemTime;
use obfstr::obfstr as s;

/// Function to get the Unix timestamp in milliseconds
pub fn get_unix_timestamp_in_millis() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => {
            // Calculate the total milliseconds from the duration
            let millis = duration.as_secs() * 1000 + duration.subsec_millis() as u64;
            millis
        }
        Err(e) => {
            // Handle errors, such as clock rollback
            panic!("{}{:?}", s!("Error getting Unix Timestamp: "), e);
        }
    }
}
