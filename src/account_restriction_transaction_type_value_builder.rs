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

use super::entity_type_dto::*;

/// Binary layout for transaction type based account restriction.
#[derive(Debug, Clone)]
pub struct AccountRestrictionTransactionTypeValueBuilder {
    /// Restriction values.
    restriction_values: Vec<EntityTypeDto>,
}


impl AccountRestrictionTransactionTypeValueBuilder {
    /// Creates an instance of AccountRestrictionTransactionTypeValueBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A AccountRestrictionTransactionTypeValueBuilder.
    pub fn from_binary(bytes_: &[u8]) -> Self {
        let mut buf = [0x0u8; 8];
        buf.copy_from_slice(&bytes_[..8]);
        let restrictionValuesCount = u64::from_le_bytes(buf); // kind:SIZE_FIELD
        let mut bytes_ = (&bytes_[8..]).to_vec();
        let mut restriction_values: Vec<EntityTypeDto> = vec![]; // kind:ARRAY
        let mut bytes_ = bytes_.to_vec();
        for _ in 0..restrictionValuesCount {
            let item = EntityTypeDto::from_binary(&bytes_);
            restriction_values.push(item.clone());
            bytes_ = (&bytes_[item.get_size()..]).to_vec();
        }
        AccountRestrictionTransactionTypeValueBuilder { restriction_values }
    }

    /// Gets restriction values.
    ///
    /// # Returns
    /// A Restriction values.
    pub fn get_restriction_values(&self) -> Vec<EntityTypeDto> {
        self.restriction_values.clone() // ARRAY or FILL_ARRAY
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += 8; // restriction_values_count;
        size += self.restriction_values.iter().map(|item| item.get_size()).sum::<usize>(); // array or fill_array;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut (self.get_restriction_values().len() as u64).to_le_bytes().to_vec()); // kind:SIZE_FIELD
        for i in &self.restriction_values {
            buf.append(&mut i.serializer()); // kind:ARRAY|FILL_ARRAY
        }
        buf
    }
}

