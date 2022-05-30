use crate::prelude::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct Editor {
    title_ref: NodeRef,
    description_ref: NodeRef,
}

pub enum EditorMsg {
    Add,
}

#[derive(Clone, PartialEq, Properties)]
pub struct EditorProps {}

impl Component for Editor {
    type Message = EditorMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            title_ref: NodeRef::default(),
            description_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let (state_context, _) = ctx
            .link()
            .context::<StateContext>(Callback::noop())
            .unwrap();

        let title_input = self.title_ref.cast::<HtmlInputElement>().unwrap();
        let description_input = self.description_ref.cast::<HtmlInputElement>().unwrap();
        match msg {
            EditorMsg::Add => {
                let title = title_input.value();
                let description = description_input.value();
                title_input.set_value("");
                description_input.set_value("");
                state_context.mutate.emit(Mutation::Todo(TodoMutation::Add(
                    TodoBuilder::default()
                        .title(title)
                        .description(description)
                        .build()
                        .unwrap(),
                )));
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onsubmit = ctx.link().callback(|event: web_sys::FocusEvent| {
            event.prevent_default();
            EditorMsg::Add
        });
        html! {
            <div>
                <form onsubmit={onsubmit}>
                    <input type="text" placeholder="Title" ref={self.title_ref.clone()} />
                    <input type="text" placeholder="Description" ref={self.description_ref.clone()} />
                    <button type="submit">{"Add"}</button>
                </form>
            </div>
        }
    }
}
