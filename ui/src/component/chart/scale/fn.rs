use super::*;

/// Computes a clean 1-2-5 nice step interval for axis tick generation.
///
/// # Arguments
///
/// - `f32` - Raw positive numeric span.
/// - `usize` - Target number of split intervals.
///
/// # Returns
///
/// - `f32` - Nice human-readable step magnitude.
#[must_use]
pub fn compute_nice_step(span: f32, split_number: usize) -> f32 {
    let safe_span: f32 = if span.is_finite() && span > 1e-6 {
        span
    } else {
        1.0
    };
    let splits: f32 = (split_number.max(1)) as f32;
    let rough: f32 = safe_span / splits;
    let exponent: f32 = rough.log10().floor();
    let mag: f32 = 10.0_f32.powf(exponent);
    let residual: f32 = rough / mag;
    let nice_mult: f32 = if residual <= 1.0 {
        1.0
    } else if residual <= 2.0 {
        2.0
    } else if residual <= 2.5 {
        2.5
    } else if residual <= 5.0 {
        5.0
    } else {
        10.0
    };
    nice_mult * mag
}

/// Formats a numeric scalar cleanly with compact `k`/`M`/`B` suffixes and optional unit suffix.
///
/// # Arguments
///
/// - `f32` - Numeric value to format.
/// - `&str` - Optional unit suffix (e.g. `"%"`, `"ms"`).
///
/// # Returns
///
/// - `String` - Formatted human-readable number string.
#[must_use]
pub fn format_chart_number(value: f32, unit_suffix: &str) -> String {
    if !value.is_finite() {
        return String::new();
    }
    let abs_v: f32 = value.abs();
    let base: String = if abs_v >= 1_000_000_000.0 {
        format!("{:.1}{}", value / 1_000_000_000.0, SCALE_SUFFIX_B)
    } else if abs_v >= 1_000_000.0 {
        format!("{:.1}{}", value / 1_000_000.0, SCALE_SUFFIX_M)
    } else if abs_v >= 10_000.0 {
        format!("{:.1}{}", value / 1_000.0, SCALE_SUFFIX_K)
    } else if (value - value.round()).abs() < 1e-3 {
        format!("{:.0}", value.round())
    } else if abs_v >= 100.0 {
        format!("{:.1}", value)
    } else {
        format!("{:.2}", value)
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    };
    if unit_suffix.is_empty() {
        base
    } else {
        format!("{}{}", base, unit_suffix)
    }
}

/// Formats a numeric timestamp on a `Time` axis into a concise `MM/DD` or `HH:MM` label.
///
/// # Arguments
///
/// - `f32` - Timestamp value (Unix seconds, day index, or hour index).
/// - `f32` - Full visible time span on the axis.
///
/// # Returns
///
/// - `String` - Formatted time tick label.
#[must_use]
pub fn format_chart_time(value: f32, span: f32) -> String {
    if !value.is_finite() {
        return String::new();
    }
    if value.abs() >= SCALE_UNIX_SECONDS_MIN {
        let total_days: i64 = (value as i64).div_euclid(86_400);
        let month_idx: usize = (total_days.div_euclid(30).rem_euclid(12)) as usize;
        let day_num: i64 = total_days.rem_euclid(30) + 1;
        if span <= 86_400.0 * 2.0 {
            let secs_in_day: i64 = (value as i64).rem_euclid(86_400);
            let hh: i64 = secs_in_day / 3600;
            let mm: i64 = (secs_in_day % 3600) / 60;
            return format!("{:02}:{:02}", hh, mm);
        }
        return format!("{} {:02}", SCALE_MONTH_NAMES[month_idx], day_num);
    }
    if span <= 48.0 {
        let total_mins: i64 = (value * 60.0).round() as i64;
        let hh: i64 = total_mins.div_euclid(60).rem_euclid(24);
        let mm: i64 = total_mins.rem_euclid(60);
        format!("{:02}:{:02}", hh, mm)
    } else {
        let day_idx: i64 = value.round() as i64;
        let month_idx: usize = (day_idx.div_euclid(30).rem_euclid(12)) as usize;
        let day_num: i64 = day_idx.rem_euclid(30) + 1;
        format!("{} {:02}", SCALE_MONTH_NAMES[month_idx], day_num)
    }
}

