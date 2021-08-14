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
use super::embedded_transaction_builder::*;
use super::entity_type_dto::*;
use super::key_dto::*;
use super::mosaic_supply_change_action_dto::*;
use super::mosaic_supply_change_transaction_body_builder::*;
use super::network_type_dto::*;
use super::unresolved_mosaic_id_dto::*;

/// Binary layout for an embedded mosaic supply change transaction.
#[derive(Debug, Clone)]
pub struct EmbeddedMosaicSupplyChangeTransactionBuilder {
    /// Embedded transaction.
    pub super_object: EmbeddedTransactionBuilder,
    /// Mosaic supply change transaction body.
    pub body: MosaicSupplyChangeTransactionBodyBuilder,
}

impl EmbeddedMosaicSupplyChangeTransactionBuilder {
    const VERSION: u8 = 1;
    const ENTITY_TYPE: u16 = 0x424d;


    /// Creates an instance of EmbeddedMosaicSupplyChangeTransactionBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A EmbeddedMosaicSupplyChangeTransactionBuilder.
    pub fn from_binary(payload: &[u8]) -> Self {
        let mut bytes_ = payload.to_vec();
        let super_object = EmbeddedTransactionBuilder::from_binary(&bytes_);
        assert_eq!(Self::VERSION, super_object.version, "Invalid entity version ({})", super_object.version);
        assert_eq!(Self::ENTITY_TYPE, super_object._type.get_value(), "Invalid entity type ({:?})", super_object._type);
        let mut bytes_ = bytes_[super_object.get_size()..].to_vec();
        let mosaic_supply_change_transaction_body = MosaicSupplyChangeTransactionBodyBuilder::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[mosaic_supply_change_transaction_body.get_size()..].to_vec();
        // create object and call.
        EmbeddedMosaicSupplyChangeTransactionBuilder { super_object, body: mosaic_supply_change_transaction_body }  // Transaction
        // nothing needed to copy into EmbeddedTransaction
    }


    pub fn get_mosaic_id(&self) -> UnresolvedMosaicIdDto {
        self.body.mosaic_id.clone()
    }

    pub fn set_mosaic_id(&mut self, mosaic_id: UnresolvedMosaicIdDto) {
        self.body.mosaic_id = mosaic_id;   // MARKER1 AttributeKind.CUSTOM
    }


    pub fn get_delta(&self) -> AmountDto {
        self.body.delta.clone()
    }

    pub fn set_delta(&mut self, delta: AmountDto) {
        self.body.delta = delta;   // MARKER1 AttributeKind.CUSTOM
    }


    pub fn get_action(&self) -> MosaicSupplyChangeActionDto {
        self.body.action.clone()
    }

    pub fn set_action(&mut self, action: MosaicSupplyChangeActionDto) {
        self.body.action = action;   // MARKER1 AttributeKind.CUSTOM
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

