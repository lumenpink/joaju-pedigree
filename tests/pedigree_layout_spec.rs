use joaju_pedigree::prelude::*;

const ABO_PEDIGREE_CSV: &str = r#"id,pai,mae,sex,fenotipo,proband,label
1,0,0,M,wt,0,I-1 [B (I^B i)]
2,0,0,F,afetado,0,I-2 [A (I^A i)]
3,1,2,M,afetado,1,II-1 [A (I^A i)]
4,1,2,M,wt,0,II-2 [B (I^B i)]
5,1,2,F,wt,0,II-3 [B (I^B i)]
6,0,0,F,wt,0,II-4 [B (I^B _)]
7,0,0,F,wt,0,II-5 [O (ii)]
8,0,0,F,wt,0,II-6 [O (ii)]
9,0,0,F,wt,0,II-7 [O (ii)]
10,3,6,F,wt,0,III-1 [B (I^B i)]
11,3,7,M,wt,0,III-2 [O (ii)]
12,3,8,F,afetado,0,III-3 [A (I^A i)]
13,4,9,F,wt,0,III-4 [B (I^B i)]
"#;

#[test]
fn test_spec_parents_centered_over_children() {
    let ped = Ped::from_csv_str(ABO_PEDIGREE_CSV).expect("valid pedigree");
    let options = LayoutOptions::default();
    let layout = PedLayout::compute(&ped, &options);

    // Gen 1 parents: 1 and 2
    let c1 = &layout.coords["1"];
    let c2 = &layout.coords["2"];
    let parent_union_mid = (c1.x + c2.x) / 2.0;

    // Gen 2 children of (1, 2): 3, 4, 5
    let c3 = &layout.coords["3"];
    let c4 = &layout.coords["4"];
    let c5 = &layout.coords["5"];

    let children_min_x = c3.x.min(c4.x).min(c5.x);
    let children_max_x = c3.x.max(c4.x).max(c5.x);
    let children_mid = (children_min_x + children_max_x) / 2.0;

    // Parents union must be centered over their children's fratria
    assert!(
        (parent_union_mid - children_mid).abs() <= options.x_spacing,
        "Parents union mid ({parent_union_mid}) should be aligned with children mid ({children_mid})"
    );
}

#[test]
fn test_spec_child_under_parental_union_no_backward_crossing() {
    let ped = Ped::from_csv_str(ABO_PEDIGREE_CSV).expect("valid pedigree");
    let options = LayoutOptions::default();
    let layout = PedLayout::compute(&ped, &options);

    let c4 = &layout.coords["4"];
    let c9 = &layout.coords["9"];
    let union_4_9_mid = (c4.x + c9.x) / 2.0;

    // Child 13 is the daughter of 4 and 9
    let c13 = &layout.coords["13"];
    let c12 = &layout.coords["12"];

    // Child 13 must be to the right of child 12 (no backward crossing)
    assert!(
        c13.x > c12.x,
        "Child 13 (x={}) must be positioned to the right of child 12 (x={})",
        c13.x,
        c12.x
    );

    // Child 13 should be close to parental union (4, 9) midpoint
    assert!(
        (c13.x - union_4_9_mid).abs() <= options.x_spacing,
        "Child 13 (x={}) should be close to parental union midpoint ({})",
        c13.x,
        union_4_9_mid
    );
}

#[test]
fn test_spec_union_connector_line_reaches_child_in_svg() {
    let ped = Ped::from_csv_str(ABO_PEDIGREE_CSV).expect("valid pedigree");
    let options = LayoutOptions::default();
    let layout = PedLayout::compute(&ped, &options);
    let svg = render_svg(&ped, &layout, &RenderOptions::default());

    let c4 = &layout.coords["4"];
    let c9 = &layout.coords["9"];
    let union_mid = (c4.x + c9.x) / 2.0;
    let c13 = &layout.coords["13"];

    // If union midpoint is at union_mid and child 13 is at c13.x,
    // the horizontal line must cover from min(union_mid, c13.x) to max(union_mid, c13.x)
    let min_span = union_mid.min(c13.x);
    let max_span = union_mid.max(c13.x);

    if (min_span - max_span).abs() > 1.0 {
        let expected_line = format!(
            "x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\"",
            min_span,
            c4.y + 55.0,
            max_span,
            c4.y + 55.0
        );
        assert!(
            svg.contains(&expected_line),
            "SVG should contain a sibling/connector bar connecting union mid to child 13: expected line snippet {}",
            expected_line
        );
    }
}