/// Computes an `EuvChartAxisScale` supporting `Category`, `Value`, `Time`, and `Log` domains with nice 1-2-5 ticks.
///
/// # Arguments
///
/// - `&EuvChartAxis` - Declarative axis configuration.
/// - `f32` - Observed data minimum across bound visible series.
/// - `f32` - Observed data maximum across bound visible series.
/// - `bool` - When `true`, anchors zero inside numeric bar/area ranges unless overridden.
/// - `Option<EuvChartZoomSlice>` - Optional active `DataZoom` window applied to category or domain bounds.
///
/// # Returns
///
/// - `EuvChartAxisScale` - Fully resolved axis scale with formatted tick labels.
#[must_use]
pub fn build_axis_scale(
    axis: &EuvChartAxis,
    data_min: f32,
    data_max: f32,
    anchor_zero: bool,
    zoom: Option<EuvChartZoomSlice>,
) -> EuvChartAxisScale {
    let split_count: usize = axis.get_split_number().clamp(2, 12);
    match axis.get_axis_type() {
        EuvChartAxisType::Category => {
            let all_cats: &[String] = axis.get_categories().as_slice();
            let (start_i, end_i): (usize, usize) = if let Some(z) = zoom {
                let len: usize = all_cats.len();
                let s: usize = z.get_start_index().min(len);
                let e: usize = z.get_end_index().clamp(s, len);
                if s < e { (s, e) } else { (0, len) }
            } else {
                (0, all_cats.len())
            };
            let visible_cats: Vec<String> = all_cats[start_i..end_i].to_vec();
            let count: usize = visible_cats.len();
            let mut ticks: Vec<f32> = Vec::with_capacity(count);
            for i in 0..count {
                ticks.push(i as f32);
            }
            let max_idx: f32 = if count > 0 { (count - 1) as f32 } else { 1.0 };
            EuvChartAxisScale {
                axis_type: EuvChartAxisType::Category,
                min: 0.0,
                max: max_idx,
                inverse: axis.get_inverse(),
                ticks,
                labels: visible_cats.clone(),
                categories: visible_cats,
            }
        }
        EuvChartAxisType::Log => {
            let safe_lo: f32 = axis
                .min_opt()
                .unwrap_or(data_min)
                .max(SCALE_LOG_MIN_POSITIVE);
            let safe_hi: f32 = axis.max_opt().unwrap_or(data_max).max(safe_lo * 10.0);
            let min_exp: i32 = safe_lo.log10().floor() as i32;
            let max_exp: i32 = (safe_hi.log10().ceil() as i32).max(min_exp + 1);
            let mut ticks: Vec<f32> = Vec::new();
            let mut labels: Vec<String> = Vec::new();
            let mut exp: i32 = min_exp;
            while exp <= max_exp && ticks.len() < 16 {
                let v: f32 = 10.0_f32.powi(exp);
                ticks.push(v);
                labels.push(format_chart_number(v, axis.get_unit_suffix().as_str()));
                exp += 1;
            }
            EuvChartAxisScale {
                axis_type: EuvChartAxisType::Log,
                min: 10.0_f32.powi(min_exp),
                max: 10.0_f32.powi(max_exp),
                inverse: axis.get_inverse(),
                ticks,
                labels,
                categories: Vec::new(),
            }
        }
        EuvChartAxisType::Value | EuvChartAxisType::Time => {
            let mut lo: f32 = if data_min.is_finite() { data_min } else { 0.0 };
            let mut hi: f32 = if data_max.is_finite() {
                data_max
            } else {
                100.0
            };
            if anchor_zero && axis.min_opt().is_none() && lo > 0.0 {
                lo = 0.0;
            }
            if anchor_zero && axis.max_opt().is_none() && hi < 0.0 {
                hi = 0.0;
            }
            if let Some(exp_min) = axis.min_opt() {
                lo = exp_min;
            }
            if let Some(exp_max) = axis.max_opt() {
                hi = exp_max;
            }
            if (hi - lo).abs() < 1e-5 {
                let pad: f32 = if hi.abs() > 1.0 { hi.abs() * 0.2 } else { 1.0 };
                lo -= pad;
                hi += pad;
            }
            if let Some(z) = zoom {
                let span_full: f32 = hi - lo;
                let z_lo: f32 = lo + span_full * (z.get_start_pct() * 0.01);
                let z_hi: f32 = lo + span_full * (z.get_end_pct() * 0.01);
                if z_hi - z_lo > 1e-5 {
                    lo = z_lo;
                    hi = z_hi;
                }
            }
            let step: f32 = compute_nice_step(hi - lo, split_count);
            let nice_min: f32 = if axis.min_opt().is_some() {
                lo
            } else {
                (lo / step).floor() * step
            };
            let nice_max: f32 = if axis.max_opt().is_some() {
                hi
            } else {
                (hi / step).ceil() * step
            };
            let span: f32 = (nice_max - nice_min).max(step);
            let mut ticks: Vec<f32> = Vec::new();
            let mut labels: Vec<String> = Vec::new();
            let is_time: bool = axis.get_axis_type() == EuvChartAxisType::Time;
            let count_steps: usize = ((span / step).round() as usize).clamp(1, 14);
            for i in 0..=count_steps {
                let tv: f32 = nice_min + (i as f32) * (span / (count_steps as f32));
                ticks.push(tv);
                let lbl: String = if is_time {
                    format_chart_time(tv, span)
                } else {
                    format_chart_number(tv, axis.get_unit_suffix().as_str())
                };
                labels.push(lbl);
            }
            EuvChartAxisScale {
                axis_type: axis.get_axis_type(),
                min: nice_min,
                max: nice_max,
                inverse: axis.get_inverse(),
                ticks,
                labels,
                categories: Vec::new(),
            }
        }
    }
}

