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
use super::generator_utils::*;
use super::hash256_dto::*;
use super::key_dto::*;
use super::lock_hash_algorithm_dto::*;
use super::network_type_dto::*;
use super::secret_lock_transaction_body_builder::*;
use super::signature_dto::*;
use super::timestamp_dto::*;
use super::transaction_builder::*;
use super::unresolved_address_dto::*;
use super::unresolved_mosaic_builder::*;

/// Binary layout for a non-embedded secret lock transaction.
#[derive(Debug, Clone)]
pub struct SecretLockTransactionBuilder {
    /// Transaction.
    pub super_object: TransactionBuilder,
    /// Secret lock transaction body.
    pub body: SecretLockTransactionBodyBuilder,
}

impl SecretLockTransactionBuilder {
    const VERSION: u8 = 1;
    const ENTITY_TYPE: u16 = 0x4152;


    /// Creates an instance of SecretLockTransactionBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A SecretLockTransactionBuilder.
    pub fn from_binary(payload: &[u8]) -> Self {
        let mut _bytes = payload.to_vec();
        let super_object = TransactionBuilder::from_binary(&_bytes);
        assert_eq!(Self::VERSION, super_object.version, "Invalid entity version ({})", super_object.version);
        assert_eq!(Self::ENTITY_TYPE, super_object._type.get_value(), "Invalid entity type ({:?})", super_object._type);
        let mut _bytes = _bytes[super_object.get_size()..].to_vec();
        let secret_lock_transaction_body = SecretLockTransactionBodyBuilder::from_binary(&_bytes); // kind:CUSTOM1
        _bytes = _bytes[secret_lock_transaction_body.get_size()..].to_vec();
        // create object and call.
        SecretLockTransactionBuilder { super_object, body: secret_lock_transaction_body }  // Transaction
    }


    pub fn get_recipient_address(&self) -> UnresolvedAddressDto {
        self.body.recipient_address.clone()
    }
    pub fn set_recipient_address(&mut self, recipient_address: UnresolvedAddressDto) {
        self.body.recipient_address = recipient_address;   // MARKER1 AttributeKind.CUSTOM
    }


    pub fn get_secret(&self) -> Hash256Dto {
        self.body.secret.clone()
    }
    pub fn set_secret(&mut self, secret: Hash256Dto) {
        self.body.secret = secret;   // MARKER1 AttributeKind.CUSTOM
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


    pub fn get_hash_algorithm(&self) -> LockHashAlgorithmDto {
        self.body.hash_algorithm.clone()
    }
    pub fn set_hash_algorithm(&mut self, hash_algorithm: LockHashAlgorithmDto) {
        self.body.hash_algorithm = hash_algorithm;   // MARKER1 AttributeKind.CUSTOM
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
        buf.append(&mut (self.get_size() as u32).to_le_bytes().to_vec());
        buf.append(&mut self.super_object.serializer());
        buf.append(&mut self.body.serializer()); // kind:CUSTOM TransactionBody
        buf
    }
}

