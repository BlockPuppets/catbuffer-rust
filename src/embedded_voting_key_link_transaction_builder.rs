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

use super::embedded_transaction_builder::*;
use super::finalization_epoch_dto::*;
use super::link_action_dto::*;
use super::voting_key_dto::*;
use super::voting_key_link_transaction_body_builder::*;

/// Binary layout for an embedded voting key link transaction.
#[derive(Debug, Clone)]
pub struct EmbeddedVotingKeyLinkTransactionBuilder {
    /// Embedded transaction.
    pub super_object: EmbeddedTransactionBuilder,
    /// Voting key link transaction body.
    pub body: VotingKeyLinkTransactionBodyBuilder,
}

impl EmbeddedVotingKeyLinkTransactionBuilder {
    const VERSION: u8 = 1;
    const ENTITY_TYPE: u16 = 0x4143;


    /// Creates an instance of EmbeddedVotingKeyLinkTransactionBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A EmbeddedVotingKeyLinkTransactionBuilder.
    pub fn from_binary(payload: &[u8]) -> Self {
        let mut bytes_ = payload.to_vec();
        let super_object = EmbeddedTransactionBuilder::from_binary(&bytes_);
        assert_eq!(Self::VERSION, super_object.version, "Invalid entity version ({})", super_object.version);
        assert_eq!(Self::ENTITY_TYPE, super_object._type.get_value(), "Invalid entity type ({:?})", super_object._type);
        let mut bytes_ = bytes_[super_object.get_size()..].to_vec();
        let voting_key_link_transaction_body = VotingKeyLinkTransactionBodyBuilder::from_binary(&bytes_); // kind:CUSTOM1
        bytes_ = bytes_[voting_key_link_transaction_body.get_size()..].to_vec();
        // create object and call.
        EmbeddedVotingKeyLinkTransactionBuilder { super_object, body: voting_key_link_transaction_body }  // Transaction
        // nothing needed to copy into EmbeddedTransaction
    }


    pub fn get_linked_public_key(&self) -> VotingKeyDto {
        self.body.linked_public_key.clone()
    }

    pub fn set_linked_public_key(&mut self, linked_public_key: VotingKeyDto) {
        self.body.linked_public_key = linked_public_key;   // MARKER1 AttributeKind.CUSTOM
    }


    pub fn get_start_epoch(&self) -> FinalizationEpochDto {
        self.body.start_epoch.clone()
    }

    pub fn set_start_epoch(&mut self, start_epoch: FinalizationEpochDto) {
        self.body.start_epoch = start_epoch;   // MARKER1 AttributeKind.CUSTOM
    }


    pub fn get_end_epoch(&self) -> FinalizationEpochDto {
        self.body.end_epoch.clone()
    }

    pub fn set_end_epoch(&mut self, end_epoch: FinalizationEpochDto) {
        self.body.end_epoch = end_epoch;   // MARKER1 AttributeKind.CUSTOM
    }


    pub fn get_link_action(&self) -> LinkActionDto {
        self.body.link_action.clone()
    }

    pub fn set_link_action(&mut self, link_action: LinkActionDto) {
        self.body.link_action = link_action;   // MARKER1 AttributeKind.CUSTOM
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

