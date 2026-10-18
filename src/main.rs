use clap::Parser;
use joaju_pedigree::prelude::*;

#[derive(Parser, Debug)]
#[command(
    name = "joaju-pedigree",
    version,
    about = "A high-performance clinical pedigree and kinship engine in Rust"
)]
struct Cli {
    #[arg(short, long)]
    input: Option<String>,

    #[arg(short, long)]
    output: Option<String>,

    #[arg(short, long, help = "Enable automatic generation of clinical legend")]
    legend: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    println!("joaju-pedigree v{} initialized.", env!("CARGO_PKG_VERSION"));

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
        println!("Pedigree written successfully to: {}", out_path);
    } else {
        println!("Pedigree generated successfully ({} bytes).", svg.len());
    }

    Ok(())
}
