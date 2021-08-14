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
use super::finalization_point_dto::*;
use super::generator_utils::*;

/// Binary layout for finalization round.
#[derive(Debug, Clone)]
pub struct FinalizationRoundBuilder {
    /// Finalization epoch.
    epoch: FinalizationEpochDto,
    /// Finalization point.
    point: FinalizationPointDto,
}


impl FinalizationRoundBuilder {
    /// Creates an instance of FinalizationRoundBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A FinalizationRoundBuilder.
    pub fn from_binary(bytes_: &[u8]) -> Self {
        let epoch = FinalizationEpochDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[epoch.get_size()..].to_vec();
        let point = FinalizationPointDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[point.get_size()..].to_vec();
        FinalizationRoundBuilder { epoch, point }
    }

    /// Gets finalization epoch.
    ///
    /// # Returns
    /// A Finalization epoch.
    pub fn get_epoch(&self) -> FinalizationEpochDto {
        self.epoch.clone()
    }

    /// Gets finalization point.
    ///
    /// # Returns
    /// A Finalization point.
    pub fn get_point(&self) -> FinalizationPointDto {
        self.point.clone()
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += self.epoch.get_size(); // epoch;
        size += self.point.get_size(); // point;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.epoch.serializer()); // kind:CUSTOM
        buf.append(&mut self.point.serializer()); // kind:CUSTOM
        buf
    }
}

