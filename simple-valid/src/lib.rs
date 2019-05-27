mod utils;

use wasm_bindgen::prelude::*;
#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn prompt(instr: &str, def_message: &str);
}


#[wasm_bindgen]
pub fn is_email(email: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?x)
            ^(?P<login>[^@\s]+)@
            ([[:word:]]+\.)*
            [[:word:]]+$
            ").unwrap();
    }
    RE.is_match(email)
}
