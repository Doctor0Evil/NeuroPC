#[proc_macro]
pub fn neuroprofile(input: TokenStream) -> TokenStream {
    // Expand into a const OrganicAccessibilityEnvelope,
    // embedding paths to `.ocpu` / `.ocpuenv` shards and RoH guard.
}

#[proc_macro]
pub fn assistiveprofile(input: TokenStream) -> TokenStream {
    // Same, but for assistive/adaptive tooling envelopes.
}
