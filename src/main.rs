use std::str::FromStr;
use serde_json::Value;
use url::Url;

use aptos_sdk::rest_client::Client;
use aptos_sdk::rest_client::aptos_api_types::{ViewRequest, EntryFunctionId, MoveType};
use once_cell::sync::Lazy;

use reqwest::Client as RequestClient;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize)]
struct GraphQLQuery {
    query: String,
    variables: serde_json::Value,
}

static NODE_URL: Lazy<Url> = Lazy::new(|| {
    Url::from_str(
        std::env::var("APTOS_NODE_URL")
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("https://fullnode.mainnet.aptoslabs.com"),
    )
    .unwrap()
});

pub async fn fetch_reserves() {
    let rest_client = Client::new(NODE_URL.clone());

    let reqw = ViewRequest {
        function: EntryFunctionId::from_str("0x54cb0bb2c18564b86e34539b9f89cfe1186e39d89fce54e1cd007b8e61673a85::pool::get_bin_fields").unwrap(),
        arguments: vec![
            Value::from_str("8388604").unwrap()
        ],
        type_arguments: vec![
            MoveType::from_str("0xf22bede237a07e121b56d91a491eb7bcdfd1f5907926a9e58338f964a01b17fa::asset::USDC").unwrap(),
            MoveType::from_str("0xf22bede237a07e121b56d91a491eb7bcdfd1f5907926a9e58338f964a01b17fa::asset::USDT").unwrap(),
            MoveType::from_str("0x54cb0bb2c18564b86e34539b9f89cfe1186e39d89fce54e1cd007b8e61673a85::bin_steps::X5").unwrap(),
        ]
    };

    let res = rest_client.view(&reqw, Option::None).await.unwrap().inner().to_owned();
    println!("Bin Fields: {:#?}", res);
}

pub async fn fetch_supply() {
    let client = RequestClient::new();
    let query = r#"
        query MyQuery {
            current_token_datas_v2(
                where: {
                    current_collection: {collection_name: {_eq: "Liquidswap v1 #2 \"USDC\"-\"USDT\"-\"X5\""}}
                    token_name: {_eq: "8388604"}
                }
            ) {
                supply
                token_name
                token_data_id
                token_properties
                token_standard
                current_collection {
                    collection_name
                }
            }
        }
    "#.to_string();

    let graphql_query = GraphQLQuery {
        query,
        variables: json!({}),
    };

    let res: Value = client.post("https://api.mainnet.aptoslabs.com/v1/graphql")
        .json(&graphql_query)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    
    println!("Supply: {:#?}", res);
}

#[tokio::main]
async fn main() -> Result<(), ()>  {
    fetch_reserves().await;
    fetch_supply().await;
    
    Ok(())
}