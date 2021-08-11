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

/// Hash512.
#[derive(Debug, Clone, Copy)]
pub struct Hash512Dto(pub [u8; 64]);

impl Hash512Dto {

    pub const LENGTH: usize = std::mem::size_of::<Self>();

    /// Gets the size of the type.
    ///
    /// # Returns
    /// A usize.
    pub fn get_size(&self) -> usize {
        Self::LENGTH
    }

    /// Gets Hash512.
    ///
    /// # Returns
    /// A Hash512.
    pub fn get_hash512(&self) -> [u8; 64] {
        self.0
    }

    /// Serializes an type to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(self.get_size());
        buffer.extend(self.0.to_vec());
        buffer
    }

    /// Creates an `Hash512Dto` from a slice.
    ///
    /// # Returns
    /// A `Hash512Dto`.
    pub fn from_binary(src: &[u8]) -> Self {
        // assert_eq!(src.len(), Self::LENGTH);
        let mut buf = [0x0u8; Self::LENGTH];
        buf.copy_from_slice(&src[..Self::LENGTH]);
        Self(buf)
    }
}