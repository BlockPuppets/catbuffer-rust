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

use super::account_restrictions_info_builder::*;
use super::address_dto::*;
use super::generator_utils::*;
use super::state_header_builder::*;

/// Binary layout for account restrictions.
#[derive(Debug, Clone)]
pub struct AccountRestrictionsBuilder {
    /// State header.
    super_object: StateHeaderBuilder,
    /// Address on which restrictions are placed.
    address: AddressDto,
    /// Account restrictions.
    restrictions: Vec<AccountRestrictionsInfoBuilder>,
}


impl AccountRestrictionsBuilder {
    /// Creates an instance of AccountRestrictionsBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A AccountRestrictionsBuilder.
    pub fn from_binary(bytes_: &[u8]) -> Self {
        let super_object = StateHeaderBuilder::from_binary(bytes_);
        let mut bytes_ = bytes_[super_object.get_size()..].to_vec();
        let address = AddressDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[address.get_size()..].to_vec();
        let mut buf = fixed_bytes::<8>(&bytes_);
        let restrictionsCount = u64::from_le_bytes(buf); // kind:SIZE_FIELD
        let mut bytes_ = (&bytes_[8..]).to_vec();
        let mut restrictions: Vec<AccountRestrictionsInfoBuilder> = vec![]; // kind:ARRAY
        let mut bytes_ = bytes_.to_vec();
        for _ in 0..restrictionsCount {
            let item = AccountRestrictionsInfoBuilder::from_binary(&bytes_);
            restrictions.push(item.clone());
            bytes_ = (&bytes_[item.get_size()..]).to_vec();
        }
        AccountRestrictionsBuilder { super_object, address, restrictions }
    }

    /// Gets address on which restrictions are placed.
    ///
    /// # Returns
    /// A Address on which restrictions are placed.
    pub fn get_address(&self) -> AddressDto {
        self.address.clone()
    }

    /// Gets account restrictions.
    ///
    /// # Returns
    /// A Account restrictions.
    pub fn get_restrictions(&self) -> Vec<AccountRestrictionsInfoBuilder> {
        self.restrictions.clone() // ARRAY or FILL_ARRAY
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = self.super_object.get_size();
        size += self.address.get_size(); // address;
        size += 8; // restrictions_count;
        size += self.restrictions.iter().map(|item| item.get_size()).sum::<usize>(); // array or fill_array;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.super_object.serializer());
        buf.append(&mut self.address.serializer()); // kind:CUSTOM
        buf.append(&mut (self.get_restrictions().len() as u64).to_le_bytes().to_vec()); // kind:SIZE_FIELD
        for i in &self.restrictions {
            buf.append(&mut i.serializer()); // kind:ARRAY|FILL_ARRAY
        }
        buf
    }
}

