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

use super::height_dto::*;

/// Binary layout for namespace lifetime.
#[derive(Debug, Clone)]
pub struct NamespaceLifetimeBuilder {
    /// Start height.
    lifetime_start: HeightDto,
    /// End height.
    lifetime_end: HeightDto,
}


impl NamespaceLifetimeBuilder {
    /// Creates an instance of NamespaceLifetimeBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A NamespaceLifetimeBuilder.
    pub fn from_binary(bytes_: &[u8]) -> Self {
        let lifetime_start = HeightDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[lifetime_start.get_size()..].to_vec();
        let lifetime_end = HeightDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[lifetime_end.get_size()..].to_vec();
        NamespaceLifetimeBuilder { lifetime_start, lifetime_end }
    }

    /// Gets start height.
    ///
    /// # Returns
    /// A Start height.
    pub fn get_lifetime_start(&self) -> HeightDto {
        self.lifetime_start.clone()
    }

    /// Gets end height.
    ///
    /// # Returns
    /// A End height.
    pub fn get_lifetime_end(&self) -> HeightDto {
        self.lifetime_end.clone()
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += self.lifetime_start.get_size();
        size += self.lifetime_end.get_size();
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.lifetime_start.serializer()); // kind:CUSTOM
        buf.append(&mut self.lifetime_end.serializer()); // kind:CUSTOM
        buf
    }
}

