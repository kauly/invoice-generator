use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct ButtonProps {
  pub text: String,
  pub on_click: Callback<MouseEvent>,
}

#[function_component(Button)]
pub fn button(ButtonProps { text, on_click }: &ButtonProps) -> Html {
  html! {
    <button class="text-zinc-900 bg-white hover:bg-gray-400 font-medium rounded-lg text-sm px-5 py-2.5 text-center" onclick={on_click}>
    {text}
    </button>
  }
}
