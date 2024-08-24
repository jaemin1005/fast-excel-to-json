use calamine::{Reader, Xlsx};
use excel_time_func::{excel_time_to_unix_time::excel_time_to_unix_time, unix_to_iso::unix_to_iso};
use serde_json::json;
use std::io::Cursor;
use wasm_bindgen::prelude::*;
use rayon::prelude::*;

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
            let headers = range.headers().unwrap();

            //* 기존의 Rows<'_, Data>는 rayon의 ParallelIterator 트레이트를 구현하지 않기 때문에 into_par_iter를 직접 사용할 수 없기 때문에 변경 */
            let rows: Vec<_> = range.rows().skip(1).collect();

            //* rayon의 into_par_iter를 사용해 데이터를 병렬로 처리한다 */ 
            let rows_json: Vec<_> = rows.into_par_iter().map(|row| {
                let mut row_json = serde_json::Map::new();

                for (header, cell) in headers.iter().zip(row.iter()) {
                    let value = match cell {
                        calamine::Data::Empty => json!(null),
                        calamine::Data::Bool(b) => json!(b),
                        calamine::Data::Float(f) => {
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
                                json!(time)
                            }
                        }
                        _ => json!(null),
                    };
                    row_json.insert(header.to_string(), value);
                }

                row_json
            }).collect();

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
