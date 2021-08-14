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

use super::namespace_alias_builder::*;
use super::namespace_id_dto::*;

/// Binary layout for a namespace path.
#[derive(Debug, Clone)]
pub struct NamespacePathBuilder {
    /// Namespace path (excluding root id).
    path: Vec<NamespaceIdDto>,
    /// Namespace alias.
    alias: NamespaceAliasBuilder,
}


impl NamespacePathBuilder {
    /// Creates an instance of NamespacePathBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A NamespacePathBuilder.
    pub fn from_binary(bytes_: &[u8]) -> Self {
        let mut buf = [0x0u8; 1];
        buf.copy_from_slice(&bytes_[..1]);
        let pathSize = u8::from_le_bytes(buf); // kind:SIZE_FIELD
        let mut bytes_ = (&bytes_[1..]).to_vec();
        let mut path: Vec<NamespaceIdDto> = vec![]; // kind:ARRAY
        let mut bytes_ = bytes_.to_vec();
        for _ in 0..pathSize {
            let item = NamespaceIdDto::from_binary(&bytes_);
            path.push(item.clone());
            bytes_ = (&bytes_[item.get_size()..]).to_vec();
        }
        let alias = NamespaceAliasBuilder::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[alias.get_size()..].to_vec();
        NamespacePathBuilder { path, alias }
    }

    /// Gets namespace path (excluding root id).
    ///
    /// # Returns
    /// A Namespace path (excluding root id).
    pub fn get_path(&self) -> Vec<NamespaceIdDto> {
        self.path.clone() // ARRAY or FILL_ARRAY
    }

    /// Gets namespace alias.
    ///
    /// # Returns
    /// A Namespace alias.
    pub fn get_alias(&self) -> NamespaceAliasBuilder {
        self.alias.clone()
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += 1; // path_size;
        size += self.path.iter().map(|item| item.get_size()).sum::<usize>(); // array or fill_array;
        size += self.alias.get_size();
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut (self.get_path().len() as u8).to_le_bytes().to_vec()); // kind:SIZE_FIELD
        for i in &self.path {
            buf.append(&mut i.serializer()); // kind:ARRAY|FILL_ARRAY
        }
        buf.append(&mut self.alias.serializer()); // kind:CUSTOM
        buf
    }
}

