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

use super::account_restriction_flags_dto::*;
use super::unresolved_address_dto::*;

/// Binary layout for an account address restriction transaction.
#[derive(Debug, Clone)]
pub struct AccountAddressRestrictionTransactionBodyBuilder {
    /// Account restriction flags.
    pub restriction_flags: Vec<AccountRestrictionFlagsDto>,
    /// Account restriction additions.
    pub restriction_additions: Vec<UnresolvedAddressDto>,
    /// Account restriction deletions.
    pub restriction_deletions: Vec<UnresolvedAddressDto>,
}

impl AccountAddressRestrictionTransactionBodyBuilder {
    /// Creates an instance of AccountAddressRestrictionTransactionBodyBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A AccountAddressRestrictionTransactionBodyBuilder.
    pub fn from_binary(payload: &[u8]) -> Self {
        let mut bytes_ = payload.to_vec();
        let restriction_flags = AccountRestrictionFlagsDto::bytes_to_flags(&bytes_[..2]); // kind:FLAGS
        let mut bytes_ = (&bytes_[2..]).to_vec();
        let mut buf = [0x0u8; 1];
        buf.copy_from_slice(&bytes_[..1]);
        let restriction_additions_count = u8::from_le_bytes(buf); // kind:SIZE_FIELD
        bytes_ = (&bytes_[1..]).to_vec();
        let mut buf = [0x0u8; 1];
        buf.copy_from_slice(&bytes_[..1]);
        let restriction_deletions_count = u8::from_le_bytes(buf); // kind:SIZE_FIELD
        bytes_ = (&bytes_[1..]).to_vec();
        let mut buf = [0x0u8; 4];
        buf.copy_from_slice(&bytes_[..4]);
        let _account_restriction_transaction_body__reserved1 = u32::from_le_bytes(buf); // kind:SIMPLE
        bytes_ = (&bytes_[4..]).to_vec();
        let mut restriction_additions: Vec<UnresolvedAddressDto> = vec![]; // kind:ARRAY
        for _ in 0..restriction_additions_count {
            let item = UnresolvedAddressDto::from_binary(&bytes_);
            restriction_additions.push(item.clone());
            bytes_ = (&bytes_[item.get_size()..]).to_vec();
        }
        let mut restriction_deletions: Vec<UnresolvedAddressDto> = vec![]; // kind:ARRAY
        let mut bytes_ = bytes_.to_vec();
        for _ in 0..restriction_deletions_count {
            let item = UnresolvedAddressDto::from_binary(&bytes_);
            restriction_deletions.push(item.clone());
            bytes_ = (&bytes_[item.get_size()..]).to_vec();
        }
        // create object and call.
        AccountAddressRestrictionTransactionBodyBuilder { restriction_flags, restriction_additions, restriction_deletions } // TransactionBody
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += 2;  // restriction_flags;
        size += 1;  // restriction_additions_count;
        size += 1;  // restriction_deletions_count;
        size += 4;  // account_restriction_transaction_body__reserved1;
        for i in &self.restriction_additions {
            size += i.get_size(); // FILL_ARRAY
        };
        for i in &self.restriction_deletions {
            size += i.get_size(); // FILL_ARRAY
        };
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut AccountRestrictionFlagsDto::flags_to_int(self.restriction_flags.clone()).to_le_bytes().to_vec()); // kind:FLAGS
        let size_value: u8 = self.restriction_additions.len() as u8;
        buf.append(&mut size_value.to_le_bytes().to_vec()); // kind:SIZE_FIELD
        let size_value: u8 = self.restriction_deletions.len() as u8;
        buf.append(&mut size_value.to_le_bytes().to_vec()); // kind:SIZE_FIELD
        buf.append(&mut [0u8; 4].to_vec()); // kind:SIMPLE and is_reserved
        for i in &self.restriction_additions {
            buf.append(&mut i.serializer()); // kind:ARRAY|FILL_ARRAY
        }
        for i in &self.restriction_deletions {
            buf.append(&mut i.serializer()); // kind:ARRAY|FILL_ARRAY
        }
        buf
    }
}

