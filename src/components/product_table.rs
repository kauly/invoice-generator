use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProductTableProps {
  pub children: Children,
}

#[function_component(ProductTable)]
pub fn product_table(props: &ProductTableProps) -> Html {
  html! {
    <div class="grid grid-cols-5 w-full mt-8">
      <div class="p-2 bg-zinc-900 text-white text-center font-bold">{"#"}</div>
      <div class="p-2 bg-zinc-900 text-white text-center font-bold">{"Description"}</div>
      <div class="p-2 bg-zinc-900 text-white text-center font-bold">{"QTY"}</div>
      <div class="p-2 bg-zinc-900 text-white text-center font-bold">{"Price"}</div>
      <div class="p-2 bg-zinc-300 text-black text-center italic">{"Action"}</div>
      { for props.children.iter() }
    </div>
  }
}
