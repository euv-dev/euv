use super::*;

/// Parses raw Mermaid source text into a structured `MermaidDiagram` AST covering flowchart, sequence, state, class, ER, pie, and Gantt diagrams.
///
/// # Arguments
///
/// - `&str` - Raw Mermaid diagram definition.
///
/// # Returns
///
/// - `MermaidDiagram` - Parsed diagram ready for native vector rendering.
#[must_use]
pub fn parse_mermaid_diagram(source: &str) -> MermaidDiagram {
    let lines: Vec<&str> = source
        .lines()
        .map(str::trim)
        .filter(|l: &&str| !l.is_empty() && !l.starts_with("%%"))
        .collect();
    let header_low: String = lines
        .first()
        .map(|l: &&str| l.to_ascii_lowercase())
        .unwrap_or_default();
    if header_low.starts_with(MERMAID_KW_SEQUENCE) {
        MermaidDiagram::Sequence(parse_mermaid_sequence(&lines))
    } else if header_low.starts_with(MERMAID_KW_STATE) {
        MermaidDiagram::State(parse_mermaid_state(&lines))
    } else if header_low.starts_with(MERMAID_KW_CLASS) {
        MermaidDiagram::Class(parse_mermaid_class(&lines))
    } else if header_low.starts_with(MERMAID_KW_ER) {
        MermaidDiagram::Er(parse_mermaid_er(&lines))
    } else if header_low.starts_with(MERMAID_KW_PIE) {
        MermaidDiagram::Pie(parse_mermaid_pie(&lines))
    } else if header_low.starts_with(MERMAID_KW_GANTT) {
        MermaidDiagram::Gantt(parse_mermaid_gantt(&lines))
    } else {
        MermaidDiagram::Flowchart(parse_mermaid_flowchart(&lines))
    }
}

/// Computes the intrinsic logical canvas `(width, height)` required to draw `diagram` cleanly with padding.
///
/// # Arguments
///
/// - `&MermaidDiagram` - Parsed Mermaid diagram AST.
///
/// # Returns
///
/// - `Size` - Intrinsic logical canvas dimensions in pixels.
#[must_use]
pub fn measure_mermaid_diagram(diagram: &MermaidDiagram) -> Size {
    match diagram {
        MermaidDiagram::Flowchart(fc) | MermaidDiagram::State(fc) => measure_flowchart(fc),
        MermaidDiagram::Sequence(seq) => {
            let p_cnt: usize = seq.get_participants().len().max(2);
            let m_cnt: usize = seq.get_messages().len().max(1);
            let w: f32 = ((p_cnt as f32) * 168.0 + 64.0).max(380.0);
            let h: f32 = ((m_cnt as f32) * 46.0 + 152.0).max(200.0);
            Size::new(w, h)
        }
        MermaidDiagram::Class(cls) => {
            let cols: usize = cls.get_classes().len().clamp(1, 3);
            let rows: usize = cls.get_classes().len().div_ceil(cols).max(1);
            let max_members: usize = cls
                .get_classes()
                .iter()
                .map(|c: &MermaidClassBox| c.get_members().len())
                .max()
                .unwrap_or(2);
            let box_h: f32 = 44.0 + (max_members as f32) * 18.0;
            let w: f32 = ((cols as f32) * 236.0 + 56.0).max(380.0);
            let h: f32 = ((rows as f32) * (box_h + 56.0) + 56.0).max(200.0);
            Size::new(w, h)
        }
        MermaidDiagram::Er(er) => {
            let cols: usize = er.get_entities().len().clamp(1, 3);
            let rows: usize = er.get_entities().len().div_ceil(cols).max(1);
            let max_attr: usize = er
                .get_entities()
                .iter()
                .map(|e: &MermaidErEntity| e.get_attributes().len())
                .max()
                .unwrap_or(2);
            let box_h: f32 = 42.0 + (max_attr as f32) * 18.0;
            let w: f32 = ((cols as f32) * 244.0 + 56.0).max(380.0);
            let h: f32 = ((rows as f32) * (box_h + 56.0) + 56.0).max(200.0);
            Size::new(w, h)
        }
        MermaidDiagram::Pie(pie) => {
            let legend_h: f32 = (pie.get_slices().len().max(1) as f32) * 26.0 + 72.0;
            let h: f32 = legend_h.max(250.0);
            Size::new(560.0, h)
        }
        MermaidDiagram::Gantt(gantt) => {
            let row_cnt: usize = gantt.get_tasks().len().max(1);
            let mut sec_cnt: usize = 0;
            let mut last_sec: &str = "";
            for task in gantt.get_tasks().iter() {
                if task.get_section().as_str() != last_sec {
                    sec_cnt += 1;
                    last_sec = task.get_section().as_str();
                }
            }
            let h: f32 = ((row_cnt as f32) * 32.0 + (sec_cnt as f32) * 26.0 + 96.0).max(180.0);
            Size::new(680.0, h)
        }
    }
}

/// Renders a parsed `MermaidDiagram` directly onto `dl` inside logical `bounds` using active theme tokens.
///
/// # Arguments
///
/// - `&MermaidDiagram` - Parsed Mermaid diagram AST.
/// - `Rect` - Target logical canvas bounds.
/// - `&mut DisplayList` - Active draw command buffer.
/// - `ThemeTokens` - Active theme tokens.
pub fn draw_mermaid_diagram(
    diagram: &MermaidDiagram,
    bounds: Rect,
    dl: &mut DisplayList,
    tokens: ThemeTokens,
) {
    let eva: CodeThemePalette = CodeThemePalette::resolve(tokens.get_mode());
    dl.push(DrawCommand::FillRoundedRect {
        rect: bounds,
        color: eva.get_background(),
        radius: CornerRadius::all(0.0),
        border: Border::default(),
    });
    match diagram {
        MermaidDiagram::Flowchart(fc) | MermaidDiagram::State(fc) => {
            draw_flowchart(fc, bounds, dl, tokens, eva);
        }
        MermaidDiagram::Sequence(seq) => draw_sequence(seq, bounds, dl, tokens, eva),
        MermaidDiagram::Class(cls) => draw_class_diagram(cls, bounds, dl, tokens, eva),
        MermaidDiagram::Er(er) => draw_er_diagram(er, bounds, dl, tokens, eva),
        MermaidDiagram::Pie(pie) => draw_pie_chart(pie, bounds, dl, tokens, eva),
        MermaidDiagram::Gantt(gantt) => draw_gantt_chart(gantt, bounds, dl, tokens, eva),
    }
}

/// Returns the uppercase header badge label identifying the parsed Mermaid diagram variant.
///
/// # Arguments
///
/// - `&MermaidDiagram` - Parsed Mermaid diagram AST.
///
/// # Returns
///
/// - `&'static str` - Header badge string.
#[must_use]
pub(crate) fn mermaid_badge_label(diagram: &MermaidDiagram) -> &'static str {
    match diagram {
        MermaidDiagram::Flowchart(_) => MERMAID_BADGE_FLOWCHART,
        MermaidDiagram::Sequence(_) => MERMAID_BADGE_SEQUENCE,
        MermaidDiagram::State(_) => MERMAID_BADGE_STATE,
        MermaidDiagram::Class(_) => MERMAID_BADGE_CLASS,
        MermaidDiagram::Er(_) => MERMAID_BADGE_ER,
        MermaidDiagram::Pie(_) => MERMAID_BADGE_PIE,
        MermaidDiagram::Gantt(_) => MERMAID_BADGE_GANTT,
    }
}

/// Extracts an optional diagram title from a parsed Mermaid diagram.
///
/// # Arguments
///
/// - `&MermaidDiagram` - Parsed Mermaid diagram AST.
///
/// # Returns
///
/// - `String` - Diagram title or empty string.
#[must_use]
pub(crate) fn mermaid_diagram_title(diagram: &MermaidDiagram) -> String {
    match diagram {
        MermaidDiagram::Pie(pie) => pie.get_title().clone(),
        MermaidDiagram::Gantt(gantt) => gantt.get_title().clone(),
        _ => String::new(),
    }
}

/// Parses a `flowchart` / `graph` Mermaid block including subgraphs, node shapes, and labeled connectors.
///
/// # Arguments
///
/// - `&[&str]` - Trimmed non-empty Mermaid lines.
///
/// # Returns
///
/// - `MermaidFlowchart` - Parsed flowchart structure.
fn parse_mermaid_flowchart(lines: &[&str]) -> MermaidFlowchart {
    let mut chart: MermaidFlowchart = MermaidFlowchart::default();
    let mut start_idx: usize = 0;
    if let Some(first) = lines.first() {
        let low: String = first.to_ascii_lowercase();
        if low.starts_with(MERMAID_KW_FLOWCHART) || low.starts_with(MERMAID_KW_GRAPH) {
            let dir_tok: &str = first.split_whitespace().nth(1).unwrap_or("TD");
            let dir: MermaidFlowDirection = match dir_tok.to_ascii_uppercase().as_str() {
                "LR" => MermaidFlowDirection::LeftRight,
                "RL" => MermaidFlowDirection::RightLeft,
                "BT" => MermaidFlowDirection::BottomTop,
                _ => MermaidFlowDirection::TopDown,
            };
            chart.set_direction(dir);
            start_idx = 1;
        }
    }
    let mut active_sg: Option<usize> = None;
    for raw_line in lines.iter().skip(start_idx) {
        let line: &str = raw_line.trim().trim_end_matches(';');
        let low: String = line.to_ascii_lowercase();
        if low.starts_with(MERMAID_KW_SUBGRAPH) {
            let rest: &str = line[MERMAID_KW_SUBGRAPH.len()..].trim();
            let title: String = if let Some(lb) = rest.find('[')
                && let Some(rb) = rest.rfind(']')
                && rb > lb
            {
                rest[lb + 1..rb].trim().trim_matches('"').to_string()
            } else {
                rest.trim_matches('"').to_string()
            };
            let idx: usize = chart.get_subgraphs().len();
            chart.get_mut_subgraphs().push(MermaidSubgraph::new(title));
            active_sg = Some(idx);
            continue;
        }
        if low == MERMAID_KW_END {
            active_sg = None;
            continue;
        }
        if low.starts_with("style ") || low.starts_with("classdef ") || low.starts_with("class ") {
            continue;
        }
        if let Some((lhs, style, label, rhs)) = split_flow_edge(line) {
            let from_id: String = register_flow_node(&mut chart, lhs, active_sg);
            let to_id: String = register_flow_node(&mut chart, rhs, active_sg);
            if !from_id.is_empty() && !to_id.is_empty() {
                chart
                    .get_mut_edges()
                    .push(MermaidFlowEdge::new(from_id, to_id, label, style));
            }
        } else {
            let _: String = register_flow_node(&mut chart, line, active_sg);
        }
    }
    chart
}

