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

use super::cosignature_builder::*;
use super::generator_utils::*;
use super::hash256_dto::*;
use super::key_dto::*;
use super::signature_dto::*;

/// Cosignature detached from an aggregate transaction.
#[derive(Debug, Clone)]
pub struct DetachedCosignatureBuilder {
    /// Cosignature.
    super_object: CosignatureBuilder,
    /// Hash of the aggregate transaction that is signed by this cosignature.
    parent_hash: Hash256Dto,
}


impl DetachedCosignatureBuilder {
    /// Creates an instance of DetachedCosignatureBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A DetachedCosignatureBuilder.
    pub fn from_binary(_bytes: &[u8]) -> Self {
        let super_object = CosignatureBuilder::from_binary(_bytes);
        let mut _bytes = _bytes[super_object.get_size()..].to_vec();
        let parent_hash = Hash256Dto::from_binary(&_bytes); // kind:CUSTOM1
        let mut _bytes = _bytes[parent_hash.get_size()..].to_vec();
        DetachedCosignatureBuilder { super_object, parent_hash }
    }

    /// Gets hash of the aggregate transaction that is signed by this cosignature.
    ///
    /// # Returns
    /// A Hash of the aggregate transaction that is signed by this cosignature.
    pub fn get_parent_hash(&self) -> Hash256Dto {
        self.parent_hash.clone()
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = self.super_object.get_size();
        size += self.parent_hash.get_size(); // parent_hash;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.super_object.serializer());
        buf.append(&mut self.parent_hash.serializer()); // kind:CUSTOM
        buf
    }
}

