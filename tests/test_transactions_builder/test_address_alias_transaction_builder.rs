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

use catbuffer_rust::address_alias_transaction_builder::AddressAliasTransactionBuilder;

const VECTOR_TRANSACTION_BUILDER: [&str; 2] = [
    "A100000000000000164DC06341FE6FAC16EF51663F04113049B5CC3B648043EDE1D8BBF4BF16B7B8933F7E42A30B84A6D1EAB5CCECD8E4462923323E5816BED2134D54013B937D1A68B3FBB18729C1FDE225C57F8CE080FA828F0067E451A3FD81FA628842B0B7630000000001904E42010000000000000001000000000000004BFA5F372D55B3849049E14BEBCA93758EB36805BAE760A57239976F009A545C01",
    "A1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001A84E4200000000000000002B020000000000002AD8FC018D9A49E18026D27E1D0A26CA4E316F901E23E55C8711DB20DFBE8F3A01"
];

#[test]
fn test_should_create_transactions() {
    for vector in VECTOR_TRANSACTION_BUILDER.iter() {
        let bytes_vector = hex::decode(vector).unwrap();
        let build_object = AddressAliasTransactionBuilder::from_binary(&*bytes_vector);
        assert_eq!(build_object.serializer(), bytes_vector);
    }
}