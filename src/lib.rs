pub mod core;
pub mod kinship;
pub mod layout;
pub mod prelude;
pub mod render;

pub use crate::core::{Individual, Ped, Sex, Status, TwinType};
pub use crate::layout::{LayoutOptions, PedLayout};
pub use crate::render::{render_svg, ClinicalSymbol, RenderOptions};
