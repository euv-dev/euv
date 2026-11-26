use super::*;

/// Parses a sequence of sibling `ViewNode` items separated by optional commas or semicolons.
///
/// # Arguments
///
/// - `ParseStream<'_>` - Token stream inside a block or root invocation.
///
/// # Returns
///
/// - `Result<Vec<ViewNode>>` - Ordered sibling nodes.
pub(crate) fn parse_node_list(input: ParseStream<'_>) -> Result<Vec<ViewNode>> {
    let mut nodes: Vec<ViewNode> = Vec::new();
    while !input.is_empty() {
        let node: ViewNode = input.parse()?;
        let _: Option<Token![,]> = input.parse()?;
        let _: Option<Token![;]> = input.parse()?;
        nodes.push(node);
    }
    Ok(nodes)
}

/// Returns whether a parsed tag `path` and optional `args` represents a primitive `view!` element.
///
/// # Arguments
///
/// - `&Path` - Parsed tag path.
/// - `bool` - Whether parenthesized call arguments were present.
///
/// # Returns
///
/// - `bool` - `true` when `path` is a built-in primitive element tag without call parens.
pub(crate) fn is_primitive_tag(path: &Path, has_args: bool) -> bool {
    if has_args {
        return false;
    }
    let Some(ident) = path.get_ident() else {
        return false;
    };
    let name: String = ident.to_string();
    matches!(
        name.as_str(),
        TAG_VIEW
            | TAG_ROW
            | TAG_COLUMN
            | TAG_COL
            | TAG_GRID
            | TAG_SCROLL
            | TAG_PORTAL
            | TAG_FRAGMENT
            | TAG_TEXT
            | TAG_INPUT
            | TAG_IMAGE
            | TAG_CANVAS
            | TAG_SLOT
            | TAG_PAGE_SLOT
    )
}

/// Returns whether a parsed element should be treated as a component builder invocation.
///
/// # Arguments
///
/// - `&Path` - Parsed element path.
/// - `bool` - Whether parenthesized call arguments were present.
///
/// # Returns
///
/// - `bool` - `true` when `path` is not a built-in primitive tag.
pub(crate) fn is_component_element(path: &Path, has_args: bool) -> bool {
    !is_primitive_tag(path, has_args)
}

/// Detaches a next-line `..spread` child mistakenly parsed as an infix `Range` on a comma-less property expression.
///
/// # Arguments
///
/// - `Expr` - Parsed property value expression.
///
/// # Returns
///
/// - `(Expr, Option<ViewNode>)` - Clean property value expression and optional detached `ViewNode::Spread`.
pub(crate) fn split_trailing_line_spread(expr: Expr) -> (Expr, Option<ViewNode>) {
    match expr {
        Expr::Range(range) => {
            let should_split: bool = match (&range.start, &range.limits, &range.end) {
                (Some(start), RangeLimits::HalfOpen(dots), Some(_)) => {
                    start.span().end().line < dots.span().start().line
                }
                _ => false,
            };
            if should_split {
                let ExprRange {
                    start: Some(start),
                    limits: RangeLimits::HalfOpen(dots),
                    end: Some(end),
                    ..
                } = range
                else {
                    return (Expr::Range(range), None);
                };
                return (*start, Some(ViewNode::Spread(dots.span(), end)));
            }
            (Expr::Range(range), None)
        }
        Expr::Closure(mut closure) => {
            let (body, spread): (Expr, Option<ViewNode>) =
                split_trailing_line_spread(*closure.body);
            closure.body = Box::new(body);
            (Expr::Closure(closure), spread)
        }
        other => (other, None),
    }
}

/// Peeks whether the next tokens inside a component block are a lowercase fluent method call (`flag()` or `method(args...)`).
///
/// # Arguments
///
/// - `ParseStream<'_>` - Current component body token stream.
///
/// # Returns
///
/// - `bool` - `true` when the next item is a lowercase `ident(args...)` not followed by `{`.
pub(crate) fn is_lowercase_flag_call(input: ParseStream<'_>) -> bool {
    parse_lowercase_flag_peek(&input.fork()).unwrap_or(false)
}

/// Helper parsing a forked stream to test for a lowercase fluent builder call (`flag()` or `method(args...)`).
///
/// # Arguments
///
/// - `ParseStream<'_>` - Forked token stream.
///
/// # Returns
///
/// - `Result<bool>` - `Ok(true)` when next tokens are `lowercase_ident(...)` not followed by `{`.
fn parse_lowercase_flag_peek(fork: ParseStream<'_>) -> Result<bool> {
    let ident: Ident = fork.parse()?;
    let name: String = ident.to_string();
    let starts_lower: bool = name
        .chars()
        .next()
        .map(|ch: char| ch.is_ascii_lowercase())
        .unwrap_or(false);
    if !starts_lower || !fork.peek(Paren) {
        return Ok(false);
    }
    let _inner: ParseBuffer<'_>;
    parenthesized!(_inner in fork);
    Ok(!fork.peek(Brace))
}

/// Expands a parsed `ViewMacroInput` into zero-VDOM arena node construction tokens.
///
/// # Arguments
///
/// - `ViewMacroInput` - Parsed `view!` macro AST.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Generated Rust expression evaluating to `::euv::NodeId`.
pub(crate) fn expand_view(input: ViewMacroInput) -> proc_macro2::TokenStream {
    let ctx_setup: proc_macro2::TokenStream = quote! {
        let mut __euv_ctx_owned: ::euv::Context = ::euv::current_context();
        let __euv_ctx: &mut ::euv::Context = &mut __euv_ctx_owned;
    };
    let body: proc_macro2::TokenStream = if input.roots.is_empty() {
        quote! { __euv_ctx.fragment() }
    } else if input.roots.len() == 1 && is_single_node_expr(&input.roots[0]) {
        expand_single_node(&input.roots[0])
    } else {
        let root_ident: Ident = Ident::new(IDENT_ROOT, Span::call_site());
        let attach: proc_macro2::TokenStream = expand_children_into(&root_ident, &input.roots);
        quote! {
            let #root_ident: ::euv::NodeId = __euv_ctx.fragment();
            #attach
            #root_ident
        }
    };
    quote! {{
        #ctx_setup
        #body
    }}
}

