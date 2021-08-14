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
use super::state_header_builder::*;

/// Binary layout for a multisig entry.
#[derive(Debug, Clone)]
pub struct MultisigEntryBuilder {
    /// State header.
    super_object: StateHeaderBuilder,
    /// Minimum approval for modifications.
    min_approval: u32,
    /// Minimum approval for removal.
    min_removal: u32,
    /// Account address.
    account_address: AddressDto,
    /// Cosignatories for account.
    cosignatory_addresses: Vec<AddressDto>,
    /// Accounts for which the entry is cosignatory.
    multisig_addresses: Vec<AddressDto>,
}


impl MultisigEntryBuilder {
    /// Creates an instance of MultisigEntryBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A MultisigEntryBuilder.
    pub fn from_binary(bytes_: &[u8]) -> Self {
        let super_object = StateHeaderBuilder::from_binary(bytes_);
        let mut bytes_ = bytes_[super_object.get_size()..].to_vec();
        let mut buf = [0x0u8; 4];
        buf.copy_from_slice(&bytes_[..4]);
        let min_approval = u32::from_le_bytes(buf); // kind:SIMPLE
        let bytes_ = (&bytes_[4..]).to_vec();
        let mut buf = [0x0u8; 4];
        buf.copy_from_slice(&bytes_[..4]);
        let min_removal = u32::from_le_bytes(buf); // kind:SIMPLE
        let bytes_ = (&bytes_[4..]).to_vec();
        let account_address = AddressDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[account_address.get_size()..].to_vec();
        let mut buf = [0x0u8; 8];
        buf.copy_from_slice(&bytes_[..8]);
        let cosignatoryAddressesCount = u64::from_le_bytes(buf); // kind:SIZE_FIELD
        let mut bytes_ = (&bytes_[8..]).to_vec();
        let mut cosignatory_addresses: Vec<AddressDto> = vec![]; // kind:ARRAY
        let mut bytes_ = bytes_.to_vec();
        for _ in 0..cosignatoryAddressesCount {
            let item = AddressDto::from_binary(&bytes_);
            cosignatory_addresses.push(item.clone());
            bytes_ = (&bytes_[item.get_size()..]).to_vec();
        }
        let mut buf = [0x0u8; 8];
        buf.copy_from_slice(&bytes_[..8]);
        let multisigAddressesCount = u64::from_le_bytes(buf); // kind:SIZE_FIELD
        let mut bytes_ = (&bytes_[8..]).to_vec();
        let mut multisig_addresses: Vec<AddressDto> = vec![]; // kind:ARRAY
        let mut bytes_ = bytes_.to_vec();
        for _ in 0..multisigAddressesCount {
            let item = AddressDto::from_binary(&bytes_);
            multisig_addresses.push(item.clone());
            bytes_ = (&bytes_[item.get_size()..]).to_vec();
        }
        MultisigEntryBuilder { super_object, min_approval, min_removal, account_address, cosignatory_addresses, multisig_addresses }
    }

    /// Gets minimum approval for modifications.
    ///
    /// # Returns
    /// A Minimum approval for modifications.
    pub fn get_min_approval(&self) -> u32 {
        self.min_approval.clone()
    }

    /// Gets minimum approval for removal.
    ///
    /// # Returns
    /// A Minimum approval for removal.
    pub fn get_min_removal(&self) -> u32 {
        self.min_removal.clone()
    }

    /// Gets account address.
    ///
    /// # Returns
    /// A Account address.
    pub fn get_account_address(&self) -> AddressDto {
        self.account_address.clone()
    }

    /// Gets cosignatories for account.
    ///
    /// # Returns
    /// A Cosignatories for account.
    pub fn get_cosignatory_addresses(&self) -> Vec<AddressDto> {
        self.cosignatory_addresses.clone() // ARRAY or FILL_ARRAY
    }

    /// Gets accounts for which the entry is cosignatory.
    ///
    /// # Returns
    /// A Accounts for which the entry is cosignatory.
    pub fn get_multisig_addresses(&self) -> Vec<AddressDto> {
        self.multisig_addresses.clone() // ARRAY or FILL_ARRAY
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = self.super_object.get_size();
        size += 4; // min_approval;
        size += 4; // min_removal;
        size += self.account_address.get_size();
        size += 8; // cosignatory_addresses_count;
        size += self.cosignatory_addresses.iter().map(|item| item.get_size()).sum::<usize>(); // array or fill_array;
        size += 8; // multisig_addresses_count;
        size += self.multisig_addresses.iter().map(|item| item.get_size()).sum::<usize>(); // array or fill_array;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.super_object.serializer());
        buf.append(&mut (self.get_min_approval() as u16).to_le_bytes().to_vec()); // kind:SIMPLE
        buf.append(&mut (self.get_min_removal() as u16).to_le_bytes().to_vec()); // kind:SIMPLE
        buf.append(&mut self.account_address.serializer()); // kind:CUSTOM
        buf.append(&mut (self.get_cosignatory_addresses().len() as u64).to_le_bytes().to_vec()); // kind:SIZE_FIELD
        for i in &self.cosignatory_addresses {
            buf.append(&mut i.serializer()); // kind:ARRAY|FILL_ARRAY
        }
        buf.append(&mut (self.get_multisig_addresses().len() as u64).to_le_bytes().to_vec()); // kind:SIZE_FIELD
        for i in &self.multisig_addresses {
            buf.append(&mut i.serializer()); // kind:ARRAY|FILL_ARRAY
        }
        buf
    }
}

