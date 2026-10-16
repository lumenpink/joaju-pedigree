use pedigreers::prelude::*;

#[test]
fn test_fenotipo_and_status_synonyms() {
    let csv_data = "individual,father,mother,sex,status,probanda,fenotipo\n\
                    I1,0,0,MALE,falecido,s,hom\n\
                    I2,0,0,FEMALE,alive,n,het\n\
                    II1,I1,I2,1,0,p,suspeita\n\
                    II2,I1,I2,2,0,0,normal\n";

    let (ped, _) = Ped::from_text(csv_data).expect("Parse de sinônimos");
    assert_eq!(ped.len(), 4);

    let i1 = ped.get("I1").unwrap();
    assert_eq!(i1.status, Status::Deceased);
    assert!(i1.proband);
    assert_eq!(i1.affected, 1);

    let i2 = ped.get("I2").unwrap();
    assert_eq!(i2.status, Status::Alive);
    assert!(!i2.proband);
    assert!(i2.carrier);

    let ii1 = ped.get("II1").unwrap();
    assert!(ii1.proband);
    assert!(ii1.suspect);

    let ii2 = ped.get("II2").unwrap();
    assert_eq!(ii2.affected, 0);
    assert!(!ii2.carrier);
    assert!(!ii2.suspect);
}

#[test]
fn test_directives_and_explicit_joaju_legend() {
    let block_data = "%% legend = c(\"Normal M\", \"Normal F\", \"Afetado M\", \"Heterozigota\", \"Investigação\", \"Falecido\")\n\
                      %% legend.type = c(\"empty_m\", \"empty_f\", \"sq_solid\", \"circ_half\", \"circ_suspect\", \"dec\")\n\
                      id,pai,mae,sexo\n\
                      1,0,0,1\n\
                      2,0,0,2\n";

    let (ped, directives) = Ped::from_text(block_data).expect("Parse de bloco com diretivas");
    assert_eq!(ped.len(), 2);
    assert!(directives.has_legend());

    let layout = ped.compute_layout(&LayoutOptions::default()).expect("Layout");
    let mut render_opts = RenderOptions::default();
    render_opts.legend = Some(directives.to_legend_config());

    let svg = ped.render_svg(&layout, &render_opts);
    assert!(svg.contains("Normal M"));
    assert!(svg.contains("Heterozigota"));
    assert!(svg.contains("Investigação"));
    assert!(svg.contains("LEGENDA CLÍNICA"));
}

#[test]
fn test_auto_legend_shortcut() {
    let csv_data = "id,pai,mae,sexo,status,probando,aff,carrier\n\
                    1,0,0,1,1,0,0,0\n\
                    2,0,0,2,0,0,0,1\n\
                    3,1,2,1,0,1,1,0\n";

    let (ped, _) = Ped::from_text(csv_data).expect("CSV válido");
    let layout = ped.compute_layout(&LayoutOptions::default()).expect("Layout");

    let mut render_opts = RenderOptions::default();
    render_opts.auto_legend = true;

    let svg = ped.render_svg(&layout, &render_opts);
    // Como a família tem Homem, Mulher, Afetado, Portador, Falecido e Probando, todos devem estar na legenda automaticamente:
    assert!(svg.contains("Homem"));
    assert!(svg.contains("Mulher"));
    assert!(svg.contains("Afetado"));
    assert!(svg.contains("Portador"));
    assert!(svg.contains("Falecido"));
    assert!(svg.contains("Probando"));
}
