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

use super::finalization_epoch_dto::*;
use super::generator_utils::*;
use super::link_action_dto::*;
use super::voting_key_dto::*;

/// Binary layout for a voting key link transaction.
#[derive(Debug, Clone)]
pub struct VotingKeyLinkTransactionBodyBuilder {
    /// Linked public key.
    pub linked_public_key: VotingKeyDto,
    /// Start finalization epoch.
    pub start_epoch: FinalizationEpochDto,
    /// End finalization epoch.
    pub end_epoch: FinalizationEpochDto,
    /// Link action.
    pub link_action: LinkActionDto,
}

impl VotingKeyLinkTransactionBodyBuilder {
    /// Creates an instance of VotingKeyLinkTransactionBodyBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A VotingKeyLinkTransactionBodyBuilder.
    pub fn from_binary(payload: &[u8]) -> Self {
        let mut bytes_ = payload.to_vec();
        let linked_public_key = VotingKeyDto::from_binary(&bytes_); // kind:CUSTOM1
        bytes_ = bytes_[linked_public_key.get_size()..].to_vec();
        let start_epoch = FinalizationEpochDto::from_binary(&bytes_); // kind:CUSTOM1
        bytes_ = bytes_[start_epoch.get_size()..].to_vec();
        let end_epoch = FinalizationEpochDto::from_binary(&bytes_); // kind:CUSTOM1
        bytes_ = bytes_[end_epoch.get_size()..].to_vec();
        let link_action = LinkActionDto::from_binary(&bytes_); // kind:CUSTOM2
        bytes_ = (&bytes_[link_action.get_size()..]).to_vec();
        // create object and call.
        VotingKeyLinkTransactionBodyBuilder { linked_public_key, start_epoch, end_epoch, link_action } // TransactionBody
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += self.linked_public_key.get_size(); // linked_public_key_size;
        size += self.start_epoch.get_size(); // start_epoch_size;
        size += self.end_epoch.get_size(); // end_epoch_size;
        size += self.link_action.get_size(); // link_action_size;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.linked_public_key.serializer()); // kind:CUSTOM
        buf.append(&mut self.start_epoch.serializer()); // kind:CUSTOM
        buf.append(&mut self.end_epoch.serializer()); // kind:CUSTOM
        buf.append(&mut self.link_action.serializer()); // kind:CUSTOM
        buf
    }
}

