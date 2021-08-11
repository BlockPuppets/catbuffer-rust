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

use super::finalization_round_builder::*;
use super::hash256_dto::*;
use super::height_dto::*;

/// Binary layout for finalized block header.
#[derive(Debug, Clone)]
pub struct FinalizedBlockHeaderBuilder {
    /// Finalization round.
    round: FinalizationRoundBuilder,
    /// Finalization height.
    height: HeightDto,
    /// Finalization hash.
    hash: Hash256Dto,
}


impl FinalizedBlockHeaderBuilder {

    /// Creates an instance of FinalizedBlockHeaderBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A FinalizedBlockHeaderBuilder.
    pub fn from_binary(bytes_: &[u8]) -> Self {
        let round = FinalizationRoundBuilder::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[round.get_size()..].to_vec();
        let height = HeightDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[height.get_size()..].to_vec();
        let hash = Hash256Dto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[hash.get_size()..].to_vec();
        FinalizedBlockHeaderBuilder{round, height, hash}
    }

    /// Gets finalization round.
    ///
    /// # Returns
    /// A Finalization round.
    pub fn get_round(&self) -> FinalizationRoundBuilder {
        self.round.clone()
    }

    /// Gets finalization height.
    ///
    /// # Returns
    /// A Finalization height.
    pub fn get_height(&self) -> HeightDto {
        self.height.clone()
    }

    /// Gets finalization hash.
    ///
    /// # Returns
    /// A Finalization hash.
    pub fn get_hash(&self) -> Hash256Dto {
        self.hash.clone()
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += self.round.get_size();
        size += self.height.get_size();
        size += self.hash.get_size();
        size
   }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.round.serializer()); // kind:CUSTOM
        buf.append(&mut self.height.serializer()); // kind:CUSTOM
        buf.append(&mut self.hash.serializer()); // kind:CUSTOM
        buf
    }
}

