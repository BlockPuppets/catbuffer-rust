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

use super::alias_action_dto::*;
use super::generator_utils::*;
use super::mosaic_id_dto::*;
use super::namespace_id_dto::*;

/// Binary layout for an mosaic alias transaction.
#[derive(Debug, Clone)]
pub struct MosaicAliasTransactionBodyBuilder {
    /// Identifier of the namespace that will become an alias.
    pub namespace_id: NamespaceIdDto,
    /// Aliased mosaic identifier.
    pub mosaic_id: MosaicIdDto,
    /// Alias action.
    pub alias_action: AliasActionDto,
}

impl MosaicAliasTransactionBodyBuilder {
    /// Creates an instance of MosaicAliasTransactionBodyBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A MosaicAliasTransactionBodyBuilder.
    pub fn from_binary(payload: &[u8]) -> Self {
        let mut _bytes = payload.to_vec();
        let namespace_id = NamespaceIdDto::from_binary(&_bytes); // kind:CUSTOM1
        _bytes = _bytes[namespace_id.get_size()..].to_vec();
        let mosaic_id = MosaicIdDto::from_binary(&_bytes); // kind:CUSTOM1
        _bytes = _bytes[mosaic_id.get_size()..].to_vec();
        let alias_action = AliasActionDto::from_binary(&_bytes); // kind:CUSTOM2
        _bytes = (&_bytes[alias_action.get_size()..]).to_vec();
        // create object and call.
        MosaicAliasTransactionBodyBuilder { namespace_id, mosaic_id, alias_action } // TransactionBody
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += self.namespace_id.get_size(); // namespace_id_size;
        size += self.mosaic_id.get_size(); // mosaic_id_size;
        size += self.alias_action.get_size(); // alias_action_size;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.namespace_id.serializer()); // kind:CUSTOM
        buf.append(&mut self.mosaic_id.serializer()); // kind:CUSTOM
        buf.append(&mut self.alias_action.serializer()); // kind:CUSTOM
        buf
    }
}

