use crate::components::product::ProductProps;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use strum_macros::{Display, EnumIter};
use yew::prelude::*;

#[derive(Clone, Debug, EnumIter, Display, PartialEq, Serialize, Deserialize)]
pub enum InvoiceActions {
  UpdateToName(String),
  UpdateToAddress1(String),
  UpdateToAddress2(String),
  UpdateFromName(String),
  UpdateFromAddress1(String),
  UpdateFromAddress2(String),
  UpdateNumber(String),
  UpdateInvoiceDate(String),
  UpdateDueDate(String),
  UpdateProductDescription(ProductReturn),
  UpdateProductQty(ProductReturn),
  UpdateProductPrice(ProductReturn),
  AddProduct,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProductReturn {
  pub name: String,
  pub value: String,
  pub key: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PartsData {
  pub name: String,
  pub address_1: String,
  pub address_2: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct InvoiceData {
  pub number: String,
  pub invoice_date: String,
  pub due_date: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct InvoiceState {
  pub from: PartsData,
  pub to: PartsData,
  pub invoice_data: InvoiceData,
  pub products: Vec<ProductProps>,
}

impl Default for ProductReturn {
  fn default() -> Self {
    Self {
      name: String::from(""),
      value: String::from(""),
      key: 0,
    }
  }
}

impl Default for InvoiceState {
  fn default() -> Self {
    Self {
      from: PartsData {
        name: String::from(""),
        address_1: String::from(""),
        address_2: String::from(""),
      },
      to: PartsData {
        name: String::from(""),
        address_1: String::from(""),
        address_2: String::from(""),
      },
      invoice_data: InvoiceData {
        number: String::from("0"),
        invoice_date: String::from(""),
        due_date: String::from(""),
      },
      products: vec![ProductProps {
        number: 1,
        description: String::from(""),
        qty: String::from(""),
        price: String::from(""),
      }],
    }
  }
}

impl Reducible for InvoiceState {
  type Action = InvoiceActions;

  fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
    match action {
      InvoiceActions::UpdateFromName(name) => {
        let mut current = self.from.clone();
        current.name = name;
        InvoiceState {
          to: self.to.clone(),
          from: current,
          invoice_data: self.invoice_data.clone(),
          products: self.products.clone(),
        }
        .into()
      }
      InvoiceActions::UpdateFromAddress1(address_1) => {
        let mut current = self.from.clone();
        current.address_1 = address_1;
        InvoiceState {
          to: self.to.clone(),
          from: current,
          invoice_data: self.invoice_data.clone(),
          products: self.products.clone(),
        }
        .into()
      }
      InvoiceActions::UpdateFromAddress2(address_2) => {
        let mut current = self.from.clone();
        current.address_2 = address_2;
        InvoiceState {
          to: self.to.clone(),
          from: current,
          invoice_data: self.invoice_data.clone(),
          products: self.products.clone(),
        }
        .into()
      }
      InvoiceActions::UpdateToName(name) => {
        let mut current = self.to.clone();
        current.name = name;
        InvoiceState {
          to: current,
          from: self.from.clone(),
          invoice_data: self.invoice_data.clone(),
          products: self.products.clone(),
        }
        .into()
      }
      InvoiceActions::UpdateToAddress1(address_1) => {
        let mut current = self.to.clone();
        current.address_1 = address_1;
        InvoiceState {
          to: current,
          from: self.from.clone(),
          invoice_data: self.invoice_data.clone(),
          products: self.products.clone(),
        }
        .into()
      }
      InvoiceActions::UpdateToAddress2(address_2) => {
        let mut current = self.to.clone();
        current.address_2 = address_2;
        InvoiceState {
          to: current,
          from: self.from.clone(),
          invoice_data: self.invoice_data.clone(),
          products: self.products.clone(),
        }
        .into()
      }
      InvoiceActions::UpdateNumber(number) => InvoiceState {
        to: self.to.clone(),
        from: self.from.clone(),
        invoice_data: InvoiceData {
          number,
          due_date: self.invoice_data.due_date.clone(),
          invoice_date: self.invoice_data.invoice_date.clone(),
        },
        products: self.products.clone(),
      }
      .into(),
      InvoiceActions::UpdateDueDate(date) => InvoiceState {
        to: self.to.clone(),
        from: self.from.clone(),
        invoice_data: InvoiceData {
          number: self.invoice_data.number.clone(),
          due_date: date,
          invoice_date: self.invoice_data.invoice_date.clone(),
        },
        products: self.products.clone(),
      }
      .into(),
      InvoiceActions::UpdateInvoiceDate(date) => InvoiceState {
        to: self.to.clone(),
        from: self.from.clone(),
        invoice_data: InvoiceData {
          number: self.invoice_data.number.clone(),
          due_date: self.invoice_data.due_date.clone(),
          invoice_date: date,
        },
        products: self.products.clone(),
      }
      .into(),
      InvoiceActions::UpdateProductDescription(product_props) => {
        let mut products = self.products.clone();
        let product = products.iter_mut().find(|p| p.number == product_props.key);
        if let Some(product) = product {
          product.description = product_props.value;
        }
        InvoiceState {
          to: self.to.clone(),
          from: self.from.clone(),
          invoice_data: self.invoice_data.clone(),
          products,
        }
        .into()
      }
      InvoiceActions::UpdateProductQty(product_props) => {
        let mut products = self.products.clone();
        let product = products.iter_mut().find(|p| p.number == product_props.key);
        if let Some(product) = product {
          product.qty = product_props.value;
        }
        InvoiceState {
          to: self.to.clone(),
          from: self.from.clone(),
          invoice_data: self.invoice_data.clone(),
          products,
        }
        .into()
      }
      InvoiceActions::UpdateProductPrice(product_props) => {
        let mut products = self.products.clone();
        let product = products.iter_mut().find(|p| p.number == product_props.key);
        if let Some(product) = product {
          product.price = product_props.value;
        }
        InvoiceState {
          to: self.to.clone(),
          from: self.from.clone(),
          invoice_data: self.invoice_data.clone(),
          products,
        }
        .into()
      }
      InvoiceActions::AddProduct => {
        let mut products = self.products.clone();
        products.push(ProductProps {
          number: products.len() + 1,
          description: String::from(""),
          qty: String::from("1"),
          price: String::from("1.00"),
        });
        InvoiceState {
          to: self.to.clone(),
          from: self.from.clone(),
          invoice_data: self.invoice_data.clone(),
          products,
        }
        .into()
      }
    }
  }
}
