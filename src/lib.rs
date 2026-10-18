//! # pedigree-rs
//!
//! Motor soberano em Rust para manipulação de pedigrees, cálculo de parentesco
//! e renderização de heredogramas clínicos conformes com as diretrizes NSGC/ACMG.
//!
//! > ⚠️ **Aviso de Versão em Desenvolvimento Ativo (pre-1.0 / dev)**: Esta biblioteca
//! > está em fase inicial de desenvolvimento ativo (`0.1.x`). A API pública pode
//! > sofrer ajustes antes da versão estável 1.0.0.
//!
//! ## Exemplo Rápido
//!
//! ```rust
//! use pedigree_rs::prelude::*;
//!
//! let ped = Ped::nuclear(2).unwrap();
//! assert_eq!(ped.len(), 4);
//!
//! let kin = KinshipMatrix::compute(&ped);
//! assert!((kin.get("1", "3").unwrap() - 0.25).abs() < 1e-6);
//! ```

pub mod core;
pub mod genetics;
pub mod graph;
pub mod kinship;
pub mod layout;
pub mod prelude;
pub mod render;

pub use crate::core::{
    AdoptionType, ChildlessType, Individual, Ped, PregnancyLossType, Sex, Status, TwinType,
};
pub use crate::genetics::{Genotype, Marker, MendelianError};
pub use crate::graph::{
    break_loops, find_inbreeding_loops, has_inbreeding_loops, peeling_order, InbreedingLoop,
    LoopBreaker,
};
pub use crate::kinship::{compute_kinship_matrix, KinshipMatrix};
pub use crate::layout::{LayoutOptions, PedLayout};
pub use crate::render::{render_svg, ClinicalSymbol, RenderOptions};

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pedigree_rs_version() {
        assert!(!version().is_empty());
    }
}
