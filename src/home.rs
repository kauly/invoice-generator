use crate::components::{
  button::Button, input::Input, input::ReturnType, product::Product, product_table::ProductTable,
};
use crate::context::InvoiceContext;
use crate::reducer::InvoiceActions;
use wasm_bindgen::JsValue;
use web_sys::console;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
  let invoice_reducer = use_context::<InvoiceContext>().unwrap();

  let handle_inputs: Callback<ReturnType> = {
    let invoice_reducer = invoice_reducer.clone();
    Callback::from(move |val: ReturnType| match val.name.as_str() {
      "to_name" => invoice_reducer.dispatch(InvoiceActions::UpdateToName(val.value)),
      "to_address_1" => invoice_reducer.dispatch(InvoiceActions::UpdateToAddress1(val.value)),
      "to_address_2" => invoice_reducer.dispatch(InvoiceActions::UpdateToAddress2(val.value)),
      "from_name" => invoice_reducer.dispatch(InvoiceActions::UpdateFromName(val.value)),
      "from_address_1" => invoice_reducer.dispatch(InvoiceActions::UpdateFromAddress1(val.value)),
      "from_address_2" => invoice_reducer.dispatch(InvoiceActions::UpdateFromAddress2(val.value)),
      "invoice_number" => invoice_reducer.dispatch(InvoiceActions::UpdateNumber(val.value)),
      "invoice_date" => invoice_reducer.dispatch(InvoiceActions::UpdateInvoiceDate(val.value)),
      "due_date" => invoice_reducer.dispatch(InvoiceActions::UpdateDueDate(val.value)),
      _ => panic!("unknown name"),
    })
  };

  let handle_save: Callback<MouseEvent> = {
    let invoice_reducer = invoice_reducer.clone();
    let products = JsValue::from_serde(&invoice_reducer.products).unwrap();

    Callback::from(move |_| {
      console::log_1(&invoice_reducer.from.name.clone().into());
      console::log_1(&invoice_reducer.from.address_1.clone().into());
      console::log_1(&invoice_reducer.from.address_2.clone().into());
      console::log_1(&invoice_reducer.to.name.clone().into());
      console::log_1(&invoice_reducer.to.address_1.clone().into());
      console::log_1(&invoice_reducer.to.address_2.clone().into());
      console::log_1(&products);
    })
  };

  html! {
      <div class="w-full h-screen flex flex-col">
          <nav class="h-20 bg-zinc-900 text-white flex items-center justify-center">
              <Button text="Save" on_click={handle_save} />
          </nav>
          <div class="bg-zinc-300 flex-1">
              <div class="shadow-lg bg-white p-8 flex   w-2/3 m-auto mt-8 flex-wrap">
                  <div class="flex flex-col mr-8 gap-y-4">
                      <h2 class="italic">{"FROM"}</h2>
                      <Input value={invoice_reducer.from.name.clone()} on_change={handle_inputs.clone()} label="Company" name="from_name" />
                      <Input value={invoice_reducer.from.address_1.clone()} on_change={handle_inputs.clone()} label="Address Line 1" name="from_address_1" />
                      <Input value={invoice_reducer.from.address_2.clone()} on_change={handle_inputs.clone()} label="Address Line 2" name="from_address_2" />
                  </div>
                  <div class="flex flex-col gap-y-4">
                      <h2 class="italic">{"TO"}</h2>
                      <Input value={invoice_reducer.to.name.clone()} on_change={handle_inputs.clone()} label="Company" name="to_name" />
                      <Input value={invoice_reducer.to.address_1.clone()} on_change={handle_inputs.clone()} label="Address Line 1" name="to_address_1" />
                      <Input value={invoice_reducer.to.address_2.clone()} on_change={handle_inputs.clone()} label="Address Line 2" name="to_address_2" />
                  </div>
                  <div class="flex gap-x-4 mt-4 flex-[1_1_100%]">
                      <Input value={invoice_reducer.invoice_data.number.clone()} on_change={handle_inputs.clone()} label="Invoice Number" name="invoice_number" input_type="number" />
                      <Input value={invoice_reducer.invoice_data.invoice_date.clone()} on_change={handle_inputs.clone()} label="Invoice Date" name="invoice_date" input_type="date" />
                      <Input value={invoice_reducer.invoice_data.due_date.clone()} on_change={handle_inputs.clone()} label="Due Date" name="due_date" input_type="date" />
                  </div>
                  <ProductTable>
                  { for invoice_reducer.products.iter().map(|product| html! {
                      <Product qty={product.qty.clone()}  price={product.price.clone()} description={product.description.clone()} number={product.number.clone()} />
                  })}
                  </ProductTable>
              </div>
          </div>
      </div>
  }
}
