use catbuffer_rust::{
    amount_dto::AmountDto, hash256_dto::Hash256Dto, key_dto::KeyDto, timestamp_dto::TimestampDto,
    unresolved_address_dto::UnresolvedAddressDto,
};

#[test]
fn test_should_create_timestamp_dto() {
    let payload = "1234567891234567";
    let bytes_payload = hex::decode(payload).unwrap();
    let build_object = TimestampDto::from_binary(&*bytes_payload);
    assert_eq!(build_object.serializer(), bytes_payload);
}

#[test]
fn test_should_create_key_dto() {
    let payload = "FA8EC085AE64CF30E44ADD18A3133D9B2190F9A20C08667A5EF44E5E9962E720";
    let bytes_payload = hex::decode(payload).unwrap();
    let build_object = KeyDto::from_binary(&*bytes_payload);
    assert_eq!(build_object.serializer(), bytes_payload);
}

#[test]
fn test_should_create_hash256_dto() {
    let payload = "4DC6F0524C486D78A6D9D775F5508C0362125420728D03DE74435EB1E3778891";
    let bytes_payload = hex::decode(payload).unwrap();
    let build_object = Hash256Dto::from_binary(&*bytes_payload);
    assert_eq!(build_object.serializer(), bytes_payload);
}

#[test]
fn test_should_create_amount_dto() {
    let payload = "0A00000000000000";
    let bytes_payload = hex::decode(payload).unwrap();
    let build_object = AmountDto::from_binary(&*bytes_payload);
    assert_eq!(build_object.serializer(), bytes_payload);
}

#[test]
fn test_should_create_unresolved_address_dto() {
    let payload = "90F36CA680C35D630662A0C38DC89D4978D10B511B3D241A";
    let bytes_payload = hex::decode(payload).unwrap();
    let build_object = UnresolvedAddressDto::from_binary(&*bytes_payload);
    assert_eq!(build_object.serializer(), bytes_payload);
}
