#![recursion_limit = "128"]
// this will force us on nightly. The alternative is to
// // Box everything which is annoying. This will *eventually* become
// // stable so we will support stable too.
#![feature(conservative_impl_trait)]

extern crate chrono;
extern crate futures;
extern crate hyper;
extern crate native_tls;
extern crate serde_json;
extern crate url;
extern crate uuid;
extern crate xml;

#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate log;

pub mod ba512_range;
#[macro_use]
pub mod enumerations;
#[macro_use]
pub mod errors;
pub mod incompletevector;
pub mod lease;
pub mod parsing;
pub mod range;
