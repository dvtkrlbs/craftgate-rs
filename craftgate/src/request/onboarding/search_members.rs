use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use crate::request::onboarding::MemberType;

#[derive(Serialize, Deserialize, Debug, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option))]
#[builder(default)]
pub struct SearchMembersRequest {
    pub page: u64,
    pub size: u64,
    pub is_buyer: Option<bool>,
    pub is_sub_merchant: Option<bool>,
    pub member_type: Option<MemberType>,
    pub member_external_id: Option<String>,
    pub member_ids: Option<Vec<u64>>,
    pub name: Option<String>,
}

impl Default for SearchMembersRequest {
    fn default() -> Self {
        Self {
            page: 0,
            size: 25,
            is_sub_merchant: None,
            is_buyer: None,
            member_type: None,
            member_ids: None,
            name: None,
            member_external_id: None
        }
    }
}