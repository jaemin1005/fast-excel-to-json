const excel = require("../pkg/excel_to_json");
const XLSX = require("xlsx");
const fs = require("fs");

function run() {
  fs.readFile("test.xlsx", (err, data) => {
    if (err) return;

    console.time("Excel to JSON Conversion");
    const vec = excel.excel_to_json(new Uint8Array(data), false);
    console.timeEnd("Excel to JSON Conversion");
  });
}

function run2() {
  fs.readFile("test.xlsx", (err, data) => {
    if (err) throw err;

    // ArrayBuffer로 변환
    const arrayBuffer = data.buffer.slice(
      data.byteOffset,
      data.byteOffset + data.byteLength
    );

    console.time("Excel to JSON Conversion xlsx");
    // 엑셀 파일 읽기
    const workbook = XLSX.read(new Uint8Array(arrayBuffer), { type: "array" });

    // 첫 번째 시트 가져오기
    const sheetName = workbook.SheetNames[0];
    const sheet = workbook.Sheets[sheetName];

    // 시트를 JSON으로 변환
    const jsonData = XLSX.utils.sheet_to_json(sheet);
    console.timeEnd("Excel to JSON Conversion xlsx");
  });
}

run();
run2();