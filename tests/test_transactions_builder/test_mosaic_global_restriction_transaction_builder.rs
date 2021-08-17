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

use catbuffer_rust::mosaic_global_restriction_transaction_builder::MosaicGlobalRestrictionTransactionBuilder;

const VECTOR_TRANSACTION_BUILDER: [&str; 2] = [
    "AA00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000009801508C58666C746F471538E43002B85B1CD542F9874B2861183919BA8787B6000000000190514100000000000000000100000000000000800D00000000000002000000000000000100000000000000090000000000000008000000000000000106",
    "AA000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001A8514100000000000000002B02000000000000010000000000000000000000000000005C11000000000000000000000000000000000000000000000006"
];

#[test]
fn test_should_create_transactions() {
    for vector in VECTOR_TRANSACTION_BUILDER.iter() {
        let bytes_vector = hex::decode(vector).unwrap();
        let build_object = MosaicGlobalRestrictionTransactionBuilder::from_binary(&*bytes_vector);
        assert_eq!(build_object.serializer(), bytes_vector);
    }
}