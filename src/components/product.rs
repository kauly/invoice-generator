use crate::context::InvoiceContext;
use crate::reducer::{InvoiceActions, ProductReturn};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Event;
use web_sys::HtmlInputElement;
use web_sys::InputEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct ProductProps {
  pub number: usize,
  pub description: String,
  pub qty: String,
  pub price: String,
}

impl Default for ProductProps {
  fn default() -> Self {
    Self {
      number: 1,
      description: String::from(""),
      qty: String::from("1"),
      price: String::from("1.00"),
    }
  }
}

#[function_component(Product)]
pub fn product(props: &ProductProps) -> Html {
  let invoice_reducer = use_context::<InvoiceContext>().unwrap();
  let ProductProps {
    description,
    number,
    qty,
    price,
  } = props.clone();

  let oninput = {
    let invoice_reducer = invoice_reducer.clone();
    Callback::from(move |e: InputEvent| {
      let event: Event = e.dyn_into().unwrap_throw();
      let event_target = event.target().unwrap_throw();
      let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
      let props = ProductReturn {
        value: target.value(),
        name: target.name(),
        key: number,
      };
      match props.name.as_str() {
        "description" => invoice_reducer.dispatch(InvoiceActions::UpdateProductDescription(props)),
        "qty" => invoice_reducer.dispatch(InvoiceActions::UpdateProductQty(props)),
        "price" => invoice_reducer.dispatch(InvoiceActions::UpdateProductPrice(props)),
        _ => panic!("unknown name"),
      }
    })
  };

  let onclick = {
    let invoice_reducer = invoice_reducer.clone();
    Callback::from(move |_| {
      invoice_reducer.dispatch(InvoiceActions::AddProduct);
    })
  };

  html! {
    <>
      <div class="p-2 bg-zinc-300 text-black text-center font-bold">{number}</div>
      <div class="py-2 px-4 text-center bg-zinc-300">
        <input class="pl-2 rounded w-40" name="description" value={description} oninput={oninput.clone()} />
      </div>
      <div class="py-2 px-4 text-center bg-zinc-300">
        <input class="pl-2 rounded w-40" name="qty" type="number" min="0" value={qty} oninput={oninput.clone()} />
      </div>
      <div class="py-2 px-4 text-center bg-zinc-300">
        <input class="pl-2 rounded w-40" name="price" type="number" min="0" step="0.01" value={price} oninput={oninput.clone()} />
      </div>
      <div class="bg-zinc-300 text-black text-center">
        <button {onclick} >{"ADD"}</button>
      </div>
    </>
  }
}
