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
use super::voting_key_dto::*;

/// Pinned voting key.
#[derive(Debug, Clone)]
pub struct PinnedVotingKeyBuilder {
    /// Voting key.
    voting_key: VotingKeyDto,
    /// Start finalization epoch.
    start_epoch: FinalizationEpochDto,
    /// End finalization epoch.
    end_epoch: FinalizationEpochDto,
}


impl PinnedVotingKeyBuilder {
    /// Creates an instance of PinnedVotingKeyBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A PinnedVotingKeyBuilder.
    pub fn from_binary(_bytes: &[u8]) -> Self {
        let voting_key = VotingKeyDto::from_binary(&_bytes); // kind:CUSTOM1
        let mut _bytes = _bytes[voting_key.get_size()..].to_vec();
        let start_epoch = FinalizationEpochDto::from_binary(&_bytes); // kind:CUSTOM1
        let mut _bytes = _bytes[start_epoch.get_size()..].to_vec();
        let end_epoch = FinalizationEpochDto::from_binary(&_bytes); // kind:CUSTOM1
        let mut _bytes = _bytes[end_epoch.get_size()..].to_vec();
        PinnedVotingKeyBuilder { voting_key, start_epoch, end_epoch }
    }

    /// Gets voting key.
    ///
    /// # Returns
    /// A Voting key.
    pub fn get_voting_key(&self) -> VotingKeyDto {
        self.voting_key.clone()
    }

    /// Gets start finalization epoch.
    ///
    /// # Returns
    /// A Start finalization epoch.
    pub fn get_start_epoch(&self) -> FinalizationEpochDto {
        self.start_epoch.clone()
    }

    /// Gets end finalization epoch.
    ///
    /// # Returns
    /// A End finalization epoch.
    pub fn get_end_epoch(&self) -> FinalizationEpochDto {
        self.end_epoch.clone()
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += self.voting_key.get_size(); // voting_key;
        size += self.start_epoch.get_size(); // start_epoch;
        size += self.end_epoch.get_size(); // end_epoch;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.voting_key.serializer()); // kind:CUSTOM
        buf.append(&mut self.start_epoch.serializer()); // kind:CUSTOM
        buf.append(&mut self.end_epoch.serializer()); // kind:CUSTOM
        buf
    }
}

