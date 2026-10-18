use joaju_pedigree::prelude::*;

#[test]
fn test_spec_marker_creation_and_genotypes() {
    let mut ped = Ped::nuclear(2).expect("nuclear ped");
    let mut marker = Marker::new("D1S80");
    marker
        .set_alleles(
            vec!["18".into(), "24".into(), "28".into()],
            Some(vec![0.2, 0.5, 0.3]),
        )
        .unwrap();

    marker.set_genotype("1", "18", "24");
    marker.set_genotype("2", "24", "28");
    marker.set_genotype("3", "18", "28"); // Herdado 18 do pai, 28 da mãe -> Válido
    marker.set_genotype("4", "24", "24"); // Herdado 24 do pai, 24 da mãe -> Válido

    ped.add_marker(marker).unwrap();

    let m = ped.marker("D1S80").expect("marker exists");
    assert_eq!(m.name, "D1S80");
    assert_eq!(
        m.get_genotype("1"),
        Some(&Genotype::from_alleles("18", "24"))
    );
    assert_eq!(
        m.get_genotype("3"),
        Some(&Genotype::from_alleles("18", "28"))
    );

    let errors = ped.mendelian_check();
    assert!(errors.is_empty(), "Esperado 0 erros de herança mendeliana");
}

#[test]
fn test_spec_mendelian_incompatibility_both_parents() {
    let mut ped = Ped::nuclear(1).expect("nuclear ped");
    // Pai: A/A, Mãe: B/B -> Filho deve ser A/B
    let mut marker = Marker::new("SNP_AB");
    marker.set_genotype("1", "A", "A");
    marker.set_genotype("2", "B", "B");
    marker.set_genotype("3", "A", "A"); // Impossível, mãe não tem alelo A

    ped.add_marker(marker).unwrap();

    let errors = ped.mendelian_check();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].child_id, "3");
    assert_eq!(errors[0].marker_name, "SNP_AB");
}

#[test]
fn test_spec_mendelian_incompatibility_single_parent() {
    // Pedigree onde apenas a mãe é conhecida/tipada
    let mut ped = Ped::new();
    ped.add(Individual::founder("2", Sex::Female)).unwrap();
    ped.add(Individual::child("3", "", "2", Sex::Male)).unwrap();

    let mut marker = Marker::new("SNP_M");
    marker.set_genotype("2", "A", "A");
    marker.set_genotype("3", "B", "B"); // Filho não tem nenhum alelo da mãe A

    ped.add_marker(marker).unwrap();

    let errors = ped.mendelian_check();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].child_id, "3");
}

#[test]
fn test_spec_mutation_allele_frequency_validation() {
    let mut marker = Marker::new("SNP_FREQ");
    // Frequências que não somam 1.0 devem retornar erro
    let err = marker.set_alleles(vec!["A".into(), "B".into()], Some(vec![0.3, 0.3]));
    assert!(
        err.is_err(),
        "Soma das frequências diferente de 1.0 deve falhar"
    );
}

#[test]
fn test_mutation_incomplete_genotype_no_false_errors() {
    let mut ped = Ped::nuclear(1).expect("nuclear ped");
    let mut marker = Marker::new("SNP_INC");
    // Pai não tipado, mãe tipada A/B, filho tipado B/? (incompleto)
    marker
        .genotypes
        .insert("2".into(), Genotype::from_alleles("A", "B"));
    marker
        .genotypes
        .insert("3".into(), Genotype::new(Some("B".into()), None));

    ped.add_marker(marker).unwrap();

    let errors = ped.mendelian_check();
    assert!(
        errors.is_empty(),
        "Genótipo incompleto não deve disparar falso positivo"
    );
}
