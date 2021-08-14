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
use super::namespace_alias_builder::*;
use super::namespace_id_dto::*;
use super::namespace_lifetime_builder::*;
use super::namespace_path_builder::*;
use super::state_header_builder::*;

/// Binary layout for non-historical root namespace history.
#[derive(Debug, Clone)]
pub struct RootNamespaceHistoryBuilder {
    /// State header.
    super_object: StateHeaderBuilder,
    /// Id of the root namespace history.
    id: NamespaceIdDto,
    /// Namespace owner address.
    owner_address: AddressDto,
    /// Lifetime in blocks.
    lifetime: NamespaceLifetimeBuilder,
    /// Root namespace alias.
    root_alias: NamespaceAliasBuilder,
    /// Save child sub-namespace paths.
    paths: Vec<NamespacePathBuilder>,
}


impl RootNamespaceHistoryBuilder {
    /// Creates an instance of RootNamespaceHistoryBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A RootNamespaceHistoryBuilder.
    pub fn from_binary(bytes_: &[u8]) -> Self {
        let super_object = StateHeaderBuilder::from_binary(bytes_);
        let mut bytes_ = bytes_[super_object.get_size()..].to_vec();
        let id = NamespaceIdDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[id.get_size()..].to_vec();
        let owner_address = AddressDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[owner_address.get_size()..].to_vec();
        let lifetime = NamespaceLifetimeBuilder::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[lifetime.get_size()..].to_vec();
        let root_alias = NamespaceAliasBuilder::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[root_alias.get_size()..].to_vec();
        let mut buf = fixed_bytes::<8>(&bytes_);
        let childrenCount = u64::from_le_bytes(buf); // kind:SIZE_FIELD
        let mut bytes_ = (&bytes_[8..]).to_vec();
        let mut paths: Vec<NamespacePathBuilder> = vec![]; // kind:ARRAY
        let mut bytes_ = bytes_.to_vec();
        for _ in 0..childrenCount {
            let item = NamespacePathBuilder::from_binary(&bytes_);
            paths.push(item.clone());
            bytes_ = (&bytes_[item.get_size()..]).to_vec();
        }
        RootNamespaceHistoryBuilder { super_object, id, owner_address, lifetime, root_alias, paths }
    }

    /// Gets id of the root namespace history.
    ///
    /// # Returns
    /// A Id of the root namespace history.
    pub fn get_id(&self) -> NamespaceIdDto {
        self.id.clone()
    }

    /// Gets namespace owner address.
    ///
    /// # Returns
    /// A Namespace owner address.
    pub fn get_owner_address(&self) -> AddressDto {
        self.owner_address.clone()
    }

    /// Gets lifetime in blocks.
    ///
    /// # Returns
    /// A Lifetime in blocks.
    pub fn get_lifetime(&self) -> NamespaceLifetimeBuilder {
        self.lifetime.clone()
    }

    /// Gets root namespace alias.
    ///
    /// # Returns
    /// A Root namespace alias.
    pub fn get_root_alias(&self) -> NamespaceAliasBuilder {
        self.root_alias.clone()
    }

    /// Gets save child sub-namespace paths.
    ///
    /// # Returns
    /// A Save child sub-namespace paths.
    pub fn get_paths(&self) -> Vec<NamespacePathBuilder> {
        self.paths.clone() // ARRAY or FILL_ARRAY
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = self.super_object.get_size();
        size += self.id.get_size(); // id;
        size += self.owner_address.get_size(); // owner_address;
        size += self.lifetime.get_size(); // lifetime;
        size += self.root_alias.get_size(); // root_alias;
        size += 8; // children_count;
        size += self.paths.iter().map(|item| item.get_size()).sum::<usize>(); // array or fill_array;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.super_object.serializer());
        buf.append(&mut self.id.serializer()); // kind:CUSTOM
        buf.append(&mut self.owner_address.serializer()); // kind:CUSTOM
        buf.append(&mut self.lifetime.serializer()); // kind:CUSTOM
        buf.append(&mut self.root_alias.serializer()); // kind:CUSTOM
        buf.append(&mut (self.get_paths().len() as u64).to_le_bytes().to_vec()); // kind:SIZE_FIELD
        for i in &self.paths {
            buf.append(&mut i.serializer()); // kind:ARRAY|FILL_ARRAY
        }
        buf
    }
}

