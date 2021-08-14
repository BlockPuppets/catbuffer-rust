use crate::voting_key_link_transaction_builder::VotingKeyLinkTransactionBuilder;

const VECTOR_TRANSACTION_BUILDER: [&str; 1] = [
    "A9000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001A8434100000000000000002B02000000000000C614558647D02037384A2FECA80ACE95B235D9B9D90035FA46102FE79ECCBA75010000000300000001",
];

#[test]
fn test_should_create_transactions() {
    for vector in VECTOR_TRANSACTION_BUILDER.iter() {
        let bytes_vector = hex::decode(vector).unwrap();
        let build_object = VotingKeyLinkTransactionBuilder::from_binary(&*bytes_vector);
        assert_eq!(build_object.serializer(), bytes_vector);
    }
}