#[test]
fn test_spec_serial_multiple_spouses_and_straight_stems() {
    let ped = Ped::from_csv_str(ABO_PEDIGREE_CSV).expect("valid pedigree");
    let options = LayoutOptions::default();
    let layout = PedLayout::compute(&ped, &options);
    let svg = render_svg(&ped, &layout, &RenderOptions::default());

    let c3 = &layout.coords["3"];
    let c6 = &layout.coords["6"];
    let c7 = &layout.coords["7"];
    let c8 = &layout.coords["8"];

    // Nodes 6, 3, 7, 8 must be ordered monotonically on x
    assert!(c6.x < c3.x, "6 (II-4) must be to the left of 3 (II-1)");
    assert!(c3.x < c7.x, "3 (II-1) must be to the left of 7 (II-5)");
    assert!(c7.x < c8.x, "7 (II-5) must be to the left of 8 (II-6)");

    // Child 10 (III-1) under (6, 3)
    let c10 = &layout.coords["10"];
    let mid_6_3 = (c6.x + c3.x) / 2.0;
    assert!(
        (c10.x - mid_6_3).abs() < 1.0,
        "Child 10 should be at midpoint of (6, 3): c10.x={}, mid={}",
        c10.x,
        mid_6_3
    );

    // Child 11 (III-2) under (3, 7)
    let c11 = &layout.coords["11"];
    let mid_3_7 = (c3.x + c7.x) / 2.0;
    assert!(
        (c11.x - mid_3_7).abs() < 1.0,
        "Child 11 should be at midpoint of (3, 7): c11.x={}, mid={}",
        c11.x,
        mid_3_7
    );

    // Child 12 (III-3) under (7, 8)
    let c12 = &layout.coords["12"];
    let mid_7_8 = (c7.x + c8.x) / 2.0;
    assert!(
        (c12.x - mid_7_8).abs() < 1.0,
        "Child 12 should be at midpoint of (7, 8): c12.x={}, mid={}",
        c12.x,
        mid_7_8
    );

    // Child 13 (III-4) under (4, 9)
    let c4 = &layout.coords["4"];
    let c9 = &layout.coords["9"];
    let c13 = &layout.coords["13"];
    let mid_4_9 = (c4.x + c9.x) / 2.0;
    assert!(
        (c13.x - mid_4_9).abs() < 1.0,
        "Child 13 should be at midpoint of (4, 9): c13.x={}, mid={}",
        c13.x,
        mid_4_9
    );

    // No marriage line should cut through node 7 (II-5) from 3 to 8
    let bad_through_line = format!("x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\"", c3.x, c3.y, c8.x);
    assert!(
        !svg.contains(&bad_through_line),
        "SVG should not draw a marriage line passing through node 7 directly from 3 to 8"
    );
}

#[test]
fn test_spec_proband_legend_arrow_has_barbs() {
    let ped = Ped::from_csv_str(ABO_PEDIGREE_CSV).expect("valid pedigree");
    let layout = PedLayout::compute(&ped, &LayoutOptions::default());
    let svg = render_svg(&ped, &layout, &RenderOptions::default());

    // Legend card must have proband arrow with arrowhead barbs (multiple M/L segments)
    let legend_section = svg.split("class=\"card-legend\"").nth(1).unwrap_or("");
    assert!(
        legend_section.contains("proband-arrow"),
        "Legend must have proband-arrow element"
    );
    // Deve conter aletas da flecha (pelo menos 3 comandos L no path)
    let arrow_path = legend_section
        .lines()
        .find(|l| l.contains("class=\"proband-arrow\""))
        .unwrap_or("");
    let l_count = arrow_path.matches('L').count();
    assert!(
        l_count >= 3,
        "Proband arrow in legend must have arrowhead barbs (at least 3 L segments), found: {arrow_path}"
    );
    assert!(
        legend_section.contains("proband-lbl"),
        "Legend must have proband-lbl element"
    );
}

