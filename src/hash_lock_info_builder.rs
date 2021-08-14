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
use super::hash256_dto::*;
use super::height_dto::*;
use super::lock_status_dto::*;
use super::mosaic_builder::*;
use super::state_header_builder::*;

/// Binary layout for hash lock transaction info.
#[derive(Debug, Clone)]
pub struct HashLockInfoBuilder {
    /// State header.
    super_object: StateHeaderBuilder,
    /// Owner address.
    owner_address: AddressDto,
    /// Mosaic associated with lock.
    mosaic: MosaicBuilder,
    /// Height at which the lock expires.
    end_height: HeightDto,
    /// Flag indicating whether or not the lock was already used.
    status: LockStatusDto,
    /// Hash.
    hash: Hash256Dto,
}


impl HashLockInfoBuilder {
    /// Creates an instance of HashLockInfoBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A HashLockInfoBuilder.
    pub fn from_binary(bytes_: &[u8]) -> Self {
        let super_object = StateHeaderBuilder::from_binary(bytes_);
        let mut bytes_ = bytes_[super_object.get_size()..].to_vec();
        let owner_address = AddressDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[owner_address.get_size()..].to_vec();
        let mosaic = MosaicBuilder::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[mosaic.get_size()..].to_vec();
        let end_height = HeightDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[end_height.get_size()..].to_vec();
        let status = LockStatusDto::from_binary(&bytes_); // kind:CUSTOM2
        let mut bytes_ = bytes_[status.get_size()..].to_vec();
        let hash = Hash256Dto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[hash.get_size()..].to_vec();
        HashLockInfoBuilder { super_object, owner_address, mosaic, end_height, status, hash }
    }

    /// Gets owner address.
    ///
    /// # Returns
    /// A Owner address.
    pub fn get_owner_address(&self) -> AddressDto {
        self.owner_address.clone()
    }

    /// Gets mosaic associated with lock.
    ///
    /// # Returns
    /// A Mosaic associated with lock.
    pub fn get_mosaic(&self) -> MosaicBuilder {
        self.mosaic.clone()
    }

    /// Gets height at which the lock expires.
    ///
    /// # Returns
    /// A Height at which the lock expires.
    pub fn get_end_height(&self) -> HeightDto {
        self.end_height.clone()
    }

    /// Gets flag indicating whether or not the lock was already used.
    ///
    /// # Returns
    /// A Flag indicating whether or not the lock was already used.
    pub fn get_status(&self) -> LockStatusDto {
        self.status.clone()
    }

    /// Gets hash.
    ///
    /// # Returns
    /// A Hash.
    pub fn get_hash(&self) -> Hash256Dto {
        self.hash.clone()
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = self.super_object.get_size();
        size += self.owner_address.get_size();
        size += self.mosaic.get_size();
        size += self.end_height.get_size();
        size += self.status.get_size();
        size += self.hash.get_size();
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.super_object.serializer());
        buf.append(&mut self.owner_address.serializer()); // kind:CUSTOM
        buf.append(&mut self.mosaic.serializer()); // kind:CUSTOM
        buf.append(&mut self.end_height.serializer()); // kind:CUSTOM
        buf.append(&mut self.status.serializer()); // kind:CUSTOM
        buf.append(&mut self.hash.serializer()); // kind:CUSTOM
        buf
    }
}

