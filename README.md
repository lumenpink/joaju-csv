# joaju-csv

[![crates.io](https://img.shields.io/crates/v/joaju-csv.svg)](https://crates.io/crates/joaju-csv)
[![docs.rs](https://docs.rs/joaju-csv/badge.svg)](https://docs.rs/joaju-csv)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![GOST](https://img.shields.io/badge/GOST-1.4.0-emerald.svg)](https://codeberg.org/lumenpink/gost)

A fast, deterministic Rust library for tabular CSV manipulation, relational projections, conditional filtering, and header canonicalization.

---

## 📖 Overview

`joaju-csv` provides robust tools for loading tabular datasets, parsing header directives (`%%`), performing relational projections (`apply_select`) with aliases, and evaluating structured conditional expressions (`apply_where`).

For comprehensive API guides, interactive examples, and architectural specifications, see the documentation in `site/` or visit [joaju-csv documentation](https://joaju.2lp.in).

---

## 🚀 Quick Start

Add `joaju-csv` to your `Cargo.toml`:

```toml
[dependencies]
joaju-csv = "0.2.0"
```

```rust
use joaju_csv::Dataset;

let ds = Dataset::from_csv("name,age\nAlice,30\nBob,25")?;
let filtered = ds.apply_where("age >= 30")?;
assert_eq!(filtered.row_count(), 1);
```

---

## 📋 GOST Compliance

This project adheres to the **[GOST](https://codeberg.org/lumenpink/gost)** specification:
- **GOST Version:** `1.4.0`
- **Declared Languages:** `rust,shell`
- **License:** `Apache-2.0`

---

## 📄 License

Distributed under the [Apache-2.0](./LICENSE) license.
