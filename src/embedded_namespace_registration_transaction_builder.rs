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
use super::embedded_transaction_builder::*;
use super::entity_type_dto::*;
use super::key_dto::*;
use super::namespace_id_dto::*;
use super::namespace_registration_transaction_body_builder::*;
use super::namespace_registration_type_dto::*;
use super::network_type_dto::*;

/// Binary layout for an embedded namespace registration transaction.
#[derive(Debug, Clone)]
pub struct EmbeddedNamespaceRegistrationTransactionBuilder {
    /// Embedded transaction.
    pub super_object: EmbeddedTransactionBuilder,
    /// Namespace registration transaction body.
    pub body: NamespaceRegistrationTransactionBodyBuilder,
}

impl EmbeddedNamespaceRegistrationTransactionBuilder {
    const VERSION: u8 = 1;
    const ENTITY_TYPE: u16 = 0x414e;



    /// Creates an instance of EmbeddedNamespaceRegistrationTransactionBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A EmbeddedNamespaceRegistrationTransactionBuilder.
    pub fn from_binary(payload: &[u8]) -> Self {
        let mut bytes_ = payload.to_vec();
        let super_object = EmbeddedTransactionBuilder::from_binary(&bytes_);
        assert_eq!( Self::VERSION, super_object.version, "Invalid entity version ({})", super_object.version);
        assert_eq!( Self::ENTITY_TYPE, super_object._type.get_value(), "Invalid entity type ({:?})", super_object._type);
        let mut bytes_ = bytes_[super_object.get_size()..].to_vec();
        let namespace_registration_transaction_body = NamespaceRegistrationTransactionBodyBuilder::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[namespace_registration_transaction_body.get_size()..].to_vec();
        // create object and call.
        EmbeddedNamespaceRegistrationTransactionBuilder{ super_object, body: namespace_registration_transaction_body }  // Transaction
        // nothing needed to copy into EmbeddedTransaction
    }


    pub fn get_duration(&self) -> Option<BlockDurationDto> {
        self.body.duration.clone()
    }

    pub fn set_duration(&mut self, duration: BlockDurationDto) {
        self.body.duration = Some(duration);   // MARKER1 AttributeKind.CUSTOM
    }


    pub fn get_parent_id(&self) -> Option<NamespaceIdDto> {
        self.body.parent_id.clone()
    }

    pub fn set_parent_id(&mut self, parent_id: NamespaceIdDto) {
        self.body.parent_id = Some(parent_id);   // MARKER1 AttributeKind.CUSTOM
    }


    pub fn get_id(&self) -> NamespaceIdDto {
        self.body.id.clone()
    }

    pub fn set_id(&mut self, id: NamespaceIdDto) {
        self.body.id = id;   // MARKER1 AttributeKind.CUSTOM
    }


    pub fn get_registration_type(&self) -> NamespaceRegistrationTypeDto {
        self.body.registration_type.clone()
    }

    pub fn set_registration_type(&mut self, registration_type: NamespaceRegistrationTypeDto) {
        self.body.registration_type = registration_type;   // MARKER1 AttributeKind.CUSTOM
    }


    pub fn get_name(&self) -> Vec<u8> {
        self.body.name.clone()
    }

    pub fn set_name(&mut self, name: Vec<u8>) {
        self.body.name = name;   // MARKER1 AttributeKind.BUFFER
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

