use super::*;

/// Parser for `StyleBlockInput`.
impl Parse for StyleBlockInput {
    /// Parses either named `vis name[(params...)] { stmt\n ... }` definitions or an inline `style! { stmt\n ... }` expression.
    ///
    /// # Arguments
    ///
    /// - `ParseStream<'_>` - Token input stream.
    ///
    /// # Returns
    ///
    /// - `Result<Self>` - Parsed style block with one-rule-per-line syntax enforced (no semicolons).
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        if input.is_empty() {
            return Ok(Self::Inline(Vec::new()));
        }
        if is_named_style_def(input) {
            let mut defs: Vec<StyleDef> = Vec::new();
            while !input.is_empty() {
                let vis: Visibility = input.parse()?;
                let _: Option<Token![const]> = input.parse()?;
                let name: Ident = input.parse()?;
                let params: Vec<FnArg> = if input.peek(Paren) {
                    let param_buf: ParseBuffer<'_>;
                    parenthesized!(param_buf in input);
                    let parsed: Punctuated<FnArg, Token![,]> =
                        param_buf.parse_terminated(FnArg::parse, Token![,])?;
                    parsed.into_iter().collect()
                } else {
                    Vec::new()
                };
                let body: ParseBuffer<'_>;
                braced!(body in input);
                let stmts: Vec<StyleStmt> = parse_style_stmts(&body)?;
                defs.push(StyleDef {
                    vis,
                    name,
                    params,
                    stmts,
                });
            }
            return Ok(Self::Defs(defs));
        }
        let stmts: Vec<StyleStmt> = parse_style_stmts(input)?;
        Ok(Self::Inline(stmts))
    }
}

/// Parser for `TokenBlockInput`.
impl Parse for TokenBlockInput {
    /// Parses `vis NAME: Type = expr;` token entries.
    ///
    /// # Arguments
    ///
    /// - `ParseStream<'_>` - Token input stream.
    ///
    /// # Returns
    ///
    /// - `Result<Self>` - Parsed token definitions.
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut entries: Vec<TokenEntry> = Vec::new();
        while !input.is_empty() {
            let vis: Visibility = input.parse()?;
            let _: Option<Token![const]> = input.parse()?;
            let name: Ident = input.parse()?;
            let _: Token![:] = input.parse()?;
            let ty: Type = input.parse()?;
            let _: Token![=] = input.parse()?;
            let value: Expr = input.parse()?;
            let _: Option<Token![;]> = input.parse()?;
            entries.push(TokenEntry {
                vis,
                name,
                ty,
                value,
            });
        }
        Ok(Self { entries })
    }
}
