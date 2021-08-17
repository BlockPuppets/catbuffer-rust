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
use super::mosaic_restriction_type_dto::*;
use super::unresolved_mosaic_id_dto::*;

/// Binary layout for a mosaic global restriction transaction.
#[derive(Debug, Clone)]
pub struct MosaicGlobalRestrictionTransactionBodyBuilder {
    /// Identifier of the mosaic being restricted.
    pub mosaic_id: UnresolvedMosaicIdDto,
    /// Identifier of the mosaic providing the restriction key.
    pub reference_mosaic_id: UnresolvedMosaicIdDto,
    /// Restriction key relative to the reference mosaic identifier.
    pub restriction_key: u64,
    /// Previous restriction value.
    pub previous_restriction_value: u64,
    /// New restriction value.
    pub new_restriction_value: u64,
    /// Previous restriction type.
    pub previous_restriction_type: MosaicRestrictionTypeDto,
    /// New restriction type.
    pub new_restriction_type: MosaicRestrictionTypeDto,
}

impl MosaicGlobalRestrictionTransactionBodyBuilder {
    /// Creates an instance of MosaicGlobalRestrictionTransactionBodyBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A MosaicGlobalRestrictionTransactionBodyBuilder.
    pub fn from_binary(payload: &[u8]) -> Self {
        let mut _bytes = payload.to_vec();
        let mosaic_id = UnresolvedMosaicIdDto::from_binary(&_bytes); // kind:CUSTOM1
        _bytes = _bytes[mosaic_id.get_size()..].to_vec();
        let reference_mosaic_id = UnresolvedMosaicIdDto::from_binary(&_bytes); // kind:CUSTOM1
        _bytes = _bytes[reference_mosaic_id.get_size()..].to_vec();
        let buf = fixed_bytes::<8>(&_bytes);
        let restriction_key = u64::from_le_bytes(buf); // kind:SIMPLE
        _bytes = (&_bytes[8..]).to_vec();
        let buf = fixed_bytes::<8>(&_bytes);
        let previous_restriction_value = u64::from_le_bytes(buf); // kind:SIMPLE
        _bytes = (&_bytes[8..]).to_vec();
        let buf = fixed_bytes::<8>(&_bytes);
        let new_restriction_value = u64::from_le_bytes(buf); // kind:SIMPLE
        _bytes = (&_bytes[8..]).to_vec();
        let previous_restriction_type = MosaicRestrictionTypeDto::from_binary(&_bytes); // kind:CUSTOM2
        _bytes = (&_bytes[previous_restriction_type.get_size()..]).to_vec();
        let new_restriction_type = MosaicRestrictionTypeDto::from_binary(&_bytes); // kind:CUSTOM2
        _bytes = (&_bytes[new_restriction_type.get_size()..]).to_vec();
        // create object and call.
        MosaicGlobalRestrictionTransactionBodyBuilder { mosaic_id, reference_mosaic_id, restriction_key, previous_restriction_value, new_restriction_value, previous_restriction_type, new_restriction_type } // TransactionBody
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += self.mosaic_id.get_size(); // mosaic_id_size;
        size += self.reference_mosaic_id.get_size(); // reference_mosaic_id_size;
        size += 8;  // restriction_key;
        size += 8;  // previous_restriction_value;
        size += 8;  // new_restriction_value;
        size += self.previous_restriction_type.get_size(); // previous_restriction_type_size;
        size += self.new_restriction_type.get_size(); // new_restriction_type_size;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.mosaic_id.serializer()); // kind:CUSTOM
        buf.append(&mut self.reference_mosaic_id.serializer()); // kind:CUSTOM
        buf.append(&mut self.restriction_key.to_le_bytes().to_vec()); // serial_kind:SIMPLE
        buf.append(&mut self.previous_restriction_value.to_le_bytes().to_vec()); // serial_kind:SIMPLE
        buf.append(&mut self.new_restriction_value.to_le_bytes().to_vec()); // serial_kind:SIMPLE
        buf.append(&mut self.previous_restriction_type.serializer()); // kind:CUSTOM
        buf.append(&mut self.new_restriction_type.serializer()); // kind:CUSTOM
        buf
    }
}

