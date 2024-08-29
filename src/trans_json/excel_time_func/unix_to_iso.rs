use chrono::{TimeZone, Utc};

pub fn unix_to_iso(unix_time: i64) -> String {
    let datetime = Utc.timestamp_opt(unix_time, 0);

    let time = datetime.single();
    match time {
        Some(result) => result.to_rfc3339(),
        None => String::from("Invalid")
    }
}

#[cfg(test)]
mod test {
    use crate::trans_json::excel_time_func::unix_to_iso::unix_to_iso;


    #[test]
    fn valid_unix_to_iso() {
        let unix_time: i64= 1721692800;
        let expect_string = "2024-07-23T00:00:00+00:00"; 

        //* Unix Time GMT 2024.07.23 00:00:00 */
        assert_eq!(unix_to_iso(unix_time), expect_string);
    }
}
