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

use super::mosaic_restriction_key_dto::*;

/// Layout for mosaic address restriction key-value pair.
#[derive(Debug, Clone)]
pub struct AddressKeyValueBuilder {
    /// Key for value.
    key: MosaicRestrictionKeyDto,
    /// Value associated by key.
    value: u64,
}


impl AddressKeyValueBuilder {
    /// Creates an instance of AddressKeyValueBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A AddressKeyValueBuilder.
    pub fn from_binary(bytes_: &[u8]) -> Self {
        let key = MosaicRestrictionKeyDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[key.get_size()..].to_vec();
        let mut buf = [0x0u8; 8];
        buf.copy_from_slice(&bytes_[..8]);
        let value = u64::from_le_bytes(buf); // kind:SIMPLE
        let bytes_ = (&bytes_[8..]).to_vec();
        AddressKeyValueBuilder { key, value }
    }

    /// Gets key for value.
    ///
    /// # Returns
    /// A Key for value.
    pub fn get_key(&self) -> MosaicRestrictionKeyDto {
        self.key.clone()
    }

    /// Gets value associated by key.
    ///
    /// # Returns
    /// A Value associated by key.
    pub fn get_value(&self) -> u64 {
        self.value.clone()
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += self.key.get_size();
        size += 8; // value;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.key.serializer()); // kind:CUSTOM
        buf.append(&mut (self.get_value() as u16).to_le_bytes().to_vec()); // kind:SIMPLE
        buf
    }
}