/// Returns whether `node` always evaluates directly to a single `NodeId` expression.
///
/// # Arguments
///
/// - `&ViewNode` - Node AST to inspect.
///
/// # Returns
///
/// - `bool` - `true` for elements, static text, and braced expressions.
fn is_single_node_expr(node: &ViewNode) -> bool {
    matches!(
        node,
        ViewNode::Element(_) | ViewNode::StaticText(_) | ViewNode::Expr(_)
    )
}

/// Returns a helpful native migration error if `tag` is a legacy Web/HTML element name.
///
/// # Arguments
///
/// - `&Ident` - Element tag identifier.
///
/// # Returns
///
/// - `Option<proc_macro2::TokenStream>` - Compile error tokens if `tag` is an HTML tag, else `None`.
fn legacy_html_tag_error(tag: &Ident) -> Option<proc_macro2::TokenStream> {
    let name: String = tag.to_string();
    let replacement: &str = if HTML_CONTAINER_TAGS.contains(&name.as_str()) {
        HTML_REPLACE_CONTAINER
    } else if HTML_TEXT_TAGS.contains(&name.as_str()) {
        HTML_REPLACE_TEXT
    } else if HTML_BUTTON_TAGS.contains(&name.as_str()) {
        HTML_REPLACE_BUTTON
    } else if HTML_IMAGE_TAGS.contains(&name.as_str()) {
        HTML_REPLACE_IMAGE
    } else {
        return None;
    };
    let msg: String =
        format!("HTML tag `{name}` is not part of native euv `view!` — use {replacement} instead");
    Some(Error::new(tag.span(), msg).to_compile_error())
}

