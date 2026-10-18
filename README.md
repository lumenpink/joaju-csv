# joaju-ssot

[![crates.io](https://img.shields.io/crates/v/joaju-ssot.svg)](https://crates.io/crates/joaju-ssot)
[![docs.rs](https://docs.rs/joaju-ssot/badge.svg)](https://docs.rs/joaju-ssot)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![GOST](https://img.shields.io/badge/GOST-1.1.0-emerald.svg)](https://codeberg.org/lumenpink/gost)

Motor soberano em Rust para Fonte Única de Dados (*Single Source of Truth* / SSOT), projeções, filtros relacionais e normalização tabular determinística.

> ⚠️ **Early Development Notice (pre-1.0 / alpha)**  
> This library is currently in early active development (`0.1.0`). It is NOT recommended for production use beyond experimental testing and research. APIs are subject to breaking changes prior to a stable 1.0.0 release.

---

## 📖 Visão Geral

`joaju-ssot` permite carregar dados tabulares a partir de strings ou arquivos (CSV, TSV, DSV com delimitador `;`), extrair diretivas de cabeçalho iniciadas por `%%`, realizar projeções relacionais (`apply_select`) com suporte a aliases (`as`) e aplicar expressões de filtro (`apply_where`) numéricas e textuais com operadores `=`, `==`, `!=`, `<`, `<=`, `>`, `>=` e conectivos lógicos (`AND`, `and`, `&&`).

Além disso, oferece suporte nativo à canonização e resolução de sinônimos genealógicos compatíveis com a especificação internacional de pedigree e suas variantes em português.

---

## 🚀 Instalação

Adicione `joaju-ssot` ao seu `Cargo.toml`:

```toml
[dependencies]
joaju-ssot = "0.1.0"
```

Ou instale a ferramenta de linha de comando:

```bash
cargo install joaju-ssot
```

---

## 🛠️ Exemplo de Uso (Rust API)

```rust
use joaju_ssot::Dataset;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let csv_data = "\
id,nome,idade,cargo
1,Ana,30,Engenharia
2,Bruno,25,Design
3,Carla,35,Engenharia
";

    // 1. Carregar dataset
    let ds = Dataset::from_csv(csv_data)?;

    // 2. Filtrar por condição WHERE
    let filtrado = ds.apply_where("idade >= 30 and cargo == Engenharia")?;
    assert_eq!(filtrado.row_count(), 2);

    // 3. Projetar colunas com aliases (SELECT)
    let projetado = filtrado.apply_select("nome as Colaborador, idade as Anos")?;
    assert_eq!(projetado.headers(), &["Colaborador", "Anos"]);

    // 4. Exportar CSV resultante
    let saida = projetado.to_csv()?;
    println!("{}", saida);

    Ok(())
}
```

### Processamento com Diretivas de Bloco (`%%`)

```rust
use joaju_ssot::split_block_content;

let raw = "\
%% title = Tabela de Membros
%% theme = dark
id,nome
1,Maria
";

let (directives, clean_csv) = split_block_content(raw);
assert_eq!(directives.get("title").map(|s| s.as_str()), Some("Tabela de Membros"));
assert_eq!(directives.get("theme").map(|s| s.as_str()), Some("dark"));
```

---

## 💻 Uso via CLI

A CLI `joaju-ssot` aceita arquivos e opções de consulta diretamente pelo terminal:

```bash
# Filtrar e projetar a partir de arquivo
joaju-ssot query dados.csv --where "idade >= 30" --select "nome,cargo"

# Transformar cabeçalhos para o formato canônico de pedigree (PT)
joaju-ssot canonicalize dados.csv --target pt
```

---

## 📋 Conformidade GOST

Este projeto adota a norma **[GOST](https://codeberg.org/lumenpink/gost)** (Gosplan Standard for Technical Software).
- **Versão GOST:** `1.1.0`
- **Linguagens declaradas:** `rust,shell`
- **Licença:** `Apache-2.0`

```bash
# Executar auditoria de qualidade estrita
cccp check --strict
```

---

## 📄 Licença

Distribuído sob a licença [Apache-2.0](./LICENSE).