/// Computes an index slice window for `total_len` category items from percentage bounds `[start_pct, end_pct]`.
///
/// # Arguments
///
/// - `usize` - Total item count before zooming.
/// - `f32` - Start percentage in `[0.0, 100.0]`.
/// - `f32` - End percentage in `[0.0, 100.0]`.
///
/// # Returns
///
/// - `EuvChartZoomSlice` - Clamped index slice window.
#[must_use]
pub fn compute_zoom_slice(total_len: usize, start_pct: f32, end_pct: f32) -> EuvChartZoomSlice {
    let lo_pct: f32 = start_pct.clamp(0.0, 99.0);
    let hi_pct: f32 = end_pct.clamp(lo_pct + 1.0, 100.0);
    if total_len == 0 {
        return EuvChartZoomSlice {
            start_pct: lo_pct,
            end_pct: hi_pct,
            start_index: 0,
            end_index: 0,
        };
    }
    let start_idx: usize =
        (((total_len as f32) * lo_pct * 0.01).floor() as usize).min(total_len.saturating_sub(1));
    let raw_end: usize = (((total_len as f32) * hi_pct * 0.01).ceil() as usize).min(total_len);
    let end_idx: usize = raw_end.max((start_idx + 1).min(total_len));
    EuvChartZoomSlice {
        start_pct: lo_pct,
        end_pct: hi_pct,
        start_index: start_idx,
        end_index: end_idx,
    }
}

/// Builds a ready-to-evaluate `EuvChartVisualEvaluator` from `visual_map` and active `tokens`.
///
/// # Arguments
///
/// - `Option<&EuvChartVisualMap>` - Optional declarative visual-map configuration.
/// - `f32` - Observed data minimum fallback when visual-map bounds collapse.
/// - `f32` - Observed data maximum fallback when visual-map bounds collapse.
/// - `ThemeTokens` - Active UI theme tokens supplying Light/Dark default ramps.
///
/// # Returns
///
/// - `EuvChartVisualEvaluator` - Ready color evaluator.
#[must_use]
pub fn build_visual_evaluator(
    visual_map: Option<&EuvChartVisualMap>,
    data_min: f32,
    data_max: f32,
    tokens: ThemeTokens,
) -> EuvChartVisualEvaluator {
    let default_ramp: &[u32; 4] = match tokens.get_mode() {
        ThemeMode::Light => &ECHART_VISUAL_RAMP_LIGHT,
        ThemeMode::Dark => &ECHART_VISUAL_RAMP_DARK,
    };
    let default_stops: Vec<Color> = default_ramp.iter().map(|&h: &u32| Color::hex(h)).collect();
    if let Some(vm) = visual_map {
        let stops: Vec<Color> = if vm.get_in_range_colors().is_empty() {
            default_stops
        } else {
            vm.get_in_range_colors().clone()
        };
        let lo: f32 = vm.get_min();
        let hi: f32 = if (vm.get_max() - lo).abs() > 1e-6 {
            vm.get_max()
        } else {
            data_max.max(lo + 1.0)
        };
        EuvChartVisualEvaluator {
            kind: vm.get_kind(),
            min: lo,
            max: hi,
            stops,
            pieces: vm.get_pieces().clone(),
        }
    } else {
        let lo: f32 = if data_min.is_finite() { data_min } else { 0.0 };
        let hi: f32 = if data_max.is_finite() && data_max > lo {
            data_max
        } else {
            lo + 100.0
        };
        EuvChartVisualEvaluator {
            kind: EuvChartVisualMapKind::Continuous,
            min: lo,
            max: hi,
            stops: default_stops,
            pieces: Vec::new(),
        }
    }
}

