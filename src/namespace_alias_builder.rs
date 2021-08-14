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
use super::mosaic_id_dto::*;
use super::namespace_alias_type_dto::*;

/// Binary layout for alias.
#[derive(Debug, Clone)]
pub struct NamespaceAliasBuilder {
    /// Namespace alias type.
    namespace_alias_type: NamespaceAliasTypeDto,
    /// Mosaic alias.
    mosaic_alias: Option<MosaicIdDto>,
    /// Address alias.
    address_alias: Option<AddressDto>,
}


impl NamespaceAliasBuilder {
    /// Creates an instance of NamespaceAliasBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A NamespaceAliasBuilder.
    pub fn from_binary(bytes_: &[u8]) -> Self {
        let namespace_alias_type = NamespaceAliasTypeDto::from_binary(&bytes_); // kind:CUSTOM2
        let mut bytes_ = bytes_[namespace_alias_type.get_size()..].to_vec();
        let mut mosaic_alias = None;
        if namespace_alias_type == NamespaceAliasTypeDto::MOSAIC_ID {
            let raw_mosaic_alias = MosaicIdDto::from_binary(&bytes_);
            bytes_ = (&bytes_[raw_mosaic_alias.get_size()..]).to_vec();
            mosaic_alias = Some(raw_mosaic_alias); // kind:CUSTOM1
        }
        let mut address_alias = None;
        if namespace_alias_type == NamespaceAliasTypeDto::ADDRESS {
            let raw_address_alias = AddressDto::from_binary(&bytes_);
            bytes_ = (&bytes_[raw_address_alias.get_size()..]).to_vec();
            address_alias = Some(raw_address_alias); // kind:CUSTOM1
        }
        NamespaceAliasBuilder { namespace_alias_type, mosaic_alias, address_alias }
    }

    /// Gets namespace alias type.
    ///
    /// # Returns
    /// A Namespace alias type.
    pub fn get_namespace_alias_type(&self) -> NamespaceAliasTypeDto {
        self.namespace_alias_type.clone()
    }

    /// Gets mosaic alias.
    ///
    /// # Returns
    /// A Mosaic alias.
    pub fn get_mosaic_alias(&self) -> Option<MosaicIdDto> {
        if self.namespace_alias_type != NamespaceAliasTypeDto::MOSAIC_ID {
            panic!("namespaceAliasType is not set to MOSAIC_ID.")
        };
        self.mosaic_alias.clone()
    }

    /// Gets address alias.
    ///
    /// # Returns
    /// A Address alias.
    pub fn get_address_alias(&self) -> Option<AddressDto> {
        if self.namespace_alias_type != NamespaceAliasTypeDto::ADDRESS {
            panic!("namespaceAliasType is not set to ADDRESS.")
        };
        self.address_alias.clone()
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += self.namespace_alias_type.get_size();
        if self.namespace_alias_type == NamespaceAliasTypeDto::MOSAIC_ID {
            size += self.mosaic_alias.as_ref().unwrap().get_size();
        }
        if self.namespace_alias_type == NamespaceAliasTypeDto::ADDRESS {
            size += self.address_alias.as_ref().unwrap().get_size();
        }
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.namespace_alias_type.serializer()); // kind:CUSTOM
        if self.namespace_alias_type == NamespaceAliasTypeDto::MOSAIC_ID {
            buf.append(&mut self.mosaic_alias.as_ref().unwrap().serializer()); // kind:CUSTOM
        };
        if self.namespace_alias_type == NamespaceAliasTypeDto::ADDRESS {
            buf.append(&mut self.address_alias.as_ref().unwrap().serializer()); // kind:CUSTOM
        };
        buf
    }
}