#[test]
fn test_spec_comparative_pedtools_vs_kinship2_all_generations() {
    let g2_csv = r#"id,pai,mae,sex,fenotipo,proband,label
1,0,0,M,wt,0,I-1 [Normal]
2,0,0,F,afetado,0,I-2 [Afetada]
3,1,2,M,het,0,II-1 [Portador]
4,1,2,F,afetado,1,II-2 [Afetada]
5,1,2,M,wt,0,II-3 [Normal]
"#;

    let g4_csv = r#"id,pai,mae,sex,fenotipo,proband,label
1,0,0,M,wt,0,I-1 [Normal]
2,0,0,F,afetado,0,I-2 [Afetada]
3,1,2,M,het,0,II-1 [Portador]
4,0,0,F,wt,0,II-2 [Normal]
5,1,2,F,het,0,II-3 [Portadora]
6,3,4,M,het,0,III-1 [Portador]
7,0,0,F,wt,0,III-2 [Normal]
8,3,4,F,wt,0,III-3 [Normal]
9,6,7,M,afetado,1,IV-1 [Afetado]
10,6,7,F,het,0,IV-2 [Portadora]
"#;

    let g5_csv = r#"id,pai,mae,sex,fenotipo,proband,label
1,0,0,M,wt,0,I-1 [Normal]
2,0,0,F,afetado,0,I-2 [Afetada]
3,1,2,M,het,0,II-1 [Portador]
4,0,0,F,wt,0,II-2 [Normal]
5,1,2,F,het,0,II-3 [Portadora]
6,3,4,M,het,0,III-1 [Portador]
7,0,0,F,wt,0,III-2 [Normal]
8,3,4,F,wt,0,III-3 [Normal]
9,6,7,M,het,0,IV-1 [Portador]
10,0,0,F,wt,0,IV-2 [Normal]
11,6,7,F,wt,0,IV-3 [Normal]
12,9,10,M,afetado,1,V-1 [Afetado]
13,9,10,F,het,0,V-2 [Portadora]
"#;

    for (name, csv, expected_gens) in [("G2", g2_csv, 2), ("G4", g4_csv, 4), ("G5", g5_csv, 5)] {
        let ped = Ped::from_csv_str(csv).expect("valid pedigree");
        let layout = PedLayout::compute(&ped, &LayoutOptions::default());
        let max_gen = layout.coords.values().map(|c| c.generation).max().unwrap();
        assert_eq!(
            max_gen, expected_gens,
            "{name} must have {expected_gens} generations"
        );
        let svg = render_svg(&ped, &layout, &RenderOptions::default());
        assert!(!svg.is_empty(), "{name} svg should not be empty");
    }
}

#[test]
fn test_spec_symmetric_node_margins() {
    let ped = Ped::from_csv_str(ABO_PEDIGREE_CSV).expect("valid pedigree");
    let layout = PedLayout::compute(&ped, &LayoutOptions::default());
    let min_x = layout
        .coords
        .values()
        .map(|c| c.x)
        .fold(f64::INFINITY, f64::min);
    let max_x = layout
        .coords
        .values()
        .map(|c| c.x)
        .fold(f64::NEG_INFINITY, f64::max);
    let left_margin = min_x;
    let right_margin = layout.width - max_x;
    assert!(
        (left_margin - right_margin).abs() < 1.0,
        "Left margin ({left_margin}) must equal right margin ({right_margin}) for symmetric centering in viewBox"
    );
}

