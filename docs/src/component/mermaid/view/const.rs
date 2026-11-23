use super::*;

class! {
    // The figure owns the frame; the diagram inside it stays unframed
    // so the engine's own geometry is the only layout the reader sees.
    pub c_docs_mermaid {
        margin: format!("{} 0", var!(space-lg));
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-lg);
        background: var!(muted);
        overflow: "hidden";
        min-width: "0";
    }

    // The scroll frame: a wide diagram scrolls sideways instead of
    // pushing the article wider than the content column.
    pub c_docs_mermaid_frame {
        padding: var!(space-lg);
        overflow-x: "auto";
        overflow-y: "hidden";
    }
}
