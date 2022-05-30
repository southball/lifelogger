#[macro_use]
extern crate derive_builder;

use crate::components::entrypoint::*;

pub mod components;
pub mod context;
pub mod models;
pub mod prelude;

fn main() {
    yew::start_app::<Entrypoint>();
}
