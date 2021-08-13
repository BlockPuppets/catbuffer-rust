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
use super::mosaic_supply_change_action_dto::*;
use super::unresolved_mosaic_id_dto::*;

/// Binary layout for a mosaic supply change transaction.
#[derive(Debug, Clone)]
pub struct MosaicSupplyChangeTransactionBodyBuilder {
    /// Affected mosaic identifier.
    pub mosaic_id: UnresolvedMosaicIdDto,
    /// Change amount.
    pub delta: AmountDto,
    /// Supply change action.
    pub action: MosaicSupplyChangeActionDto,
}

impl MosaicSupplyChangeTransactionBodyBuilder {



    /// Creates an instance of MosaicSupplyChangeTransactionBodyBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A MosaicSupplyChangeTransactionBodyBuilder.
    pub fn from_binary(payload: &[u8]) -> Self {
        let mut bytes_ = payload.to_vec();
        let mosaic_id = UnresolvedMosaicIdDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[mosaic_id.get_size()..].to_vec();
        let delta = AmountDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[delta.get_size()..].to_vec();
        let action = MosaicSupplyChangeActionDto::from_binary(&bytes_); // kind:CUSTOM2
        let mut bytes_ = bytes_[action.get_size()..].to_vec();
        // create object and call.
        MosaicSupplyChangeTransactionBodyBuilder{ mosaic_id, delta, action } // TransactionBody
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
   pub fn get_size(&self) -> usize {
       let mut size = 0;
        size += self.mosaic_id.get_size(); // mosaic_id_size;
        size += self.delta.get_size(); // delta_size;
        size += self.action.get_size(); // action_size;
        size
   }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.mosaic_id.serializer()); // kind:CUSTOM
        buf.append(&mut self.delta.serializer()); // kind:CUSTOM
        buf.append(&mut self.action.serializer()); // kind:CUSTOM
        buf
    }
}

