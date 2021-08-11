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

use super::receipt_builder::*;
use super::receipt_type_dto::*;
use super::unresolved_mosaic_id_dto::*;
use super::mosaic_resolution_entry_builder::*;

/// Binary layout for a mosaic resolution statement.
#[derive(Debug, Clone)]
pub struct MosaicResolutionStatementBuilder {
    /// Receipt.
    super_object: ReceiptBuilder,
    /// Unresolved mosaic.
    unresolved: UnresolvedMosaicIdDto,
    /// Resolution entries.
    resolution_entries: Vec<MosaicResolutionEntryBuilder>,
}


impl MosaicResolutionStatementBuilder {

    /// Creates an instance of MosaicResolutionStatementBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A MosaicResolutionStatementBuilder.
    pub fn from_binary(bytes_: &[u8]) -> Self {
        let super_object = ReceiptBuilder::from_binary(bytes_);
        let mut bytes_ = bytes_[super_object.get_size()..].to_vec();
        let unresolved = UnresolvedMosaicIdDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[unresolved.get_size()..].to_vec();
        let resolution_entries: Vec<MosaicResolutionEntryBuilder> = vec![];
        //let bytes_ = GeneratorUtils.load_from_binary(MosaicResolutionEntryBuilder, resolutionEntries, bytes_, len(bytes_));
        MosaicResolutionStatementBuilder{super_object, unresolved, resolution_entries}
    }

    /// Gets unresolved mosaic.
    ///
    /// # Returns
    /// A Unresolved mosaic.
    pub fn get_unresolved(&self) -> UnresolvedMosaicIdDto {
        self.unresolved.clone()
    }

    /// Gets resolution entries.
    ///
    /// # Returns
    /// A Resolution entries.
    pub fn get_resolution_entries(&self) -> Vec<MosaicResolutionEntryBuilder> {
        self.resolution_entries.clone() // ARRAY or FILL_ARRAY
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = self.super_object.get_size();
        size += self.unresolved.get_size();
        size += self.resolution_entries.iter().map(|item| item.get_size()).sum::<usize>(); // array or fill_array;
        size
   }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.super_object.serializer());
        buf.append(&mut self.unresolved.serializer()); // kind:CUSTOM
        for i in &self.resolution_entries {
            buf.append(&mut i.serializer()); // kind:ARRAY|FILL_ARRAY
        }
        buf
    }
}

