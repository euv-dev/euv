use super::*;

/// Expands `euv_main!(bootstrap)` into target-appropriate native and web entry points.
///
/// # Arguments
///
/// - `EntryInput` - Parsed entry point input.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Generated platform bootstrap functions.
pub(crate) fn expand_entry(input: EntryInput) -> proc_macro2::TokenStream {
    let bootstrap: Ident = input.bootstrap;
    quote! {
    /// Body of the `main` free function.
            #[cfg(not(any(target_arch = "wasm32", target_os = "android", target_os = "ios")))]
            fn main() {
                if let ::core::result::Result::Ok(mut app) = #bootstrap() {
                    let _ = app.step_frame();
                }
            }

    /// Body of the `euv_web_start` free function.
            #[cfg(target_arch = "wasm32")]
            #[::wasm_bindgen::prelude::wasm_bindgen(start)]
            pub fn euv_web_start() {
                if let ::core::result::Result::Ok(mut app) = #bootstrap() {
                    let _ = app.step_frame();
                }
            }

            #[cfg(target_os = "android")]
            #[unsafe(no_mangle)]
            pub extern "C" fn euv_android_entry() {
                if let ::core::result::Result::Ok(mut app) = #bootstrap() {
                    let _ = app.step_frame();
                }
            }

            #[cfg(target_os = "ios")]
            #[unsafe(no_mangle)]
            pub extern "C" fn euv_ios_entry() {
                if let ::core::result::Result::Ok(mut app) = #bootstrap() {
                    let _ = app.step_frame();
                }
            }
        }
}
