use pedigree_rs::prelude::*;

#[test]
fn test_spec_nuclear_kinship_values() {
    let ped = Ped::nuclear(2).expect("nuclear ped");
    let km = ped.kinship_matrix();

    // Auto-parentesco de fundadores = 0.5
    assert!((km.get("1", "1").unwrap() - 0.5).abs() < 1e-6);
    assert!((km.get("2", "2").unwrap() - 0.5).abs() < 1e-6);

    // Fundadores não aparentados = 0.0
    assert!((km.get("1", "2").unwrap() - 0.0).abs() < 1e-6);

    // Pai-filho = 0.25 (1/4)
    assert!((km.get("1", "3").unwrap() - 0.25).abs() < 1e-6);
    assert!((km.get("2", "3").unwrap() - 0.25).abs() < 1e-6);

    // Irmãos plenos = 0.25 (1/4)
    assert!((km.get("3", "4").unwrap() - 0.25).abs() < 1e-6);

    // Coeficiente de inbreeding dos filhos = 0.0
    assert!((km.inbreeding("3").unwrap() - 0.0).abs() < 1e-6);
}

#[test]
fn test_spec_half_siblings_kinship() {
    let ped = Ped::half_sibs(1, 1).expect("half sibs");
    let km = ped.kinship_matrix();

    // 4 e 5 são meio-irmãos (compartilham o pai 2)
    // Kinship entre meio-irmãos = 0.125 (1/8)
    assert!((km.get("4", "5").unwrap() - 0.125).abs() < 1e-6);
}

#[test]
fn test_spec_first_cousins_kinship() {
    let ped = Ped::cousins(1).expect("cousins 1");
    let km = ped.kinship_matrix();

    // 7 e 8 são primos de primeiro grau
    // Kinship entre primos = 0.0625 (1/16)
    assert!((km.get("7", "8").unwrap() - 0.0625).abs() < 1e-6);
}

#[test]
fn test_spec_avuncular_kinship() {
    let ped = Ped::avuncular().expect("avuncular");
    let km = ped.kinship_matrix();

    // 3 é tio de 6 (sobrinha)
    // Kinship tio-sobrinha = 0.125 (1/8)
    assert!((km.get("3", "6").unwrap() - 0.125).abs() < 1e-6);
}

#[test]
fn test_spec_double_cousins_kinship() {
    let ped = Ped::double_cousins(1, 1).expect("double cousins");
    let km = ped.kinship_matrix();

    // 9 e 10 são primos duplos
    // Kinship primos duplos = 0.125 (1/8)
    assert!((km.get("9", "10").unwrap() - 0.125).abs() < 1e-6);
}

#[test]
fn test_spec_inbreeding_wright_coefficient() {
    // Casamento entre primos gerando filho consanguíneo
    let mut ped = Ped::cousins(1).expect("cousins 1");
    // Primos 7 (M) e 8 (F) têm um filho 9
    ped.add(Individual::child("9", "7", "8", Sex::Male))
        .unwrap();

    let km = ped.kinship_matrix();
    // Coeficiente de consanguinidade Wright de 9 = kinship dos pais (7 e 8) = 0.0625 (1/16)
    assert!((km.inbreeding("9").unwrap() - 0.0625).abs() < 1e-6);
    // Auto-kinship de 9 = 0.5 * (1 + F) = 0.5 * 1.0625 = 0.53125
    assert!((km.get("9", "9").unwrap() - 0.53125).abs() < 1e-6);
}
