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
pub use crate::kinship::KinshipMatrix;
pub use crate::layout::{LayoutOptions, PedLayout};
pub use crate::render::{render_svg, ClinicalSymbol, RenderOptions};
