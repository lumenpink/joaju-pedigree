use crate::core::{Ped, Sex};
use crate::layout::PedLayout;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClinicalSymbol {
    Male,
    Female,
    Unknown,
}

impl From<Sex> for ClinicalSymbol {
    fn from(sex: Sex) -> Self {
        match sex {
            Sex::Male => ClinicalSymbol::Male,
            Sex::Female => ClinicalSymbol::Female,
            Sex::Unknown => ClinicalSymbol::Unknown,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct RenderOptions {
    pub show_id: bool,
    pub card_legend: bool,
}

pub fn render_svg(ped: &Ped, layout: &PedLayout, _opts: &RenderOptions) -> String {
    let mut svg = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {:.1} {:.1}\" width=\"{:.1}\" height=\"{:.1}\">\n",
        layout.width, layout.height, layout.width, layout.height
    );
    svg.push_str("  <style>\n");
    svg.push_str("    .sym { stroke: #000; stroke-width: 2; fill: #fff; }\n");
    svg.push_str("    .lbl { font-family: sans-serif; font-size: 12px; text-anchor: middle; }\n");
    svg.push_str("  </style>\n");

    for (id, coord) in &layout.coords {
        if let Some(ind) = ped.members.get(id) {
            match ind.sex {
                Sex::Male => {
                    svg.push_str(&format!(
                        "  <rect class=\"sym\" x=\"{:.1}\" y=\"{:.1}\" width=\"24\" height=\"24\" />\n",
                        coord.x - 12.0,
                        coord.y - 12.0
                    ));
                }
                Sex::Female => {
                    svg.push_str(&format!(
                        "  <circle class=\"sym\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"12\" />\n",
                        coord.x, coord.y
                    ));
                }
                Sex::Unknown => {
                    svg.push_str(&format!(
                        "  <polygon class=\"sym\" points=\"{:.1},{:.1} {:.1},{:.1} {:.1},{:.1} {:.1},{:.1}\" />\n",
                        coord.x, coord.y - 14.0,
                        coord.x + 14.0, coord.y,
                        coord.x, coord.y + 14.0,
                        coord.x - 14.0, coord.y
                    ));
                }
            }
            svg.push_str(&format!(
                "  <text class=\"lbl\" x=\"{:.1}\" y=\"{:.1}\">{}</text>\n",
                coord.x,
                coord.y + 26.0,
                ind.id
            ));
        }
    }

    svg.push_str("</svg>\n");
    svg
}
