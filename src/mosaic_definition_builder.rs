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

use super::address_dto::*;
use super::height_dto::*;
use super::mosaic_properties_builder::*;

/// Binary layout for mosaic definition.
#[derive(Debug, Clone)]
pub struct MosaicDefinitionBuilder {
    /// Block height.
    start_height: HeightDto,
    /// Mosaic owner.
    owner_address: AddressDto,
    /// Revision.
    revision: u32,
    /// Properties.
    properties: MosaicPropertiesBuilder,
}


impl MosaicDefinitionBuilder {
    /// Creates an instance of MosaicDefinitionBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A MosaicDefinitionBuilder.
    pub fn from_binary(bytes_: &[u8]) -> Self {
        let start_height = HeightDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[start_height.get_size()..].to_vec();
        let owner_address = AddressDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[owner_address.get_size()..].to_vec();
        let mut buf = [0x0u8; 4];
        buf.copy_from_slice(&bytes_[..4]);
        let revision = u32::from_le_bytes(buf); // kind:SIMPLE
        let bytes_ = (&bytes_[4..]).to_vec();
        let properties = MosaicPropertiesBuilder::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[properties.get_size()..].to_vec();
        MosaicDefinitionBuilder { start_height, owner_address, revision, properties }
    }

    /// Gets block height.
    ///
    /// # Returns
    /// A Block height.
    pub fn get_start_height(&self) -> HeightDto {
        self.start_height.clone()
    }

    /// Gets mosaic owner.
    ///
    /// # Returns
    /// A Mosaic owner.
    pub fn get_owner_address(&self) -> AddressDto {
        self.owner_address.clone()
    }

    /// Gets revision.
    ///
    /// # Returns
    /// A Revision.
    pub fn get_revision(&self) -> u32 {
        self.revision.clone()
    }

    /// Gets properties.
    ///
    /// # Returns
    /// A Properties.
    pub fn get_properties(&self) -> MosaicPropertiesBuilder {
        self.properties.clone()
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += self.start_height.get_size();
        size += self.owner_address.get_size();
        size += 4; // revision;
        size += self.properties.get_size();
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.start_height.serializer()); // kind:CUSTOM
        buf.append(&mut self.owner_address.serializer()); // kind:CUSTOM
        buf.append(&mut (self.get_revision() as u16).to_le_bytes().to_vec()); // kind:SIMPLE
        buf.append(&mut self.properties.serializer()); // kind:CUSTOM
        buf
    }
}

