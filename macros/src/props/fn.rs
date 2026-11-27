use super::*;

/// Expands `#[derive(Props)]` into `::euv::ComponentProps` initialization plus span-preserving fluent named-prop setters.
///
/// # Arguments
///
/// - `DeriveInput` - Parsed target struct definition.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Generated `ComponentProps` and inherent fluent setter implementations.
pub(crate) fn expand_props_derive(input: DeriveInput) -> proc_macro2::TokenStream {
    let ident: &Ident = &input.ident;
    let (impl_generics, ty_generics, where_clause): (
        ImplGenerics<'_>,
        TypeGenerics<'_>,
        Option<&WhereClause>,
    ) = input.generics.split_for_impl();
    let named_fields: &Punctuated<Field, Token![,]> = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Error::new(ident.span(), ERR_PROPS_NAMED_STRUCT_ONLY).to_compile_error();
            }
        },
        _ => {
            return Error::new(ident.span(), ERR_PROPS_NAMED_STRUCT_ONLY).to_compile_error();
        }
    };
    let mut init_fields: Vec<proc_macro2::TokenStream> = Vec::with_capacity(named_fields.len());
    let mut setters: Vec<proc_macro2::TokenStream> = Vec::with_capacity(named_fields.len());
    for field in named_fields.iter() {
        let Some(fname) = field.ident.as_ref() else {
            continue;
        };
        let (skip_setter, default_expr): (bool, Option<Expr>) =
            match parse_prop_field_attrs(&field.attrs) {
                Ok(parsed) => parsed,
                Err(err) => return err.to_compile_error(),
            };
        let init_val: proc_macro2::TokenStream = match default_expr {
            Some(expr) => quote_spanned! {expr.span()=> #expr },
            None => quote_spanned! {fname.span()=> ::core::default::Default::default() },
        };
        init_fields.push(quote_spanned! {fname.span()=> #fname: #init_val });
        if !skip_setter {
            let fty: &Type = &field.ty;
            let setter: proc_macro2::TokenStream = if is_string_type(fty) {
                quote_spanned! {fname.span()=>
                    #[must_use]
                    #[cfg_attr(not(target_arch = "wasm32"), inline)]
                    pub fn #fname<__EuvStr>(mut self, value: __EuvStr) -> Self
                    where
                        __EuvStr: ::core::convert::Into<::std::string::String>,
                    {
                        self.#fname = ::core::convert::Into::into(value);
                        self
                    }
                }
            } else {
                quote_spanned! {fname.span()=>
                    #[must_use]
                    #[cfg_attr(not(target_arch = "wasm32"), inline)]
                    pub fn #fname(mut self, value: #fty) -> Self {
                        self.#fname = value;
                        self
                    }
                }
            };
            setters.push(setter);
        }
    }
    let setters_impl: proc_macro2::TokenStream = if setters.is_empty() {
        quote! {}
    } else {
        quote! {
        /// Implements `impl #impl_generics #ident #ty_generics #where_clause`.
                    impl #impl_generics #ident #ty_generics #where_clause {
                        #(#setters)*
                    }
                }
    };
    quote! {
    /// Implements `impl #impl_generics ::euv::ComponentProps for #ident #ty_generics #where_clause`.
            impl #impl_generics ::euv::ComponentProps for #ident #ty_generics #where_clause {
    /// Implements the `init_props` operation on [`Self`].
                #[cfg_attr(not(target_arch = "wasm32"), inline)]
                fn init_props() -> Self {
                    Self {
                        #(#init_fields),*
                    }
                }
            }
            #setters_impl
        }
}

/// Parses `#[prop(skip)]` and `#[prop(default = expr)]` attributes attached to a struct field.
///
/// # Arguments
///
/// - `&[Attribute]` - Field attributes slice.
///
/// # Returns
///
/// - `Result<(bool, Option<Expr>)>` - `(skip_setter, optional_default_expr)`.
fn parse_prop_field_attrs(attrs: &[Attribute]) -> Result<(bool, Option<Expr>)> {
    let mut skip_setter: bool = false;
    let mut default_expr: Option<Expr> = None;
    for attr in attrs.iter() {
        if !attr.path().is_ident(ATTR_PROP) {
            continue;
        }
        let nested: Punctuated<Meta, Token![,]> =
            attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
        for meta in nested.into_iter() {
            match meta {
                Meta::Path(path) if path.is_ident(PROP_OPT_SKIP) => {
                    skip_setter = true;
                }
                Meta::NameValue(nv) if nv.path.is_ident(PROP_OPT_DEFAULT) => {
                    default_expr = Some(nv.value);
                }
                other => {
                    return Err(Error::new(other.span(), ERR_PROPS_UNKNOWN_OPTION));
                }
            }
        }
    }
    Ok((skip_setter, default_expr))
}

/// Returns `true` when `ty` is an unqualified or qualified `String` type without generic arguments.
///
/// # Arguments
///
/// - `&Type` - Field type AST to inspect.
///
/// # Returns
///
/// - `bool` - Whether `ty` is `String`.
fn is_string_type(ty: &Type) -> bool {
    let Type::Path(type_path) = ty else {
        return false;
    };
    if type_path.qself.is_some() {
        return false;
    }
    let Some(last) = type_path.path.segments.last() else {
        return false;
    };
    last.ident == TYPE_NAME_STRING && matches!(last.arguments, PathArguments::None)
}
