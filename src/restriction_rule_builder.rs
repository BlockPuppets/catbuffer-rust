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
use super::mosaic_id_dto::*;
use super::mosaic_restriction_type_dto::*;

/// Binary layout of restriction rule being applied.
#[derive(Debug, Clone)]
pub struct RestrictionRuleBuilder {
    /// Identifier of the mosaic providing the restriction key.
    reference_mosaic_id: MosaicIdDto,
    /// Restriction value.
    restriction_value: u64,
    /// Restriction type.
    restriction_type: MosaicRestrictionTypeDto,
}


impl RestrictionRuleBuilder {
    /// Creates an instance of RestrictionRuleBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A RestrictionRuleBuilder.
    pub fn from_binary(_bytes: &[u8]) -> Self {
        let reference_mosaic_id = MosaicIdDto::from_binary(&_bytes); // kind:CUSTOM1
        let mut _bytes = _bytes[reference_mosaic_id.get_size()..].to_vec();
        let buf = fixed_bytes::<8>(&_bytes);
        let restriction_value = u64::from_le_bytes(buf); // kind:SIMPLE
        let _bytes = (&_bytes[8..]).to_vec();
        let restriction_type = MosaicRestrictionTypeDto::from_binary(&_bytes); // kind:CUSTOM2
        let mut _bytes = _bytes[restriction_type.get_size()..].to_vec();
        RestrictionRuleBuilder { reference_mosaic_id, restriction_value, restriction_type }
    }

    /// Gets identifier of the mosaic providing the restriction key.
    ///
    /// # Returns
    /// A Identifier of the mosaic providing the restriction key.
    pub fn get_reference_mosaic_id(&self) -> MosaicIdDto {
        self.reference_mosaic_id.clone()
    }

    /// Gets restriction value.
    ///
    /// # Returns
    /// A Restriction value.
    pub fn get_restriction_value(&self) -> u64 {
        self.restriction_value.clone()
    }

    /// Gets restriction type.
    ///
    /// # Returns
    /// A Restriction type.
    pub fn get_restriction_type(&self) -> MosaicRestrictionTypeDto {
        self.restriction_type.clone()
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += self.reference_mosaic_id.get_size(); // reference_mosaic_id;
        size += 8; // restriction_value;
        size += self.restriction_type.get_size(); // restriction_type;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.reference_mosaic_id.serializer()); // kind:CUSTOM
        buf.append(&mut self.get_restriction_value().to_le_bytes().to_vec()); // kind:SIMPLE
        buf.append(&mut self.restriction_type.serializer()); // kind:CUSTOM
        buf
    }
}

