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

/// Enumeration of account restriction flags.
#[allow(non_camel_case_types)]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive, EnumIter)]
pub enum AccountRestrictionFlagsDto {
    /// Restriction type is an address.
    ADDRESS = 1,
    /// Restriction type is a mosaic identifier.
    MOSAIC_ID = 2,
    /// Restriction type is a transaction type.
    TRANSACTION_TYPE = 4,
    /// Restriction is interpreted as outgoing.
    OUTGOING = 16384,
    /// Restriction is interpreted as blocking (instead of allowing) operation.
    BLOCK = 32768,
}

impl AccountRestrictionFlagsDto {

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


    /// Converts a bit representation to a vec of AccountRestrictionFlagsDto.
    ///
    /// # Returns
    ///
    /// A vec to AccountRestrictionFlagsDto flags representing the int value.
    pub fn bytes_to_flags(bit_mask_value: &[u8]) -> Vec<AccountRestrictionFlagsDto> {
        use std::convert::TryFrom;
        let bit_mask_value = <[u8; Self::LENGTH]>::try_from(&bit_mask_value[..Self::LENGTH]).unwrap();
        Self::int_to_flags(u16::from_le_bytes(bit_mask_value))
    }

    /// Converts a bit representation to a vec of AccountRestrictionFlagsDto.
    ///
    /// # Returns
    ///
    /// A vec to AccountRestrictionFlagsDto flags representing the int value.
    pub fn int_to_flags(bit_mask_value: u16) -> Vec<AccountRestrictionFlagsDto> {
        let mut results: Vec<AccountRestrictionFlagsDto> = vec![];
        for flag in AccountRestrictionFlagsDto::iter() {
            if 0 != flag.get_value() & bit_mask_value {
            results.push(flag);
            }
        }
        results
    }

    /// Converts a vec of AccountRestrictionFlagsDto to a bit representation.
    ///
    /// # Returns
    ///
    /// A u16 value of the vec of flags
    pub fn flags_to_int(flags: Vec<AccountRestrictionFlagsDto>) -> u16 {
        let mut result: u16 = 0;
        for flag in AccountRestrictionFlagsDto::iter() {
            if flags.iter().any( | &i | i == flag ) {
               result += flag.get_value();
            }
        }
        result
    }

    /// Creates an `AccountRestrictionFlagsDto` from a slice.
    ///
    /// # Returns
    ///
    /// A `AccountRestrictionFlagsDto`.
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
