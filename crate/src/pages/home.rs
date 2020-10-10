use super::ViewState;
use yew::prelude::*;
use yew_state::{SharedHandle, SharedStateComponent};
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};
use yewtil::NeqAssign;

#[derive(Default, Clone, PartialEq)]
pub struct ExampleState {
    pub text: String,
}

pub struct HomeModel {
    handle: SharedHandle<ExampleState>,
}

impl Component for HomeModel {
    type Message = ();
    type Properties = SharedHandle<ExampleState>;

    fn create(handle: Self::Properties, _: ComponentLink<Self>) -> Self {
        handle.reduce(|example_state| example_state.text = String::from("Example"));

        HomeModel { handle }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, handle: Self::Properties) -> ShouldRender {
        self.handle.neq_assign(handle)
    }

    fn view(&self) -> Html {
        html! {
            <Container direction=Direction::Row wrap=Wrap::Wrap class_name="content">
                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <h2>{self.handle.state().text.clone()}</h2>
                    <ViewState/>
                </Item>
            </Container>
        }
    }
}

pub type Home = SharedStateComponent<HomeModel>;
