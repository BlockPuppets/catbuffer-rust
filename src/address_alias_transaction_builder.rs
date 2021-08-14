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

use super::address_alias_transaction_body_builder::*;
use super::address_dto::*;
use super::alias_action_dto::*;
use super::namespace_id_dto::*;
use super::transaction_builder::*;

/// Binary layout for a non-embedded address alias transaction.
#[derive(Debug, Clone)]
pub struct AddressAliasTransactionBuilder {
    /// Transaction.
    pub super_object: TransactionBuilder,
    /// Address alias transaction body.
    pub body: AddressAliasTransactionBodyBuilder,
}

impl AddressAliasTransactionBuilder {
    const VERSION: u8 = 1;
    const ENTITY_TYPE: u16 = 0x424e;


    /// Creates an instance of AddressAliasTransactionBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A AddressAliasTransactionBuilder.
    pub fn from_binary(payload: &[u8]) -> Self {
        let mut bytes_ = payload.to_vec();
        let super_object = TransactionBuilder::from_binary(&bytes_);
        assert_eq!(Self::VERSION, super_object.version, "Invalid entity version ({})", super_object.version);
        assert_eq!(Self::ENTITY_TYPE, super_object._type.get_value(), "Invalid entity type ({:?})", super_object._type);
        let mut bytes_ = bytes_[super_object.get_size()..].to_vec();
        let address_alias_transaction_body = AddressAliasTransactionBodyBuilder::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[address_alias_transaction_body.get_size()..].to_vec();
        // create object and call.
        AddressAliasTransactionBuilder { super_object, body: address_alias_transaction_body }  // Transaction
    }


    pub fn get_namespace_id(&self) -> NamespaceIdDto {
        self.body.namespace_id.clone()
    }

    pub fn set_namespace_id(&mut self, namespace_id: NamespaceIdDto) {
        self.body.namespace_id = namespace_id;   // MARKER1 AttributeKind.CUSTOM
    }


    pub fn get_address(&self) -> AddressDto {
        self.body.address.clone()
    }

    pub fn set_address(&mut self, address: AddressDto) {
        self.body.address = address;   // MARKER1 AttributeKind.CUSTOM
    }


    pub fn get_alias_action(&self) -> AliasActionDto {
        self.body.alias_action.clone()
    }

    pub fn set_alias_action(&mut self, alias_action: AliasActionDto) {
        self.body.alias_action = alias_action;   // MARKER1 AttributeKind.CUSTOM
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

