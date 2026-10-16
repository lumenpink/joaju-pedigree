use clap::Parser;
use pedigreers::prelude::*;

#[derive(Parser, Debug)]
#[command(name = "pedigreers", version, about = "Motor soberano de heredogramas clínicos em Rust")]
struct Cli {
    #[arg(short, long)]
    input: Option<String>,

    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    println!("pedigreers v{} inicializado.", env!("CARGO_PKG_VERSION"));

    let csv_content = if let Some(path) = cli.input {
        std::fs::read_to_string(path)?
    } else {
        "id,pai,mae,sexo,status,probando,aff\n\
         1,0,0,1,0,0,0\n\
         2,0,0,2,0,0,0\n\
         3,1,2,1,0,1,1\n"
            .to_string()
    };

    let ped = Ped::from_csv_str(&csv_content)?;
    let layout = ped.compute_layout(&LayoutOptions::default())?;
    let svg = ped.render_svg(&layout, &RenderOptions::default());

    if let Some(out_path) = cli.output {
        std::fs::write(&out_path, &svg)?;
        println!("Heredograma gravado com sucesso em: {}", out_path);
    } else {
        println!("Heredograma gerado com sucesso ({} bytes).", svg.len());
    }

    Ok(())
}
