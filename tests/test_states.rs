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
    account_restrictions_builder::AccountRestrictionsBuilder,
    finalized_block_header_builder::FinalizedBlockHeaderBuilder,
    metadata_entry_builder::MetadataEntryBuilder,
    mosaic_definition_builder::MosaicDefinitionBuilder,
};

#[test]
fn test_should_create_mosaic_definition_builder() {
    let payload = "000000000000000090F1B694E1801EEFE42846E9239B54C9D381FCDF2A04A4210100000007030A00000000000000";
    let bytes_payload = hex::decode(payload).unwrap();
    let build_object = MosaicDefinitionBuilder::from_binary(&*bytes_payload);
    assert_eq!(build_object.serializer(), bytes_payload);
}

#[test]
fn test_should_create_finalized_block_header_builder() {
    let payload = "90FD35818960C7B18B72F49A5598FA9F712A354DB38EB076C40300000000000011111111111111111111111111111111";
    let bytes_payload = hex::decode(payload).unwrap();
    let build_object = FinalizedBlockHeaderBuilder::from_binary(&*bytes_payload);
    assert_eq!(build_object.serializer(), bytes_payload);
}

#[test]
fn test_should_create_metadata_entry_builder() {
    const VECTOR_TRANSACTION_BUILDER: [&str; 3] = [
        "0100900E96DC85F6B24AC9C8DB5FFC59C35880C0B722C7A416A790FD35818960C7B18B72F49A5598FA9F712A354DB38EB0760A0000000000000068E0AE3A0168EDBD020B00536F6D6520537472696E67",
        "0100900E96DC85F6B24AC9C8DB5FFC59C35880C0B722C7A416A790FD35818960C7B18B72F49A5598FA9F712A354DB38EB0760A000000000000000000000000000000000B00536F6D6520537472696E67",
        "0100900E96DC85F6B24AC9C8DB5FFC59C35880C0B722C7A416A790FD35818960C7B18B72F49A5598FA9F712A354DB38EB0760A000000000000004460BA6E125F9C1C010B00536F6D6520537472696E67"
    ];

    for vector in VECTOR_TRANSACTION_BUILDER.iter() {
        let bytes_vector = hex::decode(vector).unwrap();
        let build_object = MetadataEntryBuilder::from_binary(&*bytes_vector);
        assert_eq!(build_object.serializer(), bytes_vector);
    }
}

#[test]
fn test_should_create_account_restrictions_builder() {
    const VECTOR_TRANSACTION_BUILDER: [&str; 4] = [
        "01009050B9837EFAB4BBE8A4B9BB32D812F9885C00D8FC1650E101000000000000000440020000000000000054415441",
        "01009050B9837EFAB4BBE8A4B9BB32D812F9885C00D8FC1650E10200000000000000044002000000000000005441544101000000000000000000",
        "01009050B9837EFAB4BBE8A4B9BB32D812F9885C00D8FC1650E1010000000000000001400000000000000000",
        "01009050B9837EFAB4BBE8A4B9BB32D812F9885C00D8FC1650E10000000000000000"
    ];

    for vector in VECTOR_TRANSACTION_BUILDER.iter() {
        let bytes_vector = hex::decode(vector).unwrap();
        let build_object = AccountRestrictionsBuilder::from_binary(&*bytes_vector);
        assert_eq!(build_object.serializer(), bytes_vector);
    }
}
