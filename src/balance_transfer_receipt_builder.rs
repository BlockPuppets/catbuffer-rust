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

use super::address_dto::*;
use super::generator_utils::*;
use super::mosaic_builder::*;
use super::receipt_builder::*;
use super::receipt_type_dto::*;

/// Binary layout for a balance transfer receipt.
#[derive(Debug, Clone)]
pub struct BalanceTransferReceiptBuilder {
    /// Receipt.
    super_object: ReceiptBuilder,
    /// Mosaic.
    mosaic: MosaicBuilder,
    /// Mosaic sender address.
    sender_address: AddressDto,
    /// Mosaic recipient address.
    recipient_address: AddressDto,
}


impl BalanceTransferReceiptBuilder {
    /// Creates an instance of BalanceTransferReceiptBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A BalanceTransferReceiptBuilder.
    pub fn from_binary(_bytes: &[u8]) -> Self {
        let super_object = ReceiptBuilder::from_binary(_bytes);
        let mut _bytes = _bytes[super_object.get_size()..].to_vec();
        let mosaic = MosaicBuilder::from_binary(&_bytes); // kind:CUSTOM1
        let mut _bytes = _bytes[mosaic.get_size()..].to_vec();
        let sender_address = AddressDto::from_binary(&_bytes); // kind:CUSTOM1
        let mut _bytes = _bytes[sender_address.get_size()..].to_vec();
        let recipient_address = AddressDto::from_binary(&_bytes); // kind:CUSTOM1
        let mut _bytes = _bytes[recipient_address.get_size()..].to_vec();
        BalanceTransferReceiptBuilder { super_object, mosaic, sender_address, recipient_address }
    }

    /// Gets mosaic.
    ///
    /// # Returns
    /// A Mosaic.
    pub fn get_mosaic(&self) -> MosaicBuilder {
        self.mosaic.clone()
    }

    /// Gets mosaic sender address.
    ///
    /// # Returns
    /// A Mosaic sender address.
    pub fn get_sender_address(&self) -> AddressDto {
        self.sender_address.clone()
    }

    /// Gets mosaic recipient address.
    ///
    /// # Returns
    /// A Mosaic recipient address.
    pub fn get_recipient_address(&self) -> AddressDto {
        self.recipient_address.clone()
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = self.super_object.get_size();
        size += self.mosaic.get_size(); // mosaic;
        size += self.sender_address.get_size(); // sender_address;
        size += self.recipient_address.get_size(); // recipient_address;
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
        buf.append(&mut self.sender_address.serializer()); // kind:CUSTOM
        buf.append(&mut self.recipient_address.serializer()); // kind:CUSTOM
        buf
    }
}

