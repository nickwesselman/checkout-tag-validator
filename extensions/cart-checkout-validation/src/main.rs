use output::FunctionError;
use shopify_function::prelude::*;
use shopify_function::Result;

use graphql_client;
use serde::{Deserialize, Serialize};

generate_types!(
    query_path = "./input.graphql",
    schema_path = "./schema.graphql"
);

#[derive(Serialize, Deserialize, Default, PartialEq)]
struct Config {}

#[shopify_function]
fn function(input: input::ResponseData) -> Result<output::FunctionResult> {
    let mut errors = Vec::new();
    
    let is_member = match input.cart.buyer_identity {
        Some(buyer) => match buyer.customer {
            Some(customer) => customer.is_member,
            None => false
        },
        None => false
    };

    if input
        .cart
        .lines
        .iter()
        .filter_map(|line| match &line.merchandise {
            input::InputCartLinesMerchandise::ProductVariant(variant) => Some(variant),
            input::InputCartLinesMerchandise::CustomProduct => None,
        })
        .any(|variant| variant.product.is_members_product)
        && !is_member
    {
        errors.push(FunctionError {
            localized_message: "Cannot purchase member products as non member".to_string(),
            target: "cart".to_owned(),
        })
    }
    Ok(output::FunctionResult { errors })
}

#[cfg(test)]
mod tests;
