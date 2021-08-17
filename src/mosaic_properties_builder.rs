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

use super::block_duration_dto::*;
use super::generator_utils::*;
use super::mosaic_flags_dto::*;

/// Binary layout for mosaic properties.
#[derive(Debug, Clone)]
pub struct MosaicPropertiesBuilder {
    /// Mosaic flags.
    flags: Vec<MosaicFlagsDto>,
    /// Mosaic divisibility.
    divisibility: u8,
    /// Mosaic duration.
    duration: BlockDurationDto,
}


impl MosaicPropertiesBuilder {
    /// Creates an instance of MosaicPropertiesBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A MosaicPropertiesBuilder.
    pub fn from_binary(_bytes: &[u8]) -> Self {
        let flags = MosaicFlagsDto::bytes_to_flags(&_bytes[..1]); // kind:FLAGS
        let mut _bytes = (&_bytes[1..]).to_vec();
        let buf = fixed_bytes::<1>(&_bytes);
        let divisibility = u8::from_le_bytes(buf); // kind:SIMPLE
        let _bytes = (&_bytes[1..]).to_vec();
        let duration = BlockDurationDto::from_binary(&_bytes); // kind:CUSTOM1
        let mut _bytes = _bytes[duration.get_size()..].to_vec();
        MosaicPropertiesBuilder { flags, divisibility, duration }
    }

    /// Gets mosaic flags.
    ///
    /// # Returns
    /// A Mosaic flags.
    pub fn get_flags(&self) -> Vec<MosaicFlagsDto> {
        self.flags.clone()
    }

    /// Gets mosaic divisibility.
    ///
    /// # Returns
    /// A Mosaic divisibility.
    pub fn get_divisibility(&self) -> u8 {
        self.divisibility.clone()
    }

    /// Gets mosaic duration.
    ///
    /// # Returns
    /// A Mosaic duration.
    pub fn get_duration(&self) -> BlockDurationDto {
        self.duration.clone()
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += 1; // flags;
        size += 1; // divisibility;
        size += self.duration.get_size(); // duration;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut MosaicFlagsDto::flags_to_int(self.get_flags()).to_le_bytes().to_vec()); // kind:FLAGS
        buf.append(&mut self.get_divisibility().to_le_bytes().to_vec()); // kind:SIMPLE
        buf.append(&mut self.duration.serializer()); // kind:CUSTOM
        buf
    }
}

