use crate::core::Ped;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone)]
pub struct LayoutOptions {
    pub symbol_size: f64,
    pub x_spacing: f64,
    pub y_spacing: f64,
    pub top_margin: f64,
    pub left_margin: f64,
}

impl Default for LayoutOptions {
    fn default() -> Self {
        Self {
            symbol_size: 28.0,
            x_spacing: 80.0,
            y_spacing: 100.0,
            top_margin: 50.0,
            left_margin: 50.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NodeCoord {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub generation: usize,
}

#[derive(Debug, Clone)]
pub struct UnionLine {
    pub p1_id: String,
    pub p2_id: String,
    pub x1: f64,
    pub x2: f64,
    pub y: f64,
    pub is_consanguineous: bool,
    pub is_divorced: bool,
    pub childless: Option<crate::core::ChildlessType>,
    pub children: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PedLayout {
    pub coords: HashMap<String, NodeCoord>,
    pub unions: Vec<UnionLine>,
    pub width: f64,
    pub height: f64,
}

impl PedLayout {
    pub fn generation_of(&self, id: &str) -> Option<usize> {
        self.coords.get(id).map(|c| c.generation)
    }

    pub fn compute(ped: &Ped, options: &LayoutOptions) -> Self {
        // 1. Cálculo de profundidade geracional (kindepth)
        let mut depths: HashMap<String, usize> = HashMap::new();

        // Inicializa fundadores em geração 1
        for id in &ped.order {
            let ind = &ped.members[id];
            if ind.fid.is_none() && ind.mid.is_none() {
                depths.insert(id.clone(), 1);
            }
        }

        // Propaga para filhos
        let mut changed = true;
        while changed {
            changed = false;
            for id in &ped.order {
                let ind = &ped.members[id];
                if let (Some(ref f), Some(ref m)) = (&ind.fid, &ind.mid) {
                    if let (Some(&df), Some(&dm)) = (depths.get(f), depths.get(m)) {
                        let expected = df.max(dm) + 1;
                        let curr = depths.get(id).copied().unwrap_or(0);
                        if curr < expected {
                            depths.insert(id.clone(), expected);
                            changed = true;
                        }
                    }
                } else if let Some(ref f) = ind.fid {
                    if let Some(&df) = depths.get(f) {
                        let expected = df + 1;
                        let curr = depths.get(id).copied().unwrap_or(0);
                        if curr < expected {
                            depths.insert(id.clone(), expected);
                            changed = true;
                        }
                    }
                } else if let Some(ref m) = ind.mid {
                    if let Some(&dm) = depths.get(m) {
                        let expected = dm + 1;
                        let curr = depths.get(id).copied().unwrap_or(0);
                        if curr < expected {
                            depths.insert(id.clone(), expected);
                            changed = true;
                        }
                    }
                }

                // Ajusta cônjuges para a mesma geração do parceiro
                let my_depth = depths.get(id).copied().unwrap_or(1);
                for spouse in ped.spouses(id) {
                    let s_depth = depths.get(spouse).copied().unwrap_or(0);
                    if s_depth < my_depth {
                        depths.insert(spouse.to_string(), my_depth);
                        changed = true;
                    }
                }
            }
        }

        // Garante que todo indivíduo possui geração atribuída
        for id in &ped.order {
            depths.entry(id.clone()).or_insert(1);
        }

        // 2. Agrupa por geração mantendo a ordem estável
        let mut gen_groups: BTreeMap<usize, Vec<String>> = BTreeMap::new();
        for id in &ped.order {
            let g = depths[id];
            gen_groups.entry(g).or_default().push(id.clone());
        }

        // 3. Atribui coordenadas (X, Y) com centralização de fratria e cônjuges
        let mut coords: HashMap<String, NodeCoord> = HashMap::new();
        let max_gen = gen_groups.keys().copied().max().unwrap_or(1);

        // Mapa de ordem de cada geração para uso nos passes
        let mut gen_ordered_ids: BTreeMap<usize, Vec<String>> = BTreeMap::new();

        // Pass 1: Top-Down (posiciona nós orientados aos pais na geração anterior)
        for (gen, ids) in &gen_groups {
            let sorted_ids = if *gen > 1 {
                sort_by_parent_position(ped, ids, &coords)
            } else {
                ids.clone()
            };
            let ordered_ids = order_generation_members(ped, &sorted_ids);
            let y = options.top_margin + ((*gen as f64) - 1.0) * options.y_spacing;

            // Agrupa por irmãos (mesmo pai e mãe)
            let mut sib_groups: HashMap<(Option<String>, Option<String>), Vec<String>> =
                HashMap::new();
            for id in &ordered_ids {
                let ind = &ped.members[id];
                if ind.fid.is_some() || ind.mid.is_some() {
                    sib_groups
                        .entry((ind.fid.clone(), ind.mid.clone()))
                        .or_default()
                        .push(id.clone());
                }
            }

            // Atribui posições alvo
            let mut target_xs: HashMap<String, f64> = HashMap::new();
            for (parents, sibs) in &sib_groups {
                let k = sibs.len() as f64;
                let p_mid = match parents {
                    (Some(f), Some(m)) => match (coords.get(f), coords.get(m)) {
                        (Some(cf), Some(cm)) => (cf.x + cm.x) / 2.0,
                        (Some(cf), None) => cf.x,
                        (None, Some(cm)) => cm.x,
                        _ => options.left_margin,
                    },
                    (Some(f), None) => coords.get(f).map(|c| c.x).unwrap_or(options.left_margin),
                    (None, Some(m)) => coords.get(m).map(|c| c.x).unwrap_or(options.left_margin),
                    _ => options.left_margin,
                };

                for (j, sib_id) in sibs.iter().enumerate() {
                    let offset = (j as f64 - (k - 1.0) / 2.0) * options.x_spacing;
                    target_xs.insert(sib_id.clone(), p_mid + offset);
                }
            }

            // Para cônjuges ou fundadores sem pais
            for (i, id) in ordered_ids.iter().enumerate() {
                if !target_xs.contains_key(id) {
                    let mut found_spouse_x = None;
                    for spouse in ped.spouses(id) {
                        if let Some(&sx) = target_xs.get(spouse) {
                            let s_idx = ordered_ids.iter().position(|x| x == spouse).unwrap_or(i);
                            if i < s_idx {
                                found_spouse_x = Some(sx - options.x_spacing);
                            } else {
                                found_spouse_x = Some(sx + options.x_spacing);
                            }
                            break;
                        }
                    }
                    let tx = found_spouse_x
                        .unwrap_or(options.left_margin + (i as f64) * options.x_spacing);
                    target_xs.insert(id.clone(), tx);
                }
            }

            // Alinha da esquerda para a direita garantindo x_{i} >= x_{i-1} + x_spacing
            let mut curr_x = f64::NEG_INFINITY;
            for id in &ordered_ids {
                let ideal_x = target_xs[id];
                let actual_x = if curr_x.is_infinite() {
                    ideal_x
                } else {
                    ideal_x.max(curr_x + options.x_spacing)
                };
                curr_x = actual_x;
                coords.insert(
                    id.clone(),
                    NodeCoord {
                        id: id.clone(),
                        x: actual_x,
                        y,
                        generation: *gen,
                    },
                );
            }

            gen_ordered_ids.insert(*gen, ordered_ids);
        }

        // Pass 2: Bottom-Up Centering (centraliza pais sobre o centro de seus filhos)
        for gen in (1..max_gen).rev() {
            if let Some(ids) = gen_ordered_ids.get(&gen) {
                let mut target_xs: HashMap<String, f64> = HashMap::new();

                let mut seen_pairs = std::collections::HashSet::new();
                for id in ids {
                    for spouse in ped.spouses(id) {
                        let key = if id.as_str() < spouse {
                            format!("{}:{}", id, spouse)
                        } else {
                            format!("{}:{}", spouse, id)
                        };
                        if seen_pairs.insert(key) {
                            let children: Vec<String> = ped
                                .children(id)
                                .into_iter()
                                .filter(|&c| {
                                    let ind = &ped.members[c];
                                    (ind.fid.as_deref() == Some(id)
                                        && ind.mid.as_deref() == Some(spouse))
                                        || (ind.fid.as_deref() == Some(spouse)
                                            && ind.mid.as_deref() == Some(id))
                                })
                                .map(|s| s.to_string())
                                .collect();

                            let child_xs: Vec<f64> = children
                                .iter()
                                .filter_map(|cid| coords.get(cid).map(|c| c.x))
                                .collect();

                            if !child_xs.is_empty() {
                                let min_c = child_xs.iter().copied().fold(f64::INFINITY, f64::min);
                                let max_c =
                                    child_xs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                                let children_mid = (min_c + max_c) / 2.0;

                                let idx_id = ids.iter().position(|x| x == id).unwrap_or(0);
                                let idx_sp = ids.iter().position(|x| x == spouse).unwrap_or(1);
                                if idx_id < idx_sp {
                                    target_xs
                                        .insert(id.clone(), children_mid - options.x_spacing / 2.0);
                                    target_xs.insert(
                                        spouse.to_string(),
                                        children_mid + options.x_spacing / 2.0,
                                    );
                                } else {
                                    target_xs.insert(
                                        spouse.to_string(),
                                        children_mid - options.x_spacing / 2.0,
                                    );
                                    target_xs
                                        .insert(id.clone(), children_mid + options.x_spacing / 2.0);
                                }
                            }
                        }
                    }
                }

                if !target_xs.is_empty() {
                    let mut curr_x = f64::NEG_INFINITY;
                    for id in ids {
                        let ideal_x = target_xs.get(id).copied().unwrap_or_else(|| coords[id].x);
                        let actual_x = if curr_x.is_infinite() {
                            ideal_x
                        } else {
                            ideal_x.max(curr_x + options.x_spacing)
                        };
                        curr_x = actual_x;
                        if let Some(nc) = coords.get_mut(id) {
                            nc.x = actual_x;
                        }
                    }
                }
            }
        }

        // Pass 3: Global Left Margin Normalization
        let min_x = coords.values().map(|c| c.x).fold(f64::INFINITY, f64::min);
        let shift = if min_x < options.left_margin {
            options.left_margin - min_x
        } else {
            0.0
        };

        if shift > 0.0 {
            for c in coords.values_mut() {
                c.x += shift;
            }
        }

        let max_x = coords.values().map(|c| c.x).fold(100.0, f64::max);

        // 4. Identifica e constrói linhas de união e consanguinidade
        let mut unions = Vec::new();
        let mut seen_unions = std::collections::HashSet::new();

        for child_id in &ped.order {
            let child = &ped.members[child_id];
            if let (Some(ref f), Some(ref m)) = (&child.fid, &child.mid) {
                let key = if f < m {
                    format!("{}:{}", f, m)
                } else {
                    format!("{}:{}", m, f)
                };

                if seen_unions.insert(key) {
                    if let (Some(cf), Some(cm)) = (coords.get(f), coords.get(m)) {
                        let is_consanguineous = ped.is_consanguineous(f, m);
                        let is_divorced = ped.is_divorced(f, m);
                        let childless = ped.childless_status(f, m);
                        let children: Vec<String> = ped
                            .children(f)
                            .into_iter()
                            .filter(|&c| ped.members[c].mid.as_deref() == Some(m))
                            .map(|s| s.to_string())
                            .collect();

                        unions.push(UnionLine {
                            p1_id: f.clone(),
                            p2_id: m.clone(),
                            x1: cf.x,
                            x2: cm.x,
                            y: cf.y,
                            is_consanguineous,
                            is_divorced,
                            childless,
                            children,
                        });
                    }
                }
            }
        }

        // Uniões de divórcio ou sem filhos adicionais sem filhos no grafo
        for (p1, p2) in ped.divorces.iter().chain(ped.childless.keys()) {
            let key = if p1 < p2 {
                format!("{}:{}", p1, p2)
            } else {
                format!("{}:{}", p2, p1)
            };
            if seen_unions.insert(key) {
                if let (Some(c1), Some(c2)) = (coords.get(p1), coords.get(p2)) {
                    unions.push(UnionLine {
                        p1_id: p1.clone(),
                        p2_id: p2.clone(),
                        x1: c1.x,
                        x2: c2.x,
                        y: c1.y,
                        is_consanguineous: ped.is_consanguineous(p1, p2),
                        is_divorced: ped.is_divorced(p1, p2),
                        childless: ped.childless_status(p1, p2),
                        children: Vec::new(),
                    });
                }
            }
        }

        let width = (max_x + options.left_margin + 50.0).max(440.0);
        let height = options.top_margin + (max_gen as f64) * options.y_spacing + 120.0;

        Self {
            coords,
            unions,
            width,
            height,
        }
    }
}

fn order_generation_members(ped: &Ped, ids: &[String]) -> Vec<String> {
    let id_set: std::collections::HashSet<&str> = ids.iter().map(|s| s.as_str()).collect();
    let mut spouse_map: HashMap<String, Vec<String>> = HashMap::new();

    for id in ids {
        for spouse in ped.spouses(id) {
            if id_set.contains(spouse) {
                spouse_map
                    .entry(id.clone())
                    .or_default()
                    .push(spouse.to_string());
            }
        }
    }

    let mut visited: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut ordered: Vec<String> = Vec::new();

    for id in ids {
        if visited.contains(id) {
            continue;
        }

        let spouses = spouse_map.get(id).cloned().unwrap_or_default();
        let unvisited_spouses: Vec<String> = spouses
            .into_iter()
            .filter(|s| !visited.contains(s))
            .collect();

        if unvisited_spouses.is_empty() {
            visited.insert(id.clone());
            ordered.push(id.clone());
        } else if unvisited_spouses.len() == 1 {
            let s = &unvisited_spouses[0];
            visited.insert(id.clone());
            visited.insert(s.clone());
            if ped.members[id].sex == crate::core::Sex::Male {
                ordered.push(id.clone());
                ordered.push(s.clone());
            } else {
                ordered.push(s.clone());
                ordered.push(id.clone());
            }
        } else {
            visited.insert(id.clone());
            for s in &unvisited_spouses {
                visited.insert(s.clone());
            }

            let mid = unvisited_spouses.len() / 2;
            let left_spouses = &unvisited_spouses[..mid];
            let right_spouses = &unvisited_spouses[mid..];

            for s in left_spouses {
                ordered.push(s.clone());
            }
            ordered.push(id.clone());
            for s in right_spouses {
                ordered.push(s.clone());
            }
        }
    }

    ordered
}

fn sort_by_parent_position(
    ped: &Ped,
    ids: &[String],
    coords: &HashMap<String, NodeCoord>,
) -> Vec<String> {
    let mut with_pos: Vec<(f64, String)> = ids
        .iter()
        .map(|id| {
            let ind = &ped.members[id];
            let p_pos = match (&ind.fid, &ind.mid) {
                (Some(f), Some(m)) => match (coords.get(f), coords.get(m)) {
                    (Some(cf), Some(cm)) => (cf.x + cm.x) / 2.0,
                    (Some(cf), None) => cf.x,
                    (None, Some(cm)) => cm.x,
                    _ => f64::INFINITY,
                },
                (Some(f), None) => coords.get(f).map(|c| c.x).unwrap_or(f64::INFINITY),
                (None, Some(m)) => coords.get(m).map(|c| c.x).unwrap_or(f64::INFINITY),
                _ => f64::INFINITY,
            };
            (p_pos, id.clone())
        })
        .collect();

    for i in 0..with_pos.len() {
        if with_pos[i].0.is_infinite() {
            let id = &with_pos[i].1;
            for spouse in ped.spouses(id) {
                if let Some(sp_entry) = with_pos.iter().find(|(_, sid)| sid == spouse) {
                    if !sp_entry.0.is_infinite() {
                        with_pos[i].0 = sp_entry.0 + 0.1;
                        break;
                    }
                }
            }
        }
    }

    with_pos.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    with_pos.into_iter().map(|(_, id)| id).collect()
}
