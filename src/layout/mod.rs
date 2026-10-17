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
            x_spacing: 65.0,
            y_spacing: 95.0,
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

        // 3. Atribui coordenadas (X, Y) com agrupamento de cônjuges adjacentes
        let mut coords = HashMap::new();
        let mut max_x: f64 = 100.0;
        let max_gen = gen_groups.keys().copied().max().unwrap_or(1);

        for (gen, ids) in &gen_groups {
            let ordered_ids = order_generation_members(ped, ids);
            let y = options.top_margin + ((*gen as f64) - 1.0) * options.y_spacing;
            for (i, id) in ordered_ids.iter().enumerate() {
                let x = options.left_margin + (i as f64) * options.x_spacing;
                if x > max_x {
                    max_x = x;
                }
                coords.insert(
                    id.clone(),
                    NodeCoord {
                        id: id.clone(),
                        x,
                        y,
                        generation: *gen,
                    },
                );
            }
        }

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
