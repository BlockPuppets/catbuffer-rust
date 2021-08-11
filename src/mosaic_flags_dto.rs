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

/// Enumeration of mosaic property flags.
#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive, EnumIter)]
pub enum MosaicFlagsDto {
    /// No flags present.
    NONE = 0,
    /// Mosaic supports supply changes even when mosaic owner owns partial supply.
    SUPPLY_MUTABLE = 1,
    /// Mosaic supports transfers between arbitrary accounts \note when not set, mosaic can only be transferred to and from mosaic owner.
    TRANSFERABLE = 2,
    /// Mosaic supports custom restrictions configured by mosaic owner.
    RESTRICTABLE = 4,
}

impl MosaicFlagsDto {

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
    /// A u8
    pub fn get_value(&self) -> u8 {
        self.to_u8().unwrap()
    }


    /// Converts a bit representation to a vec of MosaicFlagsDto.
    ///
    /// # Returns
    ///
    /// A vec to MosaicFlagsDto flags representing the int value.
    pub fn bytes_to_flags(bit_mask_value: &[u8]) -> Vec<MosaicFlagsDto> {
        use std::convert::TryFrom;
        let bit_mask_value = <[u8; Self::LENGTH]>::try_from(&bit_mask_value[..Self::LENGTH]).unwrap();
        Self::int_to_flags(u8::from_le_bytes(bit_mask_value))
    }

    /// Converts a bit representation to a vec of MosaicFlagsDto.
    ///
    /// # Returns
    ///
    /// A vec to MosaicFlagsDto flags representing the int value.
    pub fn int_to_flags(bit_mask_value: u8) -> Vec<MosaicFlagsDto> {
        let mut results: Vec<MosaicFlagsDto> = vec![];
        for flag in MosaicFlagsDto::iter() {
            if 0 != flag.get_value() & bit_mask_value {
            results.push(flag);
            }
        }
        results
    }

    /// Converts a vec of MosaicFlagsDto to a bit representation.
    ///
    /// # Returns
    ///
    /// A u16 value of the vec of flags
    pub fn flags_to_int(flags: Vec<MosaicFlagsDto>) -> u8 {
        let mut result: u8 = 0;
        for flag in MosaicFlagsDto::iter() {
            if flags.iter().any( | &i | i == flag ) {
               result += flag.get_value();
            }
        }
        result
    }

    /// Creates an `MosaicFlagsDto` from a slice.
    ///
    /// # Returns
    ///
    /// A `MosaicFlagsDto`.
    pub fn from_binary(src: &[u8]) -> Self {
        // assert_eq!(src.len(), Self::LENGTH);
        let mut buf = [0x0u8; Self::LENGTH];
        buf.copy_from_slice(&src[..Self::LENGTH]);
        Self::from_u8(u8::from_le_bytes(buf)).unwrap()
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
