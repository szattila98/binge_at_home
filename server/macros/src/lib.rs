extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use rand::Rng;

#[proc_macro]
pub fn random_emoji(_: TokenStream) -> TokenStream {
    let emojis = emojis::iter().collect::<Vec<_>>();
    let index = rand::thread_rng().gen_range(0..emojis.len());
    let emoji = emojis[index].as_str();
    quote! {
        #emoji
    }
    .into()
}
