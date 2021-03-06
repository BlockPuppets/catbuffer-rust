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

/// Header common to all serialized states.
#[derive(Debug, Clone)]
pub struct StateHeaderBuilder {
    /// Serialization version.
    version: u16,
}


impl StateHeaderBuilder {
    /// Creates an instance of StateHeaderBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A StateHeaderBuilder.
    pub fn from_binary(_bytes: &[u8]) -> Self {
        let buf = fixed_bytes::<2>(&_bytes);
        let version = u16::from_le_bytes(buf); // kind:SIMPLE
        let _bytes = (&_bytes[2..]).to_vec();
        StateHeaderBuilder { version }
    }

    /// Gets serialization version.
    ///
    /// # Returns
    /// A Serialization version.
    pub fn get_version(&self) -> u16 {
        self.version.clone()
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += 2; // version;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.get_version().to_le_bytes().to_vec()); // kind:SIMPLE
        buf
    }
}

