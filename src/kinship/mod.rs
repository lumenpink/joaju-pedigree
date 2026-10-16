use crate::core::Ped;
use std::collections::HashSet;

pub fn find_children<'a>(ped: &'a Ped, id: &str) -> Vec<&'a str> {
    ped.order
        .iter()
        .filter_map(|child_id| ped.members.get(child_id))
        .filter(|child| child.fid.as_deref() == Some(id) || child.mid.as_deref() == Some(id))
        .map(|child| child.id.as_str())
        .collect()
}

pub fn find_parents<'a>(ped: &'a Ped, id: &str) -> (Option<&'a str>, Option<&'a str>) {
    if let Some(ind) = ped.members.get(id) {
        (ind.fid.as_deref(), ind.mid.as_deref())
    } else {
        (None, None)
    }
}

pub fn find_spouses<'a>(ped: &'a Ped, id: &str) -> Vec<&'a str> {
    let mut spouses = Vec::new();
    for child_id in &ped.order {
        if let Some(child) = ped.members.get(child_id) {
            if child.fid.as_deref() == Some(id) {
                if let Some(ref m) = child.mid {
                    if !spouses.contains(&m.as_str()) {
                        spouses.push(m.as_str());
                    }
                }
            } else if child.mid.as_deref() == Some(id) {
                if let Some(ref f) = child.fid {
                    if !spouses.contains(&f.as_str()) {
                        spouses.push(f.as_str());
                    }
                }
            }
        }
    }
    spouses
}

pub fn find_siblings<'a>(ped: &'a Ped, id: &str) -> Vec<&'a str> {
    let mut sibs = Vec::new();
    let (my_fid, my_mid) = find_parents(ped, id);
    if my_fid.is_none() && my_mid.is_none() {
        return sibs;
    }

    for other_id in &ped.order {
        if other_id == id {
            continue;
        }
        let (other_fid, other_mid) = find_parents(ped, other_id);
        let same_fid = my_fid.is_some() && my_fid == other_fid;
        let same_mid = my_mid.is_some() && my_mid == other_mid;
        if same_fid || same_mid {
            sibs.push(other_id.as_str());
        }
    }
    sibs
}

pub fn find_ancestors<'a>(ped: &'a Ped, id: &str) -> HashSet<&'a str> {
    let mut ancestors = HashSet::new();
    let mut stack = vec![id];

    while let Some(curr) = stack.pop() {
        let (fid, mid) = find_parents(ped, curr);
        if let Some(f) = fid {
            if ancestors.insert(f) {
                stack.push(f);
            }
        }
        if let Some(m) = mid {
            if ancestors.insert(m) {
                stack.push(m);
            }
        }
    }

    ancestors
}

pub fn is_consanguineous(ped: &Ped, id1: &str, id2: &str) -> bool {
    let a1 = find_ancestors(ped, id1);
    let a2 = find_ancestors(ped, id2);
    a1.intersection(&a2).next().is_some()
}
