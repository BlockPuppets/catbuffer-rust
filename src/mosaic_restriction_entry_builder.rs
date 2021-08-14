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
use super::mosaic_address_restriction_entry_builder::*;
use super::mosaic_global_restriction_entry_builder::*;
use super::mosaic_restriction_entry_type_dto::*;
use super::state_header_builder::*;

/// Binary layout for a mosaic restriction.
#[derive(Debug, Clone)]
pub struct MosaicRestrictionEntryBuilder {
    /// State header.
    super_object: StateHeaderBuilder,
    /// Type of restriction being placed upon the entity.
    entry_type: MosaicRestrictionEntryTypeDto,
    /// Address restriction rule.
    address_entry: Option<MosaicAddressRestrictionEntryBuilder>,
    /// Global mosaic rule.
    global_entry: Option<MosaicGlobalRestrictionEntryBuilder>,
}


impl MosaicRestrictionEntryBuilder {
    /// Creates an instance of MosaicRestrictionEntryBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A MosaicRestrictionEntryBuilder.
    pub fn from_binary(bytes_: &[u8]) -> Self {
        let super_object = StateHeaderBuilder::from_binary(bytes_);
        let mut bytes_ = bytes_[super_object.get_size()..].to_vec();
        let entry_type = MosaicRestrictionEntryTypeDto::from_binary(&bytes_); // kind:CUSTOM2
        let mut bytes_ = bytes_[entry_type.get_size()..].to_vec();
        let mut address_entry = None;
        if entry_type == MosaicRestrictionEntryTypeDto::ADDRESS {
            let raw_address_entry = MosaicAddressRestrictionEntryBuilder::from_binary(&bytes_);
            bytes_ = (&bytes_[raw_address_entry.get_size()..]).to_vec();
            address_entry = Some(raw_address_entry); // kind:CUSTOM1
        }
        let mut global_entry = None;
        if entry_type == MosaicRestrictionEntryTypeDto::GLOBAL {
            let raw_global_entry = MosaicGlobalRestrictionEntryBuilder::from_binary(&bytes_);
            bytes_ = (&bytes_[raw_global_entry.get_size()..]).to_vec();
            global_entry = Some(raw_global_entry); // kind:CUSTOM1
        }
        MosaicRestrictionEntryBuilder { super_object, entry_type, address_entry, global_entry }
    }

    /// Gets type of restriction being placed upon the entity.
    ///
    /// # Returns
    /// A Type of restriction being placed upon the entity.
    pub fn get_entry_type(&self) -> MosaicRestrictionEntryTypeDto {
        self.entry_type.clone()
    }

    /// Gets address restriction rule.
    ///
    /// # Returns
    /// A Address restriction rule.
    pub fn get_address_entry(&self) -> Option<MosaicAddressRestrictionEntryBuilder> {
        if self.entry_type != MosaicRestrictionEntryTypeDto::ADDRESS {
            panic!("entryType is not set to ADDRESS.")
        };
        self.address_entry.clone()
    }

    /// Gets global mosaic rule.
    ///
    /// # Returns
    /// A Global mosaic rule.
    pub fn get_global_entry(&self) -> Option<MosaicGlobalRestrictionEntryBuilder> {
        if self.entry_type != MosaicRestrictionEntryTypeDto::GLOBAL {
            panic!("entryType is not set to GLOBAL.")
        };
        self.global_entry.clone()
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = self.super_object.get_size();
        size += self.entry_type.get_size(); // entry_type;
        if self.entry_type == MosaicRestrictionEntryTypeDto::ADDRESS {
            size += self.address_entry.as_ref().unwrap().get_size(); // address_entry
        }
        if self.entry_type == MosaicRestrictionEntryTypeDto::GLOBAL {
            size += self.global_entry.as_ref().unwrap().get_size(); // global_entry
        }
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.super_object.serializer());
        buf.append(&mut self.entry_type.serializer()); // kind:CUSTOM
        if self.entry_type == MosaicRestrictionEntryTypeDto::ADDRESS {
            buf.append(&mut self.address_entry.as_ref().unwrap().serializer()); // kind:CUSTOM
        };
        if self.entry_type == MosaicRestrictionEntryTypeDto::GLOBAL {
            buf.append(&mut self.global_entry.as_ref().unwrap().serializer()); // kind:CUSTOM
        };
        buf
    }
}

