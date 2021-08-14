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

use super::account_operation_restriction_transaction_body_builder::*;
use super::account_restriction_flags_dto::*;
use super::embedded_transaction_builder::*;
use super::entity_type_dto::*;

/// Binary layout for an embedded account operation restriction transaction.
#[derive(Debug, Clone)]
pub struct EmbeddedAccountOperationRestrictionTransactionBuilder {
    /// Embedded transaction.
    pub super_object: EmbeddedTransactionBuilder,
    /// Account operation restriction transaction body.
    pub body: AccountOperationRestrictionTransactionBodyBuilder,
}

impl EmbeddedAccountOperationRestrictionTransactionBuilder {
    const VERSION: u8 = 1;
    const ENTITY_TYPE: u16 = 0x4350;


    /// Creates an instance of EmbeddedAccountOperationRestrictionTransactionBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A EmbeddedAccountOperationRestrictionTransactionBuilder.
    pub fn from_binary(payload: &[u8]) -> Self {
        let mut bytes_ = payload.to_vec();
        let super_object = EmbeddedTransactionBuilder::from_binary(&bytes_);
        assert_eq!(Self::VERSION, super_object.version, "Invalid entity version ({})", super_object.version);
        assert_eq!(Self::ENTITY_TYPE, super_object._type.get_value(), "Invalid entity type ({:?})", super_object._type);
        let mut bytes_ = bytes_[super_object.get_size()..].to_vec();
        let account_operation_restriction_transaction_body = AccountOperationRestrictionTransactionBodyBuilder::from_binary(&bytes_); // kind:CUSTOM1
        bytes_ = bytes_[account_operation_restriction_transaction_body.get_size()..].to_vec();
        // create object and call.
        EmbeddedAccountOperationRestrictionTransactionBuilder { super_object, body: account_operation_restriction_transaction_body }  // Transaction
        // nothing needed to copy into EmbeddedTransaction
    }


    pub fn get_restriction_flags(&self) -> Vec<AccountRestrictionFlagsDto> {
        self.body.restriction_flags.clone()
    }

    pub fn set_restriction_flags(&mut self, restriction_flags: Vec<AccountRestrictionFlagsDto>) {
        self.body.restriction_flags = restriction_flags;   // MARKER1 AttributeKind.FLAGS
    }


    pub fn get_restriction_additions(&self) -> Vec<EntityTypeDto> {
        self.body.restriction_additions.clone()
    }


    pub fn get_restriction_deletions(&self) -> Vec<EntityTypeDto> {
        self.body.restriction_deletions.clone()
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

