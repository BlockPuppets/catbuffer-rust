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
use super::mosaic_builder::*;
use super::receipt_builder::*;
use super::receipt_type_dto::*;

/// Binary layout for an inflation receipt.
#[derive(Debug, Clone)]
pub struct InflationReceiptBuilder {
    /// Receipt.
    super_object: ReceiptBuilder,
    /// Mosaic.
    mosaic: MosaicBuilder,
}


impl InflationReceiptBuilder {
    /// Creates an instance of InflationReceiptBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A InflationReceiptBuilder.
    pub fn from_binary(_bytes: &[u8]) -> Self {
        let super_object = ReceiptBuilder::from_binary(_bytes);
        let mut _bytes = _bytes[super_object.get_size()..].to_vec();
        let mosaic = MosaicBuilder::from_binary(&_bytes); // kind:CUSTOM1
        let mut _bytes = _bytes[mosaic.get_size()..].to_vec();
        InflationReceiptBuilder { super_object, mosaic }
    }

    /// Gets mosaic.
    ///
    /// # Returns
    /// A Mosaic.
    pub fn get_mosaic(&self) -> MosaicBuilder {
        self.mosaic.clone()
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = self.super_object.get_size();
        size += self.mosaic.get_size(); // mosaic;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.super_object.serializer());
        buf.append(&mut self.mosaic.serializer()); // kind:CUSTOM
        buf
    }
}

