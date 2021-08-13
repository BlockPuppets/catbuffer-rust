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

use super::block_duration_dto::*;
use super::hash256_dto::*;
use super::unresolved_mosaic_builder::*;

/// Binary layout for a hash lock transaction.
#[derive(Debug, Clone)]
pub struct HashLockTransactionBodyBuilder {
    /// Lock mosaic.
    pub mosaic: UnresolvedMosaicBuilder,
    /// Number of blocks for which a lock should be valid.
    pub duration: BlockDurationDto,
    /// Lock hash.
    pub hash: Hash256Dto,
}

impl HashLockTransactionBodyBuilder {



    /// Creates an instance of HashLockTransactionBodyBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A HashLockTransactionBodyBuilder.
    pub fn from_binary(payload: &[u8]) -> Self {
        let mut bytes_ = payload.to_vec();
        let mosaic = UnresolvedMosaicBuilder::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[mosaic.get_size()..].to_vec();
        let duration = BlockDurationDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[duration.get_size()..].to_vec();
        let hash = Hash256Dto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[hash.get_size()..].to_vec();
        // create object and call.
        HashLockTransactionBodyBuilder{ mosaic, duration, hash } // TransactionBody
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
   pub fn get_size(&self) -> usize {
       let mut size = 0;
        size += self.mosaic.get_size(); // mosaic_size;
        size += self.duration.get_size(); // duration_size;
        size += self.hash.get_size(); // hash_size;
        size
   }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.mosaic.serializer()); // kind:CUSTOM
        buf.append(&mut self.duration.serializer()); // kind:CUSTOM
        buf.append(&mut self.hash.serializer()); // kind:CUSTOM
        buf
    }
}

