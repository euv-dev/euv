use super::*;

/// Layout flow axis for Mermaid `flowchart` / `graph` diagrams.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum MermaidFlowDirection {
    /// Top-to-bottom hierarchical layout (`TD` / `TB`).
    #[default]
    TopDown,
    /// Bottom-to-top hierarchical layout (`BT`).
    BottomTop,
    /// Left-to-right pipeline layout (`LR`).
    LeftRight,
    /// Right-to-left pipeline layout (`RL`).
    RightLeft,
}

/// Vector shape rendered for a Mermaid flowchart or state node.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum MermaidNodeShape {
    /// Standard sharp/subtle rounded box (`[Label]`).
    #[default]
    Box,
    /// Soft rounded box (`(Label)`).
    Rounded,
    /// Decision diamond (`{Label}`).
    Diamond,
    /// Pill / stadium terminal (`([Label])`).
    Stadium,
    /// Circular node (`((Label))`).
    Circle,
    /// State diagram start filled dot (`[*]`).
    StateStart,
    /// State diagram end bullseye (`[*]`).
    StateEnd,
}

/// Connector stroke style for Mermaid edges and sequence arrows.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum MermaidEdgeStyle {
    /// Standard solid directed arrow (`-->` / `->>`).
    #[default]
    Solid,
    /// Dashed directed arrow (`-.->` / `-->>`).
    Dashed,
    /// Emphasized thick solid arrow (`==>`).
    Thick,
    /// Undirected solid line (`---`).
    Plain,
}

/// Task bar visual status inside a Mermaid Gantt chart.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum MermaidGanttStatus {
    /// Default scheduled task bar.
    #[default]
    Normal,
    /// Completed task bar (`done`).
    Done,
    /// Active in-progress task bar (`active`).
    Active,
    /// Critical path task bar (`crit`).
    Critical,
}

/// High-level parsed Mermaid diagram AST rendered natively onto `euv` vector `DisplayList` primitives.
#[derive(Clone, Debug, PartialEq)]
pub enum MermaidDiagram {
    /// Directed flowchart (`flowchart` / `graph`).
    Flowchart(MermaidFlowchart),
    /// Interaction sequence diagram (`sequenceDiagram`).
    Sequence(MermaidSequence),
    /// Finite state machine diagram (`stateDiagram` / `stateDiagram-v2`).
    State(MermaidFlowchart),
    /// UML class diagram (`classDiagram`).
    Class(MermaidClassDiagram),
    /// Entity-relationship diagram (`erDiagram`).
    Er(MermaidErDiagram),
    /// Proportional pie chart (`pie`).
    Pie(MermaidPieChart),
    /// Timeline Gantt chart (`gantt`).
    Gantt(MermaidGanttChart),
}
