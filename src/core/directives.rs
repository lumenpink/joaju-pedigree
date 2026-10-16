use crate::render::{LegendConfig, LegendItem, LegendSymbolType};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct PedDirectives {
    pub raw: HashMap<String, String>,
    pub legend_labels: Option<Vec<String>>,
    pub legend_types: Option<Vec<String>>,
}

impl PedDirectives {
    pub fn has_legend(&self) -> bool {
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
            if let Some((key, val)) = parse_key_value(directive_body) {
                let lower_key = key.to_lowercase();
                if lower_key == "legend" {
                    let parsed_list = parse_c_vector(&val);
                    directives.legend_labels = Some(parsed_list);
                } else if lower_key == "legend.type" {
                    let parsed_list = parse_c_vector(&val);
                    directives.legend_types = Some(parsed_list);
                }
                directives.raw.insert(lower_key, val);
            }
        } else {
            clean_lines.push(line);
        }
    }

    (directives, clean_lines.join("\n"))
}

fn parse_key_value(s: &str) -> Option<(String, String)> {
    if let Some(pos) = s.find('=') {
        let k = s[..pos].trim().to_string();
        let v = s[pos + 1..].trim().to_string();
        Some((k, v))
    } else {
        None
    }
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
