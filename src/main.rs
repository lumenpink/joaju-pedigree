use clap::Parser;
use pedigreers::core::{Individual, Ped, Sex, Status};
use pedigreers::layout::{LayoutOptions, PedLayout};
use pedigreers::render::{render_svg, RenderOptions};

#[derive(Parser, Debug)]
#[command(
    name = "pedigreers",
    version,
    about = "Motor de heredogramas clínicos em Rust"
)]
struct Cli {
    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _cli = Cli::parse();
    println!("pedigreers v{} inicializado.", env!("CARGO_PKG_VERSION"));

    let mut ped = Ped::new();
    ped.add(Individual {
        id: "1".into(),
        fid: None,
        mid: None,
        sex: Sex::Male,
        status: Status::Alive,
        proband: false,
        affected: 0,
    })?;
    ped.add(Individual {
        id: "2".into(),
        fid: None,
        mid: None,
        sex: Sex::Female,
        status: Status::Alive,
        proband: false,
        affected: 0,
    })?;
    ped.add(Individual {
        id: "3".into(),
        fid: Some("1".into()),
        mid: Some("2".into()),
        sex: Sex::Female,
        status: Status::Alive,
        proband: true,
        affected: 1,
    })?;

    let layout = PedLayout::compute(&ped, &LayoutOptions::default());
    let svg = render_svg(&ped, &layout, &RenderOptions::default());
    println!("SVG de exemplo gerado com sucesso ({} bytes).", svg.len());

    Ok(())
}
