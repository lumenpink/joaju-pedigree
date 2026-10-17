use pedigreers::prelude::*;

#[test]
fn test_spec_nuclear_ped_builder() {
    let ped = Ped::nuclear(3).expect("geracao de nuclearPed");
    assert_eq!(ped.len(), 5);

    let father = ped.get("1").expect("pai 1 deve existir");
    let mother = ped.get("2").expect("mae 2 deve existir");
    assert_eq!(father.sex, Sex::Male);
    assert_eq!(mother.sex, Sex::Female);
    assert!(father.fid.is_none() && father.mid.is_none());
    assert!(mother.fid.is_none() && mother.mid.is_none());

    for id in ["3", "4", "5"] {
        let child = ped.get(id).expect("filho deve existir");
        assert_eq!(child.fid.as_deref(), Some("1"));
        assert_eq!(child.mid.as_deref(), Some("2"));
    }
}

#[test]
fn test_spec_half_sibs_builder() {
    let ped = Ped::half_sibs(2, 2).expect("geracao de halfSibPed");
    assert_eq!(ped.len(), 7);

    // Indivíduo 2 é o pai comum; 1 e 3 são as mães
    // 4 e 5 são filhos de (2, 1); 6 e 7 são filhos de (2, 3)
    let c4 = ped.get("4").unwrap();
    let c5 = ped.get("5").unwrap();
    let c6 = ped.get("6").unwrap();
    let c7 = ped.get("7").unwrap();

    assert_eq!(c4.fid.as_deref(), Some("2"));
    assert_eq!(c4.mid.as_deref(), Some("1"));
    assert_eq!(c5.fid.as_deref(), Some("2"));
    assert_eq!(c5.mid.as_deref(), Some("1"));

    assert_eq!(c6.fid.as_deref(), Some("2"));
    assert_eq!(c6.mid.as_deref(), Some("3"));
    assert_eq!(c7.fid.as_deref(), Some("2"));
    assert_eq!(c7.mid.as_deref(), Some("3"));
}

#[test]
fn test_spec_cousins_builder() {
    let ped = Ped::cousins(1).expect("geracao de cousinPed grau 1");
    // Primos de primeiro grau: avós (1, 2) -> irmãos (3, 4) casam com (5, 6) -> primos (7, 8)
    assert_eq!(ped.len(), 8);
    let c7 = ped.get("7").unwrap();
    let c8 = ped.get("8").unwrap();

    assert_eq!(c7.fid.as_deref(), Some("3"));
    assert_eq!(c7.mid.as_deref(), Some("5"));
    assert_eq!(c8.fid.as_deref(), Some("4"));
    assert_eq!(c8.mid.as_deref(), Some("6"));
}

#[test]
fn test_spec_avuncular_builder() {
    let ped = Ped::avuncular().expect("geracao de avuncularPed");
    // Avós (1, 2) -> filhos (3 [tio], 4 [mae]) -> mae 4 casa com fundador externo 5 -> filha 6 [sobrinha]
    // Tio 3 e sobrinha 6 têm filho comum ou união
    assert!(ped.len() >= 6);
}

#[test]
fn test_spec_linear_ped_builder() {
    let ped = Ped::linear(4).expect("geracao de linearPed 4 geracoes");
    // Cada geracao sucessiva tem um filho que casa com um novo fundador
    assert!(ped.len() >= 4);
    let layout = PedLayout::compute(&ped, &LayoutOptions::default());
    assert_eq!(layout.generation_of("1"), Some(1));
}

#[test]
fn test_spec_double_cousins_builder() {
    let ped = Ped::double_cousins(1, 1).expect("geracao de doubleCousins");
    // Dois irmãos casando com duas irmãs gerando primos duplos
    assert_eq!(ped.len(), 10);
}

#[test]
fn test_mutation_nuclear_scaling() {
    for n in 1..=20 {
        let ped = Ped::nuclear(n).unwrap();
        assert_eq!(ped.len(), 2 + n);
        assert_eq!(ped.founders().len(), 2);
    }
}
