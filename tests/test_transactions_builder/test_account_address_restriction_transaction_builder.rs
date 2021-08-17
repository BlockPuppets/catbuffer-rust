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

use catbuffer_rust::account_address_restriction_transaction_builder::AccountAddressRestrictionTransactionBuilder;

const VECTOR_TRANSACTION_BUILDER: [&str; 2] = [
    "B80000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000F6503F78FBF99544B906872DDB392F4BE707180D285E7919DBACEF2E9573B1E600000000019050410000000000000000010000000000000001000101000000009083025FF3A8AB5AD104631FB370F290004952CD1FDDC4C990B387A39C0E4607DB7056EEAAF0A0EF43B45C667EB790FF",
    "A0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001A8504100000000000000002B0200000000000001000100000000008026D27E1D0A26CA4E316F901E23E55C8711DB20DFBE8F3A"
];

#[test]
fn test_should_create_transactions() {
    for vector in VECTOR_TRANSACTION_BUILDER.iter() {
        let bytes_vector = hex::decode(vector).unwrap();
        let build_object = AccountAddressRestrictionTransactionBuilder::from_binary(&*bytes_vector);
        assert_eq!(build_object.serializer(), bytes_vector);
    }
}