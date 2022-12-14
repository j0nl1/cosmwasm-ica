use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::CosmosMsg;

/// Just needs to know the code_id of a reflect contract to spawn sub-accounts
#[cw_serde]
pub struct InstantiateMsg {
    pub cw1_code_id: u64,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Returns (reflect) account that is attached to this channel,
    /// or none.
    #[returns(AccountResponse)]
    Account { channel_id: String },
    /// Returns all (channel, reflect_account) pairs.
    /// No pagination - this is a test contract
    #[returns(ListAccountsResponse)]
    ListAccounts {},
}

#[cw_serde]
pub struct AccountResponse {
    pub account: Option<String>,
}

#[cw_serde]
pub struct ListAccountsResponse {
    pub accounts: Vec<AccountInfo>,
}

#[cw_serde]
pub struct AccountInfo {
    pub account: String,
    pub channel_id: String,
}

#[cw_serde]
pub enum ReflectExecuteMsg {
    ReflectMsg { msgs: Vec<CosmosMsg> },
}
