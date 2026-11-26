use super::*;

/// Parser for `WatchInput`.
impl Parse for WatchInput {
    /// Parses explicit `(sig1, sig2, ...) => callback` or auto-tracked `|| { ... }`.
    ///
    /// # Arguments
    ///
    /// - `ParseStream<'_>` - Token input stream.
    ///
    /// # Returns
    ///
    /// - `Result<Self>` - Parsed `WatchInput`.
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        if input.peek(Paren) {
            let fork: ParseBuffer<'_> = input.fork();
            let fork_content: ParseBuffer<'_>;
            let _: Paren = parenthesized!(fork_content in fork);
            let _: Punctuated<Expr, Token![,]> =
                fork_content.parse_terminated(Expr::parse, Token![,])?;
            if fork.peek(Token![=>]) {
                let content: ParseBuffer<'_>;
                let paren: Paren = parenthesized!(content in input);
                let list: Punctuated<Expr, Token![,]> =
                    content.parse_terminated(Expr::parse, Token![,])?;
                if list.is_empty() {
                    return Err(Error::new(paren.span.join(), ERR_WATCH_EMPTY_SIGNALS));
                }
                let _: Token![=>] = input.parse()?;
                let callback: Expr = input.parse()?;
                return Ok(Self {
                    signals: list.into_iter().collect(),
                    callback,
                });
            }
        }
        let callback: Expr = input.parse()?;
        Ok(Self {
            signals: Vec::new(),
            callback,
        })
    }
}

/// Parser for `ComputedInput`.
impl Parse for ComputedInput {
    /// Parses explicit `(sig1, sig2, ...) -> RetType => callback`, typed `-> RetType => || expr`, or auto-tracked `|| expr`.
    ///
    /// # Arguments
    ///
    /// - `ParseStream<'_>` - Token input stream.
    ///
    /// # Returns
    ///
    /// - `Result<Self>` - Parsed `ComputedInput`.
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        if input.peek(Paren) {
            let fork: ParseBuffer<'_> = input.fork();
            let fork_content: ParseBuffer<'_>;
            let _: Paren = parenthesized!(fork_content in fork);
            let _: Punctuated<Expr, Token![,]> =
                fork_content.parse_terminated(Expr::parse, Token![,])?;
            if fork.peek(Token![->]) {
                let content: ParseBuffer<'_>;
                let paren: Paren = parenthesized!(content in input);
                let list: Punctuated<Expr, Token![,]> =
                    content.parse_terminated(Expr::parse, Token![,])?;
                if list.is_empty() {
                    return Err(Error::new(paren.span.join(), ERR_COMPUTED_EMPTY_SIGNALS));
                }
                let _: Token![->] = input.parse()?;
                let return_type: Type = input.parse()?;
                let _: Token![=>] = input.parse()?;
                let callback: Expr = input.parse()?;
                return Ok(Self {
                    signals: list.into_iter().collect(),
                    return_type: Some(return_type),
                    callback,
                });
            }
        }
        if input.peek(Token![->]) {
            let _: Token![->] = input.parse()?;
            let return_type: Type = input.parse()?;
            let _: Token![=>] = input.parse()?;
            let callback: Expr = input.parse()?;
            return Ok(Self {
                signals: Vec::new(),
                return_type: Some(return_type),
                callback,
            });
        }
        let callback: Expr = input.parse()?;
        Ok(Self {
            signals: Vec::new(),
            return_type: None,
            callback,
        })
    }
}
