#[macro_use]
extern crate derive_builder;

use crate::components::entrypoint::*;
use context::*;

pub mod components;
pub mod context;
pub mod models;

fn main() {
    yew::start_app::<Entrypoint>();
}
