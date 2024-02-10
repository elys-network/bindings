use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum ElysDenom {
    // Elys
    Elys,
    // Eden
    Eden,
    // Eden Boost
    EdenBoost,
    // Usdc
    Usdc,
    // USDC
    USDC,
    // ElysSource
    ElysSource,
    // AnySource
    AnySource,
}

impl ElysDenom {
    pub fn as_str(&self) -> &'static str {
        match self {
            ElysDenom::Elys => "uelys",
            ElysDenom::Eden => "ueden",
            ElysDenom::EdenBoost => "uedenb",
            ElysDenom::Usdc => "uusdc",
            ElysDenom::USDC => "USDC",
            ElysDenom::ElysSource => "elys",
            ElysDenom::AnySource => "",
        }
    }
}