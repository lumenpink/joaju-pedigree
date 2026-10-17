use pedigreers::graph::{break_loops, find_inbreeding_loops, has_inbreeding_loops, peeling_order};
use pedigreers::prelude::*;

#[test]
fn test_spec_detect_inbreeding_loop_first_cousins() {
    // Pedigree nuclear não tem loops de consanguinidade
    let ped_nuclear = Ped::nuclear(2).expect("nuclear");
    assert!(!has_inbreeding_loops(&ped_nuclear));
    assert!(find_inbreeding_loops(&ped_nuclear).is_empty());

    // Pedigree com casamento entre primos de primeiro grau gerando um filho
    let mut ped_cousins = Ped::cousins(1).expect("cousins 1");
    assert!(!has_inbreeding_loops(&ped_cousins)); // Primos sem união consanguínea ainda não formam loop fechado

    // Agora os primos 7 (M) e 8 (F) têm um filho 9:
    ped_cousins
        .add(Individual::child("9", "7", "8", Sex::Male))
        .unwrap();

    assert!(has_inbreeding_loops(&ped_cousins));
    let loops = find_inbreeding_loops(&ped_cousins);
    assert!(!loops.is_empty(), "Deve identificar loop de consanguinidade no filho de primos");
    assert!(loops.iter().any(|l| l.child_id == "9"));
}

#[test]
fn test_spec_break_loops_transforms_to_acyclic_kinship() {
    let mut ped = Ped::cousins(1).expect("cousins 1");
    ped.add(Individual::child("9", "7", "8", Sex::Male)).unwrap();

    let (broken_ped, breakers) = break_loops(&ped).expect("break loops successfully");
    assert!(!breakers.is_empty(), "Deve encontrar pelo menos 1 loop breaker");
    assert!(!has_inbreeding_loops(&broken_ped), "Pedigree pós-break_loops não deve possuir loops de consanguinidade");

    // O indivíduo duplicado deve existir no novo pedigree como fundador
    let b = &breakers[0];
    let dup_ind = broken_ped.get(&b.duplicate).expect("duplicate node exists");
    assert!(dup_ind.fid.is_none() && dup_ind.mid.is_none(), "Duplicado deve ser fundador");
}

#[test]
fn test_spec_peeling_order_nuclear_pedigree() {
    let ped = Ped::nuclear(2).expect("nuclear");
    let order = peeling_order(&ped).expect("peeling order");

    // Todos os indivíduos devem estar presentes na ordem de peeling
    assert_eq!(order.len(), ped.len());
    // Filhos devem ser descascados (peeled) antes dos pais na ordem bottom-up
    let pos_child1 = order.iter().position(|id| id == "3").unwrap();
    let pos_father = order.iter().position(|id| id == "1").unwrap();
    assert!(pos_child1 < pos_father, "Filhos devem ser eliminados antes dos pais em peeling bottom-up");
}

#[test]
fn test_spec_peeling_order_errors_on_unbroken_loops() {
    let mut ped = Ped::cousins(1).expect("cousins 1");
    ped.add(Individual::child("9", "7", "8", Sex::Male)).unwrap();

    // Peeling direto em pedigree com loop deve retornar erro exigindo break_loops
    let result = peeling_order(&ped);
    assert!(result.is_err(), "Peeling direto em pedigree com loop deve falhar");
}

#[test]
fn test_mutation_peeling_after_break_loops() {
    let mut ped = Ped::cousins(1).expect("cousins 1");
    ped.add(Individual::child("9", "7", "8", Sex::Male)).unwrap();

    let (broken_ped, _) = break_loops(&ped).expect("break loops");
    let order = peeling_order(&broken_ped).expect("peeling order after break_loops");
    assert_eq!(order.len(), broken_ped.len());
}
