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
            c4.y + 40.0,
            max_span,
            c4.y + 40.0
        );
        assert!(
            svg.contains(&expected_line),
            "SVG should contain a sibling/connector bar connecting union mid to child 13: expected line snippet {}",
            expected_line
        );
    }
}
