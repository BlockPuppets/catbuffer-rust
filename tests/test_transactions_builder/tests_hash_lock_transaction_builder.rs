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

use catbuffer_rust::hash_lock_transaction_builder::HashLockTransactionBuilder;

const VECTOR_TRANSACTION_BUILDER: [&str; 1] = [
    "B800000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002134E47AEE6F2392A5B3D1238CD7714EABEB739361B7CCF24BAE127F10DF17F200000000019048410000000000000000010000000000000044B262C46CEABB85809698000000000064000000000000008498B38D89C1DC8A448EA5824938FF828926CD9F7747B1844B59B4B6807E878B"
];

#[test]
fn test_should_create_transactions() {
    for vector in VECTOR_TRANSACTION_BUILDER.iter() {
        let bytes_vector = hex::decode(vector).unwrap();
        let build_object = HashLockTransactionBuilder::from_binary(&*bytes_vector);
        assert_eq!(build_object.serializer(), bytes_vector);
    }
}