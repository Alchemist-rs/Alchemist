#![deny(missing_docs, missing_debug_implementations,
        missing_copy_implementations, trivial_casts, trivial_numeric_casts,
        unsafe_code, unstable_features,
        unused_import_braces, unused_qualifications)]

#![cfg_attr(feature = "clippy", allow(unstable_features))]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#![allow(missing_docs)]
fn main() {
    println!("Hello, world!");
}
