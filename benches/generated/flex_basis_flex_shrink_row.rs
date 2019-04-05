pub fn compute() -> stretch::layout::Node {
    stretch::compute(
        &stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100f32),
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node { flex_basis: stretch::style::Dimension::Points(100f32), ..Default::default() },
                stretch::style::Node { flex_basis: stretch::style::Dimension::Points(50f32), ..Default::default() },
            ],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap()
}