use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, AttributeArgs, ItemStruct};

#[proc_macro_derive(BrainPrint, attributes(brainprint_ext))]
pub fn derive_brainprint(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name  = input.ident;

    // Enforce that the struct has a field named `core` of type BrainPrintCore.
    let expanded = quote! {
        impl brainprint_core::BrainPrintRecord for #name {
            fn core(&self) -> &brainprint_core::BrainPrintCore {
                &self.core
            }
        }

        // Compile-time range checks can be generated into constructors/builders
        // rather than raw struct literals; here we only enforce type-level shape.
    };

    expanded.into()
}

/// Attribute macro for sovereign-approved, namespaced extensions:
/// #[brainprint_ext(ns = "host.bostrom", approved = "true")]
#[proc_macro_attribute]
pub fn brainprint_ext(args: TokenStream, item: TokenStream) -> TokenStream {
    let _args = parse_macro_input!(args as AttributeArgs);
    let item  = parse_macro_input!(item as ItemStruct);
    let name  = &item.ident;

    // Convention: extension structs must be pure-data, Serialize + Deserialize,
    // and live under a host-controlled namespace prefix.
    let ext_ident = format_ident!("{}Ext", name);

    let expanded = quote! {
        #item

        impl #name {
            pub const BRAINPRINT_EXT_NS: &'static str = module_path!();
        }

        // Re-export a type alias for clarity.
        pub type #ext_ident = #name;
    };

    expanded.into()
}
