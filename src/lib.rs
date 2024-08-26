use calamine::{Reader, Xlsx};
use excel_time_func::{excel_time_to_unix_time::excel_time_to_unix_time, unix_to_iso::unix_to_iso};
use js_sys::Object;
use rayon::prelude::*;
use serde_json::json;
use std::{io::Cursor, usize};
use wasm_bindgen::prelude::*;

use crate::map_to_object::map_to_object;

pub mod excel_time_func;
pub mod map_to_object;

#[wasm_bindgen]
pub fn excel_to_json(excel_data: &[u8], sheet_index: usize, is_iso8601: bool) -> Vec<Object> {
    //* Cursor는 데이터를 메모리에 버퍼로 저장하고, 이를 파일처럼 읽고 쓸 수 있도록 해줌 */
    let cursor = Cursor::new(excel_data);

    //* 파일처럼 만들어진 cursor를 이용해 엑셀 파일처럼 연다 */
    let mut workbook = Xlsx::new(cursor).unwrap();

    //* 워크북의 모든 시트 이름 목록을 가져옴 */
    let sheet_names = workbook.sheet_names().to_owned();

    //* sheet_index가 유효한지 확인 */
    if sheet_index >= sheet_names.len() {
        panic!("Invalid sheet index: {}", sheet_index);
    }

    //* 지정된 인덱스의 시트 이름을 가져옴 */
    let sheet_name = &sheet_names[sheet_index];

    //* 해당 시트의 데이터를 가져옴 */
    if let Ok(range) = workbook.worksheet_range(sheet_name) {
        let headers = range.headers().unwrap();

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

        //* Json 직렬화 후 반환 */
        map_to_object(rows_json)
    } else {
        vec![] // 시트를 읽지 못하면 빈 벡터를 반환
    }
}

#[wasm_bindgen]
pub fn all_excel_to_json(excel_data: &[u8], is_iso8601: bool) -> Vec<JsValue>{
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
              sheet_json.push(<Vec<js_sys::Object> as Into<JsValue>>::into(map_to_object(rows_json)) as JsValue);
          }
      }
  
      sheet_json
}


#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_read_excel_by_index() {
        // 엑셀 파일 대신 직접 데이터를 제공
        let excel_data = include_bytes!("../test.xlsx");
        let json = excel_to_json(excel_data, 0, true);

        // JSON 데이터가 예상대로 생성되었는지 확인
        assert_eq!(json.len(), 1);
    }
}
