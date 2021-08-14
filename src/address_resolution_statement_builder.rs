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

use super::address_resolution_entry_builder::*;
use super::generator_utils::*;
use super::receipt_builder::*;
use super::unresolved_address_dto::*;

/// Binary layout for an address resolution statement.
#[derive(Debug, Clone)]
pub struct AddressResolutionStatementBuilder {
    /// Receipt.
    super_object: ReceiptBuilder,
    /// Unresolved address.
    unresolved: UnresolvedAddressDto,
    /// Resolution entries.
    resolution_entries: Vec<AddressResolutionEntryBuilder>,
}


impl AddressResolutionStatementBuilder {
    /// Creates an instance of AddressResolutionStatementBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A AddressResolutionStatementBuilder.
    pub fn from_binary(bytes_: &[u8]) -> Self {
        let super_object = ReceiptBuilder::from_binary(bytes_);
        let mut bytes_ = bytes_[super_object.get_size()..].to_vec();
        let unresolved = UnresolvedAddressDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[unresolved.get_size()..].to_vec();
        let resolution_entries: Vec<AddressResolutionEntryBuilder> = vec![];
        //let bytes_ = GeneratorUtils.load_from_binary(AddressResolutionEntryBuilder, resolutionEntries, bytes_, len(bytes_));
        AddressResolutionStatementBuilder { super_object, unresolved, resolution_entries }
    }

    /// Gets unresolved address.
    ///
    /// # Returns
    /// A Unresolved address.
    pub fn get_unresolved(&self) -> UnresolvedAddressDto {
        self.unresolved.clone()
    }

    /// Gets resolution entries.
    ///
    /// # Returns
    /// A Resolution entries.
    pub fn get_resolution_entries(&self) -> Vec<AddressResolutionEntryBuilder> {
        self.resolution_entries.clone() // ARRAY or FILL_ARRAY
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = self.super_object.get_size();
        size += self.unresolved.get_size(); // unresolved;
        size += self.resolution_entries.iter().map(|item| item.get_size()).sum::<usize>(); // array or fill_array;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.super_object.serializer());
        buf.append(&mut self.unresolved.serializer()); // kind:CUSTOM
        for i in &self.resolution_entries {
            buf.append(&mut i.serializer()); // kind:ARRAY|FILL_ARRAY
        }
        buf
    }
}

