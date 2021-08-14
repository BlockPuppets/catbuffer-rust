use crate::mosaic_definition_transaction_builder::MosaicDefinitionTransactionBuilder;

const VECTOR_TRANSACTION_BUILDER: [&str; 3] = [
    "96000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001904D410000000000000000010000000000000000000000000000001027000000000000000000000504",
    "96000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001A84D4100000000000000002B020000000000000100000000000000E803000000000000E6DE84B80003",
    "96000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001A84D4100000000000000002B0200000000000001000000000000000000000000000000E6DE84B80003",
];

#[test]
fn test_should_create_transactions() {
    for vector in VECTOR_TRANSACTION_BUILDER.iter() {
        let bytes_vector = hex::decode(vector).unwrap();
        let build_object = MosaicDefinitionTransactionBuilder::from_binary(&*bytes_vector);
        assert_eq!(build_object.serializer(), bytes_vector);
    }
}