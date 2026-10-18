use joaju_pedigree::prelude::*;

#[test]
fn test_spec_multiline_labels() {
    let csv = r#"id,pai,mae,sex,label
1,0,0,M,I-1\n[Normal]\n45a
2,0,0,F,I-2\n[Afetada]\n43a
3,1,2,M,II-1\n[Azul bb]\n12a
"#;
    let ped = Ped::from_csv_str(csv).expect("parse multiline labels");
    let layout = PedLayout::compute(&ped, &LayoutOptions::default());
    let svg = render_svg(&ped, &layout, &RenderOptions::default());

    assert!(svg.contains("I-1"), "SVG deve conter a 1ª linha do rótulo");
    assert!(
        svg.contains("[Normal]"),
        "SVG deve conter a 2ª linha do rótulo"
    );
    assert!(svg.contains("45a"), "SVG deve conter a 3ª linha do rótulo");
}

#[test]
fn test_spec_abortion_triangle_symbol() {
    let csv = r#"id,pai,mae,sex,status,aborto,fenotipo
1,0,0,M,0,0,wt
2,0,0,F,0,0,wt
3,1,2,U,1,espontaneo,wt
4,1,2,U,1,induzido,afetado
"#;
    let ped = Ped::from_csv_str(csv).expect("parse abortion symbols");
    let ind3 = ped.get("3").unwrap();
    let ind4 = ped.get("4").unwrap();

    assert!(
        ind3.is_pregnancy_loss(),
        "3 deve ser classificado como perda gestacional/aborto"
    );
    assert!(
        ind4.is_pregnancy_loss(),
        "4 deve ser classificado como perda gestacional/aborto"
    );

    let layout = PedLayout::compute(&ped, &LayoutOptions::default());
    let svg = render_svg(&ped, &layout, &RenderOptions::default());

    assert!(
        svg.contains("<polygon"),
        "SVG deve renderizar triângulo para aborto/perda gestacional"
    );
    assert!(
        svg.contains("class=\"sym-triangle"),
        "SVG deve conter classe sym-triangle"
    );
}

#[test]
fn test_spec_adoption_brackets() {
    let csv = r#"id,pai,mae,sex,adocao
1,0,0,M,none
2,0,0,F,none
3,1,2,M,in
4,1,2,F,out
"#;
    let ped = Ped::from_csv_str(csv).expect("parse adoption");
    let ind3 = ped.get("3").unwrap();
    let ind4 = ped.get("4").unwrap();

    assert_eq!(ind3.adoption, AdoptionType::In);
    assert_eq!(ind4.adoption, AdoptionType::Out);

    let layout = PedLayout::compute(&ped, &LayoutOptions::default());
    let svg = render_svg(&ped, &layout, &RenderOptions::default());

    assert!(
        svg.contains("class=\"adopted-bracket\""),
        "SVG deve conter colchetes de adocao"
    );
}

#[test]
fn test_spec_divorce_marker() {
    let text = r#"
%% divorce=1,2
id,pai,mae,sex
1,0,0,M
2,0,0,F
3,1,2,M
"#;
    let (ped, _dir) = Ped::from_text(text).expect("parse com diretiva de divorcio");
    let layout = PedLayout::compute(&ped, &LayoutOptions::default());
    let svg = render_svg(&ped, &layout, &RenderOptions::default());

    assert!(
        svg.contains("divorce-slash"),
        "SVG deve conter barra dupla de divorcio"
    );
}

#[test]
fn test_spec_childless_t_stem() {
    let text = r#"
%% childless=1,2,choice
id,pai,mae,sex
1,0,0,M
2,0,0,F
"#;
    let (ped, _dir) = Ped::from_text(text).expect("parse childless");
    let layout = PedLayout::compute(&ped, &LayoutOptions::default());
    let svg = render_svg(&ped, &layout, &RenderOptions::default());

    assert!(
        svg.contains("childless-bar"),
        "SVG deve conter terminador em T para casal sem filhos"
    );
}

#[test]
fn test_mutation_multiline_label_spacing() {
    let text = "id,pai,mae,sex,label\n1,0,0,M,L1\\nL2\\nL3\\nL4\\nL5\n";
    let ped = Ped::from_csv_str(text).unwrap();
    let layout = PedLayout::compute(&ped, &LayoutOptions::default());
    let svg = render_svg(&ped, &layout, &RenderOptions::default());

    assert!(svg.contains("L1"));
    assert!(svg.contains("L5"));
}
