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

use super::amount_dto::*;
use super::block_duration_dto::*;
use super::entity_type_dto::*;
use super::hash256_dto::*;
use super::hash_lock_transaction_body_builder::*;
use super::key_dto::*;
use super::network_type_dto::*;
use super::signature_dto::*;
use super::timestamp_dto::*;
use super::transaction_builder::*;
use super::unresolved_mosaic_builder::*;

/// Binary layout for a non-embedded hash lock transaction.
#[derive(Debug, Clone)]
pub struct HashLockTransactionBuilder {
    /// Transaction.
    pub super_object: TransactionBuilder,
    /// Hash lock transaction body.
    pub body: HashLockTransactionBodyBuilder,
}

impl HashLockTransactionBuilder {
    const VERSION: u8 = 1;
    const ENTITY_TYPE: u16 = 0x4148;



    /// Creates an instance of HashLockTransactionBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A HashLockTransactionBuilder.
    pub fn from_binary(payload: &[u8]) -> Self {
        let mut bytes_ = payload.to_vec();
        let super_object = TransactionBuilder::from_binary(&bytes_);
        assert_eq!( Self::VERSION, super_object.version, "Invalid entity version ({})", super_object.version);
        assert_eq!( Self::ENTITY_TYPE, super_object._type.get_value(), "Invalid entity type ({:?})", super_object._type);
        let mut bytes_ = bytes_[super_object.get_size()..].to_vec();
        let hash_lock_transaction_body = HashLockTransactionBodyBuilder::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[hash_lock_transaction_body.get_size()..].to_vec();
        // create object and call.
        HashLockTransactionBuilder{ super_object, body: hash_lock_transaction_body }  // Transaction
    }


    pub fn get_mosaic(&self) -> UnresolvedMosaicBuilder {
        self.body.mosaic.clone()
    }

    pub fn set_mosaic(&mut self, mosaic: UnresolvedMosaicBuilder) {
        self.body.mosaic = mosaic;   // MARKER1 AttributeKind.CUSTOM
    }


    pub fn get_duration(&self) -> BlockDurationDto {
        self.body.duration.clone()
    }

    pub fn set_duration(&mut self, duration: BlockDurationDto) {
        self.body.duration = duration;   // MARKER1 AttributeKind.CUSTOM
    }


    pub fn get_hash(&self) -> Hash256Dto {
        self.body.hash.clone()
    }

    pub fn set_hash(&mut self, hash: Hash256Dto) {
        self.body.hash = hash;   // MARKER1 AttributeKind.CUSTOM
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
   pub fn get_size(&self) -> usize {
       let mut size = self.super_object.get_size();
        size += self.body.get_size();
        size
   }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.super_object.serializer());
        buf.append(&mut self.body.serializer()); // kind:CUSTOM TransactionBody
        buf
    }
}

