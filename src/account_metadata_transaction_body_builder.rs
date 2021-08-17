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
use super::unresolved_address_dto::*;

/// Binary layout for an account metadata transaction.
#[derive(Debug, Clone)]
pub struct AccountMetadataTransactionBodyBuilder {
    /// Metadata target address.
    pub target_address: UnresolvedAddressDto,
    /// Metadata key scoped to source, target and type.
    pub scoped_metadata_key: u64,
    /// Change in value size in bytes.
    pub value_size_delta: u16,
    /// Difference between existing value and new value \note when there is no existing value, new value is same this value \note when there is an existing value, new value is calculated as xor(previous-value, value).
    pub value: Vec<u8>,
}

impl AccountMetadataTransactionBodyBuilder {
    /// Creates an instance of AccountMetadataTransactionBodyBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A AccountMetadataTransactionBodyBuilder.
    pub fn from_binary(payload: &[u8]) -> Self {
        let mut _bytes = payload.to_vec();
        let target_address = UnresolvedAddressDto::from_binary(&_bytes); // kind:CUSTOM1
        _bytes = _bytes[target_address.get_size()..].to_vec();
        let buf = fixed_bytes::<8>(&_bytes);
        let scoped_metadata_key = u64::from_le_bytes(buf); // kind:SIMPLE
        _bytes = (&_bytes[8..]).to_vec();
        let buf = fixed_bytes::<2>(&_bytes);
        let value_size_delta = u16::from_le_bytes(buf); // kind:SIMPLE
        _bytes = (&_bytes[2..]).to_vec();
        let buf = fixed_bytes::<2>(&_bytes);
        let value_size = u16::from_le_bytes(buf); // kind:SIZE_FIELD
        _bytes = (&_bytes[2..]).to_vec();
        let value = (&_bytes[..value_size as usize]).to_vec(); // kind:BUFFER
        _bytes = (&_bytes[value_size as usize..]).to_vec();
        // create object and call.
        AccountMetadataTransactionBodyBuilder { target_address, scoped_metadata_key, value_size_delta, value } // TransactionBody
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += self.target_address.get_size(); // target_address_size;
        size += 8;  // scoped_metadata_key;
        size += 2;  // value_size_delta;
        size += 2;  // value_size;
        size += self.value.len();
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.target_address.serializer()); // kind:CUSTOM
        buf.append(&mut self.scoped_metadata_key.to_le_bytes().to_vec()); // serial_kind:SIMPLE
        buf.append(&mut self.value_size_delta.to_le_bytes().to_vec()); // serial_kind:SIMPLE
        let size_value: u16 = self.value.len() as u16;
        buf.append(&mut size_value.to_le_bytes().to_vec()); // kind:SIZE_FIELD
        buf.append(&mut self.value.clone()); // kind:BUFFER
        buf
    }
}

