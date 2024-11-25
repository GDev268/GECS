extern crate proc_macro;
use std::str::FromStr;

use proc_macro::TokenStream;

#[proc_macro]
pub fn bundle_macro(input: TokenStream) -> TokenStream {

    let mut types = String::new();
    let mut generics = String::new();
    let mut components = String::new();
    let mut component_count  = 0;

    for token in input {
        if component_count == 0 {
            types += &format!("{token}");
            generics += &format!("{token}: Component");
            components += &format!("Box::new(self.{component_count})")
        } else {
            types += &format!(", {token}");
            generics += &format!(", {token}: Component");
            components += &format!(", Box::new(self.{component_count})")
        }

        component_count += 1;
    }

    if component_count == 1 {
        components = format!("Box::new(self)")
    }

    TokenStream::from_str(&format!(
        "
        impl<{generics}> Bundle for ({types}) {{
            fn components(self) -> Vec<Box<dyn Component>> {{
                vec![{components}]
            }}
        }}
            
        "
    )).unwrap()
}