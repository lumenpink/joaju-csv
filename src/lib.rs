use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SsotError {
    #[error("Erro de I/O: {0}")]
    Io(#[from] std::io::Error),

    #[error("Erro de parsing CSV: {0}")]
    Csv(#[from] csv::Error),

    #[error("Coluna não encontrada: {0}")]
    ColumnNotFound(String),

    #[error("Expressão de where inválida: {0}")]
    InvalidWhereClause(String),

    #[error("Expressão de select inválida: {0}")]
    InvalidSelectClause(String),

    #[error("Dataset vazio")]
    EmptyDataset,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dataset {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl Dataset {
    pub fn new(headers: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        Self { headers, rows }
    }

    pub fn headers(&self) -> &[String] {
        &self.headers
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub fn column_count(&self) -> usize {
        self.headers.len()
    }

    pub fn get_row(&self, index: usize) -> Option<&[String]> {
        self.rows.get(index).map(|r| r.as_slice())
    }

    pub fn rows(&self) -> &[Vec<String>] {
        &self.rows
    }

    pub fn from_csv(content: &str) -> Result<Self, SsotError> {
        let clean_content: Vec<&str> = content
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty() && !l.starts_with("%%") && !l.starts_with('#'))
            .collect();

        if clean_content.is_empty() {
            return Err(SsotError::EmptyDataset);
        }

        let first_line = clean_content[0];
        let delimiter = if first_line.contains('\t') {
            b'\t'
        } else if first_line.contains(';') && !first_line.contains(',') {
            b';'
        } else {
            b','
        };

        let joined_body = clean_content.join("\n");
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(delimiter)
            .has_headers(true)
            .flexible(true)
            .trim(csv::Trim::All)
            .from_reader(joined_body.as_bytes());

        let headers = reader
            .headers()?
            .iter()
            .map(|h| h.trim().to_string())
            .collect::<Vec<_>>();

        let mut rows = Vec::new();
        for record in reader.records() {
            let rec = record?;
            let mut row = Vec::with_capacity(headers.len());
            for field in rec.iter() {
                row.push(field.trim().to_string());
            }
            while row.len() < headers.len() {
                row.push(String::new());
            }
            rows.push(row);
        }

        Ok(Self { headers, rows })
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, SsotError> {
        let content = fs::read_to_string(path)?;
        Self::from_csv(&content)
    }

    pub fn to_csv(&self) -> Result<String, SsotError> {
        let mut wtr = csv::WriterBuilder::new().from_writer(vec![]);
        wtr.write_record(&self.headers)?;
        for row in &self.rows {
            wtr.write_record(row)?;
        }
        wtr.flush()?;
        let bytes = wtr.into_inner().map_err(|e| e.into_error())?;
        Ok(String::from_utf8_lossy(&bytes).to_string())
    }

    pub fn apply_select(&self, select_expr: &str) -> Result<Self, SsotError> {
        let trimmed = select_expr.trim();
        if trimmed.is_empty() || trimmed == "*" {
            return Ok(self.clone());
        }

        let mut projected_headers = Vec::new();
        let mut col_indices = Vec::new();

        let re_alias = Regex::new(r"(?i)^(.+?)\s+as\s+(.+)$").unwrap();

        for item in trimmed.split(',') {
            let raw_item = item.trim();
            if raw_item.is_empty() {
                continue;
            }

            let (src_name, target_name) = if let Some(caps) = re_alias.captures(raw_item) {
                let src = caps.get(1).unwrap().as_str().trim();
                let dst = caps.get(2).unwrap().as_str().trim();
                (src, dst)
            } else {
                (raw_item, raw_item)
            };

            let idx = self
                .headers
                .iter()
                .position(|h| h.eq_ignore_ascii_case(src_name))
                .ok_or_else(|| SsotError::ColumnNotFound(src_name.to_string()))?;

            col_indices.push(idx);
            projected_headers.push(target_name.to_string());
        }

        if projected_headers.is_empty() {
            return Err(SsotError::InvalidSelectClause(
                "Nenhuma coluna selecionada".to_string(),
            ));
        }

        let mut projected_rows = Vec::with_capacity(self.rows.len());
        for row in &self.rows {
            let mut new_row = Vec::with_capacity(col_indices.len());
            for &idx in &col_indices {
                new_row.push(row.get(idx).cloned().unwrap_or_default());
            }
            projected_rows.push(new_row);
        }

        Ok(Self {
            headers: projected_headers,
            rows: projected_rows,
        })
    }

    pub fn apply_where(&self, where_expr: &str) -> Result<Self, SsotError> {
        let trimmed = where_expr.trim();
        if trimmed.is_empty() {
            return Ok(self.clone());
        }

        let clauses: Vec<&str> = if trimmed.contains(" and ") {
            trimmed.split(" and ").collect()
        } else if trimmed.contains(" AND ") {
            trimmed.split(" AND ").collect()
        } else if trimmed.contains("&&") {
            trimmed.split("&&").collect()
        } else {
            vec![trimmed]
        };

        let parsed_clauses: Result<Vec<Condition>, SsotError> = clauses
            .iter()
            .map(|c| Condition::parse(c.trim(), &self.headers))
            .collect();

        let conditions = parsed_clauses?;

        let mut filtered_rows = Vec::new();
        for row in &self.rows {
            let mut matches_all = true;
            for cond in &conditions {
                if !cond.evaluate(row) {
                    matches_all = false;
                    break;
                }
            }
            if matches_all {
                filtered_rows.push(row.clone());
            }
        }

        Ok(Self {
            headers: self.headers.clone(),
            rows: filtered_rows,
        })
    }

    pub fn apply_query(
        &self,
        select_expr: Option<&str>,
        where_expr: Option<&str>,
    ) -> Result<Self, SsotError> {
        let mut result = if let Some(w) = where_expr {
            self.apply_where(w)?
        } else {
            self.clone()
        };

        if let Some(s) = select_expr {
            result = result.apply_select(s)?;
        }

        Ok(result)
    }
}

pub fn parse_block_directives(block: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in block.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("%%") {
            let body = trimmed.trim_start_matches('%').trim();
            if let Some((k, v)) = body.split_once('=') {
                map.insert(k.trim().to_lowercase(), v.trim().to_string());
            }
        }
    }
    map
}

pub fn split_block_content(block: &str) -> (HashMap<String, String>, String) {
    let directives = parse_block_directives(block);
    let body_lines: Vec<&str> = block
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.starts_with("%%"))
        .collect();
    (directives, body_lines.join("\n"))
}

#[derive(Debug, Clone)]
enum Operator {
    Eq,
    Neq,
    Lte,
    Gte,
    Lt,
    Gt,
}

#[derive(Debug, Clone)]
struct Condition {
    col_index: usize,
    op: Operator,
    target_value: String,
}

impl Condition {
    fn parse(clause: &str, headers: &[String]) -> Result<Self, SsotError> {
        let raw = clause.trim();
        let ops = [
            ("<=", Operator::Lte),
            (">=", Operator::Gte),
            ("!=", Operator::Neq),
            ("==", Operator::Eq),
            ("=", Operator::Eq),
            ("<", Operator::Lt),
            (">", Operator::Gt),
        ];

        for (sym, op) in ops {
            if let Some((left, right)) = raw.split_once(sym) {
                let col_name = left.trim();
                let target = right
                    .trim()
                    .trim_matches('"')
                    .trim_matches('\'')
                    .to_string();

                let col_index = headers
                    .iter()
                    .position(|h| h.eq_ignore_ascii_case(col_name))
                    .ok_or_else(|| SsotError::ColumnNotFound(col_name.to_string()))?;

                return Ok(Self {
                    col_index,
                    op,
                    target_value: target,
                });
            }
        }

        Err(SsotError::InvalidWhereClause(format!(
            "Operador não reconhecido em '{}'",
            raw
        )))
    }

    fn evaluate(&self, row: &[String]) -> bool {
        let val = match row.get(self.col_index) {
            Some(v) => v.trim(),
            None => "",
        };

        if let (Ok(num_val), Ok(num_target)) =
            (val.parse::<f64>(), self.target_value.parse::<f64>())
        {
            match self.op {
                Operator::Eq => (num_val - num_target).abs() < f64::EPSILON,
                Operator::Neq => (num_val - num_target).abs() >= f64::EPSILON,
                Operator::Lte => num_val <= num_target,
                Operator::Gte => num_val >= num_target,
                Operator::Lt => num_val < num_target,
                Operator::Gt => num_val > num_target,
            }
        } else {
            let left = val.to_lowercase();
            let right = self.target_value.to_lowercase();
            match self.op {
                Operator::Eq => left == right,
                Operator::Neq => left != right,
                Operator::Lte => left <= right,
                Operator::Gte => left >= right,
                Operator::Lt => left < right,
                Operator::Gt => left > right,
            }
        }
    }
}
