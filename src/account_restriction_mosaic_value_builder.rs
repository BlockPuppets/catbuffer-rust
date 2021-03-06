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

/// Binary layout for mosaic id based account restriction.
#[derive(Debug, Clone)]
pub struct AccountRestrictionMosaicValueBuilder {
    /// Restriction values.
    restriction_values: Vec<MosaicIdDto>,
}


impl AccountRestrictionMosaicValueBuilder {
    /// Creates an instance of AccountRestrictionMosaicValueBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A AccountRestrictionMosaicValueBuilder.
    pub fn from_binary(_bytes: &[u8]) -> Self {
        let buf = fixed_bytes::<8>(&_bytes);
        let restrictionValuesCount = u64::from_le_bytes(buf); // kind:SIZE_FIELD
        let mut _bytes = (&_bytes[8..]).to_vec();
        let mut restriction_values: Vec<MosaicIdDto> = vec![]; // kind:ARRAY
        let mut _bytes = _bytes.to_vec();
        for _ in 0..restrictionValuesCount {
            let item = MosaicIdDto::from_binary(&_bytes);
            restriction_values.push(item.clone());
            _bytes = (&_bytes[item.get_size()..]).to_vec();
        }
        AccountRestrictionMosaicValueBuilder { restriction_values }
    }

    /// Gets restriction values.
    ///
    /// # Returns
    /// A Restriction values.
    pub fn get_restriction_values(&self) -> Vec<MosaicIdDto> {
        self.restriction_values.clone() // ARRAY or FILL_ARRAY
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += 8; // restriction_values_count;
        size += self.restriction_values.iter().map(|item| item.get_size()).sum::<usize>(); // array or fill_array;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut (self.get_restriction_values().len() as u64).to_le_bytes().to_vec()); // kind:SIZE_FIELD
        for i in &self.restriction_values {
            buf.append(&mut i.serializer()); // kind:ARRAY|FILL_ARRAY
        }
        buf
    }
}

