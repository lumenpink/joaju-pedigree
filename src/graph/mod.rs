use crate::core::{AdoptionType, Individual, Ped, PedError, PregnancyLossType, TwinType};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InbreedingLoop {
    pub child_id: String,
    pub father_id: String,
    pub mother_id: String,
    pub common_ancestors: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoopBreaker {
    pub original: String,
    pub duplicate: String,
}

pub fn find_inbreeding_loops(ped: &Ped) -> Vec<InbreedingLoop> {
    let mut loops = Vec::new();

    for id in &ped.order {
        let ind = &ped.members[id];
        if let (Some(ref fid), Some(ref mid)) = (&ind.fid, &ind.mid) {
            let mut anc_f = ped.ancestors(fid);
            anc_f.insert(fid.as_str());

            let mut anc_m = ped.ancestors(mid);
            anc_m.insert(mid.as_str());

            let common: Vec<String> = anc_f.intersection(&anc_m).map(|s| s.to_string()).collect();

            if !common.is_empty() {
                loops.push(InbreedingLoop {
                    child_id: id.clone(),
                    father_id: fid.clone(),
                    mother_id: mid.clone(),
                    common_ancestors: common,
                });
            }
        }
    }

    loops
}

pub fn has_inbreeding_loops(ped: &Ped) -> bool {
    !find_inbreeding_loops(ped).is_empty()
}

pub fn break_loops(ped: &Ped) -> Result<(Ped, Vec<LoopBreaker>), PedError> {
    let mut current_ped = ped.clone();
    let mut breakers = Vec::new();

    while has_inbreeding_loops(&current_ped) {
        let loops = find_inbreeding_loops(&current_ped);
        if loops.is_empty() {
            break;
        }
        let lp = &loops[0];

        let breaker_id = if current_ped.members[&lp.father_id].fid.is_some()
            || current_ped.members[&lp.father_id].mid.is_some()
        {
            lp.father_id.clone()
        } else {
            lp.mother_id.clone()
        };

        let dup_id = format!("{}_dup", breaker_id);
        let orig = &current_ped.members[&breaker_id];
        let dup_ind = Individual {
            id: dup_id.clone(),
            fid: None,
            mid: None,
            sex: orig.sex,
            status: orig.status,
            proband: false,
            affected: orig.affected,
            carrier: orig.carrier,
            suspect: orig.suspect,
            twin_group: None,
            twin_type: TwinType::None,
            label: orig.label.as_ref().map(|l| format!("{} (dup)", l)),
            pregnancy_loss: PregnancyLossType::None,
            adoption: AdoptionType::None,
        };
        current_ped.add(dup_ind)?;

        for id in &current_ped.order.clone() {
            let ind = current_ped.members.get_mut(id).unwrap();
            if ind.fid.as_deref() == Some(&breaker_id) && ind.mid.as_deref() == Some(&lp.mother_id)
            {
                ind.fid = Some(dup_id.clone());
            } else if ind.mid.as_deref() == Some(&breaker_id)
                && ind.fid.as_deref() == Some(&lp.father_id)
            {
                ind.mid = Some(dup_id.clone());
            }
        }

        breakers.push(LoopBreaker {
            original: breaker_id,
            duplicate: dup_id,
        });
    }

    Ok((current_ped, breakers))
}

pub fn peeling_order(ped: &Ped) -> Result<Vec<String>, PedError> {
    if has_inbreeding_loops(ped) {
        return Err(PedError::LoopyPedigree(
            "Pedigree possui loops de consanguinidade. Execute break_loops() primeiro.".into(),
        ));
    }

    let mut remaining: HashSet<String> = ped.order.iter().cloned().collect();
    let mut order = Vec::with_capacity(ped.len());

    while !remaining.is_empty() {
        let parents_in_remaining: HashSet<&str> = remaining
            .iter()
            .flat_map(|id| {
                let ind = &ped.members[id];
                vec![ind.fid.as_deref(), ind.mid.as_deref()]
            })
            .flatten()
            .collect();

        let mut leaves: Vec<String> = remaining
            .iter()
            .filter(|id| !parents_in_remaining.contains(id.as_str()))
            .cloned()
            .collect();

        if leaves.is_empty() {
            leaves = remaining.iter().take(1).cloned().collect();
        }

        leaves.sort();

        for leaf in leaves {
            remaining.remove(&leaf);
            order.push(leaf);
        }
    }

    Ok(order)
}
