use super::*;

/// Parser for `WatchInput`.
impl Parse for WatchInput {
    /// Parses `(sig1, sig2, ...) => callback`.
    ///
    /// # Arguments
    ///
    /// - `ParseStream<'_>` - Token input stream.
    ///
    /// # Returns
    ///
    /// - `Result<Self>` - Parsed `WatchInput`.
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content: ParseBuffer<'_>;
        parenthesized!(content in input);
        let list: Punctuated<Expr, Token![,]> = content.parse_terminated(Expr::parse, Token![,])?;
        let _: Token![=>] = input.parse()?;
        let callback: Expr = input.parse()?;
        Ok(Self {
            signals: list.into_iter().collect(),
            callback,
        })
    }
}

/// Parser for `ComputedInput`.
impl Parse for ComputedInput {
    /// Parses `(sig1, sig2, ...) -> RetType => callback`.
    ///
    /// # Arguments
    ///
    /// - `ParseStream<'_>` - Token input stream.
    ///
    /// # Returns
    ///
    /// - `Result<Self>` - Parsed `ComputedInput`.
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content: ParseBuffer<'_>;
        parenthesized!(content in input);
        let list: Punctuated<Expr, Token![,]> = content.parse_terminated(Expr::parse, Token![,])?;
        let _: Token![->] = input.parse()?;
        let return_type: Type = input.parse()?;
        let _: Token![=>] = input.parse()?;
        let callback: Expr = input.parse()?;
        Ok(Self {
            signals: list.into_iter().collect(),
            return_type,
            callback,
        })
    }
}
