use super::*;

/// The prefix of a diagram's scoped id.
///
/// The id names the diagram's marker ids and its scoped stylesheet
/// selectors, so two diagrams on one page never collide.
pub(crate) const DIAGRAM_ID_PREFIX: &str = "docs-mermaid-";

/// The stylesheet the engine appends to every diagram it renders.
///
/// Every colour is a docs theme token, and CSS custom properties
/// inherit into an inline SVG, so the diagram follows the root theme
/// class — light or dark — without a re-render and without
/// `!important`. The engine's own defaults stay in place for anything
/// this sheet does not name.
pub(crate) const SCOPED_CSS: &str = r#"
.actor{stroke:var(--foreground);fill:var(--muted);}
.actor-line{stroke:var(--foreground);}
.messageLine0,.messageLine1{stroke:var(--foreground);}
.note{stroke:var(--border);fill:var(--muted);}
text.actor>tspan{fill:var(--foreground);stroke:none;}
.noteText,.noteText>tspan{fill:var(--foreground);stroke:none;}
.messageText,.messageText>tspan{fill:var(--foreground);stroke:none;}
.labelBox{stroke:var(--border);fill:var(--muted);}
.labelText,.labelText>tspan{fill:var(--foreground);stroke:none;}
.loopText,.loopText>tspan{fill:var(--foreground);stroke:none;}
.sectionTitle,.sectionTitle>tspan{fill:var(--foreground);stroke:none;}
.loopLabel{stroke:var(--foreground);fill:var(--muted);}
.loopLabel>tspan{fill:var(--foreground);stroke:none;}
"#;

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
