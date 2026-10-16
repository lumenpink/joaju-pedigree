use crate::core::{Individual, Ped, Sex, Status};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegendSymbolType {
    EmptyM,
    EmptyF,
    SqHalf,
    CircHalf,
    SqSolid,
    CircSolid,
    Suspect,
    Proband,
    Deceased,
}

impl LegendSymbolType {
    pub fn parse_str(s: &str) -> Self {
        let clean = s.trim().to_lowercase();
        match clean.as_str() {
            "empty_m" | "sq_empty" => LegendSymbolType::EmptyM,
            "empty_f" | "circ_empty" => LegendSymbolType::EmptyF,
            "half" | "sq_half" => LegendSymbolType::SqHalf,
            "circ_half" => LegendSymbolType::CircHalf,
            "solid" | "sq_solid" => LegendSymbolType::SqSolid,
            "circ_solid" => LegendSymbolType::CircSolid,
            "suspect" | "sq_suspect" | "circ_suspect" => LegendSymbolType::Suspect,
            "proband" | "prob" => LegendSymbolType::Proband,
            "deceased" | "dec" => LegendSymbolType::Deceased,
            _ => LegendSymbolType::EmptyM,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LegendItem {
    pub symbol: LegendSymbolType,
    pub label: String,
}

#[derive(Debug, Clone, Default)]
pub struct LegendConfig {
    pub items: Vec<LegendItem>,
}

#[derive(Debug, Clone)]
pub struct RenderOptions {
    pub show_id: bool,
    pub card_legend: bool,
    pub legend: Option<LegendConfig>,
    pub auto_legend: bool,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            show_id: true,
            card_legend: true,
            legend: None,
            auto_legend: false,
        }
    }
}

pub fn render_svg(ped: &Ped, layout: &PedLayout, options: &RenderOptions) -> String {
    let mut svg = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {:.1} {:.1}\" width=\"{:.1}\" height=\"{:.1}\">\n",
        layout.width, layout.height, layout.width, layout.height
    );

    // CSS Stylesheet
    svg.push_str("  <style>\n");
    svg.push_str("    .sym { stroke: #000000; stroke-width: 2.2; fill: #ffffff; }\n");
    svg.push_str("    .sym-aff { stroke: #000000; stroke-width: 2.2; fill: #000000; }\n");
    svg.push_str("    .kin-line { stroke: #000000; stroke-width: 2.0; stroke-linecap: round; }\n");
    svg.push_str("    .consanguinity-line { stroke: #000000; stroke-width: 2.0; }\n");
    svg.push_str("    .deceased-slash { stroke: #000000; stroke-width: 2.2; }\n");
    svg.push_str("    .proband-arrow { stroke: #000000; stroke-width: 2.2; fill: none; }\n");
    svg.push_str("    .lbl { font-family: 'Liberation Sans', sans-serif; font-size: 13px; font-weight: bold; text-anchor: middle; fill: #222222; }\n");
    svg.push_str("    .proband-lbl { font-family: 'Liberation Sans', sans-serif; font-size: 13px; font-weight: bold; fill: #000000; }\n");
    svg.push_str("    .legend-title { font-family: 'Liberation Sans', sans-serif; font-size: 13px; font-weight: bold; text-anchor: middle; fill: #333333; }\n");
    svg.push_str("    .legend-lbl { font-family: 'Liberation Sans', sans-serif; font-size: 11px; text-anchor: middle; fill: #555555; }\n");
    svg.push_str("  </style>\n\n");

