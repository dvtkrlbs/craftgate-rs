use crate::request::onboarding::MemberType;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(field_defaults(default))]
pub struct SearchMembersRequest {
    #[builder(!default)]
    pub page: u64,
    #[builder(!default)]
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
            member_external_id: None,
        }
    }
}
