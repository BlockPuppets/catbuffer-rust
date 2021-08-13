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
use super::multisig_account_modification_transaction_body_builder::*;
use super::network_type_dto::*;
use super::signature_dto::*;
use super::timestamp_dto::*;
use super::transaction_builder::*;
use super::unresolved_address_dto::*;

/// Binary layout for a non-embedded multisig account modification transaction.
#[derive(Debug, Clone)]
pub struct MultisigAccountModificationTransactionBuilder {
    /// Transaction.
    pub super_object: TransactionBuilder,
    /// Multisig account modification transaction body.
    pub body: MultisigAccountModificationTransactionBodyBuilder,
}

impl MultisigAccountModificationTransactionBuilder {
    const VERSION: u8 = 1;
    const ENTITY_TYPE: u16 = 0x4155;



    /// Creates an instance of MultisigAccountModificationTransactionBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A MultisigAccountModificationTransactionBuilder.
    pub fn from_binary(payload: &[u8]) -> Self {
        let mut bytes_ = payload.to_vec();
        let super_object = TransactionBuilder::from_binary(&bytes_);
        assert_eq!( Self::VERSION, super_object.version, "Invalid entity version ({})", super_object.version);
        assert_eq!( Self::ENTITY_TYPE, super_object._type.get_value(), "Invalid entity type ({:?})", super_object._type);
        let mut bytes_ = bytes_[super_object.get_size()..].to_vec();
        let multisig_account_modification_transaction_body = MultisigAccountModificationTransactionBodyBuilder::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[multisig_account_modification_transaction_body.get_size()..].to_vec();
        // create object and call.
        MultisigAccountModificationTransactionBuilder{ super_object, body: multisig_account_modification_transaction_body }  // Transaction
    }


    pub fn get_min_removal_delta(&self) -> u8 {
        self.body.min_removal_delta.clone()
    }

    pub fn set_min_removal_delta(&mut self, min_removal_delta: u8) {
        self.body.min_removal_delta = min_removal_delta;   // MARKER1 AttributeKind.SIMPLE
    }


    pub fn get_min_approval_delta(&self) -> u8 {
        self.body.min_approval_delta.clone()
    }

    pub fn set_min_approval_delta(&mut self, min_approval_delta: u8) {
        self.body.min_approval_delta = min_approval_delta;   // MARKER1 AttributeKind.SIMPLE
    }


    pub fn get_address_additions(&self) -> Vec<UnresolvedAddressDto> {
        self.body.address_additions.clone()
    }


    pub fn get_address_deletions(&self) -> Vec<UnresolvedAddressDto> {
        self.body.address_deletions.clone()
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