/// Expands a single-node `ViewNode` into a span-preserving `::euv::NodeId` expression.
///
/// # Arguments
///
/// - `&ViewNode` - Node AST to expand.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Expanded node expression.
fn expand_single_node(node: &ViewNode) -> proc_macro2::TokenStream {
    match node {
        ViewNode::StaticText(lit) => {
            quote_spanned! {lit.span()=>
                __euv_ctx.text(::euv::into_text(#lit), ::euv::Style::default())
            }
        }
        ViewNode::Expr(expr) => match expr.as_ref() {
            Expr::Closure(_) => {
                quote_spanned! {expr.span()=>
                    __euv_ctx.reactive_text(::euv::Style::default(), #expr)
                }
            }
            other => {
                quote_spanned! {other.span()=>
                    ::euv::Component::mount(#other)
                }
            }
        },
        ViewNode::Element(elem) => expand_element(elem),
        ViewNode::Spread(span, _) => {
            Error::new(*span, ERR_SPREAD_IN_EXPR_CONTEXT).to_compile_error()
        }
        ViewNode::If(_) | ViewNode::For(_) | ViewNode::Match(_) => {
            let frag_ident: Ident = Ident::new(IDENT_FRAG, Span::call_site());
            let attach: proc_macro2::TokenStream = expand_node_into(&frag_ident, node);
            quote! {{
                let #frag_ident: ::euv::NodeId = __euv_ctx.fragment();
                #attach
                #frag_ident
            }}
        }
    }
}

/// Expands a slice of sibling `ViewNode` items into optimal batched `append_many` / `append` calls onto `parent`.
///
/// # Arguments
///
/// - `&Ident` - Parent `NodeId` variable identifier.
/// - `&[ViewNode]` - Ordered sibling children.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Statements appending all siblings to `parent`.
fn expand_children_into(parent: &Ident, children: &[ViewNode]) -> proc_macro2::TokenStream {
    if children.is_empty() {
        return quote! {};
    }
    let mut out: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut batch: Vec<proc_macro2::TokenStream> = Vec::new();
    for child in children.iter() {
        if is_single_node_expr(child) {
            batch.push(expand_single_node(child));
        } else {
            flush_child_batch(parent, &mut batch, &mut out);
            out.push(expand_node_into(parent, child));
        }
    }
    flush_child_batch(parent, &mut batch, &mut out);
    quote! { #(#out)* }
}

/// Flushes accumulated contiguous single-node children into a single `append_many` (or `append` for 1 child).
///
/// # Arguments
///
/// - `&Ident` - Parent `NodeId` identifier.
/// - `&mut Vec<proc_macro2::TokenStream>` - Accumulated child expressions to drain.
/// - `&mut Vec<proc_macro2::TokenStream>` - Output statement sink.
fn flush_child_batch(
    parent: &Ident,
    batch: &mut Vec<proc_macro2::TokenStream>,
    out: &mut Vec<proc_macro2::TokenStream>,
) {
    if batch.is_empty() {
        return;
    }
    if batch.len() == 1 {
        let single: proc_macro2::TokenStream = batch.remove(0);
        out.push(quote! {
            let __euv_child: ::euv::NodeId = #single;
            __euv_ctx.append(#parent, __euv_child);
        });
    } else {
        let items: Vec<proc_macro2::TokenStream> = take(batch);
        let len: usize = items.len();
        out.push(quote! {
            let __euv_children: [::euv::NodeId; #len] = [#(#items),*];
            __euv_ctx.append_many(#parent, &__euv_children);
        });
    }
}

/// Expands a single `ViewNode` (including control flow and spread) into statements appending into `parent`.
///
/// # Arguments
///
/// - `&Ident` - Parent `NodeId` identifier.
/// - `&ViewNode` - Child node AST.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Statements mounting and attaching `node` into `parent`.
fn expand_node_into(parent: &Ident, node: &ViewNode) -> proc_macro2::TokenStream {
    match node {
        ViewNode::Spread(span, expr) => quote_spanned! {*span=>
            for __euv_spread_item in #expr {
                let __euv_child: ::euv::NodeId = ::euv::Component::mount(__euv_spread_item);
                __euv_ctx.append(#parent, __euv_child);
            }
        },
        ViewNode::If(if_node) => expand_if_into(parent, if_node),
        ViewNode::For(for_node) => {
            let pat: &Pat = &for_node.pat;
            let iter: &Expr = &for_node.iter;
            let body: proc_macro2::TokenStream = expand_children_into(parent, &for_node.body);
            quote_spanned! {for_node.span=>
                for #pat in #iter {
                    #body
                }
            }
        }
        ViewNode::Match(match_node) => {
            let expr: &Expr = &match_node.expr;
            let arms: Vec<proc_macro2::TokenStream> = match_node
                .arms
                .iter()
                .map(|arm: &ViewMatchArm| {
                    let pat: &Pat = &arm.pat;
                    let guard: proc_macro2::TokenStream = match &arm.guard {
                        Some(g) => quote! { if #g },
                        None => quote! {},
                    };
                    let body: proc_macro2::TokenStream = expand_children_into(parent, &arm.body);
                    quote! { #pat #guard => { #body } }
                })
                .collect();
            quote_spanned! {match_node.span=>
                match #expr {
                    #(#arms)*
                }
            }
        }
        other => {
            let expr: proc_macro2::TokenStream = expand_single_node(other);
            quote! {
                let __euv_child: ::euv::NodeId = #expr;
                __euv_ctx.append(#parent, __euv_child);
            }
        }
    }
}

/// Expands a `ViewIf` node recursively into `if` / `if let` / `else if` / `else` statements targeting `parent`.
///
/// # Arguments
///
/// - `&Ident` - Parent `NodeId` identifier.
/// - `&ViewIf` - Conditional AST node.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Conditional mounting statements.
fn expand_if_into(parent: &Ident, if_node: &ViewIf) -> proc_macro2::TokenStream {
    let cond_tokens: proc_macro2::TokenStream = match &if_node.cond {
        ViewIfCond::Expr(expr) => quote! { #expr },
        ViewIfCond::Let(pat, expr) => quote! { let #pat = #expr },
    };
    let then_body: proc_macro2::TokenStream = expand_children_into(parent, &if_node.then_nodes);
    let else_tokens: proc_macro2::TokenStream = match &if_node.else_branch {
        Some(ViewElseBranch::ElseIf(nested)) => {
            let nested_if: proc_macro2::TokenStream = expand_if_into(parent, nested);
            quote! { else #nested_if }
        }
        Some(ViewElseBranch::ElseBlock(nodes)) => {
            let else_body: proc_macro2::TokenStream = expand_children_into(parent, nodes);
            quote! { else { #else_body } }
        }
        None => quote! {},
    };
    quote_spanned! {if_node.span=>
        if #cond_tokens {
            #then_body
        } #else_tokens
    }
}

/// Expands a `ViewElement` either as a built-in primitive element or as a typed component builder invocation.
///
/// # Arguments
///
/// - `&ViewElement` - Element AST to expand.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Expanded expression returning `::euv::NodeId`.
fn expand_element(elem: &ViewElement) -> proc_macro2::TokenStream {
    if elem.args.is_none()
        && let Some(ident) = elem.path.get_ident()
        && let Some(html_err) = legacy_html_tag_error(ident)
    {
        return html_err;
    }
    if is_component_element(&elem.path, elem.args.is_some()) {
        return expand_component_element(elem);
    }
    expand_primitive_element(elem)
}

/// Expands a typed component invocation `Comp(args...) { flag() prop: val children... }` with zero runtime overhead.
///
/// # Arguments
///
/// - `&ViewElement` - Component AST node.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Expanded component mount expression returning `::euv::NodeId`.
fn expand_component_element(elem: &ViewElement) -> proc_macro2::TokenStream {
    let path: &Path = &elem.path;
    let span: Span = path.span();
    let last_name: String = path
        .segments
        .last()
        .map(|seg: &PathSegment| seg.ident.to_string())
        .unwrap_or_default();
    let is_type_ctor: bool = last_name
        .chars()
        .next()
        .map(|ch: char| ch.is_ascii_uppercase())
        .unwrap_or(false);
    let args: &[Expr] = elem.args.as_deref().unwrap_or(&[]);
    let mut builder_expr: proc_macro2::TokenStream = if is_type_ctor {
        quote_spanned! {span=> #path::new(#(#args),*) }
    } else {
        quote_spanned! {span=> #path(#(#args),*) }
    };
    let mut seen_props: HashSet<String> = HashSet::new();
    let mut node_ref_expr: Option<&Expr> = None;
    for flag in elem.flags.iter() {
        let fname: &Ident = &flag.name;
        let fargs: &[Expr] = &flag.args;
        builder_expr = quote_spanned! {fname.span()=> (#builder_expr).#fname(#(#fargs),*) };
    }
    for prop in elem.props.iter() {
        let pname_str: String = prop.name.to_string();
        if pname_str == PROP_NODE_REF {
            if node_ref_expr.is_some() {
                let msg: String = format!("{ERR_DUPLICATE_PROP_PREFIX} `{pname_str}`");
                return Error::new(prop.name.span(), msg).to_compile_error();
            }
            node_ref_expr = Some(&prop.value);
            continue;
        }
        let repeatable_methods: [&str; 5] = [
            METHOD_CHILD,
            METHOD_SECTION,
            METHOD_FIELD,
            METHOD_ACTION,
            METHOD_CARD,
        ];
        let repeatable: bool = repeatable_methods.contains(&pname_str.as_str());
        if !repeatable && !seen_props.insert(pname_str.clone()) {
            let msg: String = format!("{ERR_DUPLICATE_PROP_PREFIX} `{pname_str}`");
            return Error::new(prop.name.span(), msg).to_compile_error();
        }
        let pname: &Ident = &prop.name;
        let val: &Expr = &prop.value;
        builder_expr = quote_spanned! {pname.span()=> (#builder_expr).#pname(#val) };
    }
    let child_method_name: &str = match last_name.as_str() {
        COMP_EUV_PAGE => METHOD_SECTION,
        COMP_EUV_FORM => METHOD_FIELD,
        COMP_EUV_NAVBAR => METHOD_ACTION,
        COMP_EUV_STAT_GRID => METHOD_CARD,
        _ => METHOD_CHILD,
    };
    let child_method: Ident = Ident::new(child_method_name, span);
    let mount_expr: proc_macro2::TokenStream = if elem.children.is_empty() {
        quote_spanned! {span=> ::euv::Component::mount(#builder_expr) }
    } else if elem.children.iter().all(is_single_node_expr) {
        let mut chained: proc_macro2::TokenStream = builder_expr;
        for child in elem.children.iter() {
            let child_expr: proc_macro2::TokenStream = expand_single_node(child);
            chained = quote! { (#chained).#child_method(#child_expr) };
        }
        quote_spanned! {span=> ::euv::Component::mount(#chained) }
    } else {
        let comp_ident: Ident = Ident::new(IDENT_COMP, span);
        let child_stmts: proc_macro2::TokenStream =
            expand_component_children_into(&comp_ident, &child_method, &elem.children);
        quote_spanned! {span=> {
            let mut #comp_ident = #builder_expr;
            #child_stmts
            ::euv::Component::mount(#comp_ident)
        }}
    };
    match node_ref_expr {
        Some(ref_target) => quote_spanned! {ref_target.span()=> {
            let __euv_mounted: ::euv::NodeId = #mount_expr;
            ::euv::NodeRefTarget::bind_node_ref(&mut (#ref_target), __euv_mounted);
            __euv_mounted
        }},
        None => mount_expr,
    }
}

/// Expands mixed static/control-flow/spread children into `.child(...)` / `.section(...)` mutations on `comp`.
///
/// # Arguments
///
/// - `&Ident` - Mutable component builder local identifier.
/// - `&Ident` - Builder child method (`child`, `section`, `field`, or `action`).
/// - `&[ViewNode]` - Child AST nodes.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Statements chaining each produced child onto `comp`.
fn expand_component_children_into(
    comp: &Ident,
    method: &Ident,
    children: &[ViewNode],
) -> proc_macro2::TokenStream {
    let stmts: Vec<proc_macro2::TokenStream> = children
        .iter()
        .map(|child: &ViewNode| match child {
            ViewNode::Spread(span, expr) => quote_spanned! {*span=>
                for __euv_spread_item in #expr {
                    let __euv_child: ::euv::NodeId = ::euv::Component::mount(__euv_spread_item);
                    #comp = #comp.#method(__euv_child);
                }
            },
            ViewNode::If(if_node) => expand_component_if_into(comp, method, if_node),
            ViewNode::For(for_node) => {
                let pat: &Pat = &for_node.pat;
                let iter: &Expr = &for_node.iter;
                let body: proc_macro2::TokenStream =
                    expand_component_children_into(comp, method, &for_node.body);
                quote_spanned! {for_node.span=>
                    for #pat in #iter {
                        #body
                    }
                }
            }
            ViewNode::Match(match_node) => {
                let expr: &Expr = &match_node.expr;
                let arms: Vec<proc_macro2::TokenStream> = match_node
                    .arms
                    .iter()
                    .map(|arm: &ViewMatchArm| {
                        let pat: &Pat = &arm.pat;
                        let guard: proc_macro2::TokenStream = match &arm.guard {
                            Some(g) => quote! { if #g },
                            None => quote! {},
                        };
                        let body: proc_macro2::TokenStream =
                            expand_component_children_into(comp, method, &arm.body);
                        quote! { #pat #guard => { #body } }
                    })
                    .collect();
                quote_spanned! {match_node.span=>
                    match #expr {
                        #(#arms)*
                    }
                }
            }
            other => {
                let child_expr: proc_macro2::TokenStream = expand_single_node(other);
                quote! {
                    let __euv_child: ::euv::NodeId = #child_expr;
                    #comp = #comp.#method(__euv_child);
                }
            }
        })
        .collect();
    quote! { #(#stmts)* }
}

/// Expands a `ViewIf` node targeting a component builder method (`child`/`section`/`field`/`action`).
///
/// # Arguments
///
/// - `&Ident` - Mutable component builder identifier.
/// - `&Ident` - Builder child method identifier.
/// - `&ViewIf` - Conditional AST node.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Conditional builder update statements.
fn expand_component_if_into(
    comp: &Ident,
    method: &Ident,
    if_node: &ViewIf,
) -> proc_macro2::TokenStream {
    let cond_tokens: proc_macro2::TokenStream = match &if_node.cond {
        ViewIfCond::Expr(expr) => quote! { #expr },
        ViewIfCond::Let(pat, expr) => quote! { let #pat = #expr },
    };
    let then_body: proc_macro2::TokenStream =
        expand_component_children_into(comp, method, &if_node.then_nodes);
    let else_tokens: proc_macro2::TokenStream = match &if_node.else_branch {
        Some(ViewElseBranch::ElseIf(nested)) => {
            let nested_if: proc_macro2::TokenStream =
                expand_component_if_into(comp, method, nested);
            quote! { else #nested_if }
        }
        Some(ViewElseBranch::ElseBlock(nodes)) => {
            let else_body: proc_macro2::TokenStream =
                expand_component_children_into(comp, method, nodes);
            quote! { else { #else_body } }
        }
        None => quote! {},
    };
    quote_spanned! {if_node.span=>
        if #cond_tokens {
            #then_body
        } #else_tokens
    }
}

/// Expands a built-in primitive `ViewElement` with full compile-time safety checks and compact chained `Style`.
///
/// # Arguments
///
/// - `&ViewElement` - Primitive element AST.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Expanded block returning `::euv::NodeId`.
fn expand_primitive_element(elem: &ViewElement) -> proc_macro2::TokenStream {
    let tag_ident: &Ident = match elem.path.get_ident() {
        Some(id) => id,
        None => return quote! { __euv_ctx.fragment() },
    };
    let tag_name: String = tag_ident.to_string();
    let tag_span: Span = tag_ident.span();
    let leaf_tags: [&str; 3] = [TAG_INPUT, TAG_IMAGE, TAG_CANVAS];
    let is_leaf: bool = leaf_tags.contains(&tag_name.as_str());
    if is_leaf && !elem.children.is_empty() {
        return Error::new(tag_span, ERR_LEAF_CHILDREN_FORBIDDEN).to_compile_error();
    }
    let mut seen_props: HashSet<String> = HashSet::new();
    let mut base_style: proc_macro2::TokenStream = quote! { ::euv::Style::default() };
    let mut style_chain: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut style_stmts: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut has_custom_style: bool = false;
    let mut bind_style: Option<&Expr> = None;
    let mut bind_children: Option<&Expr> = None;
    let mut node_ref: Option<&Expr> = None;
    let mut focusable: Option<&Expr> = None;
    let mut on_click: Option<&Expr> = None;
    let mut on_change: Option<&Expr> = None;
    let mut on_scroll: Option<&Expr> = None;
    let mut on_pointer_down: Option<&Expr> = None;
    let mut on_pointer_move: Option<&Expr> = None;
    let mut on_pointer_up: Option<&Expr> = None;
    let mut on_pointer_enter: Option<&Expr> = None;
    let mut on_pointer_leave: Option<&Expr> = None;
    let mut on_hover: Option<&Expr> = None;
    let mut on_key: Option<&Expr> = None;
    let mut on_focus: Option<&Expr> = None;
    let mut on_blur: Option<&Expr> = None;
    let mut a11y_full: Option<&Expr> = None;
    let mut a11y_role: Option<&Expr> = None;
    let mut a11y_label: Option<&Expr> = None;
    let mut a11y_hint: Option<&Expr> = None;
    let mut a11y_value: Option<&Expr> = None;
    let mut a11y_disabled: Option<&Expr> = None;
    let mut a11y_checked: Option<&Expr> = None;
    let mut input_value: Option<&Expr> = None;
    let mut input_placeholder: Option<&Expr> = None;
    let mut input_password: Option<&Expr> = None;
    let mut input_multiline: Option<&Expr> = None;
    let mut input_disabled: Option<&Expr> = None;
    let mut input_caret: Option<&Expr> = None;
    let mut image_source: Option<&Expr> = None;
    let mut canvas_painter: Option<&Expr> = None;
    let mut slot_id: Option<&Expr> = None;
    let mut slot_route: Option<&Expr> = None;
    let mut slot_hub: Option<&Expr> = None;
    let mut slot_fallback: Option<&Expr> = None;
    for prop in elem.props.iter() {
        let pname: String = prop.name.to_string();
        let val: &Expr = &prop.value;
        let pspan: Span = prop.name.span();
        if !seen_props.insert(pname.clone()) {
            let msg: String =
                format!("{ERR_DUPLICATE_PROP_PREFIX} `{pname}` in `view!` element `{tag_name}`");
            return Error::new(pspan, msg).to_compile_error();
        }
        match pname.as_str() {
            PROP_STYLE => {
                base_style = quote_spanned! {val.span()=> #val };
                has_custom_style = true;
            }
            PROP_BIND_STYLE => bind_style = Some(val),
            PROP_BIND_CHILDREN => bind_children = Some(val),
            PROP_NODE_REF => node_ref = Some(val),
            PROP_FOCUSABLE => focusable = Some(val),
            PROP_DISPLAY => style_chain.push(quote_spanned! {pspan=> .display(#val) }),
            PROP_POSITION => style_chain.push(quote_spanned! {pspan=> .position(#val) }),
            PROP_INSET => {
                style_stmts.push(quote_spanned! {val.span()=>
                    let (__euv_it, __euv_ir, __euv_ib, __euv_il): (
                        ::core::option::Option<f32>,
                        ::core::option::Option<f32>,
                        ::core::option::Option<f32>,
                        ::core::option::Option<f32>,
                    ) = #val;
                    __euv_style = __euv_style.inset(__euv_it, __euv_ir, __euv_ib, __euv_il);
                });
            }
            PROP_WIDTH => style_chain.push(quote_spanned! {pspan=> .width(#val) }),
            PROP_HEIGHT => style_chain.push(quote_spanned! {pspan=> .height(#val) }),
            PROP_MIN_WIDTH => style_chain.push(quote_spanned! {pspan=> .min_width(#val) }),
            PROP_MIN_HEIGHT => style_chain.push(quote_spanned! {pspan=> .min_height(#val) }),
            PROP_MAX_WIDTH => style_chain.push(quote_spanned! {pspan=> .max_width(#val) }),
            PROP_MAX_HEIGHT => style_chain.push(quote_spanned! {pspan=> .max_height(#val) }),
            PROP_ASPECT_RATIO => style_chain.push(quote_spanned! {pspan=> .aspect_ratio(#val) }),
            PROP_PADDING => style_chain.push(quote_spanned! {pspan=> .padding(#val) }),
            PROP_MARGIN => style_chain.push(quote_spanned! {pspan=> .margin(#val) }),
            PROP_GAP => style_chain.push(quote_spanned! {pspan=> .gap(#val) }),
            PROP_BACKGROUND => style_chain.push(quote_spanned! {pspan=> .background(#val) }),
            PROP_GRADIENT => style_chain.push(quote_spanned! {pspan=> .gradient(#val) }),
            PROP_SHADOW => style_chain.push(quote_spanned! {pspan=> .shadow(#val) }),
            PROP_FOCUS_RING => style_chain.push(quote_spanned! {pspan=> .focus_ring(#val) }),
            PROP_CURSOR => style_chain.push(quote_spanned! {pspan=> .cursor(#val) }),
            PROP_POINTER_EVENTS => {
                style_chain.push(quote_spanned! {pspan=> .pointer_events(#val) });
            }
            PROP_STOP_SCROLL_PROPAGATION => {
                style_chain.push(quote_spanned! {pspan=> .stop_scroll_propagation(#val) });
            }
            PROP_STOP_EVENT_PROPAGATION => {
                style_chain.push(quote_spanned! {pspan=> .stop_event_propagation(#val) });
            }
            PROP_COLOR => style_chain.push(quote_spanned! {pspan=> .color(#val) }),
            PROP_RADIUS => style_chain.push(quote_spanned! {pspan=> .radius(#val) }),
            PROP_BORDER => {
                style_stmts.push(quote_spanned! {val.span()=>
                    let (__euv_bw, __euv_bc): (f32, ::euv::Color) = #val;
                    __euv_style = __euv_style.border(__euv_bw, __euv_bc);
                });
            }
            PROP_ALIGN => style_chain.push(quote_spanned! {pspan=> .align(#val) }),
            PROP_JUSTIFY => style_chain.push(quote_spanned! {pspan=> .justify(#val) }),
            PROP_FLEX_GROW => style_chain.push(quote_spanned! {pspan=> .flex_grow(#val) }),
            PROP_FLEX_SHRINK => style_chain.push(quote_spanned! {pspan=> .flex_shrink(#val) }),
            PROP_FLEX_WRAP => style_chain.push(quote_spanned! {pspan=> .wrap(#val) }),
            PROP_OVERFLOW => style_chain.push(quote_spanned! {pspan=> .overflow(#val) }),
            PROP_FONT_SIZE => style_chain.push(quote_spanned! {pspan=> .font_size(#val) }),
            PROP_FONT_WEIGHT => style_chain.push(quote_spanned! {pspan=> .font_weight(#val) }),
            PROP_FONT_FAMILY => style_chain.push(quote_spanned! {pspan=> .font_family(#val) }),
            PROP_TEXT_ALIGN => style_chain.push(quote_spanned! {pspan=> .text_align(#val) }),
            PROP_TEXT_OVERFLOW => style_chain.push(quote_spanned! {pspan=> .text_overflow(#val) }),
            PROP_Z_INDEX => style_chain.push(quote_spanned! {pspan=> .z_index(#val) }),
            PROP_OPACITY => style_chain.push(quote_spanned! {pspan=> .opacity(#val) }),
            PROP_COLUMNS => style_chain.push(quote_spanned! {pspan=> .grid(#val) }),
            PROP_ON_CLICK => on_click = Some(val),
            PROP_ON_CHANGE => on_change = Some(val),
            PROP_ON_SCROLL => on_scroll = Some(val),
            PROP_ON_POINTER_DOWN => on_pointer_down = Some(val),
            PROP_ON_POINTER_MOVE => on_pointer_move = Some(val),
            PROP_ON_POINTER_UP => on_pointer_up = Some(val),
            PROP_ON_POINTER_ENTER => on_pointer_enter = Some(val),
            PROP_ON_POINTER_LEAVE => on_pointer_leave = Some(val),
            PROP_ON_HOVER => on_hover = Some(val),
            PROP_ON_KEY => on_key = Some(val),
            PROP_ON_FOCUS => on_focus = Some(val),
            PROP_ON_BLUR => on_blur = Some(val),
            PROP_VALUE => {
                if tag_name != TAG_INPUT {
                    return Error::new(pspan, ERR_INPUT_PROP_ONLY_ON_INPUT).to_compile_error();
                }
                input_value = Some(val);
            }
            PROP_PLACEHOLDER => {
                if tag_name != TAG_INPUT {
                    return Error::new(pspan, ERR_INPUT_PROP_ONLY_ON_INPUT).to_compile_error();
                }
                input_placeholder = Some(val);
            }
            PROP_PASSWORD => {
                if tag_name != TAG_INPUT {
                    return Error::new(pspan, ERR_INPUT_PROP_ONLY_ON_INPUT).to_compile_error();
                }
                input_password = Some(val);
            }
            PROP_MULTILINE => {
                if tag_name != TAG_INPUT {
                    return Error::new(pspan, ERR_INPUT_PROP_ONLY_ON_INPUT).to_compile_error();
                }
                input_multiline = Some(val);
            }
            PROP_DISABLED => {
                if tag_name != TAG_INPUT {
                    return Error::new(pspan, ERR_INPUT_PROP_ONLY_ON_INPUT).to_compile_error();
                }
                input_disabled = Some(val);
            }
            PROP_CARET => {
                if tag_name != TAG_INPUT {
                    return Error::new(pspan, ERR_INPUT_PROP_ONLY_ON_INPUT).to_compile_error();
                }
                input_caret = Some(val);
            }
            PROP_SOURCE => {
                if tag_name != TAG_IMAGE {
                    return Error::new(pspan, ERR_SOURCE_ONLY_ON_IMAGE).to_compile_error();
                }
                image_source = Some(val);
            }
            PROP_PAINTER => {
                if tag_name != TAG_CANVAS {
                    return Error::new(pspan, ERR_PAINTER_ONLY_ON_CANVAS).to_compile_error();
                }
                canvas_painter = Some(val);
            }
            PROP_A11Y => a11y_full = Some(val),
            PROP_ROLE => a11y_role = Some(val),
            PROP_A11Y_LABEL => a11y_label = Some(val),
            PROP_A11Y_HINT => a11y_hint = Some(val),
            PROP_A11Y_VALUE => a11y_value = Some(val),
            PROP_A11Y_DISABLED => a11y_disabled = Some(val),
            PROP_A11Y_CHECKED => a11y_checked = Some(val),
            PROP_ID => {
                if tag_name != TAG_SLOT {
                    return Error::new(pspan, ERR_ID_ONLY_ON_SLOT).to_compile_error();
                }
                slot_id = Some(val);
            }
            PROP_ROUTE => {
                if tag_name != TAG_PAGE_SLOT {
                    return Error::new(pspan, ERR_ROUTE_ONLY_ON_PAGE_SLOT).to_compile_error();
                }
                slot_route = Some(val);
            }
            PROP_HUB => {
                if tag_name != TAG_SLOT && tag_name != TAG_PAGE_SLOT {
                    return Error::new(pspan, ERR_HUB_ONLY_ON_DYNAMIC_SLOT).to_compile_error();
                }
                slot_hub = Some(val);
            }
            PROP_FALLBACK => {
                if tag_name != TAG_SLOT && tag_name != TAG_PAGE_SLOT {
                    return Error::new(pspan, ERR_HUB_ONLY_ON_DYNAMIC_SLOT).to_compile_error();
                }
                slot_fallback = Some(val);
            }
            _ => {
                let scope: String = format!("view! element `{tag_name}`");
                return unknown_prop_error(&prop.name, &scope, KNOWN_VIEW_PROPS).to_compile_error();
            }
        }
    }
    if bind_children.is_some() && !elem.children.is_empty() {
        return Error::new(tag_span, ERR_BIND_CHILDREN_CONFLICT).to_compile_error();
    }
    let has_individual_a11y: bool = a11y_role.is_some()
        || a11y_label.is_some()
        || a11y_hint.is_some()
        || a11y_value.is_some()
        || a11y_disabled.is_some()
        || a11y_checked.is_some();
    if a11y_full.is_some() && has_individual_a11y {
        return Error::new(tag_span, ERR_A11Y_CONFLICT).to_compile_error();
    }
    let dir_chain: proc_macro2::TokenStream = match tag_name.as_str() {
        TAG_ROW => quote_spanned! {tag_span=> .row() },
        TAG_COLUMN | TAG_COL => quote_spanned! {tag_span=> .column() },
        TAG_SCROLL if !has_custom_style => quote_spanned! {tag_span=> .column() },
        TAG_GRID => quote_spanned! {tag_span=> .grid(2) },
        _ => quote! {},
    };
    let has_style_expr: bool = has_custom_style
        || !dir_chain.is_empty()
        || !style_chain.is_empty()
        || !style_stmts.is_empty();
    let style_expr: proc_macro2::TokenStream = if !has_style_expr {
        quote! { ::euv::Style::default() }
    } else if style_stmts.is_empty() {
        quote! { (#base_style) #dir_chain #(#style_chain)* }
    } else {
        quote! {{
            let mut __euv_style: ::euv::Style = (#base_style) #dir_chain #(#style_chain)*;
            #(#style_stmts)*
            __euv_style
        }}
    };
    let node_creation: proc_macro2::TokenStream = match tag_name.as_str() {
        TAG_VIEW | TAG_ROW | TAG_COLUMN | TAG_COL | TAG_GRID => {
            quote_spanned! {tag_span=> __euv_ctx.view(#style_expr) }
        }
        TAG_SCROLL => quote_spanned! {tag_span=> __euv_ctx.scroll_view(#style_expr) },
        TAG_PORTAL => quote_spanned! {tag_span=> __euv_ctx.portal(#style_expr) },
        TAG_FRAGMENT => quote_spanned! {tag_span=> __euv_ctx.fragment() },
        TAG_TEXT => {
            if elem.children.len() > 1 {
                return Error::new(tag_span, ERR_TEXT_CHILD_INVALID).to_compile_error();
            }
            if let Some(first) = elem.children.first() {
                match first {
                    ViewNode::StaticText(lit) => {
                        quote_spanned! {lit.span()=> __euv_ctx.text(::euv::into_text(#lit), #style_expr) }
                    }
                    ViewNode::Expr(expr) => match expr.as_ref() {
                        Expr::Closure(closure) => {
                            quote_spanned! {closure.span()=> __euv_ctx.reactive_text(#style_expr, #closure) }
                        }
                        other => {
                            quote_spanned! {other.span()=> __euv_ctx.text(::euv::into_text(#other), #style_expr) }
                        }
                    },
                    _ => return Error::new(tag_span, ERR_TEXT_CHILD_INVALID).to_compile_error(),
                }
            } else {
                quote_spanned! {tag_span=> __euv_ctx.text(::std::string::String::new(), #style_expr) }
            }
        }
        TAG_INPUT => {
            let val_expr: proc_macro2::TokenStream = match input_value {
                Some(v) => quote! { ::euv::into_text(#v) },
                None => quote! { ::std::string::String::new() },
            };
            let ph_expr: proc_macro2::TokenStream = match input_placeholder {
                Some(p) => quote! { ::euv::into_text(#p) },
                None => quote! { ::std::string::String::new() },
            };
            let caret_expr: proc_macro2::TokenStream = match input_caret {
                Some(c) => quote! { #c },
                None => quote! { 0usize },
            };
            let pw_expr: proc_macro2::TokenStream = match input_password {
                Some(pw) => quote! { #pw },
                None => quote! { false },
            };
            let ml_expr: proc_macro2::TokenStream = match input_multiline {
                Some(ml) => quote! { #ml },
                None => quote! { false },
            };
            let dis_expr: proc_macro2::TokenStream = match input_disabled {
                Some(d) => quote! { #d },
                None => quote! { false },
            };
            quote_spanned! {tag_span=>
                __euv_ctx.input(
                    ::euv::InputNodeState::new(#val_expr, #ph_expr, #caret_expr, #pw_expr, #ml_expr, #dis_expr),
                    #style_expr,
                )
            }
        }
        TAG_IMAGE => {
            let Some(src) = image_source else {
                return Error::new(tag_span, ERR_IMAGE_MISSING_SOURCE).to_compile_error();
            };
            quote_spanned! {src.span()=> __euv_ctx.image(#src, #style_expr) }
        }
        TAG_CANVAS => {
            let Some(painter) = canvas_painter else {
                return Error::new(tag_span, ERR_CANVAS_MISSING_PAINTER).to_compile_error();
            };
            quote_spanned! {painter.span()=> __euv_ctx.canvas(#style_expr, #painter) }
        }
        TAG_SLOT => {
            let Some(id_expr) = slot_id else {
                return Error::new(tag_span, ERR_SLOT_MISSING_ID).to_compile_error();
            };
            let fb_expr: proc_macro2::TokenStream =
                build_slot_fallback(slot_fallback, &elem.children);
            let hub_stmt: proc_macro2::TokenStream = match slot_hub {
                Some(h) => {
                    quote_spanned! {h.span()=> __euv_slot.set_hub(::core::option::Option::Some(#h)); }
                }
                None => quote! {},
            };
            quote_spanned! {tag_span=> {
                let mut __euv_slot: ::euv::DynamicComponentSlot = ::euv::DynamicComponentSlot::new(
                    ::euv::into_text(#id_expr),
                    #fb_expr,
                );
                __euv_slot.set_style(#style_expr);
                #hub_stmt
                ::euv::Component::mount(__euv_slot)
            }}
        }
        TAG_PAGE_SLOT => {
            let Some(route_expr) = slot_route else {
                return Error::new(tag_span, ERR_PAGE_SLOT_MISSING_ROUTE).to_compile_error();
            };
            let fb_expr: proc_macro2::TokenStream =
                build_slot_fallback(slot_fallback, &elem.children);
            let hub_stmt: proc_macro2::TokenStream = match slot_hub {
                Some(h) => {
                    quote_spanned! {h.span()=> __euv_slot.set_hub(::core::option::Option::Some(#h)); }
                }
                None => quote! {},
            };
            quote_spanned! {tag_span=> {
                let mut __euv_slot: ::euv::DynamicPageSlot = ::euv::DynamicPageSlot::new(
                    ::euv::into_text(#route_expr),
                    #fb_expr,
                );
                __euv_slot.set_style(#style_expr);
                #hub_stmt
                ::euv::Component::mount(__euv_slot)
            }}
        }
        _ => quote! { __euv_ctx.fragment() },
    };
    let node_ident: Ident = Ident::new(IDENT_NODE, tag_span);
    let mut post_stmts: Vec<proc_macro2::TokenStream> = Vec::new();
    if let Some(ref_target) = node_ref {
        post_stmts.push(quote_spanned! {ref_target.span()=>
            ::euv::NodeRefTarget::bind_node_ref(&mut (#ref_target), #node_ident);
        });
    }
    if let Some(cb) = bind_style {
        post_stmts.push(quote_spanned! {cb.span()=> __euv_ctx.bind_style(#node_ident, #cb); });
    }
    if let Some(cb) = bind_children {
        post_stmts.push(quote_spanned! {cb.span()=> __euv_ctx.bind_children(#node_ident, #cb); });
    }
    if let Some(f) = focusable {
        post_stmts.push(quote_spanned! {f.span()=> __euv_ctx.set_focusable(#node_ident, #f); });
    }
    if let Some(cb) = on_click {
        post_stmts.push(quote_spanned! {cb.span()=> __euv_ctx.on_click(#node_ident, #cb); });
    }
    if let Some(cb) = on_change {
        post_stmts.push(quote_spanned! {cb.span()=> __euv_ctx.on_change(#node_ident, #cb); });
    }
    if let Some(cb) = on_scroll {
        post_stmts.push(quote_spanned! {cb.span()=> __euv_ctx.on_scroll(#node_ident, #cb); });
    }
    if let Some(cb) = on_pointer_down {
        post_stmts.push(quote_spanned! {cb.span()=> __euv_ctx.on_pointer_down(#node_ident, #cb); });
    }
    if let Some(cb) = on_pointer_move {
        post_stmts.push(quote_spanned! {cb.span()=> __euv_ctx.on_pointer_move(#node_ident, #cb); });
    }
    if let Some(cb) = on_pointer_up {
        post_stmts.push(quote_spanned! {cb.span()=> __euv_ctx.on_pointer_up(#node_ident, #cb); });
    }
    if let Some(cb) = on_pointer_enter {
        post_stmts
            .push(quote_spanned! {cb.span()=> __euv_ctx.on_pointer_enter(#node_ident, #cb); });
    }
    if let Some(cb) = on_pointer_leave {
        post_stmts
            .push(quote_spanned! {cb.span()=> __euv_ctx.on_pointer_leave(#node_ident, #cb); });
    }
    if let Some(cb) = on_hover {
        post_stmts.push(quote_spanned! {cb.span()=> __euv_ctx.on_hover(#node_ident, #cb); });
    }
    if let Some(cb) = on_key {
        post_stmts.push(quote_spanned! {cb.span()=> __euv_ctx.on_key(#node_ident, #cb); });
    }
    if let Some(cb) = on_focus {
        post_stmts.push(quote_spanned! {cb.span()=> __euv_ctx.on_focus(#node_ident, #cb); });
    }
    if let Some(cb) = on_blur {
        post_stmts.push(quote_spanned! {cb.span()=> __euv_ctx.on_blur(#node_ident, #cb); });
    }
    if let Some(props_expr) = a11y_full {
        post_stmts.push(
            quote_spanned! {props_expr.span()=> __euv_ctx.set_a11y(#node_ident, #props_expr); },
        );
    } else if has_individual_a11y {
        let role_expr: proc_macro2::TokenStream = match a11y_role {
            Some(r) => quote_spanned! {r.span()=> #r },
            None => quote! { ::euv::A11yRole::Group },
        };
        let label_expr: proc_macro2::TokenStream = match a11y_label {
            Some(l) => quote! { ::euv::into_text(#l) },
            None => quote! { ::std::string::String::new() },
        };
        let hint_stmt: proc_macro2::TokenStream = match a11y_hint {
            Some(h) => quote! { __euv_a11y.set_hint(::euv::into_text(#h)); },
            None => quote! {},
        };
        let val_stmt: proc_macro2::TokenStream = match a11y_value {
            Some(v) => quote! { __euv_a11y.set_value(::euv::into_text(#v)); },
            None => quote! {},
        };
        let dis_stmt: proc_macro2::TokenStream = match a11y_disabled {
            Some(d) => quote! { __euv_a11y.set_disabled(#d); },
            None => quote! {},
        };
        let chk_stmt: proc_macro2::TokenStream = match a11y_checked {
            Some(c) => quote! { __euv_a11y.set_checked(#c); },
            None => quote! {},
        };
        post_stmts.push(quote! {
            let mut __euv_a11y: ::euv::A11yProps = ::euv::A11yProps::new(#role_expr, #label_expr);
            #hint_stmt
            #val_stmt
            #dis_stmt
            #chk_stmt
            __euv_ctx.set_a11y(#node_ident, __euv_a11y);
        });
    }
    let has_children: bool = tag_name != TAG_TEXT
        && tag_name != TAG_SLOT
        && tag_name != TAG_PAGE_SLOT
        && !elem.children.is_empty();
    if post_stmts.is_empty() && !has_children {
        return node_creation;
    }
    let children_stmts: proc_macro2::TokenStream = if has_children {
        expand_children_into(&node_ident, &elem.children)
    } else {
        quote! {}
    };
    quote! {{
        let #node_ident: ::euv::NodeId = #node_creation;
        #(#post_stmts)*
        #children_stmts
        #node_ident
    }}
}

/// Builds the `::euv::DynamicBuilderFn` fallback closure for `slot` and `page_slot` elements.
///
/// # Arguments
///
/// - `Option<&Expr>` - Optional explicit `fallback:` closure expression.
/// - `&[ViewNode]` - Inline fallback children declared inside the slot block.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Expanded `Rc::new(...)` builder closure.
fn build_slot_fallback(
    explicit_fallback: Option<&Expr>,
    children: &[ViewNode],
) -> proc_macro2::TokenStream {
    if let Some(fb) = explicit_fallback {
        return quote_spanned! {fb.span()=> ::std::rc::Rc::new(#fb) };
    }
    if children.is_empty() {
        return quote! {
            ::std::rc::Rc::new(move || -> ::euv::NodeId {
                let mut __euv_ctx_owned: ::euv::Context = ::euv::current_context();
                __euv_ctx_owned.view(::euv::Style::default())
            })
        };
    }
    if children.len() == 1 && is_single_node_expr(&children[0]) {
        let single: proc_macro2::TokenStream = expand_single_node(&children[0]);
        return quote! {
            ::std::rc::Rc::new(move || -> ::euv::NodeId {
                let mut __euv_ctx_owned: ::euv::Context = ::euv::current_context();
                let __euv_ctx: &mut ::euv::Context = &mut __euv_ctx_owned;
                #single
            })
        };
    }
    let fb_root: Ident = Ident::new(IDENT_FB_ROOT, Span::call_site());
    let attach: proc_macro2::TokenStream = expand_children_into(&fb_root, children);
    quote! {
        ::std::rc::Rc::new(move || -> ::euv::NodeId {
            let mut __euv_ctx_owned: ::euv::Context = ::euv::current_context();
            let __euv_ctx: &mut ::euv::Context = &mut __euv_ctx_owned;
            let #fb_root: ::euv::NodeId =
                __euv_ctx.view(::euv::Style::default().column().width(::euv::Length::Percent(100.0)));
            #attach
            #fb_root
        })
    }
}
