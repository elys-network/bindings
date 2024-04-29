use chrono::NaiveDateTime;
use cosmwasm_std::BlockInfo;

pub fn get_raw_today(block_info: &BlockInfo) -> NaiveDateTime {
    NaiveDateTime::from_timestamp_opt(block_info.time.seconds() as i64, 0)
        .expect("Failed to convert block time to date")
}

pub fn get_today(block_info: &BlockInfo) -> String {
    get_raw_today(block_info).format("%Y-%m-%d").to_string()
}
