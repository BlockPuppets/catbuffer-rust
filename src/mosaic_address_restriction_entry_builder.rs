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
use super::address_key_value_set_builder::*;
use super::mosaic_id_dto::*;

/// Binary layout for a mosaic restriction.
#[derive(Debug, Clone)]
pub struct MosaicAddressRestrictionEntryBuilder {
    /// Identifier of the mosaic to which the restriction applies.
    mosaic_id: MosaicIdDto,
    /// Address being restricted.
    address: AddressDto,
    /// Address key value restriction set.
    key_pairs: AddressKeyValueSetBuilder,
}


impl MosaicAddressRestrictionEntryBuilder {

    /// Creates an instance of MosaicAddressRestrictionEntryBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A MosaicAddressRestrictionEntryBuilder.
    pub fn from_binary(bytes_: &[u8]) -> Self {
        let mosaic_id = MosaicIdDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[mosaic_id.get_size()..].to_vec();
        let address = AddressDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[address.get_size()..].to_vec();
        let key_pairs = AddressKeyValueSetBuilder::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[key_pairs.get_size()..].to_vec();
        MosaicAddressRestrictionEntryBuilder{mosaic_id, address, key_pairs}
    }

    /// Gets identifier of the mosaic to which the restriction applies.
    ///
    /// # Returns
    /// A Identifier of the mosaic to which the restriction applies.
    pub fn get_mosaic_id(&self) -> MosaicIdDto {
        self.mosaic_id.clone()
    }

    /// Gets address being restricted.
    ///
    /// # Returns
    /// A Address being restricted.
    pub fn get_address(&self) -> AddressDto {
        self.address.clone()
    }

    /// Gets address key value restriction set.
    ///
    /// # Returns
    /// A Address key value restriction set.
    pub fn get_key_pairs(&self) -> AddressKeyValueSetBuilder {
        self.key_pairs.clone()
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += self.mosaic_id.get_size();
        size += self.address.get_size();
        size += self.key_pairs.get_size();
        size
   }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.mosaic_id.serializer()); // kind:CUSTOM
        buf.append(&mut self.address.serializer()); // kind:CUSTOM
        buf.append(&mut self.key_pairs.serializer()); // kind:CUSTOM
        buf
    }
}
