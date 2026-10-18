# joaju-csv

[![crates.io](https://img.shields.io/crates/v/joaju-csv.svg)](https://crates.io/crates/joaju-csv)
[![docs.rs](https://docs.rs/joaju-csv/badge.svg)](https://docs.rs/joaju-csv)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![GOST](https://img.shields.io/badge/GOST-1.1.0-emerald.svg)](https://codeberg.org/lumenpink/gost)

A fast, deterministic Rust library for tabular CSV manipulation, relational projections, conditional filtering, and header canonicalization.

> ⚠️ **Early Development Notice (pre-1.0 / alpha)**  
> This library is currently in early active development (`0.1.0`). It is NOT recommended for production use beyond experimental testing and research. APIs are subject to breaking changes prior to a stable 1.0.0 release.

---

## 📖 Overview

`joaju-csv` enables loading tabular data from strings or files (CSV, TSV, DSV with `;` delimiter), extracting header directives prefixed by `%%`, performing relational projections (`apply_select`) with alias support (`as`), and applying filter expressions (`apply_where`) using numeric and textual comparisons (`=`, `==`, `!=`, `<`, `<=`, `>`, `>=`) along with logical connectives (`AND`, `and`, `&&`).

Additionally, it provides built-in support for canonicalizing and resolving pedigree column synonyms compatible with international pedigree format specifications and localized variants.

---

## 🚀 Installation

Add `joaju-csv` to your `Cargo.toml`:

```toml
[dependencies]
joaju-csv = "0.1.0"
```

Or install the CLI tool:

```bash
cargo install joaju-csv
```

---

## 🛠️ Usage Example (Rust API)

```rust
use joaju_csv::Dataset;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let csv_data = "\
id,name,age,department
1,Alice,30,Engineering
2,Bob,25,Design
3,Carol,35,Engineering
";

    // 1. Load dataset
    let ds = Dataset::from_csv(csv_data)?;

    // 2. Filter rows using WHERE condition
    let filtered = ds.apply_where("age >= 30 and department == Engineering")?;
    assert_eq!(filtered.row_count(), 2);

    // 3. Project columns with aliases (SELECT)
    let projected = filtered.apply_select("name as Employee, age as Years")?;
    assert_eq!(projected.headers(), &["Employee", "Years"]);

    // 4. Export resulting CSV
    let output = projected.to_csv()?;
    println!("{}", output);

    Ok(())
}
```

### Processing Block Directives (`%%`)

```rust
use joaju_csv::split_block_content;

let raw = "\
%% title = Member Directory
%% theme = dark
id,name
1,Mary
";

let (directives, clean_csv) = split_block_content(raw);
assert_eq!(directives.get("title").map(|s| s.as_str()), Some("Member Directory"));
assert_eq!(directives.get("theme").map(|s| s.as_str()), Some("dark"));
```

---

## 💻 CLI Usage

The `joaju-csv` CLI accepts input files and query parameters directly from the terminal:

```bash
# Filter and project columns from a file
joaju-csv query data.csv --where "age >= 30" --select "name,department"

# Transform headers into canonical pedigree format
joaju-csv canonicalize data.csv --target pt
```

---

## 📋 GOST Compliance

This project adheres to the **[GOST](https://codeberg.org/lumenpink/gost)** specification (Gosplan Standard for Technical Software).
- **GOST Version:** `1.1.0`
- **Declared Languages:** `rust,shell`
- **License:** `Apache-2.0`

```bash
# Run strict compliance verification
cccp check --strict
```

---

## 📄 License

Distributed under the [Apache-2.0](./LICENSE) license.
