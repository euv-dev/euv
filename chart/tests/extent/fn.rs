use super::*;

#[test]
fn nice_scale_expands_degenerate_domains_and_normalizes_reversed_bounds() {
    let equal: (f64, f64, f64) = model::nice_scale(4.0, 4.0, 5);
    assert!(equal.0 < 4.0 && equal.1 > 4.0 && equal.2 > 0.0);
    let forward: (f64, f64, f64) = model::nice_scale(-3.2, 8.4, 6);
    let reversed: (f64, f64, f64) = model::nice_scale(8.4, -3.2, 6);
    assert_eq!(forward, reversed);
    assert!(forward.0 <= -3.2 && forward.1 >= 8.4 && forward.2 > 0.0);
}

#[test]
fn nice_scale_rejects_nonfinite_domains_without_producing_nonfinite_ticks() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(model::nice_scale(value, 3.0, 5), (0.0, 1.0, 1.0));
        assert_eq!(model::nice_scale(3.0, value, 5), (0.0, 1.0, 1.0));
    }
}

fn reference_extent(series: &[SeriesOption], axis: Option<&AxisOption>) -> (f64, f64) {
    let mut min: f64 = f64::INFINITY;
    let mut max: f64 = f64::NEG_INFINITY;
    for item in series {
        if !matches!(
            item.kind,
            SeriesType::Line
                | SeriesType::Bar
                | SeriesType::Scatter
                | SeriesType::EffectScatter
                | SeriesType::Candlestick
                | SeriesType::Boxplot
                | SeriesType::Heatmap
        ) {
            continue;
        }
        for value in &item.data {
            let numbers: Vec<f64> = match value {
                DataValue::List(items) => items.iter().filter_map(DataValue::to_number).collect(),
                DataValue::Object(_) => value
                    .get("value")
                    .and_then(DataValue::to_number)
                    .into_iter()
                    .collect(),
                other => other.to_number().into_iter().collect(),
            };
            let selected: Option<f64> =
                if axis.is_some_and(|axis: &AxisOption| axis.axis_type == AxisType::Category) {
                    numbers.first().copied()
                } else {
                    numbers.last().copied()
                };
            if let Some(number) = selected {
                min = min.min(number);
                max = max.max(number);
            }
        }
    }
    if min.is_finite() && max.is_finite() {
        (min, max)
    } else {
        (0.0, 1.0)
    }
}

#[test]
fn streaming_extents_match_materialized_reference_for_all_value_shapes() {
    let values: Vec<DataValue> = vec![
        DataValue::Null,
        DataValue::Number(-4.0),
        DataValue::Number(f64::NAN),
        DataValue::Number(f64::INFINITY),
        DataValue::Bool(true),
        DataValue::Text(String::from(" 12.5 ")),
        DataValue::Text(String::from("missing")),
        DataValue::List(Vec::new()),
        DataValue::List(vec![
            DataValue::Null,
            DataValue::Text(String::from("-7")),
            DataValue::Text(String::from("missing")),
            DataValue::Number(42.0),
            DataValue::Null,
        ]),
        DataValue::List(vec![DataValue::Number(f64::NAN), DataValue::Number(-2.0)]),
        DataValue::Object(vec![(String::from("value"), DataValue::Number(6.0))]),
        DataValue::Object(vec![(String::from("name"), DataValue::Number(99.0))]),
        DataValue::Object(vec![(
            String::from("value"),
            DataValue::List(vec![DataValue::Number(99.0)]),
        )]),
    ];
    let category: AxisOption = AxisOption::category();
    let value_axis: AxisOption = AxisOption::value();
    for length in 0..=values.len() {
        for start in 0..=length {
            let series: [SeriesOption; 2] = [
                SeriesOption::line().data(values[start..length].to_vec()),
                SeriesOption::pie().data(vec![DataValue::Number(-999.0)]),
            ];
            for axis in [None, Some(&category), Some(&value_axis)] {
                assert_eq!(
                    model::value_extent(&series, axis),
                    reference_extent(&series, axis)
                );
            }
        }
    }
}

#[test]
fn tuple_axes_select_first_or_last_numeric_value_without_changing_domains() {
    let series: [SeriesOption; 1] = [SeriesOption::scatter().data(vec![
        DataValue::List(vec![
            DataValue::Null,
            DataValue::Number(-20.0),
            DataValue::Number(3.0),
            DataValue::Null,
        ]),
        DataValue::List(vec![
            DataValue::Text(String::from("bad")),
            DataValue::Number(10.0),
            DataValue::Number(8.0),
        ]),
    ])];
    assert_eq!(
        model::value_extent(&series, Some(&AxisOption::category())),
        (-20.0, 10.0)
    );
    assert_eq!(
        model::value_extent(&series, Some(&AxisOption::value())),
        (3.0, 8.0)
    );
    assert_eq!(model::value_extent(&series, None), (3.0, 8.0));
}
