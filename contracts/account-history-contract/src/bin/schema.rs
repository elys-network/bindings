use account_history_contract::msg::{InstantiateMsg, QueryMsg};
use cosmwasm_schema::write_api;
use cosmwasm_std::Empty;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: Empty,
        query: QueryMsg
    }
}
