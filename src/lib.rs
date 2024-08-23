use calamine::{Reader, Xlsx};
use excel_time_func::{excel_time_to_unix_time::excel_time_to_unix_time, unix_to_iso::unix_to_iso};
use serde_json::json;
use std::io::Cursor;
use wasm_bindgen::prelude::*;

pub mod excel_time_func;

#[wasm_bindgen]
pub fn excel_to_json(excel_data: &[u8], is_iso8601: bool) -> Vec<String>{
    //* Cursor는 데이터를 메모리에 버퍼로 저장하고, 이를 파일처럼 읽고 쓸 수 있도록 해줌 */
    let cursor = Cursor::new(excel_data);

    //* 파일처럼 만들어진 curosr를 이용해 엑셀파일처럼 연다. */
    let mut workbook = Xlsx::new(cursor).unwrap();

    let mut sheet_json = vec![];

    for sheet_name in workbook.sheet_names().to_owned() {
        if let Ok(range) = workbook.worksheet_range(&sheet_name) {
            let mut rows_json = vec![];
            let headers = range.headers().unwrap();

            for row in range.rows().into_iter().skip(1) {
                //* ROW의 Cell 데이터들 match를 이용해 변환 */
                let mut row_json = serde_json::Map::new();

                for (header, cell) in headers.iter().zip(row.iter()) {
                    let value = match cell {
                        calamine::Data::Empty => json!(null),
                        calamine::Data::Bool(b) => json!(b),
                        calamine::Data::Float(f) => {
                            //* 실수형 0.0으로 끝날 때 정수형으로 변경 */
                            if f.fract() == 0.0 {
                                let num = *f as i64;
                                json!(num)
                            } else {
                                json!(f)
                            }
                        }
                        calamine::Data::Int(i) => json!(i),
                        calamine::Data::String(s) => json!(s),
                        calamine::Data::DateTime(dt) => {
                            let time = excel_time_to_unix_time(dt.as_f64());

                            if is_iso8601 {
                                json!(unix_to_iso(time))
                            } else {
                                json!(excel_time_to_unix_time(dt.as_f64()))
                            }
                        }
                        _ => json!(null),
                    };
                    row_json.insert(header.to_string(), value);
                }
                rows_json.push(row_json);
            }

            //* Json 직렬화한다 */
            let json_string = serde_json::to_string(&rows_json).unwrap();
            sheet_json.push(json_string);
        }
    }

    sheet_json
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::Read};

    #[test]
    fn read_exel() {
        let path = "test.xlsx";
        let mut file = File::open(path).expect("Don't find File");
        let mut buffer = Vec::new();
        let _ = file.read_to_end(&mut buffer);
        let json = excel_to_json(&buffer, true);
        
        assert_eq!(json.len(), 3);
    }
}
