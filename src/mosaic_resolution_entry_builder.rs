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

use super::mosaic_id_dto::*;
use super::receipt_source_builder::*;

/// Binary layout for mosaic resolution entry.
#[derive(Debug, Clone)]
pub struct MosaicResolutionEntryBuilder {
    /// Source of resolution within block.
    source: ReceiptSourceBuilder,
    /// Resolved value.
    resolved: MosaicIdDto,
}


impl MosaicResolutionEntryBuilder {
    /// Creates an instance of MosaicResolutionEntryBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A MosaicResolutionEntryBuilder.
    pub fn from_binary(bytes_: &[u8]) -> Self {
        let source = ReceiptSourceBuilder::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[source.get_size()..].to_vec();
        let resolved = MosaicIdDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[resolved.get_size()..].to_vec();
        MosaicResolutionEntryBuilder { source, resolved }
    }

    /// Gets source of resolution within block.
    ///
    /// # Returns
    /// A Source of resolution within block.
    pub fn get_source(&self) -> ReceiptSourceBuilder {
        self.source.clone()
    }

    /// Gets resolved value.
    ///
    /// # Returns
    /// A Resolved value.
    pub fn get_resolved(&self) -> MosaicIdDto {
        self.resolved.clone()
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += self.source.get_size();
        size += self.resolved.get_size();
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.source.serializer()); // kind:CUSTOM
        buf.append(&mut self.resolved.serializer()); // kind:CUSTOM
        buf
    }
}

