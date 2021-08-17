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
use super::unresolved_address_dto::*;
use super::unresolved_mosaic_builder::*;

/// Binary layout for a transfer transaction.
#[derive(Debug, Clone)]
pub struct TransferTransactionBodyBuilder {
    /// Recipient address.
    pub recipient_address: UnresolvedAddressDto,
    /// Attached mosaics.
    pub mosaics: Vec<UnresolvedMosaicBuilder>,
    /// Attached message.
    pub message: Vec<u8>,
}

impl TransferTransactionBodyBuilder {
    /// Creates an instance of TransferTransactionBodyBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A TransferTransactionBodyBuilder.
    pub fn from_binary(payload: &[u8]) -> Self {
        let mut _bytes = payload.to_vec();
        let recipient_address = UnresolvedAddressDto::from_binary(&_bytes); // kind:CUSTOM1
        _bytes = _bytes[recipient_address.get_size()..].to_vec();
        let buf = fixed_bytes::<2>(&_bytes);
        let message_size = u16::from_le_bytes(buf); // kind:SIZE_FIELD
        _bytes = (&_bytes[2..]).to_vec();
        let buf = fixed_bytes::<1>(&_bytes);
        let mosaics_count = u8::from_le_bytes(buf); // kind:SIZE_FIELD
        _bytes = (&_bytes[1..]).to_vec();
        let buf = fixed_bytes::<4>(&_bytes);
        let _ = u32::from_le_bytes(buf); // kind:SIMPLE
        _bytes = (&_bytes[4..]).to_vec();
        let buf = fixed_bytes::<1>(&_bytes);
        let _ = u8::from_le_bytes(buf); // kind:SIMPLE
        _bytes = (&_bytes[1..]).to_vec();
        let mut mosaics: Vec<UnresolvedMosaicBuilder> = vec![]; // kind:ARRAY
        for _ in 0..mosaics_count {
            let item = UnresolvedMosaicBuilder::from_binary(&_bytes);
            mosaics.push(item.clone());
            _bytes = (&_bytes[item.get_size()..]).to_vec();
        }
        let message = (&_bytes[..message_size as usize]).to_vec(); // kind:BUFFER
        _bytes = (&_bytes[message_size as usize..]).to_vec();
        // create object and call.
        TransferTransactionBodyBuilder { recipient_address, mosaics, message } // TransactionBody
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += self.recipient_address.get_size(); // recipient_address_size;
        size += 2;  // message_size;
        size += 1;  // mosaics_count;
        size += 4;  // transfer_transaction_body__reserved1;
        size += 1;  // transfer_transaction_body__reserved2;
        for i in &self.mosaics {
            size += i.get_size(); // FILL_ARRAY
        };
        size += self.message.len();
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.recipient_address.serializer()); // kind:CUSTOM
        let size_value: u16 = self.message.len() as u16;
        buf.append(&mut size_value.to_le_bytes().to_vec()); // kind:SIZE_FIELD
        let size_value: u8 = self.mosaics.len() as u8;
        buf.append(&mut size_value.to_le_bytes().to_vec()); // kind:SIZE_FIELD
        buf.append(&mut [0u8; 4].to_vec()); // kind:SIMPLE and is_reserved
        buf.append(&mut [0u8; 1].to_vec()); // kind:SIMPLE and is_reserved
        for i in &self.mosaics {
            buf.append(&mut i.serializer()); // kind:ARRAY|FILL_ARRAY
        }
        buf.append(&mut self.message.clone()); // kind:BUFFER
        buf
    }
}

