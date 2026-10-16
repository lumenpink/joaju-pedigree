use crate::core::Ped;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct LayoutOptions {
    pub symbol_size: f64,
    pub x_spacing: f64,
    pub y_spacing: f64,
}

#[derive(Debug, Clone)]
pub struct NodeCoord {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub generation: usize,
}

#[derive(Debug, Clone)]
pub struct PedLayout {
    pub coords: HashMap<String, NodeCoord>,
    pub width: f64,
    pub height: f64,
}

impl PedLayout {
    pub fn compute(ped: &Ped, _options: &LayoutOptions) -> Self {
        let mut coords = HashMap::new();
        for (i, id) in ped.order.iter().enumerate() {
            coords.insert(
                id.clone(),
                NodeCoord {
                    id: id.clone(),
                    x: (i as f64) * 50.0 + 30.0,
                    y: 50.0,
                    generation: 1,
                },
            );
        }
        Self {
            coords,
            width: (ped.order.len() as f64) * 50.0 + 60.0,
            height: 120.0,
        }
    }
}
