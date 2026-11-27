use super::*;

/// Parser for `ViewMacroInput`: implicit framework-managed `view! { ... }`.
impl Parse for ViewMacroInput {
    /// Parses one or more top-level view nodes, rejecting legacy explicit `ctx` / `in ctx,` arguments.
    ///
    /// # Arguments
    ///
    /// - `ParseStream<'_>` - Token input stream.
    ///
    /// # Returns
    ///
    /// - `Result<Self>` - Parsed macro invocation.
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        if input.peek(Token![in]) {
            let in_tok: Token![in] = input.parse()?;
            return Err(Error::new(in_tok.span(), ERR_LEGACY_VIEW_CTX));
        }
        if input.peek(Ident) && input.peek2(Token![,]) {
            let legacy_ctx: Ident = input.parse()?;
            return Err(Error::new(legacy_ctx.span(), ERR_LEGACY_VIEW_CTX));
        }
        let roots: Vec<ViewNode> = if input.peek(Brace) {
            let content: ParseBuffer<'_>;
            braced!(content in input);
            parse_node_list(&content)?
        } else {
            parse_node_list(input)?
        };
        Ok(Self { roots })
    }
}

/// Parser for individual `ViewNode` items.
impl Parse for ViewNode {
    /// Parses a string literal, braced dynamic expression, spread, control-flow block, or element/component.
    ///
    /// # Arguments
    ///
    /// - `ParseStream<'_>` - Token input stream.
    ///
    /// # Returns
    ///
    /// - `Result<Self>` - Parsed `ViewNode`.
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        if input.peek(LitStr) {
            let lit: LitStr = input.parse()?;
            return Ok(Self::StaticText(lit));
        }
        if input.peek(Token![..]) {
            let dots: Token![..] = input.parse()?;
            let expr: Expr = Expr::parse_without_eager_brace(input)?;
            return Ok(Self::Spread(dots.span(), Box::new(expr)));
        }
        if input.peek(Token![if]) {
            let if_node: ViewIf = input.parse()?;
            return Ok(Self::If(Box::new(if_node)));
        }
        if input.peek(Token![for]) {
            let for_node: ViewFor = input.parse()?;
            return Ok(Self::For(Box::new(for_node)));
        }
        if input.peek(Token![match]) {
            let match_node: ViewMatch = input.parse()?;
            return Ok(Self::Match(Box::new(match_node)));
        }
        if input.peek(Brace) {
            let content: ParseBuffer<'_>;
            braced!(content in input);
            let expr: Expr = content.parse()?;
            return Ok(Self::Expr(Box::new(expr)));
        }
        let elem: ViewElement = input.parse()?;
        Ok(Self::Element(Box::new(elem)))
    }
}

/// Parser for `ViewIf`: `if cond { ... } [else if ... | else { ... }]`.
impl Parse for ViewIf {
    /// Parses a conditional `if` or `if let` view branch and optional `else` continuation.
    ///
    /// # Arguments
    ///
    /// - `ParseStream<'_>` - Token input stream.
    ///
    /// # Returns
    ///
    /// - `Result<Self>` - Parsed `ViewIf`.
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let if_tok: Token![if] = input.parse()?;
        let cond: ViewIfCond = if input.peek(Token![let]) {
            let _: Token![let] = input.parse()?;
            let pat: Pat = Pat::parse_multi_with_leading_vert(input)?;
            let _: Token![=] = input.parse()?;
            let expr: Expr = Expr::parse_without_eager_brace(input)?;
            ViewIfCond::Let(Box::new(pat), Box::new(expr))
        } else {
            let expr: Expr = Expr::parse_without_eager_brace(input)?;
            ViewIfCond::Expr(Box::new(expr))
        };
        let then_buf: ParseBuffer<'_>;
        braced!(then_buf in input);
        let then_nodes: Vec<ViewNode> = parse_node_list(&then_buf)?;
        let else_branch: Option<ViewElseBranch> = if input.peek(Token![else]) {
            let _: Token![else] = input.parse()?;
            if input.peek(Token![if]) {
                let nested: ViewIf = input.parse()?;
                Some(ViewElseBranch::ElseIf(Box::new(nested)))
            } else {
                let else_buf: ParseBuffer<'_>;
                braced!(else_buf in input);
                let else_nodes: Vec<ViewNode> = parse_node_list(&else_buf)?;
                Some(ViewElseBranch::ElseBlock(else_nodes))
            }
        } else {
            None
        };
        Ok(Self {
            span: if_tok.span(),
            cond,
            then_nodes,
            else_branch,
        })
    }
}

/// Parser for `ViewFor`: `for pat in iter { nodes... }`.
impl Parse for ViewFor {
    /// Parses a `for` loop producing sibling view nodes.
    ///
    /// # Arguments
    ///
    /// - `ParseStream<'_>` - Token input stream.
    ///
    /// # Returns
    ///
    /// - `Result<Self>` - Parsed `ViewFor`.
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let for_tok: Token![for] = input.parse()?;
        let pat: Pat = Pat::parse_multi_with_leading_vert(input)?;
        let _: Token![in] = input.parse()?;
        let iter: Expr = Expr::parse_without_eager_brace(input)?;
        let body_buf: ParseBuffer<'_>;
        braced!(body_buf in input);
        let body: Vec<ViewNode> = parse_node_list(&body_buf)?;
        Ok(Self {
            span: for_tok.span(),
            pat,
            iter,
            body,
        })
    }
}

