use yew::prelude::*;
use yew_router::{prelude::*, route::Route, switch::Permissive, Switch};
use yew_styles::{
    layouts::{
        container::{Container, Direction, Wrap},
        item::{Item, ItemLayout},
    },
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
    #[to = "/about!"]
    AboutPath,
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
                    branch=html!{<img src="./yew.svg"></img>}
                >
                    <NavbarContainer>
                            <RouterAnchor<AppRouter> classes="navbar-item" route=AppRouter::RootPath>{"Home"}</RouterAnchor<AppRouter>>
                            <RouterAnchor<AppRouter> classes="navbar-item" route=AppRouter::AboutPath>{"About"}</RouterAnchor<AppRouter>>
                    </NavbarContainer>
                </Navbar>
                <Router<AppRouter, ()>
                    render = Router::render(|switch: AppRouter | {
                        match switch {
                            AppRouter::RootPath => html!{
                                <Container direction=Direction::Row wrap=Wrap::Wrap class_name="content">
                                    <Item layouts=vec!(ItemLayout::ItXs(12))>
                                        <h2>{"Yew Parcel Template"}</h2>
                                    </Item>
                                    <Item layouts=vec!(ItemLayout::ItXs(12))>
                                        <h3>{"Libraries used in this template"}</h3>
                                    </Item>
                                    <Item layouts=vec!(ItemLayout::ItXs(12))>
                                        <ul>
                                            <li><a href="https://yew.rs" target="_blank">{"yew.rs"}</a>{" : rustwasm frontent framwork"}</li>
                                            <li><a href="https://github.com/spielrs/yew_styles" target="_blank">
                                                {"yew_styles"}</a>{" : styles framework for yew"}</li>
                                            <li><a href="https://parceljs.org/" target="_blank">
                                                {"parceljs"}</a>{" : builder js library"}</li>
                                            <li><a href="https://github.com/paulmillr/chokidar" target="_blank">
                                            {"chokidar"}</a>{" : watcher js library"}</li>
                                        </ul>
                                    </Item>
                                </Container>
                            },
                            AppRouter::AboutPath => html!{
                                <Container direction=Direction::Row wrap=Wrap::Wrap class_name="content">
                                    <Item layouts=vec!(ItemLayout::ItXs(12))>
                                        <h2>{"Thanks for using or contributing!"}</h2>
                                    </Item>
                                    <Item layouts=vec!(ItemLayout::ItXs(12))>
                                        <p>{"Yew Parcel Template is a "}
                                            <a href="https://github.com/spielrs/yew-parcel-template/blob/master/LICENSE">{"MIT licensed "}</a>
                                            {"project maintained by open source comunity"}
                                        </p>
                                    </Item>
                                </Container>
                            },
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
