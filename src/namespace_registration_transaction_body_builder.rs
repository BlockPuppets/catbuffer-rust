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
use super::namespace_id_dto::*;
use super::namespace_registration_type_dto::*;

/// Binary layout for a namespace registration transaction.
#[derive(Debug, Clone)]
pub struct NamespaceRegistrationTransactionBodyBuilder {
    /// Namespace duration.
    pub duration: Option<BlockDurationDto>,
    /// Parent namespace identifier.
    pub parent_id: Option<NamespaceIdDto>,
    /// Namespace identifier.
    pub id: NamespaceIdDto,
    /// Namespace registration type.
    pub registration_type: NamespaceRegistrationTypeDto,
    /// Namespace name.
    pub name: Vec<u8>,
}

impl NamespaceRegistrationTransactionBodyBuilder {
    /// Creates an instance of NamespaceRegistrationTransactionBodyBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A NamespaceRegistrationTransactionBodyBuilder.
    pub fn from_binary(payload: &[u8]) -> Self {
        let mut bytes_ = payload.to_vec();
        let registration_type_condition = bytes_[0..8].to_vec();
        let bytes_ = &bytes_[8..];
        let id = NamespaceIdDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[id.get_size()..].to_vec();
        let registration_type = NamespaceRegistrationTypeDto::from_binary(&bytes_); // kind:CUSTOM2
        let mut bytes_ = bytes_[registration_type.get_size()..].to_vec();
        let mut buf = [0x0u8; 1];
        buf.copy_from_slice(&bytes_[..1]);
        let name_size = u8::from_le_bytes(buf); // kind:SIZE_FIELD
        let mut bytes_ = (&bytes_[1..]).to_vec();
        let name = (&bytes_[..name_size as usize]).to_vec(); // kind:BUFFER
        let bytes_ = (&bytes_[name_size as usize..]).to_vec();
        let mut duration = None;
        if registration_type == NamespaceRegistrationTypeDto::ROOT {
            duration = Some(BlockDurationDto::from_binary(&registration_type_condition)); // kind:CUSTOM3
        }
        let mut parent_id = None;
        if registration_type == NamespaceRegistrationTypeDto::CHILD {
            parent_id = Some(NamespaceIdDto::from_binary(&registration_type_condition)); // kind:CUSTOM3
        }
        // create object and call.
        NamespaceRegistrationTransactionBodyBuilder { duration, parent_id, id, registration_type, name } // TransactionBody
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        if self.registration_type == NamespaceRegistrationTypeDto::ROOT {
            size += self.duration.as_ref().unwrap().get_size() // Conditional;
        }
        if self.registration_type == NamespaceRegistrationTypeDto::CHILD {
            size += self.parent_id.as_ref().unwrap().get_size() // Conditional;
        }
        size += self.id.get_size(); // id_size;
        size += self.registration_type.get_size(); // registration_type_size;
        size += 1;  // name_size;
        size += self.name.len();
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        if self.registration_type == NamespaceRegistrationTypeDto::ROOT {
            buf.append(&mut self.duration.as_ref().unwrap().serializer()); // kind:CUSTOM
        }
        if self.registration_type == NamespaceRegistrationTypeDto::CHILD {
            buf.append(&mut self.parent_id.as_ref().unwrap().serializer()); // kind:CUSTOM
        }
        buf.append(&mut self.id.serializer()); // kind:CUSTOM
        buf.append(&mut self.registration_type.serializer()); // kind:CUSTOM
        let size_value: u8 = self.name.len() as u8;
        buf.append(&mut size_value.to_le_bytes().to_vec()); // kind:SIZE_FIELD
        buf.append(&mut self.name.clone()); // kind:BUFFER
        buf
    }
}