    // 1. Linhas de União e Descendência
    for u in &layout.unions {
        let (x_start, x_end) = if u.x1 < u.x2 {
            (u.x1, u.x2)
        } else {
            (u.x2, u.x1)
        };
        if u.is_consanguineous {
            svg.push_str(&format!(
                "  <!-- União Consanguínea {} e {} -->\n",
                u.p1_id, u.p2_id
            ));
            svg.push_str(&format!(
                "  <line class=\"consanguinity-line\" x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" />\n",
                x_start, u.y - 2.5, x_end, u.y - 2.5
            ));
            svg.push_str(&format!(
                "  <line class=\"consanguinity-line\" x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" />\n",
                x_start, u.y + 2.5, x_end, u.y + 2.5
            ));
        } else {
            svg.push_str(&format!(
                "  <line class=\"kin-line marriage-line\" x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" />\n",
                x_start, u.y, x_end, u.y
            ));
        }

        // Descendência se houver filhos
        if !u.children.is_empty() {
            let mid_x = (x_start + x_end) / 2.0;
            let sib_bar_y = u.y + 40.0;

            // Haste vertical descendo da união
            svg.push_str(&format!(
                "  <line class=\"kin-line\" x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" />\n",
                mid_x, u.y, mid_x, sib_bar_y
            ));

            let child_xs: Vec<f64> = u
                .children
                .iter()
                .filter_map(|cid| layout.coords.get(cid).map(|c| c.x))
                .collect();

            if !child_xs.is_empty() {
                let min_c = child_xs.iter().copied().fold(f64::INFINITY, f64::min);
                let max_c = child_xs.iter().copied().fold(f64::NEG_INFINITY, f64::max);

                // Barra horizontal de irmãos
                if min_c < max_c {
                    svg.push_str(&format!(
                        "  <line class=\"kin-line\" x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" />\n",
                        min_c, sib_bar_y, max_c, sib_bar_y
                    ));
                }

                // Hastes individuais descendo para cada filho
                for cid in &u.children {
                    if let Some(coord) = layout.coords.get(cid) {
                        svg.push_str(&format!(
                            "  <line class=\"kin-line\" x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" />\n",
                            coord.x, sib_bar_y, coord.x, coord.y - 14.0
                        ));
                    }
                }
            }
        }
    }

    // 2. Renderização de Símbolos Clínicos
    for id in &ped.order {
        if let (Some(coord), Some(ind)) = (layout.coords.get(id), ped.members.get(id)) {
            render_individual(&mut svg, ind, coord.x, coord.y, options.show_id);
        }
    }

    // 3. Card de Legenda Vetorial (se solicitado ou se auto_legend estiver ativo)
    if options.card_legend || options.auto_legend || options.legend.is_some() {
        render_legend_card(&mut svg, ped, layout.width, layout.height, options);
    }

    svg.push_str("</svg>\n");
    svg
}

fn render_individual(svg: &mut String, ind: &Individual, x: f64, y: f64, show_id: bool) {
    let sym_class = if ind.affected == 1 {
        "sym-aff"
    } else {
        "sym"
    };

    match ind.sex {
        Sex::Male => {
            svg.push_str(&format!(
                "  <rect class=\"{}\" x=\"{:.1}\" y=\"{:.1}\" width=\"28\" height=\"28\" rx=\"1\" />\n",
                sym_class,
                x - 14.0,
                y - 14.0
            ));
        }
        Sex::Female => {
            svg.push_str(&format!(
                "  <circle class=\"{}\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"14\" />\n",
                sym_class, x, y
            ));
        }
        Sex::Unknown => {
            svg.push_str(&format!(
                "  <polygon class=\"{}\" points=\"{:.1},{:.1} {:.1},{:.1} {:.1},{:.1} {:.1},{:.1}\" />\n",
                sym_class,
                x, y - 16.0,
                x + 16.0, y,
                x, y + 16.0,
                x - 16.0, y
            ));
        }
    }

    // Portador (Carrier - meio preenchimento ou ponto central)
    if ind.carrier && ind.affected == 0 {
        if ind.sex == Sex::Female {
            // Meio círculo preenchido à esquerda
            svg.push_str(&format!(
                "  <path d=\"M {:.1} {:.1} A 14 14 0 0 0 {:.1} {:.1} Z\" fill=\"#000000\" />\n",
                x, y - 14.0, x, y + 14.0
            ));
        } else {
            // Meio quadrado preenchido à esquerda
            svg.push_str(&format!(
                "  <rect x=\"{:.1}\" y=\"{:.1}\" width=\"14\" height=\"28\" fill=\"#000000\" />\n",
                x - 14.0, y - 14.0
            ));
        }
    }

    // Caso Suspeito ('?')
    if ind.suspect {
        svg.push_str(&format!(
            "  <text class=\"lbl\" x=\"{:.1}\" y=\"{:.1}\" font-size=\"16\" font-weight=\"bold\">?</text>\n",
            x, y + 5.0
        ));
    }

    // Falecido (Corte diagonal)
    if ind.status == Status::Deceased {
        svg.push_str(&format!(
            "  <line class=\"deceased-slash\" x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" />\n",
            x - 19.0,
            y + 19.0,
            x + 19.0,
            y - 19.0
        ));
    }

    // Probando (Seta 45° no canto inferior esquerdo)
    if ind.proband {
        let arr_target_x = x - 17.0;
        let arr_target_y = y + 17.0;
        let arr_orig_x = arr_target_x - 15.0;
        let arr_orig_y = arr_target_y + 15.0;

        svg.push_str(&format!(
            "  <!-- Probando Seta e Marcador P -->\n  <path class=\"proband-arrow\" d=\"M {:.1} {:.1} L {:.1} {:.1} M {:.1} {:.1} L {:.1} {:.1} M {:.1} {:.1} L {:.1} {:.1}\" />\n",
            arr_orig_x, arr_orig_y, arr_target_x, arr_target_y,
            arr_target_x, arr_target_y, arr_target_x - 7.0, arr_target_y,
            arr_target_x, arr_target_y, arr_target_x, arr_target_y + 7.0
        ));
        svg.push_str(&format!(
            "  <text class=\"proband-lbl\" x=\"{:.1}\" y=\"{:.1}\">P</text>\n",
            arr_orig_x - 6.0,
            arr_orig_y + 6.0
        ));
    }

    // Identificador
    if show_id {
        svg.push_str(&format!(
            "  <text class=\"lbl\" x=\"{:.1}\" y=\"{:.1}\">{}</text>\n",
            x,
            y + 30.0,
            ind.id
        ));
    }
}

