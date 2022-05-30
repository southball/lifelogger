use crate::models::*;
use yew::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct StateContext {
    pub state: SharedState,
    pub mutate: Callback<Mutation>,
}
