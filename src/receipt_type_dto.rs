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

use num_traits::{ToPrimitive, FromPrimitive};
use num_derive::{ToPrimitive, FromPrimitive};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// Enumeration of receipt types.
#[allow(non_camel_case_types)]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive, EnumIter)]
pub enum ReceiptTypeDto {
    /// Reserved receipt type.
    RESERVED = 0,
    /// Mosaic rental fee receipt type.
    MOSAIC_RENTAL_FEE = 4685,
    /// Namespace rental fee receipt type.
    NAMESPACE_RENTAL_FEE = 4942,
    /// Harvest fee receipt type.
    HARVEST_FEE = 8515,
    /// Lock hash completed receipt type.
    LOCK_HASH_COMPLETED = 8776,
    /// Lock hash expired receipt type.
    LOCK_HASH_EXPIRED = 9032,
    /// Lock secret completed receipt type.
    LOCK_SECRET_COMPLETED = 8786,
    /// Lock secret expired receipt type.
    LOCK_SECRET_EXPIRED = 9042,
    /// Lock hash created receipt type.
    LOCK_HASH_CREATED = 12616,
    /// Lock secret created receipt type.
    LOCK_SECRET_CREATED = 12626,
    /// Mosaic expired receipt type.
    MOSAIC_EXPIRED = 16717,
    /// Namespace expired receipt type.
    NAMESPACE_EXPIRED = 16718,
    /// Namespace deleted receipt type.
    NAMESPACE_DELETED = 16974,
    /// Inflation receipt type.
    INFLATION = 20803,
    /// Transaction group receipt type.
    TRANSACTION_GROUP = 57667,
    /// Address alias resolution receipt type.
    ADDRESS_ALIAS_RESOLUTION = 61763,
    /// Mosaic alias resolution receipt type.
    MOSAIC_ALIAS_RESOLUTION = 62019,
}

impl ReceiptTypeDto {

    pub const LENGTH: usize = std::mem::size_of::<Self>();

    /// Gets the size of the type.
    ///
    /// # Returns
    ///
    /// A usize.
    pub fn get_size(&self) -> usize {
        Self::LENGTH
    }

    /// Gets the value of the enum.
    ///
    /// # Returns
    ///
    /// A u16
    pub fn get_value(&self) -> u16 {
        self.to_u16().unwrap()
    }


    /// Creates an `ReceiptTypeDto` from a slice.
    ///
    /// # Returns
    ///
    /// A `ReceiptTypeDto`.
    pub fn from_binary(src: &[u8]) -> Self {
        // assert_eq!(src.len(), Self::LENGTH);
        let mut buf = [0x0u8; Self::LENGTH];
        buf.copy_from_slice(&src[..Self::LENGTH]);
        Self::from_u16(u16::from_le_bytes(buf)).unwrap()
    }

    /// Serializes an type to bytes.
    ///
    /// # Returns
    ///
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        self.get_value().to_le_bytes().to_vec()
    }
}
