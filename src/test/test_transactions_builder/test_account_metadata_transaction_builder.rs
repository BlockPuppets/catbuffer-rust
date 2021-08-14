use crate::account_metadata_transaction_builder::AccountMetadataTransactionBuilder;

const VECTOR_TRANSACTION_BUILDER: [&str; 2] = [
    "AA0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000F6503F78FBF99544B906872DDB392F4BE707180D285E7919DBACEF2E9573B1E60000000001904441000000000000000001000000000000009083025FF3A8AB5AD104631FB370F290004952CD1FDDC4C90A000000000000000A000600313233424143",
    "AE000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001A8444100000000000000002B0200000000000080D66C33420E5411995BACFCA2B28CF1C9F5DD7AB1A9C05CE80300000000000001000A0000000000000000000000"
];

#[test]
fn test_should_create_transactions() {
    for vector in VECTOR_TRANSACTION_BUILDER.iter() {
        let bytes_vector = hex::decode(vector).unwrap();
        let build_object = AccountMetadataTransactionBuilder::from_binary(&*bytes_vector);
        assert_eq!(build_object.serializer(), bytes_vector);
    }
}