/// Splits a flowchart connector expression into `(lhs, edge_style, label, rhs)`.
///
/// # Arguments
///
/// - `&str` - Single flowchart statement line.
///
/// # Returns
///
/// - `Option<(&str, MermaidEdgeStyle, String, &str)>` - Parsed connector components when an edge operator is found.
fn split_flow_edge(line: &str) -> Option<(&str, MermaidEdgeStyle, String, &str)> {
    let ops: [(&str, MermaidEdgeStyle); 5] = [
        ("-.->", MermaidEdgeStyle::Dashed),
        ("==>", MermaidEdgeStyle::Thick),
        ("-->", MermaidEdgeStyle::Solid),
        ("---", MermaidEdgeStyle::Plain),
        ("->", MermaidEdgeStyle::Solid),
    ];
    for (op, style) in ops.into_iter() {
        if let Some(pos) = line.find(op) {
            let lhs: &str = line[..pos].trim();
            let after: &str = line[pos + op.len()..].trim();
            if lhs.is_empty() || after.is_empty() {
                continue;
            }
            if let Some(stripped) = after.strip_prefix('|')
                && let Some(end_pipe) = stripped.find('|')
            {
                let lbl: String = stripped[..end_pipe].trim().to_string();
                let rhs: &str = stripped[end_pipe + 1..].trim();
                return Some((lhs, style, lbl, rhs));
            }
            return Some((lhs, style, String::new(), after));
        }
    }
    if let Some(start_lbl) = line.find(" -- ")
        && let Some(end_lbl) = line[start_lbl + 4..].find(" --> ")
    {
        let lhs: &str = line[..start_lbl].trim();
        let lbl: String = line[start_lbl + 4..start_lbl + 4 + end_lbl]
            .trim()
            .to_string();
        let rhs: &str = line[start_lbl + 4 + end_lbl + 5..].trim();
        return Some((lhs, MermaidEdgeStyle::Solid, lbl, rhs));
    }
    None
}

/// Parses a node token (`A[Label]`, `B(Label)`, `C{Label}`, `D([Label])`, `E((Label))`) and registers or updates it inside `chart`.
///
/// # Arguments
///
/// - `&mut MermaidFlowchart` - Mutable destination flowchart.
/// - `&str` - Raw node token string.
/// - `Option<usize>` - Active enclosing subgraph index.
///
/// # Returns
///
/// - `String` - Normalized node identifier.
fn register_flow_node(chart: &mut MermaidFlowchart, raw: &str, subgraph: Option<usize>) -> String {
    let trimmed: &str = raw.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    let (id, label, shape, explicit): (String, String, MermaidNodeShape, bool) =
        parse_flow_node_token(trimmed);
    if id.is_empty() {
        return String::new();
    }
    if let Some(existing) = chart
        .get_mut_nodes()
        .iter_mut()
        .find(|n: &&mut MermaidFlowNode| n.get_id().as_str() == id.as_str())
    {
        if explicit {
            existing.set_label(label);
            existing.set_shape(shape);
        }
        if existing.subgraph.is_none() && subgraph.is_some() {
            existing.subgraph = subgraph;
        }
        return id;
    }
    chart.get_mut_nodes().push(MermaidFlowNode {
        id: id.clone(),
        label,
        shape,
        subgraph,
    });
    id
}

/// Parses a single flowchart node declaration into `(id, label, shape, has_explicit_label)`.
///
/// # Arguments
///
/// - `&str` - Trimmed node token.
///
/// # Returns
///
/// - `(String, String, MermaidNodeShape, bool)` - Parsed node identifier, label, geometric shape, and whether shape syntax was explicit.
fn parse_flow_node_token(token: &str) -> (String, String, MermaidNodeShape, bool) {
    let shapes: [(&str, &str, MermaidNodeShape); 5] = [
        ("([", "])", MermaidNodeShape::Stadium),
        ("((", "))", MermaidNodeShape::Circle),
        ("[", "]", MermaidNodeShape::Box),
        ("(", ")", MermaidNodeShape::Rounded),
        ("{", "}", MermaidNodeShape::Diamond),
    ];
    for (open, close, shape) in shapes.into_iter() {
        if let Some(open_idx) = token.find(open)
            && let Some(close_idx) = token.rfind(close)
            && close_idx > open_idx + open.len() - 1
        {
            let id: String = token[..open_idx].trim().to_string();
            let raw_lbl: &str = token[open_idx + open.len()..close_idx].trim();
            let lbl: String = raw_lbl.trim_matches('"').to_string();
            let resolved_id: String = if id.is_empty() { lbl.clone() } else { id };
            return (resolved_id, lbl, shape, true);
        }
    }
    let clean: String = token.trim_matches('"').to_string();
    (clean.clone(), clean, MermaidNodeShape::Box, false)
}

/// Parses a Mermaid `sequenceDiagram` block.
///
/// # Arguments
///
/// - `&[&str]` - Trimmed non-empty Mermaid lines.
///
/// # Returns
///
/// - `MermaidSequence` - Parsed participants and chronological message steps.
fn parse_mermaid_sequence(lines: &[&str]) -> MermaidSequence {
    let mut seq: MermaidSequence = MermaidSequence::default();
    for raw in lines.iter().skip(1) {
        let line: &str = raw.trim();
        let low: String = line.to_ascii_lowercase();
        if low.starts_with(MERMAID_KW_PARTICIPANT) || low.starts_with(MERMAID_KW_ACTOR) {
            let prefix_len: usize = if low.starts_with(MERMAID_KW_PARTICIPANT) {
                MERMAID_KW_PARTICIPANT.len()
            } else {
                MERMAID_KW_ACTOR.len()
            };
            let rest: &str = line[prefix_len..].trim();
            let (id, label): (String, String) =
                if let Some((a, b)) = rest.split_once(MERMAID_ALIAS_SEP) {
                    (a.trim().to_string(), b.trim().to_string())
                } else {
                    (rest.to_string(), rest.to_string())
                };
            let _: usize = ensure_seq_participant(&mut seq, &id, Some(&label));
            continue;
        }
        if low.starts_with(MERMAID_KW_NOTE) {
            if let Some((hdr, text)) = line.split_once(':') {
                let hdr_tokens: Vec<&str> = hdr.split_whitespace().collect();
                let target_spec: &str = hdr_tokens.last().copied().unwrap_or("");
                let (from_i, to_i): (usize, usize) =
                    if let Some((a, b)) = target_spec.split_once(',') {
                        (
                            ensure_seq_participant(&mut seq, a.trim(), None),
                            ensure_seq_participant(&mut seq, b.trim(), None),
                        )
                    } else if !target_spec.is_empty() {
                        let idx: usize = ensure_seq_participant(&mut seq, target_spec, None);
                        (idx, idx)
                    } else {
                        (0, 0)
                    };
                seq.get_mut_messages().push(MermaidSequenceMessage::new(
                    from_i,
                    to_i,
                    text.trim().to_string(),
                    MermaidEdgeStyle::Solid,
                    true,
                ));
            }
            continue;
        }
        let seq_ops: [(&str, MermaidEdgeStyle); 6] = [
            ("-->>", MermaidEdgeStyle::Dashed),
            ("->>", MermaidEdgeStyle::Solid),
            ("-->", MermaidEdgeStyle::Dashed),
            ("->", MermaidEdgeStyle::Solid),
            ("--x", MermaidEdgeStyle::Dashed),
            ("-x", MermaidEdgeStyle::Solid),
        ];
        for (op, style) in seq_ops.into_iter() {
            if let Some(pos) = line.find(op) {
                let lhs: &str = line[..pos].trim();
                let rhs_all: &str = line[pos + op.len()..].trim();
                let (rhs_raw, msg): (&str, &str) = match rhs_all.split_once(':') {
                    Some((r, m)) => (r.trim(), m.trim()),
                    None => (rhs_all, ""),
                };
                let rhs: &str = rhs_raw.trim_start_matches(['+', '-']).trim();
                if !lhs.is_empty() && !rhs.is_empty() {
                    let from_idx: usize = ensure_seq_participant(&mut seq, lhs, None);
                    let to_idx: usize = ensure_seq_participant(&mut seq, rhs, None);
                    seq.get_mut_messages().push(MermaidSequenceMessage::new(
                        from_idx,
                        to_idx,
                        msg.to_string(),
                        style,
                        false,
                    ));
                }
                break;
            }
        }
    }
    seq
}

/// Ensures participant `id` exists inside `seq`, returning its column index.
///
/// # Arguments
///
/// - `&mut MermaidSequence` - Sequence diagram builder.
/// - `&str` - Participant identifier.
/// - `Option<&str>` - Optional explicit display alias.
///
/// # Returns
///
/// - `usize` - Zero-based participant index.
fn ensure_seq_participant(seq: &mut MermaidSequence, id: &str, label: Option<&str>) -> usize {
    if let Some(pos) = seq
        .get_participants()
        .iter()
        .position(|(pid, _): &(String, String)| pid.as_str() == id)
    {
        if let Some(lbl) = label {
            seq.get_mut_participants()[pos].1 = lbl.to_string();
        }
        return pos;
    }
    let idx: usize = seq.get_participants().len();
    let display: String = label.unwrap_or(id).to_string();
    seq.get_mut_participants().push((id.to_string(), display));
    idx
}

/// Parses a Mermaid `stateDiagram` / `stateDiagram-v2` block into a `MermaidFlowchart` with terminal state shapes.
///
/// # Arguments
///
/// - `&[&str]` - Trimmed non-empty Mermaid lines.
///
/// # Returns
///
/// - `MermaidFlowchart` - Parsed state transition graph.
fn parse_mermaid_state(lines: &[&str]) -> MermaidFlowchart {
    let mut chart: MermaidFlowchart = MermaidFlowchart::default();
    let mut terminal_counter: usize = 0;
    for raw in lines.iter().skip(1) {
        let line: &str = raw.trim();
        let low: String = line.to_ascii_lowercase();
        if low.starts_with(MERMAID_KW_STATE_DECL)
            && let Some((lhs, rhs)) =
                line[MERMAID_KW_STATE_DECL.len()..].split_once(MERMAID_ALIAS_SEP)
        {
            let lbl: String = lhs.trim().trim_matches('"').to_string();
            let id: String = rhs.trim().to_string();
            chart.get_mut_nodes().push(MermaidFlowNode {
                id,
                label: lbl,
                shape: MermaidNodeShape::Rounded,
                subgraph: None,
            });
            continue;
        }
        if let Some((edge_part, lbl_part)) = line.split_once(':')
            && edge_part.contains("-->")
        {
            let (from_raw, to_raw): (&str, &str) = edge_part.split_once("-->").unwrap_or(("", ""));
            let from_id: String =
                register_state_node(&mut chart, from_raw.trim(), true, &mut terminal_counter);
            let to_id: String =
                register_state_node(&mut chart, to_raw.trim(), false, &mut terminal_counter);
            chart.get_mut_edges().push(MermaidFlowEdge::new(
                from_id,
                to_id,
                lbl_part.trim().to_string(),
                MermaidEdgeStyle::Solid,
            ));
        } else if let Some((from_raw, to_raw)) = line.split_once("-->") {
            let from_id: String =
                register_state_node(&mut chart, from_raw.trim(), true, &mut terminal_counter);
            let to_id: String =
                register_state_node(&mut chart, to_raw.trim(), false, &mut terminal_counter);
            chart.get_mut_edges().push(MermaidFlowEdge::new(
                from_id,
                to_id,
                String::new(),
                MermaidEdgeStyle::Solid,
            ));
        }
    }
    chart
}

