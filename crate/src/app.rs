use yew::prelude::*;
use yew_router::{prelude::*, route::Route, switch::Permissive, Switch};
use yew_styles::{
    navbar::{
        navbar_component::{Fixed, Navbar},
        navbar_container::NavbarContainer,
    },
    styles::{Palette, Style},
};

pub struct App;

#[derive(Switch, Debug, Clone)]
pub enum AppRouter {
    #[to = "/!"]
    RootPath,
    #[to = "/hello!"]
    HelloPath,
    #[to = "/awesome!"]
    AwesomePath,
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Navbar
                    navbar_type=Palette::Info
                    navbar_style=Style::Outline
                    fixed=Fixed::Top
                    branch=html!{<img src="/assets/spielrs_logo.png"></img>}
                >
                    <NavbarContainer>
                            <RouterAnchor<AppRouter> classes="navbar-item" route=AppRouter::RootPath>{"Home"}</RouterAnchor<AppRouter>>
                            <RouterAnchor<AppRouter> classes="navbar-item" route=AppRouter::HelloPath>{"Hello"}</RouterAnchor<AppRouter>>
                            <RouterAnchor<AppRouter> classes="navbar-item" route=AppRouter::AwesomePath>{"Awesome"}</RouterAnchor<AppRouter>>
                    </NavbarContainer>
                </Navbar>
                <Router<AppRouter, ()>
                    render = Router::render(|switch: AppRouter | {
                        match switch {
                            AppRouter::RootPath => html!{<h2>{"this is root"}</h2>},
                            AppRouter::HelloPath => html!{<h2>{"Hello world"}</h2>},
                            AppRouter::AwesomePath => html!{<h2>{"My awesome Yew with Yew-Router and Parcel application!"}</h2>},
                            AppRouter::PageNotFound(Permissive(None)) => html!{"Page not found"},
                            AppRouter::PageNotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)}
                        }
                    } )
                    redirect = Router::redirect(|route: Route<()>| {
                        AppRouter::PageNotFound(Permissive(Some(route.route)))
                    })
                />
            </div>
        }
    }
}
