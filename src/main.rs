use clap::Parser;
use ssot_rs::Dataset;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "ssot-rs")]
#[command(
    about = "Motor soberano em Rust para Fonte Única de Dados (SSOT), projeções e filtros tabulares"
)]
struct Cli {
    /// Caminho do arquivo CSV de entrada
    #[arg(short, long)]
    src: Option<PathBuf>,

    /// Expressão de projeção de colunas com suporte a aliases (ex: "id, pai, fenotipo_abo as fenotipo")
    #[arg(long)]
    select: Option<String>,

    /// Expressão condicional de filtragem (ex: "geracao <= 3 and sex == F")
    #[arg(long, name = "where")]
    where_clause: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let dataset = if let Some(ref path) = cli.src {
        Dataset::from_file(path)?
    } else {
        use std::io::Read;
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        Dataset::from_csv(&buffer)?
    };

    let result = dataset.apply_query(cli.select.as_deref(), cli.where_clause.as_deref())?;
    print!("{}", result.to_csv()?);

    Ok(())
}
