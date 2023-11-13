use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use elys_bindings::{msg_resp::*, query_resp::*, ElysMsg, ElysQuery};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(ElysMsg), &out_dir);
    export_schema(&schema_for!(ElysQuery), &out_dir);
    export_schema(&schema_for!(AllPriceResponse), &out_dir);
    export_schema(&schema_for!(QuerySwapEstimationResponse), &out_dir);
    export_schema(&schema_for!(OracleAssetInfoResponse), &out_dir);
    export_schema(&schema_for!(MsgSwapExactAmountInResp), &out_dir);
    export_schema(&schema_for!(MsgOpenResponse), &out_dir);
    export_schema(&schema_for!(MsgCloseResponse), &out_dir);
}
