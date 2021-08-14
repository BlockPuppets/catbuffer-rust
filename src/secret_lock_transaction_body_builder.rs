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
use super::generator_utils::*;
use super::hash256_dto::*;
use super::lock_hash_algorithm_dto::*;
use super::unresolved_address_dto::*;
use super::unresolved_mosaic_builder::*;

/// Binary layout for a secret lock transaction.
#[derive(Debug, Clone)]
pub struct SecretLockTransactionBodyBuilder {
    /// Locked mosaic recipient address.
    pub recipient_address: UnresolvedAddressDto,
    /// Secret.
    pub secret: Hash256Dto,
    /// Locked mosaic.
    pub mosaic: UnresolvedMosaicBuilder,
    /// Number of blocks for which a lock should be valid.
    pub duration: BlockDurationDto,
    /// Hash algorithm.
    pub hash_algorithm: LockHashAlgorithmDto,
}

impl SecretLockTransactionBodyBuilder {
    /// Creates an instance of SecretLockTransactionBodyBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A SecretLockTransactionBodyBuilder.
    pub fn from_binary(payload: &[u8]) -> Self {
        let mut bytes_ = payload.to_vec();
        let recipient_address = UnresolvedAddressDto::from_binary(&bytes_); // kind:CUSTOM1
        bytes_ = bytes_[recipient_address.get_size()..].to_vec();
        let secret = Hash256Dto::from_binary(&bytes_); // kind:CUSTOM1
        bytes_ = bytes_[secret.get_size()..].to_vec();
        let mosaic = UnresolvedMosaicBuilder::from_binary(&bytes_); // kind:CUSTOM1
        bytes_ = bytes_[mosaic.get_size()..].to_vec();
        let duration = BlockDurationDto::from_binary(&bytes_); // kind:CUSTOM1
        bytes_ = bytes_[duration.get_size()..].to_vec();
        let hash_algorithm = LockHashAlgorithmDto::from_binary(&bytes_); // kind:CUSTOM2
        bytes_ = (&bytes_[hash_algorithm.get_size()..]).to_vec();
        // create object and call.
        SecretLockTransactionBodyBuilder { recipient_address, secret, mosaic, duration, hash_algorithm } // TransactionBody
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += self.recipient_address.get_size(); // recipient_address_size;
        size += self.secret.get_size(); // secret_size;
        size += self.mosaic.get_size(); // mosaic_size;
        size += self.duration.get_size(); // duration_size;
        size += self.hash_algorithm.get_size(); // hash_algorithm_size;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.recipient_address.serializer()); // kind:CUSTOM
        buf.append(&mut self.secret.serializer()); // kind:CUSTOM
        buf.append(&mut self.mosaic.serializer()); // kind:CUSTOM
        buf.append(&mut self.duration.serializer()); // kind:CUSTOM
        buf.append(&mut self.hash_algorithm.serializer()); // kind:CUSTOM
        buf
    }
}

