use super::*;

/// Parser for `EntryInput`.
impl Parse for EntryInput {
    /// Parses the application bootstrap function identifier.
    ///
    /// # Arguments
    ///
    /// - `ParseStream<'_>` - Token input stream.
    ///
    /// # Returns
    ///
    /// - `Result<Self>` - Parsed `EntryInput`.
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let bootstrap: Ident = input.parse()?;
        Ok(Self { bootstrap })
    }
}
