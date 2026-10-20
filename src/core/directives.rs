use crate::core::TwinType;
use crate::render::{LegendConfig, LegendItem, LegendSymbolType};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TwinDirective {
    pub ids: Vec<String>,
    pub twin_type: TwinType,
}

#[derive(Debug, Clone, Default)]
pub struct PedDirectives {
    pub raw: HashMap<String, String>,
    pub legend_labels: Option<Vec<String>>,
    pub legend_types: Option<Vec<String>>,
    pub twins: Vec<TwinDirective>,
    pub divorces: Vec<(String, String)>,
    pub childless: Vec<(String, String, crate::core::ChildlessType)>,
}

use crate::render::AutoLegendMode;

impl PedDirectives {
    pub fn auto_legend_mode(&self) -> AutoLegendMode {
        if let Some(val) = self.raw.get("legend") {
            let v = val.to_lowercase();
            if v == "full"
                || v == "complete"
                || v == "auto-full"
                || v == "auto_full"
                || v == "completa"
            {
                return AutoLegendMode::Full;
            }
            if v == "auto" || v == "generic" || v == "generica" || v == "compact" || v == "true" {
                return AutoLegendMode::Generic;
            }
        }
        if let Some(val) = self.raw.get("legend.mode") {
            let v = val.to_lowercase();
            if v == "full"
                || v == "complete"
                || v == "auto-full"
                || v == "auto_full"
                || v == "completa"
            {
                return AutoLegendMode::Full;
            }
            if v == "auto" || v == "generic" || v == "generica" || v == "compact" {
                return AutoLegendMode::Generic;
            }
        }
        if let Some(val) = self.raw.get("auto_legend") {
            let v = val.to_lowercase();
            if v == "full"
                || v == "complete"
                || v == "auto-full"
                || v == "auto_full"
                || v == "completa"
            {
                return AutoLegendMode::Full;
            }
            if v == "true" || v == "auto" || v == "generic" || v.is_empty() {
                return AutoLegendMode::Generic;
            }
        }
        if self.raw.contains_key("auto_legend") {
            return AutoLegendMode::Generic;
        }
        AutoLegendMode::None
    }

    pub fn is_auto_legend(&self) -> bool {
        self.auto_legend_mode() != AutoLegendMode::None
    }

    pub fn has_legend(&self) -> bool {
        if self.is_auto_legend() {
            return false;
        }
        self.legend_labels.is_some() || self.raw.contains_key("legend")
    }

    pub fn to_legend_config(&self) -> LegendConfig {
        let mut items = Vec::new();

        if let Some(ref labels) = self.legend_labels {
            let types = self.legend_types.clone().unwrap_or_default();
            for (i, label) in labels.iter().enumerate() {
                let type_str = types.get(i).map(|s| s.as_str()).unwrap_or("empty_m");
                let sym_type = LegendSymbolType::parse_str(type_str);
                items.push(LegendItem {
                    symbol: sym_type,
                    label: label.clone(),
                });
            }
        }

        LegendConfig { items }
    }
}