fn render_legend_card(
    svg: &mut String,
    ped: &Ped,
    width: f64,
    height: f64,
    options: &RenderOptions,
) {
    let items: Vec<LegendItem> = if let Some(ref config) = options.legend {
        config.items.clone()
    } else if options.auto_legend {
        // Gera a legenda automaticamente a partir dos traços detectados na família
        let mut auto_items = Vec::new();
        let has_male = ped.members.values().any(|i| i.sex == Sex::Male);
        let has_female = ped.members.values().any(|i| i.sex == Sex::Female);
        let has_aff = ped.members.values().any(|i| i.affected > 0);
        let has_carrier = ped.members.values().any(|i| i.carrier);
        let has_suspect = ped.members.values().any(|i| i.suspect);
        let has_dec = ped.members.values().any(|i| i.status == Status::Deceased);
        let has_proband = ped.members.values().any(|i| i.proband);

        if has_male {
            auto_items.push(LegendItem {
                symbol: LegendSymbolType::EmptyM,
                label: "Homem".into(),
            });
        }
        if has_female {
            auto_items.push(LegendItem {
                symbol: LegendSymbolType::EmptyF,
                label: "Mulher".into(),
            });
        }
        if has_aff {
            auto_items.push(LegendItem {
                symbol: LegendSymbolType::SqSolid,
                label: "Afetado".into(),
            });
        }
        if has_carrier {
            auto_items.push(LegendItem {
                symbol: LegendSymbolType::CircHalf,
                label: "Portador".into(),
            });
        }
        if has_suspect {
            auto_items.push(LegendItem {
                symbol: LegendSymbolType::Suspect,
                label: "Investigação".into(),
            });
        }
        if has_dec {
            auto_items.push(LegendItem {
                symbol: LegendSymbolType::Deceased,
                label: "Falecido".into(),
            });
        }
        if has_proband {
            auto_items.push(LegendItem {
                symbol: LegendSymbolType::Proband,
                label: "Probando".into(),
            });
        }
        auto_items
    } else {
        vec![
            LegendItem {
                symbol: LegendSymbolType::EmptyM,
                label: "Homem".into(),
            },
            LegendItem {
                symbol: LegendSymbolType::EmptyF,
                label: "Mulher".into(),
            },
            LegendItem {
                symbol: LegendSymbolType::SqSolid,
                label: "Afetado".into(),
            },
            LegendItem {
                symbol: LegendSymbolType::CircHalf,
                label: "Portador".into(),
            },
            LegendItem {
                symbol: LegendSymbolType::Deceased,
                label: "Falecido".into(),
            },
            LegendItem {
                symbol: LegendSymbolType::Proband,
                label: "Probando".into(),
            },
        ]
    };

    if items.is_empty() {
        return;
    }

    let card_w = (width - 40.0).max(380.0);
    let card_h = 75.0;
    let card_x = 20.0;
    let card_y = height - card_h - 15.0;

    svg.push_str("  <!-- Card de Legenda Vetorial Integrado -->\n");
    svg.push_str("  <g class=\"card-legend\">\n");
    svg.push_str(&format!(
        "    <rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" rx=\"6\" fill=\"#fcfcfc\" stroke=\"#cccccc\" stroke-width=\"1.2\" />\n",
        card_x, card_y, card_w, card_h
    ));
    svg.push_str(&format!(
        "    <text class=\"legend-title\" x=\"{:.1}\" y=\"{:.1}\">LEGENDA CLÍNICA (NSGC/ACMG)</text>\n",
        card_x + card_w / 2.0,
        card_y + 18.0
    ));
    svg.push_str(&format!(
        "    <line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"#e0e0e0\" stroke-width=\"1\" />\n",
        card_x + 20.0,
        card_y + 24.0,
        card_x + card_w - 20.0,
        card_y + 24.0
    ));

    let step = (card_w - 30.0) / (items.len() as f64);
    for (i, item) in items.iter().enumerate() {
        let ix = card_x + 25.0 + (i as f64) * step;
        let iy = card_y + 44.0;

        match item.symbol {
            LegendSymbolType::EmptyM => {
                svg.push_str(&format!(
                    "    <rect class=\"sym\" x=\"{:.1}\" y=\"{:.1}\" width=\"16\" height=\"16\" />\n",
                    ix - 8.0, iy - 8.0
                ));
            }
            LegendSymbolType::EmptyF => {
                svg.push_str(&format!(
                    "    <circle class=\"sym\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"8\" />\n",
                    ix, iy
                ));
            }
            LegendSymbolType::SqSolid => {
                svg.push_str(&format!(
                    "    <rect class=\"sym-aff\" x=\"{:.1}\" y=\"{:.1}\" width=\"16\" height=\"16\" />\n",
                    ix - 8.0, iy - 8.0
                ));
            }
            LegendSymbolType::CircSolid => {
                svg.push_str(&format!(
                    "    <circle class=\"sym-aff\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"8\" />\n",
                    ix, iy
                ));
            }
            LegendSymbolType::SqHalf => {
                svg.push_str(&format!(
                    "    <rect class=\"sym\" x=\"{:.1}\" y=\"{:.1}\" width=\"16\" height=\"16\" />\n",
                    ix - 8.0, iy - 8.0
                ));
                svg.push_str(&format!(
                    "    <rect x=\"{:.1}\" y=\"{:.1}\" width=\"8\" height=\"16\" fill=\"#000\" />\n",
                    ix - 8.0, iy - 8.0
                ));
            }
            LegendSymbolType::CircHalf => {
                svg.push_str(&format!(
                    "    <circle class=\"sym\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"8\" />\n",
                    ix, iy
                ));
                svg.push_str(&format!(
                    "    <path d=\"M {:.1} {:.1} A 8 8 0 0 0 {:.1} {:.1} Z\" fill=\"#000\" />\n",
                    ix, iy - 8.0, ix, iy + 8.0
                ));
            }
            LegendSymbolType::Suspect => {
                svg.push_str(&format!(
                    "    <circle class=\"sym\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"8\" />\n",
                    ix, iy
                ));
                svg.push_str(&format!(
                    "    <text class=\"lbl\" font-size=\"12\" x=\"{:.1}\" y=\"{:.1}\">?</text>\n",
                    ix, iy + 4.0
                ));
            }
            LegendSymbolType::Deceased => {
                svg.push_str(&format!(
                    "    <rect class=\"sym\" x=\"{:.1}\" y=\"{:.1}\" width=\"16\" height=\"16\" />\n",
                    ix - 8.0, iy - 8.0
                ));
                svg.push_str(&format!(
                    "    <line class=\"deceased-slash\" x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" />\n",
                    ix - 10.0, iy + 10.0, ix + 10.0, iy - 10.0
                ));
            }
            LegendSymbolType::Proband => {
                svg.push_str(&format!(
                    "    <rect class=\"sym\" x=\"{:.1}\" y=\"{:.1}\" width=\"16\" height=\"16\" />\n",
                    ix - 8.0, iy - 8.0
                ));
                svg.push_str(&format!(
                    "    <path class=\"proband-arrow\" d=\"M {:.1} {:.1} L {:.1} {:.1}\" />\n",
                    ix - 16.0, iy + 16.0, ix - 9.0, iy + 9.0
                ));
                svg.push_str(&format!(
                    "    <text class=\"proband-lbl\" font-size=\"10\" x=\"{:.1}\" y=\"{:.1}\">P</text>\n",
                    ix - 20.0, iy + 18.0
                ));
            }
        }

        svg.push_str(&format!(
            "    <text class=\"legend-lbl\" x=\"{:.1}\" y=\"{:.1}\">{}</text>\n",
            ix,
            card_y + 64.0,
            item.label
        ));
    }

    svg.push_str("  </g>\n");
}
