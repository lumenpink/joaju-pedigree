use clap::Parser;
use pedigree_rs::prelude::*;

#[derive(Parser, Debug)]
#[command(
    name = "pedigree-rs",
    version,
    about = "Motor soberano de heredogramas clínicos em Rust"
)]
struct Cli {
    #[arg(short, long)]
    input: Option<String>,

    #[arg(short, long)]
    output: Option<String>,

    #[arg(short, long, help = "Ativar geração automática de legenda clínica")]
    legend: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    println!("pedigree-rs v{} inicializado.", env!("CARGO_PKG_VERSION"));

    let raw_content = if let Some(path) = cli.input {
        std::fs::read_to_string(path)?
    } else {
        "id,pai,mae,sexo,status,probando,aff\n\
         1,0,0,1,0,0,0\n\
         2,0,0,2,0,0,0\n\
         3,1,2,1,0,1,1\n"
            .to_string()
    };

    let (ped, directives) = Ped::from_text(&raw_content)?;
    let layout = ped.compute_layout(&LayoutOptions::default())?;

    let mut render_opts = RenderOptions::default();
    if directives.has_legend() {
        render_opts.legend = Some(directives.to_legend_config());
        render_opts.card_legend = true;
    } else if cli.legend {
        render_opts.auto_legend = true;
        render_opts.card_legend = true;
    }

    let svg = ped.render_svg(&layout, &render_opts);

    if let Some(out_path) = cli.output {
        std::fs::write(&out_path, &svg)?;
        println!("Heredograma gravado com sucesso em: {}", out_path);
    } else {
        println!("Heredograma gerado com sucesso ({} bytes).", svg.len());
    }

    Ok(())
}