/// Registers a state node (or unique `[*]` start/end terminal) in a state flowchart.
///
/// # Arguments
///
/// - `&mut MermaidFlowchart` - Destination state graph.
/// - `&str` - Raw state token.
/// - `bool` - `true` when used as source (`Start`), `false` when used as destination (`End`).
/// - `&mut usize` - Monotonic terminal counter for distinct `[*]` markers.
///
/// # Returns
///
/// - `String` - Normalized state node id.
fn register_state_node(
    chart: &mut MermaidFlowchart,
    raw: &str,
    is_start: bool,
    terminal_counter: &mut usize,
) -> String {
    if raw == MERMAID_STATE_TERMINAL {
        *terminal_counter += 1;
        let id: String = if is_start {
            format!("__start_{}", *terminal_counter)
        } else {
            format!("__end_{}", *terminal_counter)
        };
        let shape: MermaidNodeShape = if is_start {
            MermaidNodeShape::StateStart
        } else {
            MermaidNodeShape::StateEnd
        };
        chart.get_mut_nodes().push(MermaidFlowNode {
            id: id.clone(),
            label: String::new(),
            shape,
            subgraph: None,
        });
        return id;
    }
    if chart
        .get_nodes()
        .iter()
        .any(|n: &MermaidFlowNode| n.get_id().as_str() == raw)
    {
        return raw.to_string();
    }
    chart.get_mut_nodes().push(MermaidFlowNode {
        id: raw.to_string(),
        label: raw.to_string(),
        shape: MermaidNodeShape::Rounded,
        subgraph: None,
    });
    raw.to_string()
}

/// Parses a Mermaid `classDiagram` block.
///
/// # Arguments
///
/// - `&[&str]` - Trimmed non-empty Mermaid lines.
///
/// # Returns
///
/// - `MermaidClassDiagram` - Parsed UML classes and relationships.
fn parse_mermaid_class(lines: &[&str]) -> MermaidClassDiagram {
    let mut diag: MermaidClassDiagram = MermaidClassDiagram::default();
    let mut active_class: Option<usize> = None;
    for raw in lines.iter().skip(1) {
        let line: &str = raw.trim();
        if line == "}" {
            active_class = None;
            continue;
        }
        if let Some(cls_idx) = active_class {
            diag.get_mut_classes()[cls_idx]
                .get_mut_members()
                .push(line.to_string());
            continue;
        }
        let low: String = line.to_ascii_lowercase();
        if low.starts_with(MERMAID_KW_CLASS_DECL) {
            let rest: &str = line[MERMAID_KW_CLASS_DECL.len()..].trim();
            let opens_block: bool = rest.ends_with('{');
            let name: String = rest.trim_end_matches('{').trim().to_string();
            let idx: usize = ensure_class_box(&mut diag, &name);
            if opens_block {
                active_class = Some(idx);
            }
            continue;
        }
        let rel_ops: [(&str, MermaidEdgeStyle); 8] = [
            ("<|--", MermaidEdgeStyle::Solid),
            ("--|>", MermaidEdgeStyle::Solid),
            ("<|..", MermaidEdgeStyle::Dashed),
            ("..|>", MermaidEdgeStyle::Dashed),
            ("*--", MermaidEdgeStyle::Solid),
            ("o--", MermaidEdgeStyle::Solid),
            ("-->", MermaidEdgeStyle::Solid),
            ("..>", MermaidEdgeStyle::Dashed),
        ];
        let mut matched_rel: bool = false;
        for (op, style) in rel_ops.into_iter() {
            if let Some(pos) = line.find(op) {
                let lhs: &str = line[..pos].trim();
                let rhs_all: &str = line[pos + op.len()..].trim();
                let (rhs, lbl): (&str, &str) = match rhs_all.split_once(':') {
                    Some((r, l)) => (r.trim(), l.trim()),
                    None => (rhs_all, ""),
                };
                if !lhs.is_empty() && !rhs.is_empty() {
                    let _: usize = ensure_class_box(&mut diag, lhs);
                    let _: usize = ensure_class_box(&mut diag, rhs);
                    diag.get_mut_relations().push(MermaidClassRelation::new(
                        lhs.to_string(),
                        rhs.to_string(),
                        lbl.to_string(),
                        style,
                    ));
                    matched_rel = true;
                }
                break;
            }
        }
        if !matched_rel && let Some((cls_name, member)) = line.split_once(':') {
            let idx: usize = ensure_class_box(&mut diag, cls_name.trim());
            diag.get_mut_classes()[idx]
                .get_mut_members()
                .push(member.trim().to_string());
        }
    }
    diag
}

/// Ensures class `name` exists inside `diag`, returning its index.
///
/// # Arguments
///
/// - `&mut MermaidClassDiagram` - Class diagram builder.
/// - `&str` - Class name.
///
/// # Returns
///
/// - `usize` - Class box index.
fn ensure_class_box(diag: &mut MermaidClassDiagram, name: &str) -> usize {
    if let Some(pos) = diag
        .get_classes()
        .iter()
        .position(|c: &MermaidClassBox| c.get_name().as_str() == name)
    {
        return pos;
    }
    let idx: usize = diag.get_classes().len();
    diag.get_mut_classes()
        .push(MermaidClassBox::new(name.to_string(), Vec::new()));
    idx
}

/// Parses a Mermaid `erDiagram` block.
///
/// # Arguments
///
/// - `&[&str]` - Trimmed non-empty Mermaid lines.
///
/// # Returns
///
/// - `MermaidErDiagram` - Parsed ER entities and cardinality relationships.
fn parse_mermaid_er(lines: &[&str]) -> MermaidErDiagram {
    let mut diag: MermaidErDiagram = MermaidErDiagram::default();
    let mut active_ent: Option<usize> = None;
    for raw in lines.iter().skip(1) {
        let line: &str = raw.trim();
        if line == "}" {
            active_ent = None;
            continue;
        }
        if let Some(ent_idx) = active_ent {
            diag.get_mut_entities()[ent_idx]
                .get_mut_attributes()
                .push(line.to_string());
            continue;
        }
        if line.ends_with('{') && !line.contains("--") && !line.contains("..") {
            let name: &str = line.trim_end_matches('{').trim();
            if !name.is_empty() {
                active_ent = Some(ensure_er_entity(&mut diag, name));
            }
            continue;
        }
        if let Some(dash_pos) = line.find("--").or_else(|| line.find("..")) {
            let left_side: &str = line[..dash_pos].trim();
            let right_side: &str = line[dash_pos + 2..].trim();
            let (left_ent, left_card): (&str, &str) = match left_side.rsplit_once(' ') {
                Some((e, c)) => (e.trim(), c.trim()),
                None => (left_side, "||"),
            };
            let (right_part, lbl): (&str, &str) = match right_side.split_once(':') {
                Some((r, l)) => (r.trim(), l.trim().trim_matches('"')),
                None => (right_side, ""),
            };
            let (right_card, right_ent): (&str, &str) = match right_part.split_once(' ') {
                Some((c, e)) => (c.trim(), e.trim()),
                None => ("o{", right_part),
            };
            if !left_ent.is_empty() && !right_ent.is_empty() {
                let _: usize = ensure_er_entity(&mut diag, left_ent);
                let _: usize = ensure_er_entity(&mut diag, right_ent);
                diag.get_mut_relations().push(MermaidErRelation::new(
                    left_ent.to_string(),
                    right_ent.to_string(),
                    left_card.to_string(),
                    right_card.to_string(),
                    lbl.to_string(),
                ));
            }
        }
    }
    diag
}

/// Ensures ER entity `name` exists inside `diag`, returning its index.
///
/// # Arguments
///
/// - `&mut MermaidErDiagram` - ER diagram builder.
/// - `&str` - Entity name.
///
/// # Returns
///
/// - `usize` - Entity index.
fn ensure_er_entity(diag: &mut MermaidErDiagram, name: &str) -> usize {
    if let Some(pos) = diag
        .get_entities()
        .iter()
        .position(|e: &MermaidErEntity| e.get_name().as_str() == name)
    {
        return pos;
    }
    let idx: usize = diag.get_entities().len();
    diag.get_mut_entities()
        .push(MermaidErEntity::new(name.to_string(), Vec::new()));
    idx
}

/// Parses a Mermaid `pie` chart block.
///
/// # Arguments
///
/// - `&[&str]` - Trimmed non-empty Mermaid lines.
///
/// # Returns
///
/// - `MermaidPieChart` - Parsed pie chart title and proportional slices.
fn parse_mermaid_pie(lines: &[&str]) -> MermaidPieChart {
    let mut chart: MermaidPieChart = MermaidPieChart::default();
    if let Some(first) = lines.first() {
        let low: String = first.to_ascii_lowercase();
        if let Some(t_pos) = low.find(MERMAID_KW_TITLE) {
            chart.set_title(first[t_pos + MERMAID_KW_TITLE.len()..].trim().to_string());
        }
    }
    for raw in lines.iter().skip(1) {
        let line: &str = raw.trim();
        let low: String = line.to_ascii_lowercase();
        if low.starts_with(MERMAID_KW_TITLE) {
            chart.set_title(line[MERMAID_KW_TITLE.len()..].trim().to_string());
            continue;
        }
        if let Some((lbl_raw, val_raw)) = line.split_once(':') {
            let lbl: String = lbl_raw.trim().trim_matches('"').to_string();
            if let Ok(val) = val_raw.trim().parse::<f32>()
                && val > 0.0
            {
                chart.get_mut_slices().push(MermaidPieSlice::new(lbl, val));
            }
        }
    }
    chart
}

