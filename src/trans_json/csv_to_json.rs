use rayon::prelude::*;
use serde_json::Map;
use serde_json::Number;
use serde_json::Value;

pub fn csv_to_json(csv_data: &[u8]) -> Vec<Map<String, Value>> {
    let mut reader = csv::Reader::from_reader(csv_data);

    //* headers */
    let headers = match reader.headers() {
        Ok(headers) => headers,
        Err(_) => return vec![],
    }
    .clone();

    //println!("{:?}", headers);

    let mut records = Vec::new();

    //* records는 header를 제외한 body 행들의 모음 */
    for record in reader.records() {
        match record {
            Ok(data) => records.push(data),
            Err(_) => continue,
        }
    }

    //println!("{:?}", records);

    //* records와 헤더를 쌍으로 Map형태로 만든다 */
    let rows_json = records
        .into_par_iter()
        .map(|record| {
            let mut row_json = serde_json::Map::new();

            for (header, row) in headers.iter().zip(record.iter()) {
                row_json.insert(header.to_string(), transform_string(row));
            }

            row_json
        })
        .collect();

    rows_json
}

/**
 * * transform_string
 * * 문자열을 Rust의 각 타입으로 변환하여 Value로 반환한다.
 */
fn transform_string(value: &str) -> Value {
    //* boolean 타입 검사 */
    let trans = match value.to_lowercase().as_str() {
        "true" => return Value::Bool(true),
        "false" => return Value::Bool(false),
        _ => value,
    };

    let dot_count = trans.matches('.').count();

    //* 점 개수를 판단하여, float, int로 변환한다. 변환이 실패할 경우 문자열을 반환 */
    match dot_count {
        0 => match trans.parse::<i64>() {
            Ok(num) => return Value::Number(Number::from(num)),
            Err(_) => return Value::String(trans.to_string()),
        },
        1 => match trans.parse::<f64>() {
            Ok(num) => match Number::from_f64(num) {
                Some(num) => return Value::Number(num),
                None => return Value::String(trans.to_string()),
            },
            Err(_) => return Value::String(trans.to_string()),
        },
        _ => {
            return Value::String(trans.to_string());
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_csv_to_json() {
        let csv_data = include_bytes!("../../test.csv");
        println!("csv {:?}", csv_to_json(csv_data));
    }
}
