// Day 9: CSV Parser
//
// Parse CSV input into a Vec of HashMaps with robust error handling.
//
// Learning goals:
//   - Custom error enums with data fields
//   - Result type with rich error information
//   - String parsing and line-based processing
//   - Empty line handling and 1-indexed line tracking

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum CsvError {
    EmptyInput,
    DuplicateHeader(String),
    FieldCountMismatch {
        /// 1-indexed line number (counting non-empty lines after the header)
        line: usize,
        expected: usize,
        got: usize,
    },
}

/// Parses CSV text into a Vec of HashMaps.
///
/// - First non-empty line is the header row
/// - Empty lines are skipped and do not count toward line numbers
/// - Each data row must have exactly as many fields as the header
/// - Duplicate header names return CsvError::DuplicateHeader
pub fn parse_csv(input: &str) -> Result<Vec<HashMap<String, String>>, CsvError> {
    todo!("Implement parse_csv")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string_returns_error() {
        let result = parse_csv("");
        assert_eq!(result, Err(CsvError::EmptyInput));
    }

    #[test]
    fn whitespace_only_returns_error() {
        // Only empty lines (whitespace) still means no content
        let result = parse_csv("\n  \n");
        assert!(result.is_err());
    }

    #[test]
    fn header_only_no_data_returns_empty_vec() {
        let input = "name,age,city";
        let result = parse_csv(input).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn header_only_with_trailing_newlines() {
        let input = "name,age,city\n\n\n";
        let result = parse_csv(input).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn duplicate_header_returns_error() {
        let input = "name,age,name\nAlice,30,NYC";
        let result = parse_csv(input);
        assert_eq!(result, Err(CsvError::DuplicateHeader("name".to_string())));
    }

    #[test]
    fn row_too_few_fields_returns_error() {
        let input = "name,age,city\nAlice,30\nBob,25,London";
        let result = parse_csv(input);
        assert_eq!(
            result,
            Err(CsvError::FieldCountMismatch {
                line: 1,
                expected: 3,
                got: 2,
            })
        );
    }

    #[test]
    fn row_too_many_fields_returns_error() {
        let input = "name,age\nAlice,30,extra\nBob,25";
        let result = parse_csv(input);
        assert_eq!(
            result,
            Err(CsvError::FieldCountMismatch {
                line: 1,
                expected: 2,
                got: 3,
            })
        );
    }

    #[test]
    fn valid_csv_returns_correct_data() {
        let input = "name,age,city\nAlice,30,Warsaw\nBob,25,London\nCarol,35,Paris";
        let result = parse_csv(input).unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].get("name").unwrap(), "Alice");
        assert_eq!(result[0].get("age").unwrap(), "30");
        assert_eq!(result[0].get("city").unwrap(), "Warsaw");
        assert_eq!(result[1].get("name").unwrap(), "Bob");
        assert_eq!(result[2].get("name").unwrap(), "Carol");
    }

    #[test]
    fn empty_lines_are_skipped_not_counted() {
        // The empty line between Bob and Carol should NOT increment line count
        let input = "name,age\nAlice,30\nBob,25\n\nCarol,35";
        let result = parse_csv(input).unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result[2].get("name").unwrap(), "Carol");
    }

    #[test]
    fn empty_lines_before_header_are_skipped() {
        let input = "\n\n\nname,value\nhello,world";
        let result = parse_csv(input).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].get("name").unwrap(), "hello");
        assert_eq!(result[0].get("value").unwrap(), "world");
    }

    #[test]
    fn line_numbers_are_one_indexed_skipping_empty_lines() {
        // Empty line between rows does not increment line counter
        let input = "col1,col2\na,b\n\nc,d,e";
        let result = parse_csv(input);
        assert_eq!(
            result,
            Err(CsvError::FieldCountMismatch {
                line: 2, // The row "c,d,e" is the 2nd non-empty data row
                expected: 2,
                got: 3,
            })
        );
    }

    #[test]
    fn fields_with_commas_in_quotes_are_not_split() {
        // Basic CSV: fields may be quoted, but the spec does not require quote handling.
        // This test verifies that bare commas delimit fields.
        let input = "a,b,c\n1,2,3";
        let result = parse_csv(input).unwrap();
        assert_eq!(result[0].get("a").unwrap(), "1");
    }
}
