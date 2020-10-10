use super::ExampleState;
use yew::prelude::*;
use yew_state::{component, SharedHandle, StateView};

pub struct ViewState;

impl Component for ViewState {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        ViewState {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        type Handle = SharedHandle<ExampleState>;

        let view = component::view(|handle: &Handle| {
            let example_state = handle.state();
            html! {
                <p>{format!("Text: {}", example_state.text)}</p>
            }
        });

        html! {
            <StateView<Handle> view=view/>
        }
    }
}
