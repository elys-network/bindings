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
    export_schema(&schema_for!(OracleAllPriceResponse), &out_dir);
    export_schema(&schema_for!(AmmSwapEstimationResponse), &out_dir);
    export_schema(&schema_for!(OracleAssetInfoResponse), &out_dir);
    export_schema(&schema_for!(AmmSwapExactAmountInResp), &out_dir);
    export_schema(&schema_for!(PerpetualOpenResponse), &out_dir);
    export_schema(&schema_for!(PerpetualCloseResponse), &out_dir);
}
