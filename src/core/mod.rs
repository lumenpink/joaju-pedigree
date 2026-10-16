use crate::kinship;
use crate::layout::{LayoutOptions, PedLayout};
use crate::render::{render_svg, RenderOptions};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PedError {
    #[error("Indivíduo duplicado: {0}")]
    DuplicateId(String),
    #[error("Indivíduo não encontrado: {0}")]
    NotFound(String),
    #[error("Ciclo parental inválido detectado para {0}")]
    CyclicPedigree(String),
    #[error("Erro de parse CSV: {0}")]
    CsvError(#[from] csv::Error),
    #[error("Erro de I/O: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Sex {
    #[default]
    Unknown = 0,
    Male = 1,
    Female = 2,
}

impl Sex {
    pub fn parse_str(s: &str) -> Self {
        let clean = s.trim().to_uppercase();
        match clean.as_str() {
            "1" | "M" | "MALE" | "MAS" => Sex::Male,
            "2" | "F" | "FEMALE" | "FEM" => Sex::Female,
            _ => Sex::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Status {
    #[default]
    Alive = 0,
    Deceased = 1,
}

impl Status {
    pub fn parse_str(s: &str) -> Self {
        let clean = s.trim().to_uppercase();
        match clean.as_str() {
            "1" | "D" | "DECEASED" | "FALECIDO" | "TRUE" => Status::Deceased,
            _ => Status::Alive,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Individual {
    pub id: String,
    pub fid: Option<String>,
    pub mid: Option<String>,
    pub sex: Sex,
    pub status: Status,
    pub proband: bool,
    pub affected: u8,
    pub carrier: bool,
    pub suspect: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ped {
    pub members: HashMap<String, Individual>,
    pub order: Vec<String>,
}

impl Ped {
    pub fn new() -> Self {
        Self {
            members: HashMap::new(),
            order: Vec::new(),
        }
    }

    pub fn add(&mut self, ind: Individual) -> Result<(), PedError> {
        if self.members.contains_key(&ind.id) {
            return Err(PedError::DuplicateId(ind.id));
        }
        self.order.push(ind.id.clone());
        self.members.insert(ind.id.clone(), ind);
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&Individual> {
        self.members.get(id)
    }

    pub fn len(&self) -> usize {
        self.members.len()
    }

    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    pub fn founders(&self) -> Vec<&Individual> {
        self.order
            .iter()
            .filter_map(|id| self.members.get(id))
            .filter(|ind| ind.fid.is_none() && ind.mid.is_none())
            .collect()
    }

    pub fn from_csv_str(csv_data: &str) -> Result<Self, PedError> {
        let mut rdr = csv::ReaderBuilder::new()
            .trim(csv::Trim::All)
            .flexible(true)
            .from_reader(csv_data.as_bytes());

        let headers = rdr.headers()?.clone();
        let col_map: HashMap<String, usize> = headers
            .iter()
            .enumerate()
            .map(|(i, h)| (h.trim().to_lowercase(), i))
            .collect();

        let mut ped = Ped::new();

        let get_val = |record: &csv::StringRecord, candidates: &[&str]| -> Option<String> {
            for &c in candidates {
                if let Some(&idx) = col_map.get(c) {
                    if let Some(v) = record.get(idx) {
                        let t = v.trim();
                        if !t.is_empty() && t != "0" && t.to_uppercase() != "NA" {
                            return Some(t.to_string());
                        }
                    }
                }
            }
            None
        };

        for result in rdr.records() {
            let record = result?;
            let id = if let Some(v) = get_val(&record, &["id", "individual"]) {
                v
            } else {
                continue;
            };

            let fid = get_val(&record, &["fid", "pai", "dadid", "father"]);
            let mid = get_val(&record, &["mid", "mae", "momid", "mother"]);

            let sex_str = get_val(&record, &["sex", "sexo"]).unwrap_or_default();
            let sex = Sex::parse_str(&sex_str);

            let status_str = get_val(&record, &["status", "falecido", "deceased"]).unwrap_or_default();
            let status = Status::parse_str(&status_str);

            let proband = get_val(&record, &["proband", "probando"])
                .map(|v| v == "1" || v.to_uppercase() == "TRUE")
                .unwrap_or(false);

            let affected = get_val(&record, &["affected", "aff"])
                .and_then(|v| v.parse::<u8>().ok())
                .unwrap_or(0);

            let carrier = get_val(&record, &["carrier", "portador"])
                .map(|v| v == "1" || v.to_uppercase() == "TRUE")
                .unwrap_or(false);

            let suspect = get_val(&record, &["suspect", "suspeito"])
                .map(|v| v == "1" || v.to_uppercase() == "TRUE")
                .unwrap_or(false);

            ped.add(Individual {
                id,
                fid,
                mid,
                sex,
                status,
                proband,
                affected,
                carrier,
                suspect,
            })?;
        }

        // Validação de ciclos (DAG check)
        ped.validate_acyclic()?;

        Ok(ped)
    }

    fn validate_acyclic(&self) -> Result<(), PedError> {
        let mut fully_explored = HashSet::new();
        let mut on_stack = HashSet::new();

        fn dfs<'a>(
            ped: &'a Ped,
            curr: &'a str,
            fully_explored: &mut HashSet<&'a str>,
            on_stack: &mut HashSet<&'a str>,
        ) -> Result<(), PedError> {
            if on_stack.contains(curr) {
                return Err(PedError::CyclicPedigree(curr.to_string()));
            }
            if fully_explored.contains(curr) {
                return Ok(());
            }

            on_stack.insert(curr);
            if let Some(ind) = ped.members.get(curr) {
                if let Some(ref fid) = ind.fid {
                    dfs(ped, fid.as_str(), fully_explored, on_stack)?;
                }
                if let Some(ref mid) = ind.mid {
                    dfs(ped, mid.as_str(), fully_explored, on_stack)?;
                }
            }
            on_stack.remove(curr);
            fully_explored.insert(curr);
            Ok(())
        }

        for id in &self.order {
            dfs(self, id.as_str(), &mut fully_explored, &mut on_stack)?;
        }

        Ok(())
    }

    // Kinship helpers
    pub fn children<'a>(&'a self, id: &str) -> Vec<&'a str> {
        kinship::find_children(self, id)
    }

    pub fn spouses<'a>(&'a self, id: &str) -> Vec<&'a str> {
        kinship::find_spouses(self, id)
    }

    pub fn siblings<'a>(&'a self, id: &str) -> Vec<&'a str> {
        kinship::find_siblings(self, id)
    }

    pub fn ancestors<'a>(&'a self, id: &str) -> HashSet<&'a str> {
        kinship::find_ancestors(self, id)
    }

    pub fn is_consanguineous(&self, id1: &str, id2: &str) -> bool {
        kinship::is_consanguineous(self, id1, id2)
    }

    // Layout & Render helpers
    pub fn compute_layout(&self, options: &LayoutOptions) -> Result<PedLayout, PedError> {
        Ok(PedLayout::compute(self, options))
    }

    pub fn render_svg(&self, layout: &PedLayout, options: &RenderOptions) -> String {
        render_svg(self, layout, options)
    }
}
