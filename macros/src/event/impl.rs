use super::*;

/// Parser for `OnEventInput`.
impl Parse for OnEventInput {
    /// Parses `EventType => handler` syntax for `on_event!`.
    ///
    /// # Arguments
    ///
    /// - `ParseStream<'_>` - Token input stream.
    ///
    /// # Returns
    ///
    /// - `Result<Self>` - Parsed event listener invocation.
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let event_type: Type = input.parse()?;
        let _: Token![=>] = input.parse()?;
        let handler: Expr = input.parse()?;
        Ok(Self {
            event_type,
            handler,
        })
    }
}
