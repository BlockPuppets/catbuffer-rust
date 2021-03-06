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
use super::key_dto::*;
use super::link_action_dto::*;

/// Binary layout for a vrf key link transaction.
#[derive(Debug, Clone)]
pub struct VrfKeyLinkTransactionBodyBuilder {
    /// Linked public key.
    pub linked_public_key: KeyDto,
    /// Link action.
    pub link_action: LinkActionDto,
}

impl VrfKeyLinkTransactionBodyBuilder {
    /// Creates an instance of VrfKeyLinkTransactionBodyBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A VrfKeyLinkTransactionBodyBuilder.
    pub fn from_binary(payload: &[u8]) -> Self {
        let mut _bytes = payload.to_vec();
        let linked_public_key = KeyDto::from_binary(&_bytes); // kind:CUSTOM1
        _bytes = _bytes[linked_public_key.get_size()..].to_vec();
        let link_action = LinkActionDto::from_binary(&_bytes); // kind:CUSTOM2
        _bytes = (&_bytes[link_action.get_size()..]).to_vec();
        // create object and call.
        VrfKeyLinkTransactionBodyBuilder { linked_public_key, link_action } // TransactionBody
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += self.linked_public_key.get_size(); // linked_public_key_size;
        size += self.link_action.get_size(); // link_action_size;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.linked_public_key.serializer()); // kind:CUSTOM
        buf.append(&mut self.link_action.serializer()); // kind:CUSTOM
        buf
    }
}

