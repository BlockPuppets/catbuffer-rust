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

use super::account_restriction_address_value_builder::*;
use super::account_restriction_flags_dto::*;
use super::account_restriction_mosaic_value_builder::*;
use super::account_restriction_transaction_type_value_builder::*;
use super::generator_utils::*;

/// Binary layout for account restrictions.
#[derive(Debug, Clone)]
pub struct AccountRestrictionsInfoBuilder {
    /// Raw restriction flags.
    restriction_flags: Vec<AccountRestrictionFlagsDto>,
    /// Address restrictions.
    address_restrictions: Option<AccountRestrictionAddressValueBuilder>,
    /// Mosaic identifier restrictions.
    mosaic_id_restrictions: Option<AccountRestrictionMosaicValueBuilder>,
    /// Transaction type restrictions.
    transaction_type_restrictions: Option<AccountRestrictionTransactionTypeValueBuilder>,
}


impl AccountRestrictionsInfoBuilder {
    /// Creates an instance of AccountRestrictionsInfoBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A AccountRestrictionsInfoBuilder.
    pub fn from_binary(_bytes: &[u8]) -> Self {
        let restriction_flags = AccountRestrictionFlagsDto::bytes_to_flags(&_bytes[..2]); // kind:FLAGS
        let mut _bytes = (&_bytes[2..]).to_vec();
        let mut address_restrictions = None;
        if restriction_flags.iter().any(|&i| i == AccountRestrictionFlagsDto::ADDRESS) {
            let raw_address_restrictions = AccountRestrictionAddressValueBuilder::from_binary(&_bytes);
            _bytes = (&_bytes[raw_address_restrictions.get_size()..]).to_vec();
            address_restrictions = Some(raw_address_restrictions); // kind:CUSTOM1
        }
        let mut mosaic_id_restrictions = None;
        if restriction_flags.iter().any(|&i| i == AccountRestrictionFlagsDto::MOSAIC_ID) {
            let raw_mosaic_id_restrictions = AccountRestrictionMosaicValueBuilder::from_binary(&_bytes);
            _bytes = (&_bytes[raw_mosaic_id_restrictions.get_size()..]).to_vec();
            mosaic_id_restrictions = Some(raw_mosaic_id_restrictions); // kind:CUSTOM1
        }
        let mut transaction_type_restrictions = None;
        if restriction_flags.iter().any(|&i| i == AccountRestrictionFlagsDto::TRANSACTION_TYPE) {
            let raw_transaction_type_restrictions = AccountRestrictionTransactionTypeValueBuilder::from_binary(&_bytes);
            _bytes = (&_bytes[raw_transaction_type_restrictions.get_size()..]).to_vec();
            transaction_type_restrictions = Some(raw_transaction_type_restrictions); // kind:CUSTOM1
        }
        AccountRestrictionsInfoBuilder { restriction_flags, address_restrictions, mosaic_id_restrictions, transaction_type_restrictions }
    }

    /// Gets raw restriction flags.
    ///
    /// # Returns
    /// A Raw restriction flags.
    pub fn get_restriction_flags(&self) -> Vec<AccountRestrictionFlagsDto> {
        self.restriction_flags.clone()
    }

    /// Gets address restrictions.
    ///
    /// # Returns
    /// A Address restrictions.
    pub fn get_address_restrictions(&self) -> Option<AccountRestrictionAddressValueBuilder> {
        if self.restriction_flags.iter().any(|&i| i == AccountRestrictionFlagsDto::ADDRESS) {
            panic!("restrictionFlags is not set to ADDRESS.")
        };
        self.address_restrictions.clone()
    }

    /// Gets mosaic identifier restrictions.
    ///
    /// # Returns
    /// A Mosaic identifier restrictions.
    pub fn get_mosaic_id_restrictions(&self) -> Option<AccountRestrictionMosaicValueBuilder> {
        if self.restriction_flags.iter().any(|&i| i == AccountRestrictionFlagsDto::MOSAIC_ID) {
            panic!("restrictionFlags is not set to MOSAIC_ID.")
        };
        self.mosaic_id_restrictions.clone()
    }

    /// Gets transaction type restrictions.
    ///
    /// # Returns
    /// A Transaction type restrictions.
    pub fn get_transaction_type_restrictions(&self) -> Option<AccountRestrictionTransactionTypeValueBuilder> {
        if self.restriction_flags.iter().any(|&i| i == AccountRestrictionFlagsDto::TRANSACTION_TYPE) {
            panic!("restrictionFlags is not set to TRANSACTION_TYPE.")
        };
        self.transaction_type_restrictions.clone()
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += 2; // restriction_flags;
        if self.restriction_flags.iter().any(|&i| i == AccountRestrictionFlagsDto::ADDRESS) {
            size += self.address_restrictions.as_ref().unwrap().get_size(); // address_restrictions
        }
        if self.restriction_flags.iter().any(|&i| i == AccountRestrictionFlagsDto::MOSAIC_ID) {
            size += self.mosaic_id_restrictions.as_ref().unwrap().get_size(); // mosaic_id_restrictions
        }
        if self.restriction_flags.iter().any(|&i| i == AccountRestrictionFlagsDto::TRANSACTION_TYPE) {
            size += self.transaction_type_restrictions.as_ref().unwrap().get_size(); // transaction_type_restrictions
        }
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut AccountRestrictionFlagsDto::flags_to_int(self.get_restriction_flags()).to_le_bytes().to_vec()); // kind:FLAGS
        if self.restriction_flags.iter().any(|&i| i == AccountRestrictionFlagsDto::ADDRESS) {
            buf.append(&mut self.address_restrictions.as_ref().unwrap().serializer()); // kind:CUSTOM
        };
        if self.restriction_flags.iter().any(|&i| i == AccountRestrictionFlagsDto::MOSAIC_ID) {
            buf.append(&mut self.mosaic_id_restrictions.as_ref().unwrap().serializer()); // kind:CUSTOM
        };
        if self.restriction_flags.iter().any(|&i| i == AccountRestrictionFlagsDto::TRANSACTION_TYPE) {
            buf.append(&mut self.transaction_type_restrictions.as_ref().unwrap().serializer()); // kind:CUSTOM
        };
        buf
    }
}

