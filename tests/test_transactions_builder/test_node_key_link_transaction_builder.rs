use catbuffer_rust::node_key_link_transaction_builder::NodeKeyLinkTransactionBuilder;

const VECTOR_TRANSACTION_BUILDER: [&str; 1] = [
    "A1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001A84C4200000000000000002B020000000000009801508C58666C746F471538E43002B85B1CD542F9874B2861183919BA8787B601",
];

#[test]
fn test_should_create_transactions() {
    for vector in VECTOR_TRANSACTION_BUILDER.iter() {
        let bytes_vector = hex::decode(vector).unwrap();
        let build_object = NodeKeyLinkTransactionBuilder::from_binary(&*bytes_vector);
        assert_eq!(build_object.serializer(), bytes_vector);
    }
}