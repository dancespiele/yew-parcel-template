use yew::prelude::*;
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Home {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
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
        }
    }
}
