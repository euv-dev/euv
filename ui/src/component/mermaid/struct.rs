use super::*;

/// Single node inside a Mermaid flowchart or state diagram.
#[derive(Clone, Data, Debug, Eq, New, PartialEq)]
pub struct MermaidFlowNode {
    /// Unique identifier referenced by edges.
    pub(crate) id: String,
    /// Visible human-readable label.
    pub(crate) label: String,
    /// Rendered geometric node shape.
    #[get(type(copy))]
    pub(crate) shape: MermaidNodeShape,
    /// Optional enclosing subgraph index.
    #[get(skip)]
    pub(crate) subgraph: Option<usize>,
}

/// Directed or undirected edge connecting two flowchart/state nodes.
#[derive(Clone, Data, Debug, Eq, New, PartialEq)]
pub struct MermaidFlowEdge {
    /// Source node identifier.
    pub(crate) from: String,
    /// Destination node identifier.
    pub(crate) to: String,
    /// Optional edge label rendered along the connector.
    pub(crate) label: String,
    /// Line stroke and arrowhead style.
    #[get(type(copy))]
    pub(crate) style: MermaidEdgeStyle,
}

/// Named subgraph group box enclosing a subset of flowchart nodes.
#[derive(Clone, Data, Debug, Eq, New, PartialEq)]
pub struct MermaidSubgraph {
    /// Visible subgraph title.
    pub(crate) title: String,
}

/// Parsed Mermaid flowchart or state graph.
#[derive(Clone, Data, Debug, Default, Eq, PartialEq)]
pub struct MermaidFlowchart {
    /// Layout flow direction (`TD`, `BT`, `LR`, `RL`).
    #[get(type(copy))]
    pub(crate) direction: MermaidFlowDirection,
    /// Ordered unique nodes.
    pub(crate) nodes: Vec<MermaidFlowNode>,
    /// Ordered connectors.
    pub(crate) edges: Vec<MermaidFlowEdge>,
    /// Named subgraph frames.
    pub(crate) subgraphs: Vec<MermaidSubgraph>,
}

/// Single message or note step inside a Mermaid `sequenceDiagram`.
#[derive(Clone, Data, Debug, Eq, New, PartialEq)]
pub struct MermaidSequenceMessage {
    /// Source participant index.
    #[get(type(copy))]
    pub(crate) from_idx: usize,
    /// Target participant index.
    #[get(type(copy))]
    pub(crate) to_idx: usize,
    /// Message label or note copy.
    pub(crate) text: String,
    /// Connector style (`Solid` or `Dashed`).
    #[get(type(copy))]
    pub(crate) style: MermaidEdgeStyle,
    /// When `true`, renders as a callout note spanning `from_idx..=to_idx`.
    #[get(type(copy))]
    pub(crate) is_note: bool,
}

/// Parsed Mermaid `sequenceDiagram` with ordered participants and message rows.
#[derive(Clone, Data, Debug, Default, Eq, PartialEq)]
pub struct MermaidSequence {
    /// Ordered participant `(id, display_label)` entries.
    pub(crate) participants: Vec<(String, String)>,
    /// Ordered chronological message/note steps.
    pub(crate) messages: Vec<MermaidSequenceMessage>,
}

/// Single class entity inside a Mermaid `classDiagram`.
#[derive(Clone, Data, Debug, Eq, New, PartialEq)]
pub struct MermaidClassBox {
    /// Class identifier and header name.
    pub(crate) name: String,
    /// Declared field and method member lines.
    pub(crate) members: Vec<String>,
}

/// Relationship connector between two classes inside a `classDiagram`.
#[derive(Clone, Data, Debug, Eq, New, PartialEq)]
pub struct MermaidClassRelation {
    /// Source class name.
    pub(crate) from: String,
    /// Target class name.
    pub(crate) to: String,
    /// Relationship label.
    pub(crate) label: String,
    /// Dashed or solid connector style.
    #[get(type(copy))]
    pub(crate) style: MermaidEdgeStyle,
}

/// Parsed Mermaid `classDiagram`.
#[derive(Clone, Data, Debug, Default, Eq, PartialEq)]
pub struct MermaidClassDiagram {
    /// Ordered class boxes.
    pub(crate) classes: Vec<MermaidClassBox>,
    /// Ordered relationships.
    pub(crate) relations: Vec<MermaidClassRelation>,
}

/// Single entity table inside a Mermaid `erDiagram`.
#[derive(Clone, Data, Debug, Eq, New, PartialEq)]
pub struct MermaidErEntity {
    /// Entity name.
    pub(crate) name: String,
    /// Declared attribute rows (e.g. `"int id PK"`).
    pub(crate) attributes: Vec<String>,
}

/// Cardinality relationship between two ER entities.
#[derive(Clone, Data, Debug, Eq, New, PartialEq)]
pub struct MermaidErRelation {
    /// Left entity name.
    pub(crate) left: String,
    /// Right entity name.
    pub(crate) right: String,
    /// Left cardinality token (e.g. `"||"`).
    pub(crate) left_card: String,
    /// Right cardinality token (e.g. `"o{"`).
    pub(crate) right_card: String,
    /// Relationship verb label.
    pub(crate) label: String,
}

/// Parsed Mermaid `erDiagram`.
#[derive(Clone, Data, Debug, Default, Eq, PartialEq)]
pub struct MermaidErDiagram {
    /// Ordered entities.
    pub(crate) entities: Vec<MermaidErEntity>,
    /// Ordered cardinality relationships.
    pub(crate) relations: Vec<MermaidErRelation>,
}

/// Single proportional slice inside a Mermaid `pie` chart.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct MermaidPieSlice {
    /// Slice display label.
    pub(crate) label: String,
    /// Positive numeric weight.
    #[get(type(copy))]
    pub(crate) value: f32,
}

/// Parsed Mermaid `pie` chart.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct MermaidPieChart {
    /// Optional chart title.
    pub(crate) title: String,
    /// Ordered pie slices.
    pub(crate) slices: Vec<MermaidPieSlice>,
}

/// Single timeline task bar inside a Mermaid `gantt` chart.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct MermaidGanttTask {
    /// Enclosing section label.
    pub(crate) section: String,
    /// Task display title.
    pub(crate) label: String,
    /// Visual bar status.
    #[get(type(copy))]
    pub(crate) status: MermaidGanttStatus,
    /// Relative start offset in timeline units.
    #[get(type(copy))]
    pub(crate) start_unit: f32,
    /// Duration span in timeline units.
    #[get(type(copy))]
    pub(crate) duration_unit: f32,
}

/// Parsed Mermaid `gantt` chart.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct MermaidGanttChart {
    /// Optional chart title.
    pub(crate) title: String,
    /// Ordered timeline tasks.
    pub(crate) tasks: Vec<MermaidGanttTask>,
}

/// Native vector Mermaid diagram component rendering flowcharts, sequence, state, class, ER, pie, and Gantt charts onto `euv` `canvas` with H+V scrolling and fullscreen inspection.
#[derive(Clone, CustomDebug, Data, New, Props)]
pub struct EuvMermaid {
    /// Raw Mermaid diagram source text.
    pub(crate) source: String,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
    /// Optional maximum inline viewport height in logical pixels (`0.0` defaults to `420.0`).
    #[new(skip)]
    #[get(type(copy))]
    #[prop(custom)]
    pub(crate) max_height: f32,
    /// Optional copy callback invoked with the raw Mermaid source when Copy is clicked.
    #[new(skip)]
    #[debug(skip)]
    #[get(skip)]
    #[prop(custom)]
    pub(crate) on_copy: Option<MarkdownStrCallback>,
}
