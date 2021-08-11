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

use super::mosaic_restriction_key_dto::*;
use super::restriction_rule_builder::*;

/// Binary layout for a global key-value.
#[derive(Debug, Clone)]
pub struct GlobalKeyValueBuilder {
    /// Key associated with a restriction rule.
    key: MosaicRestrictionKeyDto,
    /// Restriction rule (the value) associated with a key.
    restriction_rule: RestrictionRuleBuilder,
}


impl GlobalKeyValueBuilder {

    /// Creates an instance of GlobalKeyValueBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A GlobalKeyValueBuilder.
    pub fn from_binary(bytes_: &[u8]) -> Self {
        let key = MosaicRestrictionKeyDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[key.get_size()..].to_vec();
        let restriction_rule = RestrictionRuleBuilder::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[restriction_rule.get_size()..].to_vec();
        GlobalKeyValueBuilder{key, restriction_rule}
    }

    /// Gets key associated with a restriction rule.
    ///
    /// # Returns
    /// A Key associated with a restriction rule.
    pub fn get_key(&self) -> MosaicRestrictionKeyDto {
        self.key.clone()
    }

    /// Gets restriction rule (the value) associated with a key.
    ///
    /// # Returns
    /// A Restriction rule (the value) associated with a key.
    pub fn get_restriction_rule(&self) -> RestrictionRuleBuilder {
        self.restriction_rule.clone()
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += self.key.get_size();
        size += self.restriction_rule.get_size();
        size
   }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.key.serializer()); // kind:CUSTOM
        buf.append(&mut self.restriction_rule.serializer()); // kind:CUSTOM
        buf
    }
}

