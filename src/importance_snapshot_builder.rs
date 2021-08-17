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

use super::generator_utils::*;
use super::importance_dto::*;
use super::importance_height_dto::*;

/// Temporal importance information.
#[derive(Debug, Clone)]
pub struct ImportanceSnapshotBuilder {
    /// Account importance.
    importance: ImportanceDto,
    /// Importance height.
    height: ImportanceHeightDto,
}


impl ImportanceSnapshotBuilder {
    /// Creates an instance of ImportanceSnapshotBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A ImportanceSnapshotBuilder.
    pub fn from_binary(_bytes: &[u8]) -> Self {
        let importance = ImportanceDto::from_binary(&_bytes); // kind:CUSTOM1
        let mut _bytes = _bytes[importance.get_size()..].to_vec();
        let height = ImportanceHeightDto::from_binary(&_bytes); // kind:CUSTOM1
        let mut _bytes = _bytes[height.get_size()..].to_vec();
        ImportanceSnapshotBuilder { importance, height }
    }

    /// Gets account importance.
    ///
    /// # Returns
    /// A Account importance.
    pub fn get_importance(&self) -> ImportanceDto {
        self.importance.clone()
    }

    /// Gets importance height.
    ///
    /// # Returns
    /// A Importance height.
    pub fn get_height(&self) -> ImportanceHeightDto {
        self.height.clone()
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += self.importance.get_size(); // importance;
        size += self.height.get_size(); // height;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.importance.serializer()); // kind:CUSTOM
        buf.append(&mut self.height.serializer()); // kind:CUSTOM
        buf
    }
}

