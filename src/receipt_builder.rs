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
use super::receipt_type_dto::*;

/// Binary layout for a receipt entity.
#[derive(Debug, Clone)]
pub struct ReceiptBuilder {
    /// Receipt version.
    version: u16,
    /// Receipt type.
    _type: ReceiptTypeDto,
}


impl ReceiptBuilder {
    /// Creates an instance of ReceiptBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A ReceiptBuilder.
    pub fn from_binary(_bytes: &[u8]) -> Self {
        let buf = fixed_bytes::<2>(&_bytes);
        let version = u16::from_le_bytes(buf); // kind:SIMPLE
        let _bytes = (&_bytes[2..]).to_vec();
        let _type = ReceiptTypeDto::from_binary(&_bytes); // kind:CUSTOM2
        let _bytes = (&_bytes[_type.get_size()..]).to_vec();
        ReceiptBuilder { version, _type }
    }

    /// Gets receipt version.
    ///
    /// # Returns
    /// A Receipt version.
    pub fn get_version(&self) -> u16 {
        self.version.clone()
    }

    /// Gets receipt type.
    ///
    /// # Returns
    /// A Receipt type.
    pub fn get_type(&self) -> ReceiptTypeDto {
        self._type
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += 4; // size;
        size += 2; // version;
        size += self._type.get_size(); // type;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        // Ignored serialization: size AttributeKind.SIMPLE
        buf.append(&mut self.get_version().to_le_bytes().to_vec()); // kind:SIMPLE
        buf.append(&mut self._type.serializer()); // kind:CUSTOM
        buf
    }
}

