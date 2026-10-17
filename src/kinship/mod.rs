use crate::core::Ped;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct KinshipMatrix {
    pub order: Vec<String>,
    id_to_idx: HashMap<String, usize>,
    matrix: Vec<Vec<f64>>,
}

impl KinshipMatrix {
    pub fn get(&self, id1: &str, id2: &str) -> Option<f64> {
        let i = self.id_to_idx.get(id1)?;
        let j = self.id_to_idx.get(id2)?;
        Some(self.matrix[*i][*j])
    }

    pub fn inbreeding(&self, id: &str) -> Option<f64> {
        let i = self.id_to_idx.get(id)?;
        let phi_ii = self.matrix[*i][*i];
        Some((2.0 * phi_ii - 1.0).max(0.0))
    }

    pub fn size(&self) -> usize {
        self.order.len()
    }

    pub fn values(&self) -> &Vec<Vec<f64>> {
        &self.matrix
    }
}

pub fn compute_kinship_matrix(ped: &Ped) -> KinshipMatrix {
    // 1. Ordenação topológica (pais sempre antes de filhos)
    let mut in_degree: HashMap<&str, usize> = HashMap::new();
    let mut children_map: HashMap<&str, Vec<&str>> = HashMap::new();

    for id in &ped.order {
        in_degree.entry(id.as_str()).or_insert(0);
        let ind = &ped.members[id];
        if let Some(ref f) = ind.fid {
            *in_degree.entry(id.as_str()).or_insert(0) += 1;
            children_map
                .entry(f.as_str())
                .or_default()
                .push(id.as_str());
        }
        if let Some(ref m) = ind.mid {
            *in_degree.entry(id.as_str()).or_insert(0) += 1;
            children_map
                .entry(m.as_str())
                .or_default()
                .push(id.as_str());
        }
    }

    let mut queue: std::collections::VecDeque<&str> = ped
        .order
        .iter()
        .map(|s| s.as_str())
        .filter(|&id| in_degree.get(id).copied().unwrap_or(0) == 0)
        .collect();

    let mut topo_order: Vec<String> = Vec::with_capacity(ped.len());
    while let Some(curr) = queue.pop_front() {
        topo_order.push(curr.to_string());
        if let Some(kids) = children_map.get(curr) {
            for &kid in kids {
                if let Some(deg) = in_degree.get_mut(kid) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(kid);
                    }
                }
            }
        }
    }

    // Se houver nós restantes (desconexos sem pais), anexa na ordem
    for id in &ped.order {
        if !topo_order.contains(id) {
            topo_order.push(id.clone());
        }
    }

    let n = topo_order.len();
    let id_to_idx: HashMap<String, usize> = topo_order
        .iter()
        .enumerate()
        .map(|(i, id)| (id.clone(), i))
        .collect();

    let mut matrix = vec![vec![0.0; n]; n];

    // 2. Algoritmo recursivo de Malécot / Lange
    for i in 0..n {
        let id_i = &topo_order[i];
        let ind_i = &ped.members[id_i];
        let p1 = ind_i.fid.as_ref().and_then(|f| id_to_idx.get(f)).copied();
        let p2 = ind_i.mid.as_ref().and_then(|m| id_to_idx.get(m)).copied();

        for j in 0..=i {
            if i == j {
                // Auto-kinship Phi_ii
                if let (Some(f_idx), Some(m_idx)) = (p1, p2) {
                    let parent_kinship = matrix[f_idx][m_idx];
                    matrix[i][i] = 0.5 * (1.0 + parent_kinship);
                } else {
                    matrix[i][i] = 0.5;
                }
            } else {
                // Kinship entre i e j
                let val = match (p1, p2) {
                    (Some(f_idx), Some(m_idx)) => 0.5 * matrix[f_idx][j] + 0.5 * matrix[m_idx][j],
                    (Some(f_idx), None) => 0.5 * matrix[f_idx][j],
                    (None, Some(m_idx)) => 0.5 * matrix[m_idx][j],
                    (None, None) => 0.0,
                };
                matrix[i][j] = val;
                matrix[j][i] = val;
            }
        }
    }

    KinshipMatrix {
        order: topo_order,
        id_to_idx,
        matrix,
    }
}

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
