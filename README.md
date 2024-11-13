# json_parser_hibskyi

Link: https://crates.io/crates/json_parser_hibskyi Docs: https://docs.rs/json_parser_hibskyi/latest/json_parser_hibskyi/

## Project Overview

This project implements a simplified JSON parser in Rust using the Pest parsing library. The parser can recognize and validate basic JSON structures, including objects, arrays, strings, numbers, booleans, and null values. The goal is to provide a foundation for parsing JSON-like data formats with essential functionality, without handling complex edge cases, such as nested escape characters in strings.

## Technical Description of the Parsing Process

The JSON parser uses a custom Pest grammar defined in `json_grammar.pest`. This grammar outlines rules for recognizing each component of JSON syntax. Key rules include:

- **json**: Represents a valid JSON document.
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

## Grammar rules

This section describes the grammar rules used in the simplified JSON parser, designed to handle the basic structure of JSON values, including objects, arrays, strings, numbers, booleans, and null values.

### json

This rule is the entry point for parsing a JSON structure. It parses a value inside the start-of-input (SOI) and end-of-input (EOI) markers, ensuring that the entire content adheres to JSON rules.

```
json = { SOI ~ value ~ EOI }
```

### value

The value rule defines the possible types that a JSON value can take, including objects, arrays, strings, numbers, booleans, and null.

```
value = _{ object | array | string | number | boolean | null }
```

### object

An object in JSON is a collection of key-value pairs enclosed in curly braces {}. Each field consists of a string (the key) followed by a colon : and the corresponding value.

```
object = { "{" ~ (field ~ ("," ~ field)*)? ~ "}" }
```

### field

A field consists of a string followed by a colon : and a value, representing a key-value pair in a JSON object.

```
field = { string ~ ":" ~ value }
```

### array

An array in JSON is a collection of values enclosed in square brackets [], separated by commas.

```
array = { "[" ~ (value ~ ("," ~ value)*)? ~ "]" }
```

### string

A string is a sequence of characters enclosed in double quotes " and can contain any characters except the closing quote. It is parsed as a sequence of any characters that are not a double quote.

```
string = @{ "\"" ~ (!"\"" ~ ANY)* ~ "\"" }
```

### number

A number can either be a positive or negative integer, with an optional decimal point and digits following it. The rule accounts for numbers like -12.34, 0, or 123.

```
number = @{ "-"? ~ ("0" | ASCII_NONZERO_DIGIT ~ ASCII_DIGIT*) ~ ("." ~ ASCII_DIGIT+)? }
```

### boolean

A boolean value can be either true or false.

```
boolean = { "true" | "false" }
```

### null

The null rule matches the literal string "null", representing a null value in JSON.

```
null = { "null" }
```
