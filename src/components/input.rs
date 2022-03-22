use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Event;
use web_sys::HtmlInputElement;
use web_sys::InputEvent;
use yew::prelude::*;

pub struct ReturnType {
  pub value: String,
  pub name: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct InputProps {
  pub label: String,
  pub value: String,
  pub name: String,
  #[prop_or("text".to_string())]
  pub input_type: String,
  pub on_change: Callback<ReturnType>,
}

fn get_value_from_event(e: InputEvent) -> ReturnType {
  let event: Event = e.dyn_into().unwrap_throw();
  let event_target = event.target().unwrap_throw();
  let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
  ReturnType {
    value: target.value(),
    name: target.name(),
  }
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
  let InputProps {
    value,
    label,
    name,
    on_change,
    input_type,
  } = props.clone();
  let oninput = Callback::from(move |input_event: InputEvent| {
    on_change.emit(get_value_from_event(input_event));
  });

  html! {
    <div class="flex flex-col w-full">
      <label class="mb-2 bold">{label}</label>
      <input type={input_type} class="p-2 border-2 border-zinc-900 rounded-md" {value} {oninput} {name} />
    </div>
  }
}
