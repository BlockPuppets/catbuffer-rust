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
use super::generator_utils::*;
use super::key_dto::*;
use super::network_type_dto::*;
use super::signature_dto::*;
use super::timestamp_dto::*;

/// Binary layout for a transaction.
#[derive(Debug, Clone)]
pub struct TransactionBuilder {
    /// Entity signature.
    pub signature: SignatureDto,
    /// Entity signer's public key.
    pub signer_public_key: KeyDto,
    /// Entity version.
    pub version: u8,
    /// Entity network.
    pub network: NetworkTypeDto,
    /// Entity type.
    pub _type: EntityTypeDto,
    /// Transaction fee.
    pub fee: AmountDto,
    /// Transaction deadline.
    pub deadline: TimestampDto,
}

impl TransactionBuilder {
    /// Creates an instance of TransactionBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A TransactionBuilder.
    pub fn from_binary(payload: &[u8]) -> Self {
        let mut bytes_ = payload.to_vec();
        bytes_ = (&bytes_[4..]).to_vec();
        let buf = fixed_bytes::<4>(&bytes_);
        let verifiable_entity_header__reserved1 = u32::from_le_bytes(buf); // kind:SIMPLE
        bytes_ = (&bytes_[4..]).to_vec();
        let signature = SignatureDto::from_binary(&bytes_); // kind:CUSTOM1
        bytes_ = bytes_[signature.get_size()..].to_vec();
        let signer_public_key = KeyDto::from_binary(&bytes_); // kind:CUSTOM1
        bytes_ = bytes_[signer_public_key.get_size()..].to_vec();
        let buf = fixed_bytes::<4>(&bytes_);
        let entity_body__reserved1 = u32::from_le_bytes(buf); // kind:SIMPLE
        bytes_ = (&bytes_[4..]).to_vec();
        let buf = fixed_bytes::<1>(&bytes_);
        let version = u8::from_le_bytes(buf); // kind:SIMPLE
        bytes_ = (&bytes_[1..]).to_vec();
        let network = NetworkTypeDto::from_binary(&bytes_); // kind:CUSTOM2
        bytes_ = (&bytes_[network.get_size()..]).to_vec();
        let _type = EntityTypeDto::from_binary(&bytes_); // kind:CUSTOM2
        bytes_ = (&bytes_[_type.get_size()..]).to_vec();
        let fee = AmountDto::from_binary(&bytes_); // kind:CUSTOM1
        bytes_ = bytes_[fee.get_size()..].to_vec();
        let deadline = TimestampDto::from_binary(&bytes_); // kind:CUSTOM1
        bytes_ = bytes_[deadline.get_size()..].to_vec();
        // create object and call. // Transaction
        TransactionBuilder { signature, signer_public_key, version, network, _type, fee, deadline }
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += 4;  // size;
        size += 4;  // verifiable_entity_header__reserved1;
        size += self.signature.get_size(); // signature_size;
        size += self.signer_public_key.get_size(); // signer_public_key_size;
        size += 4;  // entity_body__reserved1;
        size += 1;  // version;
        size += self.network.get_size(); // network_size;
        size += self._type.get_size(); // type_size;
        size += self.fee.get_size(); // fee_size;
        size += self.deadline.get_size(); // deadline_size;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        // Ignored serialization: size AttributeKind.SIMPLE
        buf.append(&mut [0u8; 4].to_vec()); // kind:SIMPLE and is_reserved
        buf.append(&mut self.signature.serializer()); // kind:CUSTOM
        buf.append(&mut self.signer_public_key.serializer()); // kind:CUSTOM
        buf.append(&mut [0u8; 4].to_vec()); // kind:SIMPLE and is_reserved
        buf.append(&mut self.version.to_le_bytes().to_vec()); // serial_kind:SIMPLE
        buf.append(&mut self.network.serializer()); // kind:CUSTOM
        buf.append(&mut self._type.serializer()); // kind:CUSTOM
        buf.append(&mut self.fee.serializer()); // kind:CUSTOM
        buf.append(&mut self.deadline.serializer()); // kind:CUSTOM
        buf
    }
}

