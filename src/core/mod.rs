pub mod builders;
pub mod directives;
use crate::kinship;
use crate::layout::{LayoutOptions, PedLayout};
use crate::render::{render_svg, RenderOptions};
pub use directives::{extract_directives, PedDirectives};
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
            "1" | "M" | "MALE" | "MAS" | "HOMEM" => Sex::Male,
            "2" | "F" | "FEMALE" | "FEM" | "MULHER" => Sex::Female,
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
            "1" | "D" | "DECEASED" | "DEAD" | "FALECIDO" | "SIM" | "S" | "TRUE" => Status::Deceased,
            _ => Status::Alive,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TwinType {
    #[default]
    None = 0,
    Monozygotic = 1,
    Dizygotic = 2,
    Unknown = 3,
}

impl TwinType {
    pub fn parse_str(s: &str) -> Self {
        let clean = s.trim().to_lowercase();
        match clean.as_str() {
            "1" | "mz" | "monozygotic" | "monozigotico" | "identico" => TwinType::Monozygotic,
            "2" | "dz" | "dizygotic" | "dizigotico" | "fraterno" => TwinType::Dizygotic,
            "3" | "?" | "unknown" | "desconhecido" => TwinType::Unknown,
            _ => TwinType::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PregnancyLossType {
    #[default]
    None = 0,
    Spontaneous = 1,
    Induced = 2,
}

impl PregnancyLossType {
    pub fn parse_str(s: &str) -> Self {
        let clean = s.trim().to_lowercase();
        match clean.as_str() {
            "1" | "sim" | "true" | "s" | "espontaneo" | "spontaneous" | "aborto"
            | "miscarriage" => PregnancyLossType::Spontaneous,
            "2" | "induzido" | "interrupcao" | "induced" | "terminado" | "terminated" => {
                PregnancyLossType::Induced
            }
            _ => PregnancyLossType::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AdoptionType {
    #[default]
    None = 0,
    In = 1,
    Out = 2,
}

impl AdoptionType {
    pub fn parse_str(s: &str) -> Self {
        let clean = s.trim().to_lowercase();
        match clean.as_str() {
            "in" | "dentro" | "adotado" | "adopted" | "1" => AdoptionType::In,
            "out" | "fora" | "cedido" | "2" => AdoptionType::Out,
            _ => AdoptionType::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ChildlessType {
    #[default]
    None = 0,
    ByChoice = 1,
    Infertility = 2,
}

impl ChildlessType {
    pub fn parse_str(s: &str) -> Self {
        let clean = s.trim().to_lowercase();
        match clean.as_str() {
            "choice" | "opcao" | "sem_filhos" | "1" => ChildlessType::ByChoice,
            "infertility" | "infertil" | "esteril" | "2" => ChildlessType::Infertility,
            _ => ChildlessType::None,
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
    pub twin_group: Option<String>,
    pub twin_type: TwinType,
    pub label: Option<String>,
    pub pregnancy_loss: PregnancyLossType,
    pub adoption: AdoptionType,
}

impl Individual {
    pub fn is_pregnancy_loss(&self) -> bool {
        self.pregnancy_loss != PregnancyLossType::None
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ped {
    pub members: HashMap<String, Individual>,
    pub order: Vec<String>,
    pub divorces: HashSet<(String, String)>,
    pub childless: HashMap<(String, String), ChildlessType>,
}

impl Ped {
    pub fn new() -> Self {
        Self {
            members: HashMap::new(),
            order: Vec::new(),
            divorces: HashSet::new(),
            childless: HashMap::new(),
        }
    }

    pub fn is_divorced(&self, p1: &str, p2: &str) -> bool {
        self.divorces.contains(&(p1.to_string(), p2.to_string()))
            || self.divorces.contains(&(p2.to_string(), p1.to_string()))
    }

    pub fn childless_status(&self, p1: &str, p2: &str) -> Option<ChildlessType> {
        self.childless
            .get(&(p1.to_string(), p2.to_string()))
            .or_else(|| self.childless.get(&(p2.to_string(), p1.to_string())))
            .copied()
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
        Self::from_text(csv_data).map(|(ped, _)| ped)
    }

    pub fn from_text(text: &str) -> Result<(Self, PedDirectives), PedError> {
        let (directives, clean_csv) = extract_directives(text);

        let mut rdr = csv::ReaderBuilder::new()
            .trim(csv::Trim::All)
            .flexible(true)
            .from_reader(clean_csv.as_bytes());

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

            let status_str =
                get_val(&record, &["status", "falecido", "deceased"]).unwrap_or_default();
            let status = Status::parse_str(&status_str);

            let proband_val = get_val(&record, &["proband", "probando", "probanda"])
                .unwrap_or_default()
                .to_uppercase();
            let proband = matches!(
                proband_val.as_str(),
                "1" | "SIM" | "S" | "TRUE" | "PROBAND" | "PROBANDO" | "PROBANDA" | "P"
            );

            let mut affected: u8 = 0;
            let mut carrier: bool = false;
            let mut suspect: bool = false;

            // Análise da coluna fenotipo / afetado / affected
            if let Some(pheno) = get_val(&record, &["fenotipo", "afetado", "affected", "aff"]) {
                let p_clean = pheno.to_lowercase();
                if matches!(
                    p_clean.as_str(),
                    "1" | "hom"
                        | "homozigoto"
                        | "afetado"
                        | "doente"
                        | "mut"
                        | "affected"
                        | "yes"
                        | "sim"
                        | "s"
                ) {
                    affected = 1;
                } else if matches!(
                    p_clean.as_str(),
                    "het" | "heterozigoto" | "portador" | "carrier"
                ) {
                    carrier = true;
                } else if matches!(p_clean.as_str(), "?" | "suspeita" | "suspect" | "duvida") {
                    suspect = true;
                }
            }

            // Sobrescrita se houver colunas dedicadas
            if let Some(c_val) = get_val(&record, &["carrier", "portador"]) {
                let c_clean = c_val.to_uppercase();
                if matches!(c_clean.as_str(), "1" | "SIM" | "S" | "TRUE" | "HET") {
                    carrier = true;
                }
            }

            if let Some(s_val) = get_val(&record, &["suspect", "suspeito"]) {
                let s_clean = s_val.to_uppercase();
                if matches!(s_clean.as_str(), "1" | "SIM" | "S" | "TRUE" | "?") {
                    suspect = true;
                }
            }

            let twin_group = get_val(
                &record,
                &[
                    "twin_group",
                    "twin",
                    "gemelar",
                    "gemeo",
                    "parto",
                    "litter",
                    "relation",
                ],
            );
            let mut twin_type = get_val(&record, &["twin_type", "zygosity", "zigosidade"])
                .map(|t| TwinType::parse_str(&t))
                .unwrap_or(TwinType::None);

            if let Some(ref tg) = twin_group {
                if twin_type == TwinType::None {
                    if tg.to_lowercase().starts_with("mz") {
                        twin_type = TwinType::Monozygotic;
                    } else {
                        twin_type = TwinType::Dizygotic;
                    }
                }
            }

            let label = get_val(&record, &["label", "rotulo", "nome", "name"]);
            let preg_str = get_val(
                &record,
                &[
                    "aborto",
                    "abortion",
                    "perda",
                    "pregnancy_loss",
                    "interrupcao",
                ],
            )
            .unwrap_or_default();
            let pregnancy_loss = PregnancyLossType::parse_str(&preg_str);

            let adocao_str =
                get_val(&record, &["adocao", "adoption", "adotado", "adopted"]).unwrap_or_default();
            let adoption = AdoptionType::parse_str(&adocao_str);

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
                twin_group,
                twin_type,
                label,
                pregnancy_loss,
                adoption,
            })?;
        }

        // Aplica diretivas de gêmeos (%% twin / %% litter)
        for (idx, tw_dir) in directives.twins.iter().enumerate() {
            let group_name = format!("tw_dir_{}", idx);
            for id in &tw_dir.ids {
                if let Some(ind) = ped.members.get_mut(id) {
                    ind.twin_group = Some(group_name.clone());
                    ind.twin_type = tw_dir.twin_type;
                }
            }
        }

        // Aplica diretivas de divórcio (%% divorce=1,2)
        for (p1, p2) in &directives.divorces {
            ped.divorces.insert((p1.clone(), p2.clone()));
        }

        // Aplica diretivas de childless (%% childless=1,2,choice)
        for (p1, p2, c_type) in &directives.childless {
            ped.childless.insert((p1.clone(), p2.clone()), *c_type);
        }

        // Validação de ciclos (DAG check com 3-color DFS)
        ped.validate_acyclic()?;

        Ok((ped, directives))
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

    pub fn kinship_matrix(&self) -> kinship::KinshipMatrix {
        kinship::compute_kinship_matrix(self)
    }

    pub fn kinship_coeff(&self, id1: &str, id2: &str) -> Option<f64> {
        self.kinship_matrix().get(id1, id2)
    }

    pub fn inbreeding_coeff(&self, id: &str) -> Option<f64> {
        self.kinship_matrix().inbreeding(id)
    }

    // Layout & Render helpers
    pub fn compute_layout(&self, options: &LayoutOptions) -> Result<PedLayout, PedError> {
        Ok(PedLayout::compute(self, options))
    }

    pub fn render_svg(&self, layout: &PedLayout, options: &RenderOptions) -> String {
        render_svg(self, layout, options)
    }
}