/// Parses a Mermaid `gantt` timeline chart block.
///
/// # Arguments
///
/// - `&[&str]` - Trimmed non-empty Mermaid lines.
///
/// # Returns
///
/// - `MermaidGanttChart` - Parsed title and scheduled timeline tasks.
fn parse_mermaid_gantt(lines: &[&str]) -> MermaidGanttChart {
    let mut chart: MermaidGanttChart = MermaidGanttChart::default();
    let mut current_section: String = MERMAID_DEFAULT_SECTION.to_string();
    let mut cursor_unit: f32 = 0.0;
    for raw in lines.iter().skip(1) {
        let line: &str = raw.trim();
        let low: String = line.to_ascii_lowercase();
        if low.starts_with(MERMAID_KW_TITLE) {
            chart.set_title(line[MERMAID_KW_TITLE.len()..].trim().to_string());
            continue;
        }
        if low.starts_with(MERMAID_KW_SECTION) {
            current_section = line[MERMAID_KW_SECTION.len()..].trim().to_string();
            continue;
        }
        if low.starts_with(MERMAID_KW_DATE_FORMAT) || low.starts_with("axisformat") {
            continue;
        }
        if let Some((label_raw, meta_raw)) = line.split_once(':') {
            let label: String = label_raw.trim().to_string();
            let mut status: MermaidGanttStatus = MermaidGanttStatus::Normal;
            let mut dur: f32 = 3.0;
            for part in meta_raw.split(',') {
                let p: &str = part.trim();
                let p_low: String = p.to_ascii_lowercase();
                if p_low == MERMAID_GANTT_DONE {
                    status = MermaidGanttStatus::Done;
                } else if p_low == MERMAID_GANTT_ACTIVE {
                    status = MermaidGanttStatus::Active;
                } else if p_low == MERMAID_GANTT_CRIT {
                    status = MermaidGanttStatus::Critical;
                } else if let Some(d_str) = p_low.strip_suffix('d')
                    && let Ok(days) = d_str.parse::<f32>()
                {
                    dur = days.clamp(1.0, 30.0);
                } else if let Some(w_str) = p_low.strip_suffix('w')
                    && let Ok(weeks) = w_str.parse::<f32>()
                {
                    dur = (weeks * 7.0).clamp(1.0, 35.0);
                }
            }
            chart.get_mut_tasks().push(MermaidGanttTask::new(
                current_section.clone(),
                label,
                status,
                cursor_unit,
                dur,
            ));
            cursor_unit += (dur * 0.72).max(1.5);
        }
    }
    chart
}

/// Runs 3-color DFS from `u` to collect acyclic forward edges while discarding cycle back-edges.
///
/// # Arguments
///
/// - `usize` - Current node index.
/// - `&[Vec<usize>]` - Directed adjacency lists.
/// - `&mut [u8]` - Visitation state (`0` unvisited, `1` active on DFS stack, `2` finished).
/// - `&mut Vec<(usize, usize)>` - Collected DAG edges.
fn dfs_collect_dag_edges(
    u: usize,
    adj: &[Vec<usize>],
    state: &mut [u8],
    dag_edges: &mut Vec<(usize, usize)>,
) {
    state[u] = 1;
    for &v in adj[u].iter() {
        if u == v || state[v] == 1 {
            continue;
        }
        dag_edges.push((u, v));
        if state[v] == 0 {
            dfs_collect_dag_edges(v, adj, state, dag_edges);
        }
    }
    state[u] = 2;
}

