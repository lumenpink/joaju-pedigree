use pedigree_rs::prelude::*;
use std::collections::HashSet;

#[test]
fn test_csv_parser_and_founders() {
    let csv_data = "id,pai,mae,sexo,status,probando,aff\n\
                    1,0,0,1,0,0,0\n\
                    2,0,0,2,0,0,0\n\
                    3,1,2,1,0,1,1\n\
                    4,1,2,2,0,0,0\n";

    let ped = Ped::from_csv_str(csv_data).expect("Falha ao parsear CSV");
    assert_eq!(ped.len(), 4);

    let founders: Vec<&str> = ped
        .founders()
        .into_iter()
        .map(|ind| ind.id.as_str())
        .collect();
    assert_eq!(founders, vec!["1", "2"]);

    let ind3 = ped.get("3").expect("Indivíduo 3 não encontrado");
    assert_eq!(ind3.sex, Sex::Male);
    assert_eq!(ind3.status, Status::Alive);
    assert!(ind3.proband);
    assert_eq!(ind3.affected, 1);
}

#[test]
fn test_cycle_detection_error() {
    // 1 é pai de 2, e 2 é pai de 1 (ciclo biologicamente impossível)
    let csv_data = "id,pai,mae,sexo\n\
                    1,2,0,1\n\
                    2,1,0,1\n";

    let err = Ped::from_csv_str(csv_data);
    assert!(err.is_err(), "Deveria detectar ciclo genealógico");
}

#[test]
fn test_kinship_queries() {
    let csv_data = "id,fid,mid,sex\n\
                    10,0,0,1\n\
                    20,0,0,2\n\
                    30,10,20,1\n\
                    40,10,20,2\n\
                    50,0,0,2\n\
                    60,30,50,1\n";

    let ped = Ped::from_csv_str(csv_data).expect("CSV válido");

    let children = ped.children("10");
    let mut expected_children = HashSet::new();
    expected_children.insert("30");
    expected_children.insert("40");
    let actual_children: HashSet<&str> = children.iter().copied().collect();
    assert_eq!(actual_children, expected_children);

    let spouses = ped.spouses("10");
    assert_eq!(spouses, vec!["20"]);

    let sibs = ped.siblings("30");
    assert_eq!(sibs, vec!["40"]);

    let ancestors = ped.ancestors("60");
    assert!(ancestors.contains("30"));
    assert!(ancestors.contains("50"));
    assert!(ancestors.contains("10"));
    assert!(ancestors.contains("20"));
    assert_eq!(ancestors.len(), 4);
}

#[test]
fn test_consanguinity_detection() {
    // Avós comuns (1 e 2). Filhos (3 e 4) são irmãos.
    // 3 casa com 5 (externo) e tem 7.
    // 4 casa com 6 (externo) e tem 8.
    // 7 e 8 são primos de primeiro grau e casam entre si, gerando 9.
    let csv_data = "id,pai,mae,sexo\n\
                    1,0,0,1\n\
                    2,0,0,2\n\
                    3,1,2,1\n\
                    4,1,2,2\n\
                    5,0,0,2\n\
                    6,0,0,1\n\
                    7,3,5,1\n\
                    8,6,4,2\n\
                    9,7,8,1\n";

    let ped = Ped::from_csv_str(csv_data).expect("CSV consanguíneo válido");
    assert!(
        ped.is_consanguineous("7", "8"),
        "Primos deveriam ser detectados como consanguíneos"
    );
    assert!(
        !ped.is_consanguineous("1", "2"),
        "Fundadores não são consanguíneos"
    );
}

#[test]
fn test_generational_depth_layout() {
    let csv_data = "id,fid,mid,sex\n\
                    1,0,0,1\n\
                    2,0,0,2\n\
                    3,1,2,1\n\
                    4,0,0,2\n\
                    5,3,4,1\n";

    let ped = Ped::from_csv_str(csv_data).expect("CSV válido");
    let layout = ped
        .compute_layout(&LayoutOptions::default())
        .expect("Layout computado");

    // Gerações esperadas:
    // 1 e 2 = Geração 1
    // 3 e 4 = Geração 2 (4 se alinha com 3)
    // 5 = Geração 3
    assert_eq!(layout.generation_of("1"), Some(1));
    assert_eq!(layout.generation_of("2"), Some(1));
    assert_eq!(layout.generation_of("3"), Some(2));
    assert_eq!(layout.generation_of("4"), Some(2));
    assert_eq!(layout.generation_of("5"), Some(3));
}

#[test]
fn test_clinical_svg_render() {
    let csv_data = "id,pai,mae,sexo,status,probando,aff,carrier\n\
                    1,0,0,1,1,0,0,0\n\
                    2,0,0,2,0,0,0,1\n\
                    3,1,2,1,0,1,1,0\n\
                    4,1,2,2,0,0,0,0\n";

    let ped = Ped::from_csv_str(csv_data).expect("CSV válido");
    let layout = ped
        .compute_layout(&LayoutOptions::default())
        .expect("Layout");
    let svg = ped.render_svg(
        &layout,
        &RenderOptions {
            show_id: true,
            card_legend: true,
            ..Default::default()
        },
    );

    // Verificações no SVG gerado:
    assert!(svg.contains("<svg"), "Deve conter abertura de tag svg");
    assert!(svg.contains("</svg>"), "Deve conter fechamento de tag svg");
    assert!(
        svg.contains("<rect"),
        "Deve desenhar retângulos para homens"
    );
    assert!(
        svg.contains("<circle"),
        "Deve desenhar círculos para mulheres"
    );
    // Seta de probando
    assert!(
        svg.contains("proband-arrow") || svg.contains(">P<"),
        "Deve renderizar seta/marcador de probando"
    );
    // Marcação de falecido (linha diagonal)
    assert!(
        svg.contains("deceased-slash"),
        "Deve conter corte diagonal de falecido"
    );
    // Card de legenda
    assert!(
        svg.contains("LEGENDA") || svg.contains("card-legend"),
        "Deve conter o card de legenda vetorial"
    );
}

#[test]
fn test_mutation_singleton_pedigree() {
    let csv_data = "id,pai,mae,sexo\n\
                    99,0,0,0\n";
    let ped = Ped::from_csv_str(csv_data).expect("Singleton válido");
    assert_eq!(ped.len(), 1);
    assert!(ped.founders().iter().any(|f| f.id == "99"));

    let layout = ped
        .compute_layout(&LayoutOptions::default())
        .expect("Layout");
    let svg = ped.render_svg(&layout, &RenderOptions::default());
    assert!(
        svg.contains("<polygon class=\"sym\""),
        "Sexo desconhecido deve ser losango"
    );
}

#[test]
fn test_mutation_half_siblings() {
    // Pai 1 tem filho 3 com Mãe 2, e filho 5 com Mãe 4
    let csv_data = "id,pai,mae,sexo\n\
                    1,0,0,1\n\
                    2,0,0,2\n\
                    3,1,2,1\n\
                    4,0,0,2\n\
                    5,1,4,2\n";
    let ped = Ped::from_csv_str(csv_data).expect("CSV válido");
    let sibs_of_3 = ped.siblings("3");
    assert!(sibs_of_3.contains(&"5"), "5 é meio-irmão de 3 pelo pai 1");

    let spouses_of_1 = ped.spouses("1");
    assert_eq!(spouses_of_1.len(), 2, "Pai 1 deve ter 2 parceiras (2 e 4)");
}
