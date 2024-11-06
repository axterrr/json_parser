# JSON Parser

## Project Overview

This project implements a simplified JSON parser in Rust using the Pest parsing library. The parser can recognize and validate basic JSON structures, including objects, arrays, strings, numbers, booleans, and null values. The goal is to provide a foundation for parsing JSON-like data formats with essential functionality, without handling complex edge cases, such as nested escape characters in strings.

## Technical Description of the Parsing Process

The JSON parser uses a custom Pest grammar defined in `json_grammar.pest`. This grammar outlines rules for recognizing each component of JSON syntax. Key rules include:

- **value**: Represents any valid JSON value (objects, arrays, strings, numbers, booleans, and null).
- **object**: Matches JSON objects, which are defined by key-value pairs inside curly braces `{}`.
- **array**: Matches JSON arrays, which are defined by comma-separated values inside square brackets `[]`.
- **string**: Matches text inside double quotes, allowing any characters except unescaped double quotes.
- **number**: Matches integers and floating-point numbers, optionally prefixed with a minus sign.
- **boolean** and **null**: Recognize the literals `true`, `false`, and `null`.

### Parsing Process

1. **Tokenizing**: Pest reads the input string and applies the grammar rules to identify tokens representing different JSON components.
2. **Recursive Parsing**: Starting from the `json` rule, the parser recursively processes the JSON structure, checking each component against its expected structure. If the input adheres to the JSON syntax, the parse tree is generated.
3. **Error Handling**: If any part of the JSON structure is invalid, an error is returned, specifying the point of failure for easier debugging.

### Usage of Parsing Results

The parsing results are intended to be used for:
- Validating JSON data before further processing or storage.
- Extracting data from the JSON structure for use in Rust applications.
- Converting JSON data into Rust objects or other data formats if additional layers are implemented.
