use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum CsvError {
    EmptyInput,
    DuplicateHeader(String),
    FieldCountMismatch { line: usize, expected: usize, got: usize },
}

pub fn parse_csv(input: &str) -> Result<Vec<HashMap<String, String>>, CsvError> {
    let mut lines = input.lines().filter(|l| !l.trim().is_empty());

    let header_line = lines.next().ok_or(CsvError::EmptyInput)?;
    let headers: Vec<&str> = header_line.split(',').collect();

    // Check for duplicate headers
    let mut seen = HashMap::new();
    for h in &headers {
        if seen.contains_key(h) {
            return Err(CsvError::DuplicateHeader(h.to_string()));
        }
        seen.insert(h, true);
    }

    let mut result = Vec::new();
    let mut line_no = 0;

    for line in lines {
        line_no += 1;
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() != headers.len() {
            return Err(CsvError::FieldCountMismatch {
                line: line_no,
                expected: headers.len(),
                got: fields.len(),
            });
        }

        let mut row = HashMap::new();
        for (i, field) in fields.iter().enumerate() {
            row.insert(headers[i].to_string(), field.to_string());
        }
        result.push(row);
    }

    Ok(result)
}

pub fn record_count(input: &str) -> Result<usize, CsvError> {
    Ok(parse_csv(input)?.len())
}
