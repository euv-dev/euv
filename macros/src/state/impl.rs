use super::*;

/// Parser for `GlobalStateInput`.
impl Parse for GlobalStateInput {
    /// Parses `Type` or `Type = init_expr` for `global_state!`.
    ///
    /// # Arguments
    ///
    /// - `ParseStream<'_>` - Token input stream.
    ///
    /// # Returns
    ///
    /// - `Result<Self>` - Parsed global state expression.
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let ty: Type = input.parse()?;
        if input.peek(Token![=]) {
            let _: Token![=] = input.parse()?;
            let init: Expr = input.parse()?;
            Ok(Self::Init {
                ty: Box::new(ty),
                init: Box::new(init),
            })
        } else {
            Ok(Self::Lookup(Box::new(ty)))
        }
    }
}
