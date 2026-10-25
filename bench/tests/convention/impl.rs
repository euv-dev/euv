use super::*;

impl<'ast> Visit<'ast> for FieldAccessAudit {
    fn visit_expr_field(&mut self, expression: &'ast ExprField) {
        let ExprField { member, .. }: &ExprField = expression;
        self.get_mut_violations()
            .push(format!("direct field access: {member:?}"));
        visit::visit_expr_field(self, expression);
    }

    fn visit_macro(&mut self, invocation: &'ast Macro) {
        let Macro { path, tokens, .. }: &Macro = invocation;
        if path.is_ident("cfg") {
            return;
        }
        let mut nested: FieldAccessAudit = FieldAccessAudit::default();
        if path.is_ident("thread_local") {
            let parsed: Result<File, Error> = parse2(tokens.clone());
            match parsed {
                Ok(file) => visit::visit_file(&mut nested, &file),
                Err(error) => nested
                    .get_mut_violations()
                    .push(format!("unparsed thread-local items: {error}")),
            }
        } else if path.is_ident("vec") {
            let parsed: Result<Expr, Error> = parse_str(&format!("[{tokens}]"));
            match parsed {
                Ok(expression) => visit::visit_expr(&mut nested, &expression),
                Err(error) => nested
                    .get_mut_violations()
                    .push(format!("unparsed vector expression: {error}")),
            }
        } else {
            let supported: bool = [
                "format",
                "println",
                "eprintln",
                "write",
                "writeln",
                "env",
                "concat",
                "stringify",
                "dbg",
                "matches",
            ]
            .iter()
            .any(|name: &&str| path.is_ident(*name));
            if !supported {
                self.get_mut_violations()
                    .push(format!("unrecognized production macro: {path:?}"));
                return;
            }
            let parsed: Result<Punctuated<Expr, Token![,]>, Error> =
                Punctuated::<Expr, Token![,]>::parse_terminated.parse2(tokens.clone());
            match parsed {
                Ok(expressions) => {
                    for expression in &expressions {
                        visit::visit_expr(&mut nested, expression);
                    }
                }
                Err(error) => nested
                    .get_mut_violations()
                    .push(format!("unparsed macro arguments: {error}")),
            }
        }
        self.get_mut_violations()
            .extend(take(nested.get_mut_violations()));
    }
}
