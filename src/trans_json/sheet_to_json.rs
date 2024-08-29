use calamine::{Reader, Xlsx};
use rayon::prelude::*;
use serde_json::json;
use serde_json::Map;
use serde_json::Value;
use std::io::Read;
use std::io::Seek;

use super::excel_time_func::excel_time_to_unix_time::excel_time_to_unix_time;
use super::excel_time_func::unix_to_iso::unix_to_iso;

pub fn sheet_to_json<R: Seek + Read>(
    workbook: &mut Xlsx<R>,
    sheet_name: &str,
    is_iso8601: bool,
) -> Vec<Map<String, Value>> {
    if let Ok(range) = workbook.worksheet_range(sheet_name) {
        let headers = match range.headers() {
            Some(headers) => headers,
            None => return vec![],
        };

        //* 기존의 Rows<'_, Data>는 rayon의 ParallelIterator 트레이트를 구현하지 않기 때문에 into_par_iter를 직접 사용할 수 없기 때문에 변경 */
        let rows: Vec<_> = range.rows().skip(1).collect();

        //* rayon의 into_par_iter를 사용해 데이터를 병렬로 처리 */
        let rows_json: Vec<_> = rows
            .into_par_iter()
            .map(|row| {
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
            })
            .collect();

        rows_json
    } else {
        vec![]
    }
}

#[cfg(test)]
mod test {
    use calamine::{open_workbook, Reader, Xlsx};

    use super::sheet_to_json;

    #[test]
    fn excel_read() {
        let mut workbook: Xlsx<_> = open_workbook("hello.xlsx").unwrap();
        let sheet_names = workbook.sheet_names();
        let sheet_name = &sheet_names[0];

        let _json = sheet_to_json(&mut workbook, sheet_name, true);
    }
}
