use calamine::{Reader, Xlsx};
use js_sys::Object;
use std::{io::Cursor, usize};
use wasm_bindgen::prelude::*;

pub mod trans_json;
use crate::trans_json::sheet_to_json;
use crate::trans_json::map_to_object::map_to_object;
use crate::trans_json::csv_to_json;

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
    let rows_json = sheet_to_json::sheet_to_json(&mut workbook, sheet_name, is_iso8601);

    //* Json 직렬화 후 반환 */
    map_to_object(rows_json)
}

#[wasm_bindgen]
pub fn all_excel_to_json(excel_data: &[u8], is_iso8601: bool) -> Vec<JsValue> {
    //* Cursor는 데이터를 메모리에 버퍼로 저장하고, 이를 파일처럼 읽고 쓸 수 있도록 해줌 */
    let cursor = Cursor::new(excel_data);

    //* 파일처럼 만들어진 curosr를 이용해 엑셀파일처럼 연다. */
    let mut workbook = Xlsx::new(cursor).unwrap();

    let mut sheet_json = vec![];

    for sheet_name in workbook.sheet_names().to_owned() {
        let rows_json = sheet_to_json::sheet_to_json(&mut workbook, &sheet_name, is_iso8601);
        sheet_json.push(
            <Vec<js_sys::Object> as Into<JsValue>>::into(map_to_object(rows_json)) as JsValue,
        );
    }

    sheet_json
}

#[wasm_bindgen]
pub fn csv_to_json(csv_data: &[u8]) -> Vec<js_sys::Object> {
   let rows_json = csv_to_json::csv_to_json(csv_data);
    map_to_object(rows_json) 
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_read_excel_by_index() {
        // 엑셀 파일 대신 직접 데이터를 제공
        let excel_data = include_bytes!("../hello.xlsx");
        let json = excel_to_json(excel_data, 0, true);

        // JSON 데이터가 예상대로 생성되었는지 확인
        assert_eq!(json.len(), 200);
        console_log!("excel sheet {:?}", json);
    }

    #[wasm_bindgen_test]
    fn test_read_excel_all() {
        // 엑셀 파일 대신 직접 데이터를 제공
        let excel_data = include_bytes!("../hello.xlsx");
        let json = all_excel_to_json(excel_data, true);
        console_log!("excel all sheets {:?}", json);
    }

    #[wasm_bindgen_test]
    pub fn test_csv_to_json() {
        let csv_data = include_bytes!("../test.csv");
        console_log!("csv {:?}", csv_to_json(csv_data)); 
    }
}
