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

use catbuffer_rust::secret_proof_transaction_builder::SecretProofTransactionBuilder;

const VECTOR_TRANSACTION_BUILDER: [&str; 2] = [
    "BF000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001905242000000000000000001000000000000009022D04812D05000F96C283657B0C17990932BC84926CDE63FC8BA10229AB5778D05D9C4B7F56676A88BF9295C185ACFC0F961DB5408CAFE0400009A493664",
    "DB000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001A8524200000000000000002B0200000000000080D66C33420E5411995BACFCA2B28CF1C9F5DD7AB1A9C05C9B3155B37159DA50AA52D5967C509B410F5A36A3B1E31ECB5AC76675D79B4A5E200000B778A39A3663719DFC5E48C9D78431B1E45C2AF9DF538782BF199C189DABEAC7",
];

#[test]
fn test_should_create_transactions() {
    for vector in VECTOR_TRANSACTION_BUILDER.iter() {
        let bytes_vector = hex::decode(vector).unwrap();
        let build_object = SecretProofTransactionBuilder::from_binary(&*bytes_vector);
        assert_eq!(build_object.serializer(), bytes_vector);
    }
}