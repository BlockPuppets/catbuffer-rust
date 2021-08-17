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

use super::account_metadata_transaction_body_builder::*;
use super::embedded_transaction_builder::*;
use super::embedded_transaction_helper::*;
use super::entity_type_dto::*;
use super::generator_utils::*;
use super::key_dto::*;
use super::network_type_dto::*;
use super::unresolved_address_dto::*;

/// Binary layout for an embedded account metadata transaction.
#[derive(Debug, Clone)]
pub struct EmbeddedAccountMetadataTransactionBuilder {
    /// Embedded transaction.
    pub super_object: EmbeddedTransactionBuilder,
    /// Account metadata transaction body.
    pub body: AccountMetadataTransactionBodyBuilder,
}

impl EmbeddedAccountMetadataTransactionBuilder {
    const VERSION: u8 = 1;
    const ENTITY_TYPE: u16 = 0x4144;


    /// Creates an instance of EmbeddedAccountMetadataTransactionBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A EmbeddedAccountMetadataTransactionBuilder.
    pub fn from_binary(payload: &[u8]) -> Self {
        let mut _bytes = payload.to_vec();
        let super_object = EmbeddedTransactionBuilder::from_binary(&_bytes);
        assert_eq!(Self::VERSION, super_object.version, "Invalid entity version ({})", super_object.version);
        assert_eq!(Self::ENTITY_TYPE, super_object._type.get_value(), "Invalid entity type ({:?})", super_object._type);
        let mut _bytes = _bytes[super_object.get_size()..].to_vec();
        let account_metadata_transaction_body = AccountMetadataTransactionBodyBuilder::from_binary(&_bytes); // kind:CUSTOM1
        _bytes = _bytes[account_metadata_transaction_body.get_size()..].to_vec();
        // create object and call.
        EmbeddedAccountMetadataTransactionBuilder { super_object, body: account_metadata_transaction_body }  // Transaction
        // nothing needed to copy into EmbeddedTransaction
    }


    pub fn get_target_address(&self) -> UnresolvedAddressDto {
        self.body.target_address.clone()
    }
    pub fn set_target_address(&mut self, target_address: UnresolvedAddressDto) {
        self.body.target_address = target_address;   // MARKER1 AttributeKind.CUSTOM
    }


    pub fn get_scoped_metadata_key(&self) -> u64 {
        self.body.scoped_metadata_key.clone()
    }
    pub fn set_scoped_metadata_key(&mut self, scoped_metadata_key: u64) {
        self.body.scoped_metadata_key = scoped_metadata_key;   // MARKER1 AttributeKind.SIMPLE
    }


    pub fn get_value_size_delta(&self) -> u16 {
        self.body.value_size_delta.clone()
    }
    pub fn set_value_size_delta(&mut self, value_size_delta: u16) {
        self.body.value_size_delta = value_size_delta;   // MARKER1 AttributeKind.SIMPLE
    }


    pub fn get_value(&self) -> Vec<u8> {
        self.body.value.clone()
    }
    pub fn set_value(&mut self, value: Vec<u8>) {
        self.body.value = value;   // MARKER1 AttributeKind.BUFFER
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

impl EmbeddedTransactionHelper for EmbeddedAccountMetadataTransactionBuilder {
    fn box_clone(&self) -> Box<dyn EmbeddedTransactionHelper> {
        Box::new((*self).clone())
    }

    fn get_size(&self) -> usize {
        self.get_size()
    }

    fn serializer(&self) -> Vec<u8> {
        self.serializer()
    }
}

