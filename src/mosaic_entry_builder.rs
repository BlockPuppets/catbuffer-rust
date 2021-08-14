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

use super::amount_dto::*;
use super::generator_utils::*;
use super::mosaic_definition_builder::*;
use super::mosaic_id_dto::*;
use super::state_header_builder::*;

/// Binary layout for mosaic entry.
#[derive(Debug, Clone)]
pub struct MosaicEntryBuilder {
    /// State header.
    super_object: StateHeaderBuilder,
    /// Entry id.
    mosaic_id: MosaicIdDto,
    /// Total supply amount.
    supply: AmountDto,
    /// Definition comprised of entry properties.
    definition: MosaicDefinitionBuilder,
}


impl MosaicEntryBuilder {
    /// Creates an instance of MosaicEntryBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A MosaicEntryBuilder.
    pub fn from_binary(bytes_: &[u8]) -> Self {
        let super_object = StateHeaderBuilder::from_binary(bytes_);
        let mut bytes_ = bytes_[super_object.get_size()..].to_vec();
        let mosaic_id = MosaicIdDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[mosaic_id.get_size()..].to_vec();
        let supply = AmountDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[supply.get_size()..].to_vec();
        let definition = MosaicDefinitionBuilder::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[definition.get_size()..].to_vec();
        MosaicEntryBuilder { super_object, mosaic_id, supply, definition }
    }

    /// Gets entry id.
    ///
    /// # Returns
    /// A Entry id.
    pub fn get_mosaic_id(&self) -> MosaicIdDto {
        self.mosaic_id.clone()
    }

    /// Gets total supply amount.
    ///
    /// # Returns
    /// A Total supply amount.
    pub fn get_supply(&self) -> AmountDto {
        self.supply.clone()
    }

    /// Gets definition comprised of entry properties.
    ///
    /// # Returns
    /// A Definition comprised of entry properties.
    pub fn get_definition(&self) -> MosaicDefinitionBuilder {
        self.definition.clone()
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = self.super_object.get_size();
        size += self.mosaic_id.get_size(); // mosaic_id;
        size += self.supply.get_size(); // supply;
        size += self.definition.get_size(); // definition;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.super_object.serializer());
        buf.append(&mut self.mosaic_id.serializer()); // kind:CUSTOM
        buf.append(&mut self.supply.serializer()); // kind:CUSTOM
        buf.append(&mut self.definition.serializer()); // kind:CUSTOM
        buf
    }
}

