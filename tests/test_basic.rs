/*
 * // Copyright (c) 2016-2019, Jaguar0625, gimre, BloodyRookie, Tech Bureau, Corp.
 * // Copyright (c) 2020-present, Jaguar0625, gimre, BloodyRookie.
 * // All rights reserved.
 * //
 * // This file is part of Catapult.
 * //
 * // Catapult is free software: you can redistribute it and/or modify
 * // it under the terms of the GNU Lesser General Public License as published by
 * // the Free Software Foundation, either version 3 of the License, or
 * // (at your option) any later version.
 * //
 * // Catapult is distributed in the hope that it will be useful,
 * // but WITHOUT ANY WARRANTY; without even the implied warranty of
 * // MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * // GNU Lesser General Public License for more details.
 * //
 * // You should have received a copy of the GNU Lesser General Public License
 * // along with Catapult. If not, see <http://www.gnu.org/licenses/>.
 */

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
