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
use super::key_dto::*;
use super::mosaic_address_restriction_transaction_body_builder::*;
use super::network_type_dto::*;
use super::signature_dto::*;
use super::timestamp_dto::*;
use super::transaction_builder::*;
use super::unresolved_address_dto::*;
use super::unresolved_mosaic_id_dto::*;

/// Binary layout for a non-embedded mosaic address restriction transaction.
#[derive(Debug, Clone)]
pub struct MosaicAddressRestrictionTransactionBuilder {
    /// Transaction.
    pub super_object: TransactionBuilder,
    /// Mosaic address restriction transaction body.
    pub body: MosaicAddressRestrictionTransactionBodyBuilder,
}

impl MosaicAddressRestrictionTransactionBuilder {
    const VERSION: u8 = 1;
    const ENTITY_TYPE: u16 = 0x4251;



    /// Creates an instance of MosaicAddressRestrictionTransactionBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A MosaicAddressRestrictionTransactionBuilder.
    pub fn from_binary(payload: &[u8]) -> Self {
        let mut bytes_ = payload.to_vec();
        let super_object = TransactionBuilder::from_binary(&bytes_);
        assert_eq!( Self::VERSION, super_object.version, "Invalid entity version ({})", super_object.version);
        assert_eq!( Self::ENTITY_TYPE, super_object._type.get_value(), "Invalid entity type ({:?})", super_object._type);
        let mut bytes_ = bytes_[super_object.get_size()..].to_vec();
        let mosaic_address_restriction_transaction_body = MosaicAddressRestrictionTransactionBodyBuilder::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[mosaic_address_restriction_transaction_body.get_size()..].to_vec();
        // create object and call.
        MosaicAddressRestrictionTransactionBuilder{ super_object, body: mosaic_address_restriction_transaction_body }  // Transaction
    }


    pub fn get_mosaic_id(&self) -> UnresolvedMosaicIdDto {
        self.body.mosaic_id.clone()
    }

    pub fn set_mosaic_id(&mut self, mosaic_id: UnresolvedMosaicIdDto) {
        self.body.mosaic_id = mosaic_id;   // MARKER1 AttributeKind.CUSTOM
    }


    pub fn get_restriction_key(&self) -> u64 {
        self.body.restriction_key.clone()
    }

    pub fn set_restriction_key(&mut self, restriction_key: u64) {
        self.body.restriction_key = restriction_key;   // MARKER1 AttributeKind.SIMPLE
    }


    pub fn get_previous_restriction_value(&self) -> u64 {
        self.body.previous_restriction_value.clone()
    }

    pub fn set_previous_restriction_value(&mut self, previous_restriction_value: u64) {
        self.body.previous_restriction_value = previous_restriction_value;   // MARKER1 AttributeKind.SIMPLE
    }


    pub fn get_new_restriction_value(&self) -> u64 {
        self.body.new_restriction_value.clone()
    }

    pub fn set_new_restriction_value(&mut self, new_restriction_value: u64) {
        self.body.new_restriction_value = new_restriction_value;   // MARKER1 AttributeKind.SIMPLE
    }


    pub fn get_target_address(&self) -> UnresolvedAddressDto {
        self.body.target_address.clone()
    }

    pub fn set_target_address(&mut self, target_address: UnresolvedAddressDto) {
        self.body.target_address = target_address;   // MARKER1 AttributeKind.CUSTOM
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

