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
use super::entity_type_dto::*;
use super::hash256_dto::*;
use super::key_dto::*;
use super::lock_hash_algorithm_dto::*;
use super::network_type_dto::*;
use super::secret_proof_transaction_body_builder::*;
use super::signature_dto::*;
use super::timestamp_dto::*;
use super::transaction_builder::*;
use super::unresolved_address_dto::*;

/// Binary layout for a non-embedded secret proof transaction.
#[derive(Debug, Clone)]
pub struct SecretProofTransactionBuilder {
    /// Transaction.
    pub super_object: TransactionBuilder,
    /// Secret proof transaction body.
    pub body: SecretProofTransactionBodyBuilder,
}

impl SecretProofTransactionBuilder {
    const VERSION: u8 = 1;
    const ENTITY_TYPE: u16 = 0x4252;


    /// Creates an instance of SecretProofTransactionBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A SecretProofTransactionBuilder.
    pub fn from_binary(payload: &[u8]) -> Self {
        let mut bytes_ = payload.to_vec();
        let super_object = TransactionBuilder::from_binary(&bytes_);
        assert_eq!(Self::VERSION, super_object.version, "Invalid entity version ({})", super_object.version);
        assert_eq!(Self::ENTITY_TYPE, super_object._type.get_value(), "Invalid entity type ({:?})", super_object._type);
        let mut bytes_ = bytes_[super_object.get_size()..].to_vec();
        let secret_proof_transaction_body = SecretProofTransactionBodyBuilder::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[secret_proof_transaction_body.get_size()..].to_vec();
        // create object and call.
        SecretProofTransactionBuilder { super_object, body: secret_proof_transaction_body }  // Transaction
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


    pub fn get_hash_algorithm(&self) -> LockHashAlgorithmDto {
        self.body.hash_algorithm.clone()
    }

    pub fn set_hash_algorithm(&mut self, hash_algorithm: LockHashAlgorithmDto) {
        self.body.hash_algorithm = hash_algorithm;   // MARKER1 AttributeKind.CUSTOM
    }


    pub fn get_proof(&self) -> Vec<u8> {
        self.body.proof.clone()
    }

    pub fn set_proof(&mut self, proof: Vec<u8>) {
        self.body.proof = proof;   // MARKER1 AttributeKind.BUFFER
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
        buf.append(&mut (self.get_size() as u32).to_le_bytes().to_vec()); // # serial_kind:SIMPLE
        buf.append(&mut self.super_object.serializer());
        buf.append(&mut self.body.serializer()); // kind:CUSTOM TransactionBody
        buf
    }
}

