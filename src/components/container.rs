use crate::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ContainerProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Container)]
pub fn container(props: &ContainerProps) -> Html {
    html! {
        <div>
            <nav
                class="navbar navbar-dark navbar-expand flex-shrink-1"
                style="background-color: #0d503c">
                <div class="container-fluid">
                    <Link<AppRoute> to={AppRoute::Home} classes="navbar-brand">
                        {"Lifelogger"}
                    </Link<AppRoute>>
                    <ul class="navbar-nav me-auto">
                        <li class="nav-item">
                            <Link<AppRoute> to={AppRoute::Home} classes="nav-link">
                                {"Home"}
                            </Link<AppRoute>>
                        </li>
                        <li class="nav-item">
                            <a href="#" class="nav-link">
                                {"Settings"}
                            </a>
                        </li>
                    </ul>
                </div>
            </nav>

            <div class="container">
                {for props.children.iter()}
            </div>
        </div>
    }
}
