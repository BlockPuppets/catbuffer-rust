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

use super::address_dto::*;
use super::generator_utils::*;
use super::metadata_type_dto::*;
use super::metadata_value_builder::*;
use super::scoped_metadata_key_dto::*;
use super::state_header_builder::*;

/// Binary layout of a metadata entry.
#[derive(Debug, Clone)]
pub struct MetadataEntryBuilder {
    /// State header.
    super_object: StateHeaderBuilder,
    /// Metadata source address (provider).
    source_address: AddressDto,
    /// Metadata target address.
    target_address: AddressDto,
    /// Metadata key scoped to source, target and type.
    scoped_metadata_key: ScopedMetadataKeyDto,
    /// Target id.
    target_id: u64,
    /// Metadata type.
    metadata_type: MetadataTypeDto,
    /// Value.
    value: MetadataValueBuilder,
}


impl MetadataEntryBuilder {
    /// Creates an instance of MetadataEntryBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A MetadataEntryBuilder.
    pub fn from_binary(_bytes: &[u8]) -> Self {
        let super_object = StateHeaderBuilder::from_binary(_bytes);
        let mut _bytes = _bytes[super_object.get_size()..].to_vec();
        let source_address = AddressDto::from_binary(&_bytes); // kind:CUSTOM1
        let mut _bytes = _bytes[source_address.get_size()..].to_vec();
        let target_address = AddressDto::from_binary(&_bytes); // kind:CUSTOM1
        let mut _bytes = _bytes[target_address.get_size()..].to_vec();
        let scoped_metadata_key = ScopedMetadataKeyDto::from_binary(&_bytes); // kind:CUSTOM1
        let mut _bytes = _bytes[scoped_metadata_key.get_size()..].to_vec();
        let buf = fixed_bytes::<8>(&_bytes);
        let target_id = u64::from_le_bytes(buf); // kind:SIMPLE
        let _bytes = (&_bytes[8..]).to_vec();
        let metadata_type = MetadataTypeDto::from_binary(&_bytes); // kind:CUSTOM2
        let mut _bytes = _bytes[metadata_type.get_size()..].to_vec();
        let value = MetadataValueBuilder::from_binary(&_bytes); // kind:CUSTOM1
        let mut _bytes = _bytes[value.get_size()..].to_vec();
        MetadataEntryBuilder { super_object, source_address, target_address, scoped_metadata_key, target_id, metadata_type, value }
    }

    /// Gets metadata source address (provider).
    ///
    /// # Returns
    /// A Metadata source address (provider).
    pub fn get_source_address(&self) -> AddressDto {
        self.source_address.clone()
    }

    /// Gets metadata target address.
    ///
    /// # Returns
    /// A Metadata target address.
    pub fn get_target_address(&self) -> AddressDto {
        self.target_address.clone()
    }

    /// Gets metadata key scoped to source, target and type.
    ///
    /// # Returns
    /// A Metadata key scoped to source, target and type.
    pub fn get_scoped_metadata_key(&self) -> ScopedMetadataKeyDto {
        self.scoped_metadata_key.clone()
    }

    /// Gets target id.
    ///
    /// # Returns
    /// A Target id.
    pub fn get_target_id(&self) -> u64 {
        self.target_id.clone()
    }

    /// Gets metadata type.
    ///
    /// # Returns
    /// A Metadata type.
    pub fn get_metadata_type(&self) -> MetadataTypeDto {
        self.metadata_type.clone()
    }

    /// Gets value.
    ///
    /// # Returns
    /// A Value.
    pub fn get_value(&self) -> MetadataValueBuilder {
        self.value.clone()
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = self.super_object.get_size();
        size += self.source_address.get_size(); // source_address;
        size += self.target_address.get_size(); // target_address;
        size += self.scoped_metadata_key.get_size(); // scoped_metadata_key;
        size += 8; // target_id;
        size += self.metadata_type.get_size(); // metadata_type;
        size += self.value.get_size(); // value;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.super_object.serializer());
        buf.append(&mut self.source_address.serializer()); // kind:CUSTOM
        buf.append(&mut self.target_address.serializer()); // kind:CUSTOM
        buf.append(&mut self.scoped_metadata_key.serializer()); // kind:CUSTOM
        buf.append(&mut self.get_target_id().to_le_bytes().to_vec()); // kind:SIMPLE
        buf.append(&mut self.metadata_type.serializer()); // kind:CUSTOM
        buf.append(&mut self.value.serializer()); // kind:CUSTOM
        buf
    }
}

