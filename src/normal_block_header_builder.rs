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
use super::block_fee_multiplier_dto::*;
use super::block_header_builder::*;
use super::difficulty_dto::*;
use super::entity_type_dto::*;
use super::hash256_dto::*;
use super::height_dto::*;
use super::key_dto::*;
use super::network_type_dto::*;
use super::signature_dto::*;
use super::timestamp_dto::*;
use super::vrf_proof_builder::*;

/// Binary layout for a normal block header.
#[derive(Debug, Clone)]
pub struct NormalBlockHeaderBuilder {
    /// Block header.
    super_object: BlockHeaderBuilder,
}


impl NormalBlockHeaderBuilder {

    /// Creates an instance of NormalBlockHeaderBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A NormalBlockHeaderBuilder.
    pub fn from_binary(bytes_: &[u8]) -> Self {
        let super_object = BlockHeaderBuilder::from_binary(bytes_);
        let mut bytes_ = bytes_[super_object.get_size()..].to_vec();
        NormalBlockHeaderBuilder{super_object}
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = self.super_object.get_size();
        size += 4; // block_header__reserved1;
        size
   }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.super_object.serializer());
        buf.append(&mut 4u16.to_le_bytes().to_vec());
        buf
    }
}

