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

/// Binary layout for receipt source.
#[derive(Debug, Clone)]
pub struct ReceiptSourceBuilder {
    /// Transaction primary source (e.g. index within block).
    primary_id: u32,
    /// Transaction secondary source (e.g. index within aggregate).
    secondary_id: u32,
}


impl ReceiptSourceBuilder {
    /// Creates an instance of ReceiptSourceBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A ReceiptSourceBuilder.
    pub fn from_binary(bytes_: &[u8]) -> Self {
        let mut buf = fixed_bytes::<4>(&bytes_);
        let primary_id = u32::from_le_bytes(buf); // kind:SIMPLE
        let bytes_ = (&bytes_[4..]).to_vec();
        let mut buf = fixed_bytes::<4>(&bytes_);
        let secondary_id = u32::from_le_bytes(buf); // kind:SIMPLE
        let bytes_ = (&bytes_[4..]).to_vec();
        ReceiptSourceBuilder { primary_id, secondary_id }
    }

    /// Gets transaction primary source (e.g. index within block).
    ///
    /// # Returns
    /// A Transaction primary source (e.g. index within block).
    pub fn get_primary_id(&self) -> u32 {
        self.primary_id.clone()
    }

    /// Gets transaction secondary source (e.g. index within aggregate).
    ///
    /// # Returns
    /// A Transaction secondary source (e.g. index within aggregate).
    pub fn get_secondary_id(&self) -> u32 {
        self.secondary_id.clone()
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += 4; // primary_id;
        size += 4; // secondary_id;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut (self.get_primary_id() as u16).to_le_bytes().to_vec()); // kind:SIMPLE
        buf.append(&mut (self.get_secondary_id() as u16).to_le_bytes().to_vec()); // kind:SIMPLE
        buf
    }
}

