use super::*;

/// Parser for `StyleBlockInput`.
impl Parse for StyleBlockInput {
    /// Parses one or more `vis name { key: val; ... }` definitions.
    ///
    /// # Arguments
    ///
    /// - `ParseStream<'_>` - Token input stream.
    ///
    /// # Returns
    ///
    /// - `Result<Self>` - Parsed style block.
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut defs: Vec<StyleDef> = Vec::new();
        while !input.is_empty() {
            let vis: Visibility = input.parse()?;
            let _: Option<Token![const]> = input.parse()?;
            let name: Ident = input.parse()?;
            let body: ParseBuffer<'_>;
            braced!(body in input);
            let mut rules: Vec<StyleRule> = Vec::new();
            while !body.is_empty() {
                let key: Ident = body.parse()?;
                let _: Token![:] = body.parse()?;
                let value: Expr = body.parse()?;
                let _: Option<Token![;]> = body.parse()?;
                let _: Option<Token![,]> = body.parse()?;
                rules.push(StyleRule { key, value });
            }
            defs.push(StyleDef { vis, name, rules });
        }
        Ok(Self { defs })
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
