## Fast Excel to JSON conversion using multithreading in Rust with WebAssembly
This library provides functions for converting Excel files into JSON objects, with an automatic conversion of Excel date formats into Unix timestamps by default. The library is optimized for performance by processing Excel rows in parallel, resulting in significant speed improvements when handling large datasets.


## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
  - [Functions](#functions)
    - [`excel_to_json`](#excel_to_json)
    - [`all_excel_to_json`](#all_excel_to_json)
- [Contributing](#contributing)
- [License](#license)

## Installation

You can install this package using npm:

```javascript
npm install fast-excel-to-json
```

## Usage
To use the library, first require or import it in your JavaScript code:

```javascript
const excelToJson = require('excel-to-json-conversion');
```

### Functions

#### `excel_to_json`

Converts a specific sheet from an Excel file into a JSON object.

##### Parameters:

- `excel_data` (`Uint8Array`): The binary data of the Excel file.
- `sheet_index` (`number`): The index of the sheet you want to convert (0-based).
- `is_iso8601` (`boolean`): A flag indicating whether dates should be formatted in ISO 8601 format.

##### Returns:

- `(object)[]`: An array of JSON objects representing the data from the specified sheet.

##### Example:

```javascript
const fs = require('fs');
const {excel_to_json} = require('fast-excel-to-json')

const excelData = fs.readFileSync('example.xlsx');
const sheetIndex = 0;
const isIso8601 = true;

const jsonData = excel_to_json(excelData, sheetIndex, isIso8601);
console.log(jsonData);
```

#### `all_excel_to_json`

Converts all sheets from an Excel file into JSON objects.

##### Parameters:

- `excel_data` (`Uint8Array`): The binary data of the Excel file.
- `is_iso8601` (`boolean`): A flag indicating whether dates should be formatted in ISO 8601 format.

##### Returns:

- `any[]`: An array of JSON objects, where each object corresponds to a sheet in the Excel file.

##### Example:

```javascript
const fs = require('fs');
const {all_excel_to_json} = require('fast-excel-to-json')

const excelData = fs.readFileSync('example.xlsx');
const isIso8601 = true;

const allJsonData = all_excel_to_json(excelData, isIso8601);
console.log(allJsonData);
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub if you have any improvements or suggestions.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

---