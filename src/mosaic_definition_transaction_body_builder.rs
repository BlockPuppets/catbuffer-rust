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

use super::block_duration_dto::*;
use super::generator_utils::*;
use super::mosaic_flags_dto::*;
use super::mosaic_id_dto::*;
use super::mosaic_nonce_dto::*;

/// Binary layout for a mosaic definition transaction.
#[derive(Debug, Clone)]
pub struct MosaicDefinitionTransactionBodyBuilder {
    /// Mosaic identifier.
    pub id: MosaicIdDto,
    /// Mosaic duration.
    pub duration: BlockDurationDto,
    /// Mosaic nonce.
    pub nonce: MosaicNonceDto,
    /// Mosaic flags.
    pub flags: Vec<MosaicFlagsDto>,
    /// Mosaic divisibility.
    pub divisibility: u8,
}

impl MosaicDefinitionTransactionBodyBuilder {
    /// Creates an instance of MosaicDefinitionTransactionBodyBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A MosaicDefinitionTransactionBodyBuilder.
    pub fn from_binary(payload: &[u8]) -> Self {
        let mut _bytes = payload.to_vec();
        let id = MosaicIdDto::from_binary(&_bytes); // kind:CUSTOM1
        _bytes = _bytes[id.get_size()..].to_vec();
        let duration = BlockDurationDto::from_binary(&_bytes); // kind:CUSTOM1
        _bytes = _bytes[duration.get_size()..].to_vec();
        let nonce = MosaicNonceDto::from_binary(&_bytes); // kind:CUSTOM1
        _bytes = _bytes[nonce.get_size()..].to_vec();
        let flags = MosaicFlagsDto::bytes_to_flags(&_bytes[..1]); // kind:FLAGS
        let mut _bytes = (&_bytes[1..]).to_vec();
        let buf = fixed_bytes::<1>(&_bytes);
        let divisibility = u8::from_le_bytes(buf); // kind:SIMPLE
        _bytes = (&_bytes[1..]).to_vec();
        // create object and call.
        MosaicDefinitionTransactionBodyBuilder { id, duration, nonce, flags, divisibility } // TransactionBody
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += self.id.get_size(); // id_size;
        size += self.duration.get_size(); // duration_size;
        size += self.nonce.get_size(); // nonce_size;
        size += 1;  // flags;
        size += 1;  // divisibility;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.id.serializer()); // kind:CUSTOM
        buf.append(&mut self.duration.serializer()); // kind:CUSTOM
        buf.append(&mut self.nonce.serializer()); // kind:CUSTOM
        buf.append(&mut MosaicFlagsDto::flags_to_int(self.flags.clone()).to_le_bytes().to_vec()); // kind:FLAGS
        buf.append(&mut self.divisibility.to_le_bytes().to_vec()); // serial_kind:SIMPLE
        buf
    }
}

