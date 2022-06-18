#[macro_use]
extern crate derive_builder;
extern crate derive_more;

use crate::components::entrypoint::*;

pub mod components;
pub mod context;
pub mod models;
pub mod prelude;
pub mod router;

fn main() {
    yew::start_app::<Entrypoint>();
}
