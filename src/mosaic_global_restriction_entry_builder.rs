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
use super::global_key_value_set_builder::*;
use super::mosaic_id_dto::*;

/// Binary layout for a mosaic restriction.
#[derive(Debug, Clone)]
pub struct MosaicGlobalRestrictionEntryBuilder {
    /// Identifier of the mosaic to which the restriction applies.
    mosaic_id: MosaicIdDto,
    /// Global key value restriction set.
    key_pairs: GlobalKeyValueSetBuilder,
}


impl MosaicGlobalRestrictionEntryBuilder {
    /// Creates an instance of MosaicGlobalRestrictionEntryBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A MosaicGlobalRestrictionEntryBuilder.
    pub fn from_binary(_bytes: &[u8]) -> Self {
        let mosaic_id = MosaicIdDto::from_binary(&_bytes); // kind:CUSTOM1
        let mut _bytes = _bytes[mosaic_id.get_size()..].to_vec();
        let key_pairs = GlobalKeyValueSetBuilder::from_binary(&_bytes); // kind:CUSTOM1
        let mut _bytes = _bytes[key_pairs.get_size()..].to_vec();
        MosaicGlobalRestrictionEntryBuilder { mosaic_id, key_pairs }
    }

    /// Gets identifier of the mosaic to which the restriction applies.
    ///
    /// # Returns
    /// A Identifier of the mosaic to which the restriction applies.
    pub fn get_mosaic_id(&self) -> MosaicIdDto {
        self.mosaic_id.clone()
    }

    /// Gets global key value restriction set.
    ///
    /// # Returns
    /// A Global key value restriction set.
    pub fn get_key_pairs(&self) -> GlobalKeyValueSetBuilder {
        self.key_pairs.clone()
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += self.mosaic_id.get_size(); // mosaic_id;
        size += self.key_pairs.get_size(); // key_pairs;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.mosaic_id.serializer()); // kind:CUSTOM
        buf.append(&mut self.key_pairs.serializer()); // kind:CUSTOM
        buf
    }
}

