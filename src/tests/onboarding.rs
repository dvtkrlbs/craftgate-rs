use crate::request::onboarding::create_member::Member;
use crate::request::onboarding::MemberType;
#[cfg(test)]
use crate::tests::get_test_client;

#[tokio::test]
#[ignore]
async fn test_create_member() {
    let member = Member {
        is_buyer: Some(true),
        is_sub_merchant: Some(false),
        contact_name: Some("Haluk".to_owned()),
        contact_surname: Some("Demir".to_owned()),
        email: "haluk.demir@example.com".to_owned(),
        phone_number: "905551111111".to_owned(),
        identity_number: Some("11111111110".to_owned()),
        name: Some("Haluk Demir".to_owned()),
        member_type: Some(MemberType::Personal),
        member_external_id: "d8fa867b-000b-4b96-ad3c-43ea22e65e3f".to_owned(),
        address: "Suadiye Mah. Örnek Cd. No:23, 34740 Kadıköy/İstanbul".to_owned(),
        ..Default::default()
    };

    let member = get_test_client().create_member(member).await.unwrap();

    assert_eq!(
        member.data.member_external_id,
        "d8fa867b-000b-4b96-ad3c-43ea22e65e3f".to_owned()
    );
}
