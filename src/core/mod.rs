use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Status {
    #[default]
    Alive = 0,
    Deceased = 1,
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
}