/// Extracts category labels from an `EuvChartDataset` column matching `dim`.
///
/// # Arguments
///
/// - `&EuvChartDataset` - Shared tabular dataset.
/// - `&str` - Dimension name or column index string.
///
/// # Returns
///
/// - `Vec<String>` - Extracted category strings per row.
#[must_use]
pub fn dataset_extract_labels(dataset: &EuvChartDataset, dim: &str) -> Vec<String> {
    let Some(col) = dataset.column_index(dim) else {
        return Vec::new();
    };
    let mut out: Vec<String> = Vec::with_capacity(dataset.get_rows().len());
    for row in dataset.get_rows().iter() {
        let cell_str: String = match row.get(col) {
            Some(EuvChartCell::Text(s)) => s.clone(),
            Some(EuvChartCell::Number(v)) => format_chart_number(*v, ""),
            Some(EuvChartCell::Null) | None => String::new(),
        };
        out.push(cell_str);
    }
    out
}

/// Extracts nullable numeric values from an `EuvChartDataset` column matching `dim`.
///
/// # Arguments
///
/// - `&EuvChartDataset` - Shared tabular dataset.
/// - `&str` - Dimension name or column index string.
///
/// # Returns
///
/// - `Vec<Option<f32>>` - Extracted scalar values per row.
#[must_use]
pub fn dataset_extract_numbers(dataset: &EuvChartDataset, dim: &str) -> Vec<Option<f32>> {
    let Some(col) = dataset.column_index(dim) else {
        return Vec::new();
    };
    let mut out: Vec<Option<f32>> = Vec::with_capacity(dataset.get_rows().len());
    for row in dataset.get_rows().iter() {
        let val: Option<f32> = match row.get(col) {
            Some(EuvChartCell::Number(v)) if v.is_finite() => Some(*v),
            _ => None,
        };
        out.push(val);
    }
    out
}

/// Extracts scatter `(x, y, size, value, label)` points from an `EuvChartDataset` using `encode`.
///
/// # Arguments
///
/// - `&EuvChartDataset` - Shared tabular dataset.
/// - `&EuvChartEncode` - Column encoding mapping.
/// - `f32` - Fallback default symbol radius.
///
/// # Returns
///
/// - `Vec<EuvChartScatterPoint>` - Extracted scatter points.
#[must_use]
pub fn dataset_extract_scatter(
    dataset: &EuvChartDataset,
    encode: &EuvChartEncode,
    default_size: f32,
) -> Vec<EuvChartScatterPoint> {
    let x_vals: Vec<Option<f32>> = encode
        .x_opt()
        .map(|d: &str| dataset_extract_numbers(dataset, d))
        .unwrap_or_default();
    let y_vals: Vec<Option<f32>> = encode
        .y_opt()
        .or_else(|| encode.value_opt())
        .map(|d: &str| dataset_extract_numbers(dataset, d))
        .unwrap_or_default();
    let sz_vals: Vec<Option<f32>> = encode
        .size_opt()
        .map(|d: &str| dataset_extract_numbers(dataset, d))
        .unwrap_or_default();
    let val_vals: Vec<Option<f32>> = encode
        .value_opt()
        .map(|d: &str| dataset_extract_numbers(dataset, d))
        .unwrap_or_default();
    let lbls: Vec<String> = encode
        .item_name_opt()
        .map(|d: &str| dataset_extract_labels(dataset, d))
        .unwrap_or_default();
    let row_count: usize = dataset.get_rows().len();
    let mut out: Vec<EuvChartScatterPoint> = Vec::with_capacity(row_count);
    for i in 0..row_count {
        let xv: f32 = x_vals
            .get(i)
            .and_then(|v: &Option<f32>| *v)
            .unwrap_or(i as f32);
        let Some(yv) = y_vals.get(i).and_then(|v: &Option<f32>| *v) else {
            continue;
        };
        let sz: f32 = sz_vals
            .get(i)
            .and_then(|v: &Option<f32>| *v)
            .unwrap_or(default_size);
        let mut pt: EuvChartScatterPoint = EuvChartScatterPoint::new(xv, yv).size(sz);
        if let Some(mv) = val_vals.get(i).and_then(|v: &Option<f32>| *v) {
            pt = pt.value(mv);
        }
        if let Some(lbl) = lbls.get(i) {
            pt = pt.label(lbl.clone());
        }
        out.push(pt);
    }
    out
}
