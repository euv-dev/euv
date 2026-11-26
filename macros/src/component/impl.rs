use super::*;

/// Parser for `ComponentAttrArgs`.
impl Parse for ComponentAttrArgs {
    /// Parses optional `retention = Freeze | Destroy | Alive` arguments inside `#[component(...)]`.
    ///
    /// # Arguments
    ///
    /// - `ParseStream<'_>` - Attribute token stream.
    ///
    /// # Returns
    ///
    /// - `Result<Self>` - Parsed component attribute options.
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut retention: Option<Ident> = None;
        while !input.is_empty() {
            let key: Ident = input.parse()?;
            if key == COMP_ATTR_RETENTION {
                let _: Token![=] = input.parse()?;
                let val: Ident = input.parse()?;
                retention = Some(val);
            } else {
                return Err(Error::new(key.span(), ERR_COMPONENT_UNKNOWN_ATTR));
            }
            let _: Option<Token![,]> = input.parse()?;
        }
        Ok(Self { retention })
    }
}
