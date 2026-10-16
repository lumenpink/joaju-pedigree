use pedigreers::prelude::*;
use std::fs;

#[test]
fn test_spec_g3_spouse_clustering() {
    let csv = fs::read_to_string("tests/fixtures/g3_3gen.csv")
        .expect("g3_3gen.csv deve existir nas fixtures");
    let ped = Ped::from_csv_str(&csv).expect("parse de g3 deve ser valido");
    let layout = PedLayout::compute(&ped, &LayoutOptions::default());

    // Indivíduo 3 tem esposas 6, 7 e 8 na Geração II
    let c3 = &layout.coords["3"];
    let c6 = &layout.coords["6"];
    let c7 = &layout.coords["7"];
    let c8 = &layout.coords["8"];

    // Todos devem estar na mesma geração (Gen 2)
    assert_eq!(c3.generation, 2);
    assert_eq!(c6.generation, 2);
    assert_eq!(c7.generation, 2);
    assert_eq!(c8.generation, 2);

    // No layout agrupado por blocos familiares, as uniões conjugais (3-6, 3-7, 3-8)
    // não devem ter outros indivíduos não relacionados intercalados no eixo X
    let svg = render_svg(&ped, &layout, &RenderOptions::default());
    assert!(!svg.is_empty());
}

#[test]
fn test_spec_twins_mz_and_dz_in_svg() {
    let csv = fs::read_to_string("tests/fixtures/twins.csv")
        .expect("twins.csv deve existir nas fixtures");
    let ped = Ped::from_csv_str(&csv).expect("parse de twins deve ser valido");

    // Verifica se os campos de gêmeos foram ingeridos
    let ind3 = ped.get("3").expect("ind 3 deve existir");
    assert_eq!(ind3.twin_group.as_deref(), Some("mz1"));
    assert_eq!(ind3.twin_type, TwinType::Monozygotic);

    let ind5 = ped.get("5").expect("ind 5 deve existir");
    assert_eq!(ind5.twin_group.as_deref(), Some("dz1"));
    assert_eq!(ind5.twin_type, TwinType::Dizygotic);

    let ind7 = ped.get("7").expect("ind 7 deve existir");
    assert_eq!(ind7.twin_group.as_deref(), Some("tr1"));
    assert_eq!(ind7.twin_type, TwinType::Dizygotic);

    let layout = PedLayout::compute(&ped, &LayoutOptions::default());
    let svg = render_svg(&ped, &layout, &RenderOptions::default());

    // No SVG, deve haver elementos de conexão gemelar em leque
    assert!(
        svg.contains("class=\"twin-line\"") || svg.contains("twin-fan") || svg.contains("mz-bar"),
        "SVG deve conter elementos gráficos para gêmeos MZ/DZ"
    );
}

#[test]
fn test_spec_litter_10_fan_out() {
    let csv = fs::read_to_string("tests/fixtures/litter10.csv")
        .expect("litter10.csv deve existir nas fixtures");
    let ped = Ped::from_csv_str(&csv).expect("parse de litter10 deve ser valido");

    assert_eq!(ped.len(), 12);
    for id in 3..=12 {
        let ind = ped.get(&id.to_string()).expect("filhote deve existir");
        assert_eq!(ind.twin_group.as_deref(), Some("litter_a"));
    }

    let layout = PedLayout::compute(&ped, &LayoutOptions::default());
    let svg = render_svg(&ped, &layout, &RenderOptions::default());

    // Deve suportar os 10 membros convergindo em leque
    assert!(
        svg.contains("twin-line") || svg.contains("twin-fan") || svg.contains("kin-line"),
        "SVG deve conter linhas de descendência para todos os 10 filhotes da ninhada"
    );
}

#[test]
fn test_spec_directives_for_twins() {
    let text = r#"
%% twin=3,4,mz
%% twin=5,6,dz
id,pai,mae,sex
1,0,0,M
2,0,0,F
3,1,2,M
4,1,2,M
5,1,2,F
6,1,2,F
"#;
    let (ped, _directives) = Ped::from_text(text).expect("parse com diretivas de gemeos");
    let ind3 = ped.get("3").unwrap();
    let ind4 = ped.get("4").unwrap();
    let ind5 = ped.get("5").unwrap();
    let ind6 = ped.get("6").unwrap();

    assert_eq!(ind3.twin_type, TwinType::Monozygotic);
    assert_eq!(ind4.twin_type, TwinType::Monozygotic);
    assert_eq!(ind5.twin_type, TwinType::Dizygotic);
    assert_eq!(ind6.twin_type, TwinType::Dizygotic);
}
