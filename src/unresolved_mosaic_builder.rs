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
use super::unresolved_mosaic_id_dto::*;

/// Binary layout for an unresolved mosaic.
#[derive(Debug, Clone)]
pub struct UnresolvedMosaicBuilder {
    /// Mosaic identifier.
    mosaic_id: UnresolvedMosaicIdDto,
    /// Mosaic amount.
    amount: AmountDto,
}


impl UnresolvedMosaicBuilder {

    /// Creates an instance of UnresolvedMosaicBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A UnresolvedMosaicBuilder.
    pub fn from_binary(bytes_: &[u8]) -> Self {
        let mosaic_id = UnresolvedMosaicIdDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[mosaic_id.get_size()..].to_vec();
        let amount = AmountDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[amount.get_size()..].to_vec();
        UnresolvedMosaicBuilder{mosaic_id, amount}
    }

    /// Gets mosaic identifier.
    ///
    /// # Returns
    /// A Mosaic identifier.
    pub fn get_mosaic_id(&self) -> UnresolvedMosaicIdDto {
        self.mosaic_id.clone()
    }

    /// Gets mosaic amount.
    ///
    /// # Returns
    /// A Mosaic amount.
    pub fn get_amount(&self) -> AmountDto {
        self.amount.clone()
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += self.mosaic_id.get_size();
        size += self.amount.get_size();
        size
   }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.mosaic_id.serializer()); // kind:CUSTOM
        buf.append(&mut self.amount.serializer()); // kind:CUSTOM
        buf
    }
}