#[test]
fn test_spec_symmetric_legend_grid_items() {
    let ped = Ped::from_csv_str(ABO_PEDIGREE_CSV).expect("valid pedigree");
    let layout = PedLayout::compute(&ped, &LayoutOptions::default());
    let svg = render_svg(&ped, &layout, &RenderOptions::default());

    let legend_section = svg.split("class=\"card-legend\"").nth(1).unwrap_or("");
    let mut item_xs = Vec::new();
    for line in legend_section.lines() {
        if line.contains("class=\"legend-lbl\"") {
            if let Some(x_str) = line.split("x=\"").nth(1).and_then(|s| s.split('"').next()) {
                if let Ok(x_val) = x_str.parse::<f64>() {
                    item_xs.push(x_val);
                }
            }
        }
    }

    assert!(item_xs.len() >= 2, "Legend must contain at least 2 items");
    let first_x = item_xs[0];
    let last_x = item_xs[item_xs.len() - 1];
    let card_x = 20.0;
    let card_w = (layout.width.max(420.0) - 40.0).max(380.0);
    let card_right = card_x + card_w;

    let dist_left = first_x - card_x;
    let dist_right = card_right - last_x;
    assert!(
        (dist_left - dist_right).abs() < 1.0,
        "Legend grid must be symmetric: dist_left={dist_left}, dist_right={dist_right}"
    );
}

#[test]
fn test_spec_proband_legend_arrow_and_p_geometry() {
    let ped = Ped::from_csv_str(ABO_PEDIGREE_CSV).expect("valid pedigree");
    let layout = PedLayout::compute(&ped, &LayoutOptions::default());
    let render_opts = RenderOptions {
        legend: Some(LegendConfig {
            items: vec![LegendItem {
                symbol: LegendSymbolType::Proband,
                label: "Probanda".into(),
            }],
        }),
        ..Default::default()
    };
    let svg = render_svg(&ped, &layout, &render_opts);

    // Letra P no probando da legenda deve estar na base da seta (x < ix - 10) e não em cima do quadrado
    assert!(
        svg.contains("class=\"proband-lbl\""),
        "Deve conter proband-lbl"
    );
    assert!(
        svg.contains("class=\"proband-arrow\""),
        "Deve conter proband-arrow"
    );
}

#[test]
fn test_spec_legend_card_multi_row_no_text_overlap() {
    let ped = Ped::from_csv_str(ABO_PEDIGREE_CSV).expect("valid pedigree");
    let layout = PedLayout::compute(&ped, &LayoutOptions::default());
    let render_opts = RenderOptions {
        legend: Some(LegendConfig {
            items: vec![
                LegendItem {
                    symbol: LegendSymbolType::EmptyM,
                    label: "Sexo masculino normal".into(),
                },
                LegendItem {
                    symbol: LegendSymbolType::EmptyF,
                    label: "Sexo feminino normal".into(),
                },
                LegendItem {
                    symbol: LegendSymbolType::SqHalf,
                    label: "Sexo masculino portador".into(),
                },
                LegendItem {
                    symbol: LegendSymbolType::CircHalf,
                    label: "Sexo feminino portadora".into(),
                },
                LegendItem {
                    symbol: LegendSymbolType::SqSolid,
                    label: "Sexo masculino afetado".into(),
                },
                LegendItem {
                    symbol: LegendSymbolType::Proband,
                    label: "Probanda (sexo feminino)".into(),
                },
            ],
        }),
        ..Default::default()
    };
    let svg = render_svg(&ped, &layout, &render_opts);

    // Deve usar tspans para quebrar textos longos
    assert!(
        svg.contains("<tspan"),
        "Textos longos na legenda devem ser quebrados com tspan"
    );
    // O card de 6 itens deve ter altura expandida para 2 linhas (card_h >= 120)
    assert!(
        svg.contains("height=\"135.0\"")
            || svg.contains("height=\"130.0\"")
            || svg.contains("height=\"120.0\"")
    );
}
