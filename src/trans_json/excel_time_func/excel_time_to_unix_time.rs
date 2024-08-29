use std::f64;

//* 1970-01-01 까지의 엑셀 시간 */
const EXCEL_DATE_SINCE_1970: i64 = 25569;
const UNIX_24H_SECOND: i64 = 86400;

pub fn excel_time_to_unix_time(excel_time : f64) -> i64 {
    
    let date = excel_time.floor() as i64;
    let time = excel_time - date as f64;


    let exel_date_since_1970 = date - EXCEL_DATE_SINCE_1970;

    // 엑셀의 소수 부분은 하루를 기준으로 하기 때문에 86400초(24시간)로 변환
    // 0.5 => 하루의 반나절 (50%)
    let unix_second = (time * UNIX_24H_SECOND as f64).round() as i64; 
    let unix_date = exel_date_since_1970 * UNIX_24H_SECOND;

    unix_date + unix_second
}

#[cfg(test)]
mod tests {
    use crate::trans_json::excel_time_func::excel_time_to_unix_time::excel_time_to_unix_time;

    #[test]
    fn excel_time_to_unix_time_test() {
        //* 2024.07.23 */
        let excel_time = 45496.0;
        //* Unix Time GMT 2024.07.23 00:00:00 */
        let expect_unix_time = 1721692800;

        assert_eq!(excel_time_to_unix_time(excel_time), expect_unix_time);
    }
}