pub fn extract_directives(text: &str) -> (PedDirectives, String) {
    let mut directives = PedDirectives::default();
    let mut clean_lines = Vec::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("%%") {
            let directive_body = trimmed.trim_start_matches('%').trim();
            for (key, val) in parse_directives_line(directive_body) {
                let lower_key = key.to_lowercase();
                if lower_key == "legend" {
                    let v = val.to_lowercase();
                    if v == "auto"
                        || v == "true"
                        || v == "full"
                        || v == "complete"
                        || v == "auto-full"
                        || v == "auto_full"
                        || v == "completa"
                        || v == "generic"
                        || v == "compact"
                    {
                        directives.legend_labels = None;
                    } else {
                        let parsed_list = parse_c_vector(&val);
                        directives.legend_labels = Some(parsed_list);
                    }
                } else if lower_key == "legend.type" {
                    let parsed_list = parse_c_vector(&val);
                    directives.legend_types = Some(parsed_list);
                } else if lower_key.starts_with("twin")
                    || lower_key.starts_with("gemeo")
                    || lower_key.starts_with("gemelar")
                {
                    let parts: Vec<String> = val
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                    if parts.len() >= 2 {
                        let last = parts.last().unwrap().to_lowercase();
                        let (ids, t_type) = if matches!(
                            last.as_str(),
                            "mz" | "dz" | "unknown" | "?" | "monozygotic" | "dizygotic"
                        ) {
                            (
                                parts[..parts.len() - 1].to_vec(),
                                TwinType::parse_str(&last),
                            )
                        } else {
                            (parts.clone(), TwinType::Dizygotic)
                        };
                        directives.twins.push(TwinDirective {
                            ids,
                            twin_type: t_type,
                        });
                    }
                } else if lower_key.starts_with("litter") || lower_key.starts_with("ninhada") {
                    let ids: Vec<String> = val
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                    if !ids.is_empty() {
                        directives.twins.push(TwinDirective {
                            ids,
                            twin_type: TwinType::Dizygotic,
                        });
                    }
                } else if lower_key.starts_with("divorce") || lower_key.starts_with("divorcio") {
                    let parts: Vec<String> = val
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                    if parts.len() >= 2 {
                        directives
                            .divorces
                            .push((parts[0].clone(), parts[1].clone()));
                    }
                } else if lower_key.starts_with("childless") || lower_key.starts_with("sem_filhos")
                {
                    let parts: Vec<String> = val
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                    if parts.len() >= 2 {
                        let c_type = if parts.len() >= 3 {
                            crate::core::ChildlessType::parse_str(&parts[2])
                        } else {
                            crate::core::ChildlessType::ByChoice
                        };
                        directives
                            .childless
                            .push((parts[0].clone(), parts[1].clone(), c_type));
                    }
                }
                directives.raw.insert(lower_key, val);
            }
        } else {
            clean_lines.push(line);
        }
    }

    (directives, clean_lines.join("\n"))
}

pub fn parse_directives_line(s: &str) -> Vec<(String, String)> {
    let mut pairs = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    let n = chars.len();

    while i < n {
        while i < n && chars[i].is_whitespace() {
            i += 1;
        }
        if i >= n {
            break;
        }

        let key_start = i;
        while i < n && chars[i] != '=' && !chars[i].is_whitespace() {
            i += 1;
        }
        let key = chars[key_start..i].iter().collect::<String>();

        while i < n && chars[i].is_whitespace() {
            i += 1;
        }
        if i >= n || chars[i] != '=' {
            if !key.is_empty() {
                pairs.push((key, String::new()));
            }
            continue;
        }
        i += 1; // skip '='

        while i < n && chars[i].is_whitespace() {
            i += 1;
        }
        if i >= n {
            pairs.push((key, String::new()));
            break;
        }

        let val_start = i;
        if chars[i] == '"' || chars[i] == '\'' {
            let quote = chars[i];
            i += 1;
            while i < n && chars[i] != quote {
                i += 1;
            }
            if i < n {
                i += 1;
            }
        } else if i + 1 < n && chars[i] == 'c' && chars[i + 1] == '(' {
            let mut depth = 0;
            while i < n {
                if chars[i] == '(' {
                    depth += 1;
                } else if chars[i] == ')' {
                    depth -= 1;
                    if depth == 0 {
                        i += 1;
                        break;
                    }
                }
                i += 1;
            }
        } else {
            while i < n && !chars[i].is_whitespace() {
                i += 1;
            }
        }
        let val = chars[val_start..i].iter().collect::<String>();
        pairs.push((key, val));
    }

    pairs
}

fn parse_c_vector(val: &str) -> Vec<String> {
    let trimmed = val.trim();
    if trimmed.starts_with("c(") && trimmed.ends_with(')') {
        let inner = &trimmed[2..trimmed.len() - 1];
        inner
            .split(',')
            .map(|item| {
                item.trim()
                    .trim_matches('"')
                    .trim_matches('\'')
                    .trim()
                    .to_string()
            })
            .filter(|s| !s.is_empty())
            .collect()
    } else {
        vec![trimmed.trim_matches('"').trim_matches('\'').to_string()]
    }
}
