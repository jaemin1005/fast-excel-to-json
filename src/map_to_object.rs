use serde_json::{Map, Value};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::JsValue;
use js_sys::Reflect;
use wasm_bindgen_test::wasm_bindgen_test;

pub fn map_to_object(map: Vec<Map<String, Value>>) -> Vec<js_sys::Object> {
    map.iter().map(|item| {
        let js_object = js_sys::Object::new();
        for (key, value) in item {
            let key_js = JsValue::from_str(&key);
            let value_js = to_value(&value).unwrap();
            js_sys::Reflect::set(&js_object, &key_js, &value_js).unwrap();
        }

        js_object
    }).collect()
}

#[wasm_bindgen_test]
fn test_map_to_object() {
    // 테스트에 사용할 첫 번째 Map 생성
    let mut map1 = Map::new();
    map1.insert("key1".to_string(), Value::String("value1".to_string()));
    map1.insert("key2".to_string(), Value::Number(42.into()));

    // 테스트에 사용할 두 번째 Map 생성
    let mut map2 = Map::new();
    map2.insert("key3".to_string(), Value::String("value3".to_string()));
    map2.insert("key4".to_string(), Value::Bool(true));

    // Vec<Map<String, Value>>를 생성
    let maps = vec![map1, map2];

    // map_to_object 함수 호출
    let js_objects = map_to_object(maps);

    // 첫 번째 js_sys::Object 확인
    let key1_js = JsValue::from_str("key1");
    let key2_js = JsValue::from_str("key2");

    let value1_js = Reflect::get(&js_objects[0], &key1_js).unwrap();
    let value2_js = Reflect::get(&js_objects[0], &key2_js).unwrap();

    assert_eq!(value1_js.as_string().unwrap(), "value1");
    assert_eq!(value2_js.as_f64().unwrap(), 42.0);

    // 두 번째 js_sys::Object 확인
    let key3_js = JsValue::from_str("key3");
    let key4_js = JsValue::from_str("key4");

    let value3_js = Reflect::get(&js_objects[1], &key3_js).unwrap();
    let value4_js = Reflect::get(&js_objects[1], &key4_js).unwrap();

    assert_eq!(value3_js.as_string().unwrap(), "value3");
    assert_eq!(value4_js.as_bool().unwrap(), true);
}