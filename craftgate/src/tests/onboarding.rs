#[cfg(test)]
use crate::{
    request::onboarding::{
        create_member::CreateMemberRequest, search_members::SearchMembersRequest,
        update_member::UpdateMemberRequest, MemberType,
    },
    tests::get_test_client,
};
#[cfg(test)]
use assert_matches::assert_matches;

#[tokio::test]
#[ignore]
async fn test_create_member() {
    let member = CreateMemberRequest {
        is_buyer: Some(true),
        is_sub_merchant: Some(false),
        contact_name: Some("Haluk".to_owned()),
        contact_surname: Some("Demir".to_owned()),
        email: "haluk.demir@example.com".to_owned(),
        phone_number: "905551111111".to_owned(),
        identity_number: Some("11111111110".to_owned()),
        name: Some("Haluk Demir".to_owned()),
        member_type: Some(MemberType::LimitedOrJointStockCompany),
        member_external_id: "d8fa867b-000b-4b96-ad3c-43ea22e65e3f".to_owned(),
        address: "Suadiye Mah. Örnek Cd. No:23, 34740 Kadıköy/İstanbul".to_owned(),
        ..Default::default()
    };

    let member = get_test_client().create_member(member).await.unwrap();

    assert_eq!(
        member.member_external_id,
        "d8fa867b-000b-4b96-ad3c-43ea22e65e3f".to_owned()
    );
}

#[tokio::test]
#[ignore]
async fn test_update_member() {
    let member = UpdateMemberRequest::builder()
        .contact_name("Haluk".to_owned())
        .contact_surname("Demir".to_owned())
        .email("haluk.demir@example.com".to_owned())
        .phone_number(Some("905551111111".to_owned()))
        .iban(Some("TR930006701000000001111111".to_owned()))
        .identity_number(Some("11111111110".to_owned()))
        .legal_company_title(Some("Dem Zeytinyağı Üretim Ltd. Şti.".to_owned()))
        .name("Dem Zeytinyağı Üretim Ltd. Şti.".to_owned())
        .member_type(Some(MemberType::LimitedOrJointStockCompany))
        .tax_number(Some("1111111114".to_owned()))
        .tax_office(Some("Erenköy".to_owned()))
        .address("Suadiye Mah. Örnek Cd. No:23, 34740 Kadıköy/İstanbul".to_owned())
        .is_buyer(Some(true))
        .is_sub_merchant(Some(false))
        .build();

    let member = get_test_client()
        .update_member(89508, member)
        .await
        .unwrap();

    assert_eq!(
        member.member_external_id,
        "d8fa867b-000b-4b96-ad3c-43ea22e65e3f".to_owned()
    );
}

#[tokio::test]
#[ignore]
async fn test_retrieve_member() {
    let member = get_test_client().retrieve_member(89508).await.unwrap();

    assert_matches!(member, Some(member) => {
        assert_eq!(member.member_external_id, "d8fa867b-000b-4b96-ad3c-43ea22e65e3f".to_owned());
    })
}

#[tokio::test]
#[ignore]
async fn test_search_members() {
    let params = SearchMembersRequest::builder()
        .member_external_id(Some("d8fa867b-000b-4b96-ad3c-43ea22e65e3f".to_owned()))
        .size(25)
        .build();

    let members = get_test_client().search_members(params).await.unwrap();

    assert_eq!(members.items.len(), 1);
}