/// Computes cycle-safe topological layer indices for flowchart and state diagram nodes.
///
/// # Arguments
///
/// - `&MermaidFlowchart` - Parsed flowchart or state graph.
/// - `&HashMap<&str, usize>` - Node id to index lookup map.
///
/// # Returns
///
/// - `Vec<usize>` - Zero-based layer index per node.
fn compute_flowchart_dag_layers(
    fc: &MermaidFlowchart,
    id_map: &HashMap<&str, usize>,
) -> Vec<usize> {
    let n: usize = fc.get_nodes().len();
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut in_deg: Vec<usize> = vec![0; n];
    for edge in fc.get_edges().iter() {
        if let (Some(&u), Some(&v)) = (
            id_map.get(edge.get_from().as_str()),
            id_map.get(edge.get_to().as_str()),
        ) && u != v
        {
            adj[u].push(v);
            in_deg[v] += 1;
        }
    }
    let mut state: Vec<u8> = vec![0; n];
    let mut dag_edges: Vec<(usize, usize)> = Vec::with_capacity(fc.get_edges().len());
    for (idx, node) in fc.get_nodes().iter().enumerate() {
        if node.get_shape() == MermaidNodeShape::StateStart && state[idx] == 0 {
            dfs_collect_dag_edges(idx, &adj, &mut state, &mut dag_edges);
        }
    }
    for (idx, node) in fc.get_nodes().iter().enumerate() {
        if in_deg[idx] == 0 && node.get_shape() != MermaidNodeShape::StateEnd && state[idx] == 0 {
            dfs_collect_dag_edges(idx, &adj, &mut state, &mut dag_edges);
        }
    }
    for idx in 0..n {
        if state[idx] == 0 {
            dfs_collect_dag_edges(idx, &adj, &mut state, &mut dag_edges);
        }
    }
    let mut layers: Vec<usize> = vec![0; n];
    for _ in 0..n {
        let mut changed: bool = false;
        for &(u, v) in dag_edges.iter() {
            if layers[v] <= layers[u] && layers[u] + 1 < n {
                layers[v] = layers[u] + 1;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    for edge in fc.get_edges().iter() {
        if let (Some(&u), Some(&v)) = (
            id_map.get(edge.get_from().as_str()),
            id_map.get(edge.get_to().as_str()),
        ) && u != v
            && fc.get_nodes()[v].get_shape() == MermaidNodeShape::StateEnd
            && layers[v] <= layers[u]
        {
            layers[v] = layers[u] + 1;
        }
    }
    layers
}

/// Computes topological layer assignments and per-node rectangles for a flowchart or state diagram.
///
/// # Arguments
///
/// - `&MermaidFlowchart` - Parsed flowchart graph.
///
/// # Returns
///
/// - `(Vec<Rect>, Size)` - Per-node local bounding boxes and total intrinsic canvas size.
fn layout_flowchart_nodes(fc: &MermaidFlowchart) -> (Vec<Rect>, Size) {
    let n: usize = fc.get_nodes().len();
    if n == 0 {
        return (Vec::new(), Size::new(360.0, 140.0));
    }
    let mut id_map: HashMap<&str, usize> = HashMap::with_capacity(n);
    for (idx, node) in fc.get_nodes().iter().enumerate() {
        let _: Option<usize> = id_map.insert(node.get_id().as_str(), idx);
    }
    let layers: Vec<usize> = compute_flowchart_dag_layers(fc, &id_map);
    let max_layer: usize = layers.iter().copied().max().unwrap_or(0);
    let mut buckets: Vec<Vec<usize>> = vec![Vec::new(); max_layer + 1];
    for (idx, &lay) in layers.iter().enumerate() {
        buckets[lay].push(idx);
    }
    let horiz: bool = matches!(
        fc.get_direction(),
        MermaidFlowDirection::LeftRight | MermaidFlowDirection::RightLeft
    );
    let reversed: bool = matches!(
        fc.get_direction(),
        MermaidFlowDirection::RightLeft | MermaidFlowDirection::BottomTop
    );
    let node_sizes: Vec<(f32, f32)> = fc
        .get_nodes()
        .iter()
        .map(|node: &MermaidFlowNode| match node.get_shape() {
            MermaidNodeShape::StateStart | MermaidNodeShape::StateEnd => (24.0, 24.0),
            MermaidNodeShape::Circle => {
                let d: f32 =
                    ((node.get_label().chars().count() as f32) * 7.8 + 26.0).clamp(48.0, 96.0);
                (d, d)
            }
            MermaidNodeShape::Diamond => {
                let w: f32 =
                    ((node.get_label().chars().count() as f32) * 8.2 + 48.0).clamp(108.0, 196.0);
                (w, 60.0)
            }
            _ => {
                let w: f32 =
                    ((node.get_label().chars().count() as f32) * 7.8 + 34.0).clamp(96.0, 210.0);
                (w, 40.0)
            }
        })
        .collect();
    let has_labels: bool = fc
        .get_edges()
        .iter()
        .any(|edge: &MermaidFlowEdge| !edge.get_label().is_empty());
    let mut bidir_label_w: f32 = 0.0;
    for edge in fc.get_edges().iter() {
        if let (Some(&u), Some(&v)) = (
            id_map.get(edge.get_from().as_str()),
            id_map.get(edge.get_to().as_str()),
        ) && u != v
            && fc.get_edges().iter().any(|other: &MermaidFlowEdge| {
                id_map.get(other.get_from().as_str()) == Some(&v)
                    && id_map.get(other.get_to().as_str()) == Some(&u)
            })
        {
            let lbl_w: f32 = ((edge.get_label().chars().count() as f32) * 6.8 + 14.0).max(34.0);
            bidir_label_w = bidir_label_w.max(lbl_w);
        }
    }
    let pad: f32 = 34.0;
    let cross_pad: f32 = if !horiz && bidir_label_w > 0.0 {
        (bidir_label_w + 36.0).max(pad)
    } else if horiz && bidir_label_w > 0.0 {
        56.0
    } else {
        pad
    };
    let main_gap: f32 = if has_labels {
        if horiz { 80.0 } else { 88.0 }
    } else if horiz {
        56.0
    } else {
        48.0
    };
    let cross_gap: f32 = if !horiz && bidir_label_w > 0.0 {
        (bidir_label_w + 48.0).max(28.0)
    } else {
        28.0
    };
    let layer_count: usize = buckets.len();
    let mut layer_main_dim: Vec<f32> = vec![0.0; layer_count];
    let mut layer_cross_span: Vec<f32> = vec![0.0; layer_count];
    for (l_idx, items) in buckets.iter().enumerate() {
        let mut max_main: f32 = 0.0;
        let mut sum_cross: f32 = 0.0;
        for (pos, &ni) in items.iter().enumerate() {
            let (nw, nh): (f32, f32) = node_sizes[ni];
            if horiz {
                max_main = max_main.max(nw);
                sum_cross += nh;
            } else {
                max_main = max_main.max(nh);
                sum_cross += nw;
            }
            if pos + 1 < items.len() {
                sum_cross += cross_gap;
            }
        }
        layer_main_dim[l_idx] = max_main;
        layer_cross_span[l_idx] = sum_cross;
    }
    let total_main: f32 =
        layer_main_dim.iter().sum::<f32>() + (layer_count.saturating_sub(1) as f32) * main_gap;
    let total_cross: f32 = layer_cross_span.iter().copied().fold(0.0_f32, f32::max);
    let mut rects: Vec<Rect> = vec![Rect::default(); n];
    let mut main_cursor: f32 = pad;
    let layer_order: Vec<usize> = if reversed {
        (0..layer_count).rev().collect()
    } else {
        (0..layer_count).collect()
    };
    for l_idx in layer_order.into_iter() {
        let items: &[usize] = &buckets[l_idx];
        let band_main: f32 = layer_main_dim[l_idx];
        let band_cross: f32 = layer_cross_span[l_idx];
        let mut cross_cursor: f32 = cross_pad + (total_cross - band_cross) * 0.5;
        for &ni in items.iter() {
            let (nw, nh): (f32, f32) = node_sizes[ni];
            let r: Rect = if horiz {
                let rx: f32 = main_cursor + (band_main - nw) * 0.5;
                let ry: f32 = cross_cursor;
                cross_cursor += nh + cross_gap;
                Rect::new(rx, ry, nw, nh)
            } else {
                let rx: f32 = cross_cursor;
                let ry: f32 = main_cursor + (band_main - nh) * 0.5;
                cross_cursor += nw + cross_gap;
                Rect::new(rx, ry, nw, nh)
            };
            rects[ni] = r;
        }
        main_cursor += band_main + main_gap;
    }
    let (w, h): (f32, f32) = if horiz {
        (
            (total_main + pad * 2.0).max(360.0),
            (total_cross + cross_pad * 2.0).max(160.0),
        )
    } else {
        (
            (total_cross + cross_pad * 2.0).max(360.0),
            (total_main + pad * 2.0).max(160.0),
        )
    };
    (rects, Size::new(w, h))
}

/// Computes the intrinsic canvas size of a Mermaid flowchart.
///
/// # Arguments
///
/// - `&MermaidFlowchart` - Parsed flowchart structure.
///
/// # Returns
///
/// - `Size` - Intrinsic width and height in logical pixels.
fn measure_flowchart(fc: &MermaidFlowchart) -> Size {
    let (_, sz): (Vec<Rect>, Size) = layout_flowchart_nodes(fc);
    sz
}

/// Checks whether two rectangles overlap when expanded by `pad` logical pixels on each edge.
///
/// # Arguments
///
/// - `Rect` - Candidate rectangle.
/// - `Rect` - Existing obstacle rectangle.
/// - `f32` - Minimum clearance margin in logical pixels.
///
/// # Returns
///
/// - `bool` - `true` when the rectangles collide within `pad`.
fn rects_overlap_padded(a: Rect, b: Rect, pad: f32) -> bool {
    a.get_x() - pad < b.right()
        && a.right() + pad > b.get_x()
        && a.get_y() - pad < b.bottom()
        && a.bottom() + pad > b.get_y()
}

/// Returns `true` when `cand` collides with any placed edge pill or node box.
///
/// # Arguments
///
/// - `Rect` - Candidate pill rectangle.
/// - `&[Rect]` - Previously placed edge label pills.
/// - `&[Rect]` - Placed diagram node bounding boxes.
///
/// # Returns
///
/// - `bool` - `true` when any obstacle intersects `cand`.
fn edge_pill_collides(cand: Rect, placed_pills: &[Rect], node_rects: &[Rect]) -> bool {
    placed_pills
        .iter()
        .any(|p: &Rect| rects_overlap_padded(cand, *p, 4.0))
        || node_rects
            .iter()
            .any(|nr: &Rect| rects_overlap_padded(cand, *nr, 2.0))
}

/// Resolves a collision-free edge label pill rectangle along connector `p0 -> p1`.
///
/// # Arguments
///
/// - `Point` - Connector start point.
/// - `Point` - Connector end point.
/// - `Rect` - Preferred initial pill rectangle.
/// - `&[Rect]` - Previously placed edge label pills.
/// - `&[Rect]` - Placed diagram node bounding boxes.
///
/// # Returns
///
/// - `Rect` - Final non-overlapping pill rectangle.
fn resolve_edge_pill_rect(
    p0: Point,
    p1: Point,
    base_pill: Rect,
    placed_pills: &[Rect],
    node_rects: &[Rect],
) -> Rect {
    if !edge_pill_collides(base_pill, placed_pills, node_rects) {
        return base_pill;
    }
    let pw: f32 = base_pill.get_width();
    let ph: f32 = base_pill.get_height();
    let dx: f32 = p1.get_x() - p0.get_x();
    let dy: f32 = p1.get_y() - p0.get_y();
    let fractions: [f32; 6] = [0.78, 0.22, 0.72, 0.28, 0.84, 0.16];
    for t in fractions.into_iter() {
        let cx: f32 = p0.get_x() + dx * t;
        let cy: f32 = p0.get_y() + dy * t;
        let variants: [Rect; 3] = [
            Rect::new(cx - pw * 0.5, cy - ph * 0.5, pw, ph),
            Rect::new(cx + 8.0, cy - ph * 0.5, pw, ph),
            Rect::new(cx - pw - 8.0, cy - ph * 0.5, pw, ph),
        ];
        for cand in variants.into_iter() {
            if !edge_pill_collides(cand, placed_pills, node_rects) {
                return cand;
            }
        }
    }
    let mut shifted: Rect = base_pill;
    for _ in 0..6 {
        let blocker: Option<Rect> = placed_pills
            .iter()
            .copied()
            .find(|p: &Rect| rects_overlap_padded(shifted, *p, 4.0));
        if let Some(other) = blocker {
            shifted = Rect::new(other.right() + 6.0, shifted.get_y(), pw, ph);
        } else {
            break;
        }
    }
    shifted
}

/// Draws a Mermaid flowchart or state diagram onto `dl`.
///
/// # Arguments
///
/// - `&MermaidFlowchart` - Parsed flowchart or state graph.
/// - `Rect` - Target canvas bounds.
/// - `&mut DisplayList` - Active draw command list.
/// - `ThemeTokens` - Active theme tokens.
/// - `CodeThemePalette` - Resolved Eva palette.
fn draw_flowchart(
    fc: &MermaidFlowchart,
    bounds: Rect,
    dl: &mut DisplayList,
    tokens: ThemeTokens,
    eva: CodeThemePalette,
) {
    let p: ThemePalette = tokens.get_palette();
    let (local_rects, nat_sz): (Vec<Rect>, Size) = layout_flowchart_nodes(fc);
    let ox: f32 = bounds.get_x() + ((bounds.get_width() - nat_sz.get_width()).max(0.0)) * 0.5;
    let oy: f32 = bounds.get_y() + ((bounds.get_height() - nat_sz.get_height()).max(0.0)) * 0.5;
    let rects: Vec<Rect> = local_rects
        .into_iter()
        .map(|r: Rect| {
            Rect::new(
                r.get_x() + ox,
                r.get_y() + oy,
                r.get_width(),
                r.get_height(),
            )
        })
        .collect();
    for (sg_idx, sg) in fc.get_subgraphs().iter().enumerate() {
        let mut min_x: f32 = f32::INFINITY;
        let mut min_y: f32 = f32::INFINITY;
        let mut max_x: f32 = f32::NEG_INFINITY;
        let mut max_y: f32 = f32::NEG_INFINITY;
        for (ni, node) in fc.get_nodes().iter().enumerate() {
            if node.subgraph == Some(sg_idx) {
                let r: Rect = rects[ni];
                min_x = min_x.min(r.get_x());
                min_y = min_y.min(r.get_y());
                max_x = max_x.max(r.right());
                max_y = max_y.max(r.bottom());
            }
        }
        if min_x.is_finite() {
            let box_r: Rect = Rect::new(
                min_x - 16.0,
                min_y - 26.0,
                (max_x - min_x) + 32.0,
                (max_y - min_y) + 42.0,
            );
            dl.push(DrawCommand::FillRoundedRect {
                rect: box_r,
                color: eva.get_header_bg().with_alpha(170),
                radius: CornerRadius::all(10.0),
                border: Border::new(1.2, eva.get_keyword().with_alpha(120)),
            });
            draw_centered_label(
                dl,
                Rect::new(
                    box_r.get_x() + 10.0,
                    box_r.get_y() + 4.0,
                    box_r.get_width() - 20.0,
                    18.0,
                ),
                sg.get_title(),
                11.0,
                FontWeight::Bold,
                eva.get_keyword(),
                TextAlign::Left,
            );
        }
    }
    let mut id_map: HashMap<&str, usize> = HashMap::with_capacity(fc.get_nodes().len());
    for (idx, node) in fc.get_nodes().iter().enumerate() {
        let _: Option<usize> = id_map.insert(node.get_id().as_str(), idx);
    }
    let horiz: bool = matches!(
        fc.get_direction(),
        MermaidFlowDirection::LeftRight | MermaidFlowDirection::RightLeft
    );
    let edge_col: Color = eva.get_function();
    let mut placed_pills: Vec<Rect> = Vec::with_capacity(fc.get_edges().len());
    for edge in fc.get_edges().iter() {
        let (Some(&u), Some(&v)) = (
            id_map.get(edge.get_from().as_str()),
            id_map.get(edge.get_to().as_str()),
        ) else {
            continue;
        };
        let ra: Rect = rects[u];
        let rb: Rect = rects[v];
        let is_bidir: bool = u != v
            && fc.get_edges().iter().any(|other: &MermaidFlowEdge| {
                id_map.get(other.get_from().as_str()) == Some(&v)
                    && id_map.get(other.get_to().as_str()) == Some(&u)
            });
        let bidir_sign: f32 = if u < v { -1.0 } else { 1.0 };
        let (raw_p0, raw_p1): (Point, Point) = compute_rect_anchor_points(ra, rb, horiz);
        let (p0, p1): (Point, Point) = if is_bidir {
            if horiz {
                let dy: f32 = bidir_sign * 14.0;
                (
                    Point::new(raw_p0.get_x(), raw_p0.get_y() + dy),
                    Point::new(raw_p1.get_x(), raw_p1.get_y() + dy),
                )
            } else {
                let dx: f32 = bidir_sign * 24.0;
                (
                    Point::new(raw_p0.get_x() + dx, raw_p0.get_y()),
                    Point::new(raw_p1.get_x() + dx, raw_p1.get_y()),
                )
            }
        } else {
            (raw_p0, raw_p1)
        };
        let stroke_w: f32 = if edge.get_style() == MermaidEdgeStyle::Thick {
            2.8
        } else {
            1.65
        };
        let dashed: bool = edge.get_style() == MermaidEdgeStyle::Dashed;
        let show_arrow: bool = edge.get_style() != MermaidEdgeStyle::Plain;
        draw_vector_connector(dl, p0, p1, edge_col, stroke_w, dashed, show_arrow);
        if !edge.get_label().is_empty() {
            let mx: f32 = (p0.get_x() + p1.get_x()) * 0.5;
            let my: f32 = (p0.get_y() + p1.get_y()) * 0.5;
            let lbl_w: f32 = ((edge.get_label().chars().count() as f32) * 6.8 + 14.0).max(34.0);
            let (pill_x, pill_y): (f32, f32) = if is_bidir {
                if horiz {
                    let py: f32 = if bidir_sign < 0.0 {
                        my - 24.0
                    } else {
                        my + 4.0
                    };
                    (mx - lbl_w * 0.5, py)
                } else {
                    let px: f32 = if bidir_sign < 0.0 {
                        mx - lbl_w - 8.0
                    } else {
                        mx + 8.0
                    };
                    (px, my - 10.0)
                }
            } else {
                (mx - lbl_w * 0.5, my - 10.0)
            };
            let base_pill: Rect = Rect::new(pill_x, pill_y, lbl_w, 20.0);
            let pill: Rect = resolve_edge_pill_rect(p0, p1, base_pill, &placed_pills, &rects);
            placed_pills.push(pill);
            dl.push(DrawCommand::FillRoundedRect {
                rect: pill,
                color: eva.get_header_bg(),
                radius: CornerRadius::all(5.0),
                border: Border::new(1.0, eva.get_border()),
            });
            draw_centered_label(
                dl,
                pill,
                edge.get_label(),
                10.5,
                FontWeight::SemiBold,
                eva.get_foreground(),
                TextAlign::Center,
            );
        }
    }
    for (idx, node) in fc.get_nodes().iter().enumerate() {
        let r: Rect = rects[idx];
        let fill: Color = p.get_card();
        let stroke: Color = eva.get_keyword();
        match node.get_shape() {
            MermaidNodeShape::StateStart => {
                let c: Point = Point::new(
                    r.get_x() + r.get_width() * 0.5,
                    r.get_y() + r.get_height() * 0.5,
                );
                dl.push(DrawCommand::Circle {
                    center: c,
                    radius: 9.0,
                    color: stroke,
                });
            }
            MermaidNodeShape::StateEnd => {
                let c: Point = Point::new(
                    r.get_x() + r.get_width() * 0.5,
                    r.get_y() + r.get_height() * 0.5,
                );
                dl.push(DrawCommand::Circle {
                    center: c,
                    radius: 11.0,
                    color: stroke,
                });
                dl.push(DrawCommand::Circle {
                    center: c,
                    radius: 7.5,
                    color: eva.get_background(),
                });
                dl.push(DrawCommand::Circle {
                    center: c,
                    radius: 4.5,
                    color: stroke,
                });
            }
            MermaidNodeShape::Circle => {
                let c: Point = Point::new(
                    r.get_x() + r.get_width() * 0.5,
                    r.get_y() + r.get_height() * 0.5,
                );
                let rad: f32 = r.get_width().min(r.get_height()) * 0.5;
                dl.push(DrawCommand::Circle {
                    center: c,
                    radius: rad,
                    color: eva.get_operator(),
                });
                dl.push(DrawCommand::Circle {
                    center: c,
                    radius: (rad - 1.8).max(2.0),
                    color: fill,
                });
                draw_centered_label(
                    dl,
                    r,
                    node.get_label(),
                    12.0,
                    FontWeight::SemiBold,
                    p.get_foreground(),
                    TextAlign::Center,
                );
            }
            MermaidNodeShape::Diamond => {
                let cx: f32 = r.get_x() + r.get_width() * 0.5;
                let cy: f32 = r.get_y() + r.get_height() * 0.5;
                let top: Point = Point::new(cx, r.get_y());
                let right: Point = Point::new(r.right(), cy);
                let bot: Point = Point::new(cx, r.bottom());
                let left: Point = Point::new(r.get_x(), cy);
                dl.push(DrawCommand::Triangle {
                    p0: top,
                    p1: right,
                    p2: left,
                    color: fill,
                });
                dl.push(DrawCommand::Triangle {
                    p0: left,
                    p1: right,
                    p2: bot,
                    color: fill,
                });
                let diamond_col: Color = eva.get_control();
                for (a, b) in [(top, right), (right, bot), (bot, left), (left, top)].into_iter() {
                    dl.push(DrawCommand::Line {
                        from: a,
                        to: b,
                        color: diamond_col,
                        width: 1.8,
                    });
                }
                draw_centered_label(
                    dl,
                    r,
                    node.get_label(),
                    11.5,
                    FontWeight::SemiBold,
                    p.get_foreground(),
                    TextAlign::Center,
                );
            }
            MermaidNodeShape::Stadium => {
                dl.push(DrawCommand::FillRoundedRect {
                    rect: r,
                    color: fill,
                    radius: CornerRadius::all(r.get_height() * 0.5),
                    border: Border::new(1.6, eva.get_diff_add()),
                });
                draw_centered_label(
                    dl,
                    r,
                    node.get_label(),
                    12.0,
                    FontWeight::SemiBold,
                    p.get_foreground(),
                    TextAlign::Center,
                );
            }
            MermaidNodeShape::Rounded => {
                dl.push(DrawCommand::FillRoundedRect {
                    rect: r,
                    color: fill,
                    radius: CornerRadius::all(12.0),
                    border: Border::new(1.6, stroke),
                });
                draw_centered_label(
                    dl,
                    r,
                    node.get_label(),
                    12.0,
                    FontWeight::SemiBold,
                    p.get_foreground(),
                    TextAlign::Center,
                );
            }
            MermaidNodeShape::Box => {
                dl.push(DrawCommand::FillRoundedRect {
                    rect: r,
                    color: fill,
                    radius: CornerRadius::all(6.0),
                    border: Border::new(1.6, eva.get_link()),
                });
                draw_centered_label(
                    dl,
                    r,
                    node.get_label(),
                    12.0,
                    FontWeight::SemiBold,
                    p.get_foreground(),
                    TextAlign::Center,
                );
            }
        }
    }
}

/// Draws a Mermaid `sequenceDiagram` with participants, lifelines, arrows, and callout notes.
///
/// # Arguments
///
/// - `&MermaidSequence` - Parsed sequence diagram.
/// - `Rect` - Target canvas bounds.
/// - `&mut DisplayList` - Active draw command list.
/// - `ThemeTokens` - Active theme tokens.
/// - `CodeThemePalette` - Resolved Eva palette.
fn draw_sequence(
    seq: &MermaidSequence,
    bounds: Rect,
    dl: &mut DisplayList,
    tokens: ThemeTokens,
    eva: CodeThemePalette,
) {
    let p: ThemePalette = tokens.get_palette();
    let count: usize = seq.get_participants().len();
    if count == 0 {
        return;
    }
    let col_step: f32 = ((bounds.get_width() - 48.0) / (count as f32)).max(136.0);
    let total_w: f32 = col_step * (count as f32);
    let start_x: f32 = bounds.get_x() + ((bounds.get_width() - total_w).max(0.0)) * 0.5;
    let top_y: f32 = bounds.get_y() + 20.0;
    let box_h: f32 = 34.0;
    let box_w: f32 = (col_step - 26.0).clamp(84.0, 148.0);
    let msg_start_y: f32 = top_y + box_h + 30.0;
    let row_h: f32 = 46.0;
    let bot_y: f32 = msg_start_y + (seq.get_messages().len().max(1) as f32) * row_h + 10.0;
    let mut centers: Vec<f32> = Vec::with_capacity(count);
    for (idx, (_, label)) in seq.get_participants().iter().enumerate() {
        let cx: f32 = start_x + (idx as f32 + 0.5) * col_step;
        centers.push(cx);
        draw_dashed_line(
            dl,
            Point::new(cx, top_y + box_h),
            Point::new(cx, bot_y),
            eva.get_border(),
            1.3,
        );
        for by in [top_y, bot_y].into_iter() {
            let pr: Rect = Rect::new(cx - box_w * 0.5, by, box_w, box_h);
            dl.push(DrawCommand::FillRoundedRect {
                rect: pr,
                color: p.get_card(),
                radius: CornerRadius::all(7.0),
                border: Border::new(1.5, eva.get_link()),
            });
            draw_centered_label(
                dl,
                pr,
                label,
                12.0,
                FontWeight::Bold,
                p.get_foreground(),
                TextAlign::Center,
            );
        }
    }
    for (m_idx, msg) in seq.get_messages().iter().enumerate() {
        let y: f32 = msg_start_y + (m_idx as f32) * row_h;
        let x0: f32 = centers.get(msg.get_from_idx()).copied().unwrap_or(start_x);
        let x1: f32 = centers.get(msg.get_to_idx()).copied().unwrap_or(x0);
        if msg.get_is_note() {
            let left_x: f32 = x0.min(x1) - 56.0;
            let right_x: f32 = x0.max(x1) + 56.0;
            let nr: Rect = Rect::new(left_x, y - 14.0, (right_x - left_x).max(120.0), 28.0);
            dl.push(DrawCommand::FillRoundedRect {
                rect: nr,
                color: p.get_warning().with_alpha(38),
                radius: CornerRadius::all(6.0),
                border: Border::new(1.2, p.get_warning()),
            });
            draw_centered_label(
                dl,
                nr,
                msg.get_text(),
                11.0,
                FontWeight::Medium,
                p.get_foreground(),
                TextAlign::Center,
            );
            continue;
        }
        let dashed: bool = msg.get_style() == MermaidEdgeStyle::Dashed;
        let col: Color = if dashed {
            eva.get_operator()
        } else {
            eva.get_function()
        };
        if (x1 - x0).abs() < 1.0 {
            let loop_w: f32 = 42.0;
            let loop_h: f32 = 22.0;
            let p_a: Point = Point::new(x0, y);
            let p_b: Point = Point::new(x0 + loop_w, y);
            let p_c: Point = Point::new(x0 + loop_w, y + loop_h);
            let p_d: Point = Point::new(x0, y + loop_h);
            draw_vector_connector(dl, p_a, p_b, col, 1.6, dashed, false);
            draw_vector_connector(dl, p_b, p_c, col, 1.6, dashed, false);
            draw_vector_connector(dl, p_c, p_d, col, 1.6, dashed, true);
            draw_centered_label(
                dl,
                Rect::new(x0 + 6.0, y - 18.0, 140.0, 16.0),
                msg.get_text(),
                11.0,
                FontWeight::Medium,
                eva.get_foreground(),
                TextAlign::Left,
            );
        } else {
            draw_vector_connector(
                dl,
                Point::new(x0, y + 6.0),
                Point::new(x1, y + 6.0),
                col,
                1.7,
                dashed,
                true,
            );
            let min_x: f32 = x0.min(x1);
            let span: f32 = (x1 - x0).abs().max(80.0);
            draw_centered_label(
                dl,
                Rect::new(min_x, y - 14.0, span, 18.0),
                msg.get_text(),
                11.5,
                FontWeight::Medium,
                eva.get_foreground(),
                TextAlign::Center,
            );
        }
    }
}

/// Draws a Mermaid `classDiagram` with UML compartment cards and vector relationship connectors.
///
/// # Arguments
///
/// - `&MermaidClassDiagram` - Parsed class diagram.
/// - `Rect` - Target canvas bounds.
/// - `&mut DisplayList` - Active draw command list.
/// - `ThemeTokens` - Active theme tokens.
/// - `CodeThemePalette` - Resolved Eva palette.
fn draw_class_diagram(
    cls: &MermaidClassDiagram,
    bounds: Rect,
    dl: &mut DisplayList,
    tokens: ThemeTokens,
    eva: CodeThemePalette,
) {
    let p: ThemePalette = tokens.get_palette();
    let count: usize = cls.get_classes().len();
    if count == 0 {
        return;
    }
    let cols: usize = count.clamp(1, 3);
    let box_w: f32 = 200.0;
    let gap_x: f32 = 36.0;
    let gap_y: f32 = 44.0;
    let max_m: usize = cls
        .get_classes()
        .iter()
        .map(|c: &MermaidClassBox| c.get_members().len())
        .max()
        .unwrap_or(1);
    let box_h: f32 = 42.0 + (max_m as f32) * 18.0;
    let grid_w: f32 = (cols as f32) * box_w + ((cols.saturating_sub(1)) as f32) * gap_x;
    let start_x: f32 = bounds.get_x() + ((bounds.get_width() - grid_w).max(0.0)) * 0.5;
    let start_y: f32 = bounds.get_y() + 24.0;
    let mut map: HashMap<&str, Rect> = HashMap::with_capacity(count);
    let mut rects: Vec<Rect> = Vec::with_capacity(count);
    for (idx, item) in cls.get_classes().iter().enumerate() {
        let col: usize = idx % cols;
        let row: usize = idx / cols;
        let rx: f32 = start_x + (col as f32) * (box_w + gap_x);
        let ry: f32 = start_y + (row as f32) * (box_h + gap_y);
        let r: Rect = Rect::new(rx, ry, box_w, box_h);
        let _: Option<Rect> = map.insert(item.get_name().as_str(), r);
        rects.push(r);
    }
    for rel in cls.get_relations().iter() {
        if let (Some(&ra), Some(&rb)) = (
            map.get(rel.get_from().as_str()),
            map.get(rel.get_to().as_str()),
        ) {
            let horiz: bool = (ra.get_y() - rb.get_y()).abs() < 8.0;
            let (p0, p1): (Point, Point) = compute_rect_anchor_points(ra, rb, horiz);
            draw_vector_connector(
                dl,
                p0,
                p1,
                eva.get_function(),
                1.6,
                rel.get_style() == MermaidEdgeStyle::Dashed,
                true,
            );
            if !rel.get_label().is_empty() {
                let mx: f32 = (p0.get_x() + p1.get_x()) * 0.5;
                let my: f32 = (p0.get_y() + p1.get_y()) * 0.5;
                draw_centered_label(
                    dl,
                    Rect::new(mx - 48.0, my - 10.0, 96.0, 18.0),
                    rel.get_label(),
                    10.5,
                    FontWeight::SemiBold,
                    eva.get_foreground(),
                    TextAlign::Center,
                );
            }
        }
    }
    for (idx, item) in cls.get_classes().iter().enumerate() {
        let r: Rect = rects[idx];
        dl.push(DrawCommand::FillRoundedRect {
            rect: r,
            color: p.get_card(),
            radius: CornerRadius::all(8.0),
            border: Border::new(1.5, eva.get_keyword()),
        });
        let hdr_r: Rect = Rect::new(r.get_x(), r.get_y(), r.get_width(), 28.0);
        dl.push(DrawCommand::FillRoundedRect {
            rect: hdr_r,
            color: eva.get_keyword().with_alpha(32),
            radius: CornerRadius::new(8.0, 8.0, 0.0, 0.0),
            border: Border::default(),
        });
        dl.push(DrawCommand::Line {
            from: Point::new(r.get_x(), r.get_y() + 28.0),
            to: Point::new(r.right(), r.get_y() + 28.0),
            color: eva.get_border(),
            width: 1.0,
        });
        draw_centered_label(
            dl,
            hdr_r,
            item.get_name(),
            12.0,
            FontWeight::Bold,
            p.get_foreground(),
            TextAlign::Center,
        );
        for (mi, mem) in item.get_members().iter().enumerate() {
            let my: f32 = r.get_y() + 34.0 + (mi as f32) * 18.0;
            draw_centered_label(
                dl,
                Rect::new(r.get_x() + 10.0, my, r.get_width() - 20.0, 16.0),
                mem,
                11.0,
                FontWeight::Normal,
                eva.get_foreground(),
                TextAlign::Left,
            );
        }
    }
}

/// Draws a Mermaid `erDiagram` with entity attribute boxes and cardinality lines.
///
/// # Arguments
///
/// - `&MermaidErDiagram` - Parsed ER diagram.
/// - `Rect` - Target canvas bounds.
/// - `&mut DisplayList` - Active draw command list.
/// - `ThemeTokens` - Active theme tokens.
/// - `CodeThemePalette` - Resolved Eva palette.
fn draw_er_diagram(
    er: &MermaidErDiagram,
    bounds: Rect,
    dl: &mut DisplayList,
    tokens: ThemeTokens,
    eva: CodeThemePalette,
) {
    let p: ThemePalette = tokens.get_palette();
    let count: usize = er.get_entities().len();
    if count == 0 {
        return;
    }
    let cols: usize = count.clamp(1, 3);
    let box_w: f32 = 204.0;
    let gap_x: f32 = 42.0;
    let gap_y: f32 = 44.0;
    let max_a: usize = er
        .get_entities()
        .iter()
        .map(|e: &MermaidErEntity| e.get_attributes().len())
        .max()
        .unwrap_or(1);
    let box_h: f32 = 42.0 + (max_a as f32) * 18.0;
    let grid_w: f32 = (cols as f32) * box_w + ((cols.saturating_sub(1)) as f32) * gap_x;
    let start_x: f32 = bounds.get_x() + ((bounds.get_width() - grid_w).max(0.0)) * 0.5;
    let start_y: f32 = bounds.get_y() + 24.0;
    let mut map: HashMap<&str, Rect> = HashMap::with_capacity(count);
    let mut rects: Vec<Rect> = Vec::with_capacity(count);
    for (idx, ent) in er.get_entities().iter().enumerate() {
        let col: usize = idx % cols;
        let row: usize = idx / cols;
        let r: Rect = Rect::new(
            start_x + (col as f32) * (box_w + gap_x),
            start_y + (row as f32) * (box_h + gap_y),
            box_w,
            box_h,
        );
        let _: Option<Rect> = map.insert(ent.get_name().as_str(), r);
        rects.push(r);
    }
    for rel in er.get_relations().iter() {
        if let (Some(&ra), Some(&rb)) = (
            map.get(rel.get_left().as_str()),
            map.get(rel.get_right().as_str()),
        ) {
            let horiz: bool = (ra.get_y() - rb.get_y()).abs() < 8.0;
            let (p0, p1): (Point, Point) = compute_rect_anchor_points(ra, rb, horiz);
            draw_vector_connector(dl, p0, p1, eva.get_operator(), 1.6, false, false);
            dl.push(DrawCommand::Circle {
                center: p0,
                radius: 3.5,
                color: eva.get_operator(),
            });
            dl.push(DrawCommand::Circle {
                center: p1,
                radius: 3.5,
                color: eva.get_keyword(),
            });
            let caption: String = if rel.get_label().is_empty() {
                format!("{}..{}", rel.get_left_card(), rel.get_right_card())
            } else {
                rel.get_label().clone()
            };
            let mx: f32 = (p0.get_x() + p1.get_x()) * 0.5;
            let my: f32 = (p0.get_y() + p1.get_y()) * 0.5;
            draw_centered_label(
                dl,
                Rect::new(mx - 54.0, my - 10.0, 108.0, 18.0),
                &caption,
                10.5,
                FontWeight::SemiBold,
                eva.get_foreground(),
                TextAlign::Center,
            );
        }
    }
    for (idx, ent) in er.get_entities().iter().enumerate() {
        let r: Rect = rects[idx];
        dl.push(DrawCommand::FillRoundedRect {
            rect: r,
            color: p.get_card(),
            radius: CornerRadius::all(8.0),
            border: Border::new(1.5, eva.get_operator()),
        });
        let hdr_r: Rect = Rect::new(r.get_x(), r.get_y(), r.get_width(), 28.0);
        dl.push(DrawCommand::FillRoundedRect {
            rect: hdr_r,
            color: eva.get_operator().with_alpha(32),
            radius: CornerRadius::new(8.0, 8.0, 0.0, 0.0),
            border: Border::default(),
        });
        draw_centered_label(
            dl,
            hdr_r,
            ent.get_name(),
            12.0,
            FontWeight::Bold,
            p.get_foreground(),
            TextAlign::Center,
        );
        for (ai, attr) in ent.get_attributes().iter().enumerate() {
            let ay: f32 = r.get_y() + 34.0 + (ai as f32) * 18.0;
            draw_centered_label(
                dl,
                Rect::new(r.get_x() + 10.0, ay, r.get_width() - 20.0, 16.0),
                attr,
                11.0,
                FontWeight::Normal,
                eva.get_foreground(),
                TextAlign::Left,
            );
        }
    }
}

/// Draws a proportional Mermaid `pie` chart using radial vector triangles plus an aligned swatch legend.
///
/// # Arguments
///
/// - `&MermaidPieChart` - Parsed pie chart.
/// - `Rect` - Target canvas bounds.
/// - `&mut DisplayList` - Active draw command list.
/// - `ThemeTokens` - Active theme tokens.
/// - `CodeThemePalette` - Resolved Eva palette.
fn draw_pie_chart(
    pie: &MermaidPieChart,
    bounds: Rect,
    dl: &mut DisplayList,
    tokens: ThemeTokens,
    eva: CodeThemePalette,
) {
    let p: ThemePalette = tokens.get_palette();
    let total: f32 = pie
        .get_slices()
        .iter()
        .map(|s: &MermaidPieSlice| s.get_value())
        .sum::<f32>()
        .max(0.001);
    let title_h: f32 = if pie.get_title().is_empty() {
        14.0
    } else {
        36.0
    };
    if !pie.get_title().is_empty() {
        draw_centered_label(
            dl,
            Rect::new(
                bounds.get_x() + 16.0,
                bounds.get_y() + 10.0,
                bounds.get_width() - 32.0,
                22.0,
            ),
            pie.get_title(),
            13.5,
            FontWeight::Bold,
            p.get_foreground(),
            TextAlign::Center,
        );
    }
    let avail_h: f32 = (bounds.get_height() - title_h - 24.0).max(120.0);
    let radius: f32 = (avail_h * 0.44).clamp(52.0, 92.0);
    let cx: f32 = bounds.get_x() + radius + 44.0;
    let cy: f32 = bounds.get_y() + title_h + avail_h * 0.5;
    let center: Point = Point::new(cx, cy);
    let mut start_angle: f32 = -FRAC_PI_2;
    let legend_x: f32 = cx + radius + 38.0;
    let legend_y0: f32 = bounds.get_y() + title_h + 18.0;
    for (idx, slice) in pie.get_slices().iter().enumerate() {
        let frac: f32 = slice.get_value() / total;
        let sweep: f32 = frac * TAU;
        let col: Color = Color::hex(MERMAID_PIE_PALETTE[idx % MERMAID_PIE_PALETTE.len()]);
        let segs: usize = ((sweep / (PI / 24.0)).ceil() as usize).clamp(3, 64);
        let step: f32 = sweep / (segs as f32);
        for s_i in 0..segs {
            let a0: f32 = start_angle + (s_i as f32) * step;
            let a1: f32 = a0 + step;
            let p1: Point = Point::new(cx + a0.cos() * radius, cy + a0.sin() * radius);
            let p2: Point = Point::new(cx + a1.cos() * radius, cy + a1.sin() * radius);
            dl.push(DrawCommand::Triangle {
                p0: center,
                p1,
                p2,
                color: col,
            });
        }
        start_angle += sweep;
        let ly: f32 = legend_y0 + (idx as f32) * 26.0;
        dl.push(DrawCommand::FillRoundedRect {
            rect: Rect::new(legend_x, ly + 3.0, 12.0, 12.0),
            color: col,
            radius: CornerRadius::all(3.0),
            border: Border::default(),
        });
        let pct_txt: String = format!("{} ({:.1}%)", slice.get_label(), frac * 100.0);
        draw_centered_label(
            dl,
            Rect::new(
                legend_x + 20.0,
                ly,
                (bounds.right() - legend_x - 28.0).max(120.0),
                18.0,
            ),
            &pct_txt,
            12.0,
            FontWeight::Medium,
            eva.get_foreground(),
            TextAlign::Left,
        );
    }
    dl.push(DrawCommand::Circle {
        center,
        radius: radius * 0.42,
        color: eva.get_background(),
    });
}

/// Draws a Mermaid `gantt` timeline chart with sections, grid ticks, and status-colored rounded bars.
///
/// # Arguments
///
/// - `&MermaidGanttChart` - Parsed Gantt chart.
/// - `Rect` - Target canvas bounds.
/// - `&mut DisplayList` - Active draw command list.
/// - `ThemeTokens` - Active theme tokens.
/// - `CodeThemePalette` - Resolved Eva palette.
fn draw_gantt_chart(
    gantt: &MermaidGanttChart,
    bounds: Rect,
    dl: &mut DisplayList,
    tokens: ThemeTokens,
    eva: CodeThemePalette,
) {
    let p: ThemePalette = tokens.get_palette();
    let mut y: f32 = bounds.get_y() + 14.0;
    if !gantt.get_title().is_empty() {
        draw_centered_label(
            dl,
            Rect::new(bounds.get_x() + 18.0, y, bounds.get_width() - 36.0, 22.0),
            gantt.get_title(),
            13.5,
            FontWeight::Bold,
            p.get_foreground(),
            TextAlign::Left,
        );
        y += 28.0;
    }
    let label_col_w: f32 = 172.0;
    let track_x: f32 = bounds.get_x() + label_col_w + 16.0;
    let track_w: f32 = (bounds.get_width() - label_col_w - 40.0).max(180.0);
    let max_end: f32 = gantt
        .get_tasks()
        .iter()
        .map(|t: &MermaidGanttTask| t.get_start_unit() + t.get_duration_unit())
        .fold(1.0_f32, f32::max);
    for tick in 0..=4 {
        let tx: f32 = track_x + ((tick as f32) / 4.0) * track_w;
        dl.push(DrawCommand::Line {
            from: Point::new(tx, y),
            to: Point::new(tx, bounds.bottom() - 16.0),
            color: eva.get_border().with_alpha(120),
            width: 1.0,
        });
    }
    let mut last_sec: &str = "";
    for task in gantt.get_tasks().iter() {
        if task.get_section().as_str() != last_sec {
            last_sec = task.get_section().as_str();
            draw_centered_label(
                dl,
                Rect::new(bounds.get_x() + 18.0, y, label_col_w, 20.0),
                last_sec,
                11.5,
                FontWeight::Bold,
                eva.get_keyword(),
                TextAlign::Left,
            );
            y += 24.0;
        }
        draw_centered_label(
            dl,
            Rect::new(bounds.get_x() + 24.0, y + 2.0, label_col_w - 14.0, 22.0),
            task.get_label(),
            11.5,
            FontWeight::Medium,
            eva.get_foreground(),
            TextAlign::Left,
        );
        let bx: f32 = track_x + (task.get_start_unit() / max_end) * track_w;
        let bw: f32 = ((task.get_duration_unit() / max_end) * track_w).max(24.0);
        let bar_col: Color = match task.get_status() {
            MermaidGanttStatus::Done => eva.get_diff_add(),
            MermaidGanttStatus::Active => eva.get_function(),
            MermaidGanttStatus::Critical => eva.get_diff_del(),
            MermaidGanttStatus::Normal => eva.get_operator(),
        };
        dl.push(DrawCommand::FillRoundedRect {
            rect: Rect::new(bx, y + 3.0, bw, 20.0),
            color: bar_col,
            radius: CornerRadius::all(5.0),
            border: Border::default(),
        });
        y += 32.0;
    }
}

/// Computes clean boundary anchor points between two rectangles along the active flow axis.
///
/// # Arguments
///
/// - `Rect` - Source box bounds.
/// - `Rect` - Target box bounds.
/// - `bool` - Whether primary layout flow is horizontal.
///
/// # Returns
///
/// - `(Point, Point)` - Connector `(start, end)` anchor points.
fn compute_rect_anchor_points(ra: Rect, rb: Rect, horiz: bool) -> (Point, Point) {
    let ax: f32 = ra.get_x() + ra.get_width() * 0.5;
    let ay: f32 = ra.get_y() + ra.get_height() * 0.5;
    let bx: f32 = rb.get_x() + rb.get_width() * 0.5;
    let by: f32 = rb.get_y() + rb.get_height() * 0.5;
    if horiz {
        if bx >= ax {
            (Point::new(ra.right(), ay), Point::new(rb.get_x(), by))
        } else {
            (Point::new(ra.get_x(), ay), Point::new(rb.right(), by))
        }
    } else if by >= ay {
        (Point::new(ax, ra.bottom()), Point::new(bx, rb.get_y()))
    } else {
        (Point::new(ax, ra.get_y()), Point::new(bx, rb.bottom()))
    }
}

/// Draws a solid or dashed vector connector with an optional filled triangular arrowhead.
///
/// # Arguments
///
/// - `&mut DisplayList` - Active draw command list.
/// - `Point` - Start point.
/// - `Point` - End point.
/// - `Color` - Stroke and arrowhead color.
/// - `f32` - Stroke width in logical pixels.
/// - `bool` - When `true`, draws dashed line segments.
/// - `bool` - When `true`, draws a filled arrowhead pointing at `to`.
fn draw_vector_connector(
    dl: &mut DisplayList,
    from: Point,
    to: Point,
    color: Color,
    width: f32,
    dashed: bool,
    arrow: bool,
) {
    let dx: f32 = to.get_x() - from.get_x();
    let dy: f32 = to.get_y() - from.get_y();
    let len: f32 = dx.hypot(dy);
    if len < 1.0 {
        return;
    }
    let ux: f32 = dx / len;
    let uy: f32 = dy / len;
    let line_end: Point = if arrow {
        Point::new(to.get_x() - ux * 7.5, to.get_y() - uy * 7.5)
    } else {
        to
    };
    if dashed {
        draw_dashed_line(dl, from, line_end, color, width);
    } else {
        dl.push(DrawCommand::Line {
            from,
            to: line_end,
            color,
            width,
        });
    }
    if arrow {
        let head_len: f32 = 8.5;
        let head_w: f32 = 4.6;
        let base_x: f32 = to.get_x() - ux * head_len;
        let base_y: f32 = to.get_y() - uy * head_len;
        let left: Point = Point::new(base_x - uy * head_w, base_y + ux * head_w);
        let right: Point = Point::new(base_x + uy * head_w, base_y - ux * head_w);
        dl.push(DrawCommand::Triangle {
            p0: to,
            p1: left,
            p2: right,
            color,
        });
    }
}

/// Draws a crisp dashed vector line segment from `from` to `to`.
///
/// # Arguments
///
/// - `&mut DisplayList` - Active draw command list.
/// - `Point` - Segment start point.
/// - `Point` - Segment end point.
/// - `Color` - Dash color.
/// - `f32` - Stroke width in logical pixels.
fn draw_dashed_line(dl: &mut DisplayList, from: Point, to: Point, color: Color, width: f32) {
    let dx: f32 = to.get_x() - from.get_x();
    let dy: f32 = to.get_y() - from.get_y();
    let len: f32 = dx.hypot(dy);
    if len < 1.0 {
        return;
    }
    let ux: f32 = dx / len;
    let uy: f32 = dy / len;
    let dash: f32 = 6.0;
    let gap: f32 = 4.0;
    let mut dist: f32 = 0.0;
    while dist < len {
        let seg_end: f32 = (dist + dash).min(len);
        dl.push(DrawCommand::Line {
            from: Point::new(from.get_x() + ux * dist, from.get_y() + uy * dist),
            to: Point::new(from.get_x() + ux * seg_end, from.get_y() + uy * seg_end),
            color,
            width,
        });
        dist += dash + gap;
    }
}

/// Pushes a vertically centered `DrawCommand::TextRun` inside `rect`.
///
/// # Arguments
///
/// - `&mut DisplayList` - Active draw command list.
/// - `Rect` - Bounding box for text placement.
/// - `&str` - Label string.
/// - `f32` - Font size in logical pixels.
/// - `FontWeight` - Font weight.
/// - `Color` - Text foreground color.
/// - `TextAlign` - Horizontal text alignment.
fn draw_centered_label(
    dl: &mut DisplayList,
    rect: Rect,
    text: &str,
    font_size: f32,
    weight: FontWeight,
    color: Color,
    align: TextAlign,
) {
    if text.is_empty() {
        return;
    }
    let line_h: f32 = font_size * 1.25;
    let ty: f32 = rect.get_y() + ((rect.get_height() - line_h).max(0.0)) * 0.5;
    let mut style: TextStyle = TextStyle::default();
    style
        .set_font_size(font_size)
        .set_weight(weight)
        .set_color(color)
        .set_align(align)
        .set_overflow(TextOverflow::Clip);
    dl.push(DrawCommand::TextRun {
        rect: Rect::new(rect.get_x(), ty, rect.get_width().max(8.0), line_h),
        text: text.to_string(),
        style,
    });
}
