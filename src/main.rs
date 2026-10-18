use clap::Parser;
use joaju_csv::Dataset;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "joaju-csv")]
#[command(
    about = "A fast and deterministic Rust CLI for tabular CSV manipulation, relational projections, and filtering"
)]
struct Cli {
    /// Path to input CSV file (reads from stdin if omitted)
    #[arg(short, long)]
    src: Option<PathBuf>,

    /// Column projection expression with optional aliases (e.g. "id, father, phenotype as trait")
    #[arg(long)]
    select: Option<String>,

    /// Conditional filtering expression (e.g. "generation <= 3 and sex == F")
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
