use crate::core::Ped;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Genotype {
    pub a1: Option<String>,
    pub a2: Option<String>,
}

impl Genotype {
    pub fn new(a1: Option<String>, a2: Option<String>) -> Self {
        Self { a1, a2 }
    }

    pub fn from_alleles(a1: &str, a2: &str) -> Self {
        Self {
            a1: Some(a1.to_string()),
            a2: Some(a2.to_string()),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.a1.is_none() && self.a2.is_none()
    }

    pub fn is_complete(&self) -> bool {
        self.a1.is_some() && self.a2.is_some()
    }

    pub fn contains(&self, allele: &str) -> bool {
        self.a1.as_deref() == Some(allele) || self.a2.as_deref() == Some(allele)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Marker {
    pub name: String,
    pub chrom: Option<String>,
    pub pos_mb: Option<f64>,
    pub alleles: Vec<String>,
    pub afreq: Vec<f64>,
    pub genotypes: HashMap<String, Genotype>,
}

impl Marker {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            chrom: None,
            pos_mb: None,
            alleles: Vec::new(),
            afreq: Vec::new(),
            genotypes: HashMap::new(),
        }
    }

    pub fn set_alleles(
        &mut self,
        alleles: Vec<String>,
        afreq: Option<Vec<f64>>,
    ) -> Result<(), String> {
        if let Some(freqs) = afreq {
            if freqs.len() != alleles.len() {
                return Err(format!(
                    "Número de frequências ({}) diferente de alelos ({})",
                    freqs.len(),
                    alleles.len()
                ));
            }
            let sum: f64 = freqs.iter().sum();
            if (sum - 1.0).abs() > 1e-4 {
                return Err(format!("Soma das frequências ({:.4}) deve ser 1.0", sum));
            }
            self.afreq = freqs;
        } else if !alleles.is_empty() {
            let eq = 1.0 / alleles.len() as f64;
            self.afreq = vec![eq; alleles.len()];
        }
        self.alleles = alleles;
        Ok(())
    }

    pub fn set_genotype(&mut self, id: &str, a1: &str, a2: &str) {
        self.genotypes
            .insert(id.to_string(), Genotype::from_alleles(a1, a2));
    }

    pub fn get_genotype(&self, id: &str) -> Option<&Genotype> {
        self.genotypes.get(id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MendelianError {
    pub marker_name: String,
    pub child_id: String,
    pub fid: Option<String>,
    pub mid: Option<String>,
    pub message: String,
}

pub fn check_marker_mendelian(ped: &Ped, marker: &Marker) -> Vec<MendelianError> {
    let mut errors = Vec::new();

    for id in &ped.order {
        let child = &ped.members[id];
        let child_gt = match marker.get_genotype(id) {
            Some(gt) if gt.is_complete() => gt,
            _ => continue,
        };

        let c1 = child_gt.a1.as_ref().unwrap();
        let c2 = child_gt.a2.as_ref().unwrap();

        let f_gt = child
            .fid
            .as_ref()
            .and_then(|fid| marker.get_genotype(fid))
            .filter(|gt| gt.is_complete());

        let m_gt = child
            .mid
            .as_ref()
            .and_then(|mid| marker.get_genotype(mid))
            .filter(|gt| gt.is_complete());

        match (f_gt, m_gt) {
            (Some(f), Some(m)) => {
                let f1 = f.a1.as_ref().unwrap();
                let f2 = f.a2.as_ref().unwrap();
                let m1 = m.a1.as_ref().unwrap();
                let m2 = m.a2.as_ref().unwrap();

                let valid = ((c1 == f1 || c1 == f2) && (c2 == m1 || c2 == m2))
                    || ((c2 == f1 || c2 == f2) && (c1 == m1 || c1 == m2));

                if !valid {
                    errors.push(MendelianError {
                        marker_name: marker.name.clone(),
                        child_id: id.clone(),
                        fid: child.fid.clone(),
                        mid: child.mid.clone(),
                        message: format!(
                            "Genótipo do filho ({}/{}) incompatível com pais: pai ({}/{}), mãe ({}/{})",
                            c1, c2, f1, f2, m1, m2
                        ),
                    });
                }
            }
            (Some(f), None) => {
                let f1 = f.a1.as_ref().unwrap();
                let f2 = f.a2.as_ref().unwrap();
                let valid = c1 == f1 || c1 == f2 || c2 == f1 || c2 == f2;

                if !valid {
                    errors.push(MendelianError {
                        marker_name: marker.name.clone(),
                        child_id: id.clone(),
                        fid: child.fid.clone(),
                        mid: child.mid.clone(),
                        message: format!(
                            "Genótipo do filho ({}/{}) não contém nenhum alelo do pai ({}/{})",
                            c1, c2, f1, f2
                        ),
                    });
                }
            }
            (None, Some(m)) => {
                let m1 = m.a1.as_ref().unwrap();
                let m2 = m.a2.as_ref().unwrap();
                let valid = c1 == m1 || c1 == m2 || c2 == m1 || c2 == m2;

                if !valid {
                    errors.push(MendelianError {
                        marker_name: marker.name.clone(),
                        child_id: id.clone(),
                        fid: child.fid.clone(),
                        mid: child.mid.clone(),
                        message: format!(
                            "Genótipo do filho ({}/{}) não contém nenhum alelo da mãe ({}/{})",
                            c1, c2, m1, m2
                        ),
                    });
                }
            }
            (None, None) => {}
        }
    }

    errors
}
