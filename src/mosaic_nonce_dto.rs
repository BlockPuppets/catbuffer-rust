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

/// Mosaic nonce.
#[derive(Debug, Clone, Copy)]
pub struct MosaicNonceDto(pub u32);

impl MosaicNonceDto {
    pub const LENGTH: usize = std::mem::size_of::<Self>();

    /// Gets the size of the type.
    ///
    /// # Returns
    /// A usize.
    pub fn get_size(&self) -> usize {
        Self::LENGTH
    }

    /// Gets Mosaic nonce.
    ///
    /// # Returns
    /// A Mosaic nonce.
    pub fn get_mosaic_nonce(&self) -> u32 {
        self.0
    }

    /// Serializes an type to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        self.0.to_le_bytes().to_vec()
    }

    /// Creates an `MosaicNonceDto` from a slice.
    ///
    /// # Returns
    /// A `MosaicNonceDto`.
    pub fn from_binary(src: &[u8]) -> Self {
        // assert_eq!(src.len(), Self::LENGTH);
        let buf = fixed_bytes::<{ Self::LENGTH }>(src);
        Self(u32::from_le_bytes(buf))
    }
}
