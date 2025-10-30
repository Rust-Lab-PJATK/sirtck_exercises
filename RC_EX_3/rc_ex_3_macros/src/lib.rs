use proc_macro::TokenStream;

#[proc_macro_derive(EventBlueprint, attributes(event))]
pub fn derive_event_blueprint(input: TokenStream) -> TokenStream {
    let _ = input;
    unimplemented!("Zaimplementuj derive EventBlueprint");
}
