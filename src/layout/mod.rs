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

        // 3. Atribui coordenadas (X, Y)
        let mut coords = HashMap::new();
        let mut max_x: f64 = 100.0;
        let max_gen = gen_groups.keys().copied().max().unwrap_or(1);

        for (gen, ids) in &gen_groups {
            let y = options.top_margin + ((*gen as f64) - 1.0) * options.y_spacing;
            for (i, id) in ids.iter().enumerate() {
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
                            children,
                        });
                    }
                }
            }
        }

        let width = max_x + options.left_margin + 50.0;
        let height = options.top_margin + (max_gen as f64) * options.y_spacing + 120.0;

        Self {
            coords,
            unions,
            width,
            height,
        }
    }
}
