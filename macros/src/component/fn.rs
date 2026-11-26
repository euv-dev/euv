use super::*;

/// Expands `#[component]` on an `impl Type { ... }` block into clean `impl Lifecycle` + `impl Component` (plus inherent helpers).
///
/// # Arguments
///
/// - `ComponentAttrArgs` - Parsed `#[component(...)]` options.
/// - `ItemImpl` - Annotated `impl` block.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Generated `Lifecycle`, `Component`, and optional helper `impl` blocks.
pub(crate) fn expand_component_attr(
    args: ComponentAttrArgs,
    item: ItemImpl,
) -> proc_macro2::TokenStream {
    let self_ty: &Type = item.self_ty.as_ref();
    let (impl_generics, _, where_clause) = item.generics.split_for_impl();
    let mut lifecycle_items: Vec<ImplItemFn> = Vec::new();
    let mut render_fn: Option<ImplItemFn> = None;
    let mut other_items: Vec<ImplItem> = Vec::new();
    let mut has_retention_fn: bool = false;
    for impl_item in item.items.into_iter() {
        match impl_item {
            ImplItem::Fn(func) => {
                let fname: String = func.sig.ident.to_string();
                if fname == COMPONENT_RENDER_FN {
                    render_fn = Some(func);
                } else if LIFECYCLE_METHOD_NAMES.contains(&fname.as_str()) {
                    if fname == "retention_policy" {
                        has_retention_fn = true;
                    }
                    lifecycle_items.push(func);
                } else {
                    other_items.push(ImplItem::Fn(func));
                }
            }
            other => other_items.push(other),
        }
    }
    let Some(render_method) = render_fn else {
        return Error::new(self_ty.span(), ERR_COMPONENT_MISSING_RENDER).to_compile_error();
    };
    let retention_override: proc_macro2::TokenStream =
        if !has_retention_fn && let Some(ret_ident) = args.retention {
            quote_spanned! {ret_ident.span()=>
            /// Body of the `retention_policy` free function.
            ///
            /// # Returns
            ///
            /// - `::euv::PageRetention` - A `::euv::PageRetention` value.
                        fn retention_policy(&self) -> ::euv::PageRetention {
                            ::euv::PageRetention::#ret_ident
                        }
                    }
        } else {
            quote! {}
        };
    let inherent_block: proc_macro2::TokenStream = if other_items.is_empty() {
        quote! {}
    } else {
        quote! {
        /// Implements `impl #impl_generics #self_ty #where_clause`.
                    impl #impl_generics #self_ty #where_clause {
                        #(#other_items)*
                    }
                }
    };
    quote! {
            #inherent_block

    /// Implements `impl #impl_generics ::euv::Lifecycle for #self_ty #where_clause`.
            impl #impl_generics ::euv::Lifecycle for #self_ty #where_clause {
                #retention_override
                #(#lifecycle_items)*
            }

    /// Implements `impl #impl_generics ::euv::Component for #self_ty #where_clause`.
            impl #impl_generics ::euv::Component for #self_ty #where_clause {
                #render_method
            }
        }
}