/// Parser for `ViewMatch`: `match expr { pat [if guard] => { nodes... } | node, ... }`.
impl Parse for ViewMatch {
    /// Parses a `match` expression branching across sibling view nodes.
    ///
    /// # Arguments
    ///
    /// - `ParseStream<'_>` - Token input stream.
    ///
    /// # Returns
    ///
    /// - `Result<Self>` - Parsed `ViewMatch`.
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let match_tok: Token![match] = input.parse()?;
        let expr: Expr = Expr::parse_without_eager_brace(input)?;
        let arms_buf: ParseBuffer<'_>;
        braced!(arms_buf in input);
        let mut arms: Vec<ViewMatchArm> = Vec::new();
        while !arms_buf.is_empty() {
            let pat: Pat = Pat::parse_multi_with_leading_vert(&arms_buf)?;
            let guard: Option<Expr> = if arms_buf.peek(Token![if]) {
                let _: Token![if] = arms_buf.parse()?;
                Some(arms_buf.parse()?)
            } else {
                None
            };
            let _: Token![=>] = arms_buf.parse()?;
            let body: Vec<ViewNode> = if arms_buf.peek(Brace) {
                let arm_body_buf: ParseBuffer<'_>;
                braced!(arm_body_buf in arms_buf);
                parse_node_list(&arm_body_buf)?
            } else {
                let single: ViewNode = arms_buf.parse()?;
                vec![single]
            };
            let _: Option<Token![,]> = arms_buf.parse()?;
            arms.push(ViewMatchArm { pat, guard, body });
        }
        Ok(Self {
            span: match_tok.span(),
            expr,
            arms,
        })
    }
}

/// Parser for `ViewElement`: `Path[(args...)] [{ flags/props/children }]`.
impl Parse for ViewElement {
    /// Parses a primitive element or typed component invocation with arguments, fluent flags, properties, and nested children.
    ///
    /// # Arguments
    ///
    /// - `ParseStream<'_>` - Token input stream.
    ///
    /// # Returns
    ///
    /// - `Result<Self>` - Parsed `ViewElement`.
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let path: Path = input.parse()?;
        let (args, args_close_line): (Option<Vec<Expr>>, Option<usize>) = if input.peek(Paren) {
            let arg_buf: ParseBuffer<'_>;
            let paren_tok: Paren = parenthesized!(arg_buf in input);
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
            if is_type_ctor {
                let msg: String = format!(
                    "{ERR_COMPONENT_POSITIONAL_ARGS} — rewrite `{last_name}(...)` as `{last_name} {{ prop: value }}`"
                );
                return Err(Error::new(paren_tok.span.join(), msg));
            }
            let parsed_args: Punctuated<Expr, Token![,]> =
                arg_buf.parse_terminated(Expr::parse, Token![,])?;
            (
                Some(parsed_args.into_iter().collect()),
                Some(paren_tok.span.close().end().line),
            )
        } else {
            (None, None)
        };
        let is_component: bool = is_component_element(&path, args.is_some());
        let mut flags: Vec<ViewFlag> = Vec::new();
        let mut props: Vec<ViewProp> = Vec::new();
        let mut children: Vec<ViewNode> = Vec::new();
        let body_follows: bool = input.peek(Brace)
            && args_close_line
                .map(|line: usize| line == input.span().start().line)
                .unwrap_or(true);
        if body_follows {
            let content: ParseBuffer<'_>;
            braced!(content in input);
            while !content.is_empty() {
                if content.peek(Ident) && content.peek2(Token![:]) && !content.peek2(Token![::]) {
                    let name: Ident = content.parse()?;
                    let _: Token![:] = content.parse()?;
                    let raw_value: Expr = Expr::parse_without_eager_brace(&content)?;
                    let _: Option<Token![,]> = content.parse()?;
                    let _: Option<Token![;]> = content.parse()?;
                    let (value, trailing_spread): (Expr, Option<ViewNode>) =
                        split_trailing_line_spread(raw_value);
                    props.push(ViewProp { name, value });
                    if let Some(spread_node) = trailing_spread {
                        children.push(spread_node);
                    }
                } else if is_component
                    && content.peek(Ident)
                    && content.peek2(Paren)
                    && is_lowercase_flag_call(&content)
                {
                    let name: Ident = content.parse()?;
                    let arg_buf: ParseBuffer<'_>;
                    parenthesized!(arg_buf in content);
                    let parsed_args: Punctuated<Expr, Token![,]> =
                        arg_buf.parse_terminated(Expr::parse, Token![,])?;
                    let _: Option<Token![,]> = content.parse()?;
                    let _: Option<Token![;]> = content.parse()?;
                    flags.push(ViewFlag {
                        name,
                        args: parsed_args.into_iter().collect(),
                    });
                } else {
                    let child: ViewNode = content.parse()?;
                    let _: Option<Token![,]> = content.parse()?;
                    let _: Option<Token![;]> = content.parse()?;
                    children.push(child);
                }
            }
        } else if args.is_none() {
            return Err(Error::new(path.span(), ERR_EXPECTED_BODY_OR_ARGS));
        }
        Ok(Self {
            path,
            args,
            flags,
            props,
            children,
        })
    }
}
