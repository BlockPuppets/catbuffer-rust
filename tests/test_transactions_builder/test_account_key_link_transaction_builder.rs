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

use catbuffer_rust::account_key_link_transaction_builder::AccountKeyLinkTransactionBuilder;

const VECTOR_TRANSACTION_BUILDER: [&str; 2] = [
    "A10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000F6503F78FBF99544B906872DDB392F4BE707180D285E7919DBACEF2E9573B1E60000000001904C4100000000000000000100000000000000F6503F78FBF99544B906872DDB392F4BE707180D285E7919DBACEF2E9573B1E601",
    "A1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001A84C4100000000000000002B020000000000009801508C58666C746F471538E43002B85B1CD542F9874B2861183919BA8787B601"
];

#[test]
fn test_should_create_transactions() {
    for vector in VECTOR_TRANSACTION_BUILDER.iter() {
        let bytes_vector = hex::decode(vector).unwrap();
        let build_object = AccountKeyLinkTransactionBuilder::from_binary(&*bytes_vector);
        assert_eq!(build_object.serializer(), bytes_vector);
    }
}