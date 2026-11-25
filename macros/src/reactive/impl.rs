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
        let paren: Paren = parenthesized!(content in input);
        let list: Punctuated<Expr, Token![,]> = content.parse_terminated(Expr::parse, Token![,])?;
        if list.is_empty() {
            return Err(Error::new(
                paren.span.join(),
                "watch! requires at least one signal, e.g. `watch!((count) => move |n: i32| { ... })`",
            ));
        }
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
        let paren: Paren = parenthesized!(content in input);
        let list: Punctuated<Expr, Token![,]> = content.parse_terminated(Expr::parse, Token![,])?;
        if list.is_empty() {
            return Err(Error::new(
                paren.span.join(),
                "computed! requires at least one signal, e.g. `computed!((count) -> i32 => move |n: i32| n * 2)`",
            ));
        }
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
