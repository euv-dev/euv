/// Default maximum height in logical pixels for inline scrollable Mermaid diagram viewports.
pub(crate) const MERMAID_DEFAULT_MAX_HEIGHT: f32 = 420.0;
/// Overlay stack identifier for the `EuvMermaid` fullscreen viewer modal.
pub(crate) const MERMAID_FULLSCREEN_OVERLAY_NAME: &str = "EuvMermaidFullscreen";
/// Header badge prefix displayed on Mermaid diagrams.
pub(crate) const MERMAID_BADGE_FLOWCHART: &str = "MERMAID FLOWCHART";
/// Header badge label for sequence diagrams.
pub(crate) const MERMAID_BADGE_SEQUENCE: &str = "MERMAID SEQUENCE";
/// Header badge label for state diagrams.
pub(crate) const MERMAID_BADGE_STATE: &str = "MERMAID STATE";
/// Header badge label for class diagrams.
pub(crate) const MERMAID_BADGE_CLASS: &str = "MERMAID CLASS";
/// Header badge label for entity-relationship diagrams.
pub(crate) const MERMAID_BADGE_ER: &str = "MERMAID ER";
/// Header badge label for pie charts.
pub(crate) const MERMAID_BADGE_PIE: &str = "MERMAID PIE";
/// Header badge label for Gantt timelines.
pub(crate) const MERMAID_BADGE_GANTT: &str = "MERMAID GANTT";
/// Button label opening fullscreen Mermaid modal.
pub(crate) const MERMAID_BTN_FULLSCREEN: &str = "Fullscreen";
/// Button label closing fullscreen Mermaid modal.
pub(crate) const MERMAID_BTN_CLOSE: &str = "Close";
/// Button label copying Mermaid source.
pub(crate) const MERMAID_BTN_COPY: &str = "Copy";
/// Button label confirming Mermaid source copy.
pub(crate) const MERMAID_BTN_COPIED: &str = "Copied";
/// Keyword identifying a Mermaid flowchart header.
pub(crate) const MERMAID_KW_FLOWCHART: &str = "flowchart";
/// Keyword identifying a Mermaid graph header.
pub(crate) const MERMAID_KW_GRAPH: &str = "graph";
/// Keyword identifying a Mermaid sequence diagram.
pub(crate) const MERMAID_KW_SEQUENCE: &str = "sequencediagram";
/// Keyword identifying a Mermaid state diagram.
pub(crate) const MERMAID_KW_STATE: &str = "statediagram";
/// Keyword identifying a Mermaid class diagram.
pub(crate) const MERMAID_KW_CLASS: &str = "classdiagram";
/// Keyword identifying a Mermaid entity-relationship diagram.
pub(crate) const MERMAID_KW_ER: &str = "erdiagram";
/// Keyword identifying a Mermaid pie chart.
pub(crate) const MERMAID_KW_PIE: &str = "pie";
/// Keyword identifying a Mermaid Gantt chart.
pub(crate) const MERMAID_KW_GANTT: &str = "gantt";
/// Keyword identifying a flowchart subgraph opening.
pub(crate) const MERMAID_KW_SUBGRAPH: &str = "subgraph ";
/// Alias separator token used in Mermaid participant and state declarations.
pub(crate) const MERMAID_ALIAS_SEP: &str = " as ";
/// Keyword identifying a Mermaid block close.
pub(crate) const MERMAID_KW_END: &str = "end";
/// Keyword identifying a sequence diagram participant.
pub(crate) const MERMAID_KW_PARTICIPANT: &str = "participant ";
/// Keyword identifying a sequence diagram actor.
pub(crate) const MERMAID_KW_ACTOR: &str = "actor ";
/// Keyword identifying a sequence diagram note.
pub(crate) const MERMAID_KW_NOTE: &str = "note ";
/// Keyword identifying a title directive.
pub(crate) const MERMAID_KW_TITLE: &str = "title ";
/// Keyword identifying a Gantt section directive.
pub(crate) const MERMAID_KW_SECTION: &str = "section ";
/// Keyword identifying a Gantt dateFormat directive.
pub(crate) const MERMAID_KW_DATE_FORMAT: &str = "dateformat";
/// Keyword identifying a Mermaid state declaration.
pub(crate) const MERMAID_KW_STATE_DECL: &str = "state ";
/// Keyword identifying a Mermaid class declaration.
pub(crate) const MERMAID_KW_CLASS_DECL: &str = "class ";
/// Marker identifying a start or end terminal state.
pub(crate) const MERMAID_STATE_TERMINAL: &str = "[*]";
/// Default Gantt section name when omitted.
pub(crate) const MERMAID_DEFAULT_SECTION: &str = "Timeline";
/// Pie chart curated categorical color palette (RGB hex).
pub(crate) const MERMAID_PIE_PALETTE: [u32; 8] = [
    0x437AED, 0x7C4DFF, 0x00BEC4, 0x44C145, 0xFF6D12, 0xC838C6, 0xCD6069, 0xF0AA0B,
];
/// Gantt task status keyword for completed bars.
pub(crate) const MERMAID_GANTT_DONE: &str = "done";
/// Gantt task status keyword for active bars.
pub(crate) const MERMAID_GANTT_ACTIVE: &str = "active";
/// Gantt task status keyword for critical-path bars.
pub(crate) const MERMAID_GANTT_CRIT: &str = "crit";
