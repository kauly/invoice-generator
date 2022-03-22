use crate::reducer::InvoiceState;
use yew::prelude::*;

pub type InvoiceContext = UseReducerHandle<InvoiceState>;

#[derive(Properties, Debug, PartialEq)]
pub struct InvoiceProviderProps {
  #[prop_or_default]
  pub children: Children,
}

#[function_component]
pub fn InvoiceProvider(props: &InvoiceProviderProps) -> Html {
  let invoice_reducer = use_reducer(InvoiceState::default);
  html! {
    <ContextProvider<InvoiceContext> context={invoice_reducer}>
      {props.children.clone()}
    </ContextProvider<InvoiceContext>>
  }
}
