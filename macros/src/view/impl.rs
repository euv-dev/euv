use super::*;

/// Parser for `ViewMacroInput`: zero-context `{ node }` or `node`.
impl Parse for ViewMacroInput {
    /// Parses the root view node with ambient context management.
    ///
    /// # Arguments
    ///
    /// - `ParseStream<'_>` - Token input stream.
    ///
    /// # Returns
    ///
    /// - `Result<Self>` - Parsed macro invocation.
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        if input.peek(Ident) && input.peek2(Token![,]) {
            let legacy_ctx: Ident = input.parse()?;
            return Err(Error::new(legacy_ctx.span(), ERR_LEGACY_VIEW_CTX));
        }
        let root: ViewNode = if input.peek(Brace) {
            let content: ParseBuffer<'_>;
            braced!(content in input);
            content.parse()?
        } else {
            input.parse()?
        };
        Ok(Self { root })
    }
}

/// Parser for individual `ViewNode` items.
impl Parse for ViewNode {
    /// Parses a string literal, braced dynamic expression, or tag element.
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
        if input.peek(Brace) {
            let content: ParseBuffer<'_>;
            braced!(content in input);
            let expr: Expr = content.parse()?;
            return Ok(Self::Expr(expr));
        }
        let elem: ViewElement = input.parse()?;
        Ok(Self::Element(elem))
    }
}

/// Parser for `ViewElement`: `tag { props... children... }`.
impl Parse for ViewElement {
    /// Parses an element tag, properties, and nested child nodes.
    ///
    /// # Arguments
    ///
    /// - `ParseStream<'_>` - Token input stream.
    ///
    /// # Returns
    ///
    /// - `Result<Self>` - Parsed `ViewElement`.
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let tag: Ident = input.parse()?;
        let content: ParseBuffer<'_>;
        braced!(content in input);
        let mut props: Vec<ViewProp> = Vec::new();
        let mut children: Vec<ViewNode> = Vec::new();
        while !content.is_empty() {
            if content.peek(Ident) && content.peek2(Token![:]) {
                let name: Ident = content.parse()?;
                let _: Token![:] = content.parse()?;
                let value: Expr = Expr::parse_without_eager_brace(&content)?;
                let _: Option<Token![,]> = content.parse()?;
                props.push(ViewProp { name, value });
            } else {
                let child: ViewNode = content.parse()?;
                let _: Option<Token![,]> = content.parse()?;
                children.push(child);
            }
        }
        Ok(Self {
            tag,
            props,
            children,
        })
    }
}
