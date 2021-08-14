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
use super::key_dto::*;
use super::mosaic_definition_transaction_body_builder::*;
use super::mosaic_flags_dto::*;
use super::mosaic_id_dto::*;
use super::mosaic_nonce_dto::*;
use super::network_type_dto::*;
use super::signature_dto::*;
use super::timestamp_dto::*;
use super::transaction_builder::*;

/// Binary layout for a non-embedded mosaic definition transaction.
#[derive(Debug, Clone)]
pub struct MosaicDefinitionTransactionBuilder {
    /// Transaction.
    pub super_object: TransactionBuilder,
    /// Mosaic definition transaction body.
    pub body: MosaicDefinitionTransactionBodyBuilder,
}

impl MosaicDefinitionTransactionBuilder {
    const VERSION: u8 = 1;
    const ENTITY_TYPE: u16 = 0x414d;


    /// Creates an instance of MosaicDefinitionTransactionBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A MosaicDefinitionTransactionBuilder.
    pub fn from_binary(payload: &[u8]) -> Self {
        let mut bytes_ = payload.to_vec();
        let super_object = TransactionBuilder::from_binary(&bytes_);
        assert_eq!(Self::VERSION, super_object.version, "Invalid entity version ({})", super_object.version);
        assert_eq!(Self::ENTITY_TYPE, super_object._type.get_value(), "Invalid entity type ({:?})", super_object._type);
        let mut bytes_ = bytes_[super_object.get_size()..].to_vec();
        let mosaic_definition_transaction_body = MosaicDefinitionTransactionBodyBuilder::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[mosaic_definition_transaction_body.get_size()..].to_vec();
        // create object and call.
        MosaicDefinitionTransactionBuilder { super_object, body: mosaic_definition_transaction_body }  // Transaction
    }


    pub fn get_id(&self) -> MosaicIdDto {
        self.body.id.clone()
    }

    pub fn set_id(&mut self, id: MosaicIdDto) {
        self.body.id = id;   // MARKER1 AttributeKind.CUSTOM
    }


    pub fn get_duration(&self) -> BlockDurationDto {
        self.body.duration.clone()
    }

    pub fn set_duration(&mut self, duration: BlockDurationDto) {
        self.body.duration = duration;   // MARKER1 AttributeKind.CUSTOM
    }


    pub fn get_nonce(&self) -> MosaicNonceDto {
        self.body.nonce.clone()
    }

    pub fn set_nonce(&mut self, nonce: MosaicNonceDto) {
        self.body.nonce = nonce;   // MARKER1 AttributeKind.CUSTOM
    }


    pub fn get_flags(&self) -> Vec<MosaicFlagsDto> {
        self.body.flags.clone()
    }

    pub fn set_flags(&mut self, flags: Vec<MosaicFlagsDto>) {
        self.body.flags = flags;   // MARKER1 AttributeKind.FLAGS
    }


    pub fn get_divisibility(&self) -> u8 {
        self.body.divisibility.clone()
    }

    pub fn set_divisibility(&mut self, divisibility: u8) {
        self.body.divisibility = divisibility;   // MARKER1 AttributeKind.SIMPLE
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

