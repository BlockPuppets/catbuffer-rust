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

use super::account_key_type_flags_dto::*;
use super::account_state_format_dto::*;
use super::account_type_dto::*;
use super::address_dto::*;
use super::height_activity_buckets_builder::*;
use super::height_dto::*;
use super::importance_snapshot_builder::*;
use super::key_dto::*;
use super::mosaic_builder::*;
use super::pinned_voting_key_builder::*;
use super::state_header_builder::*;

/// Binary layout for non-historical account state.
#[derive(Debug, Clone)]
pub struct AccountStateBuilder {
    /// State header.
    super_object: StateHeaderBuilder,
    /// Address of account.
    address: AddressDto,
    /// Height at which address has been obtained.
    address_height: HeightDto,
    /// Public key of account.
    public_key: KeyDto,
    /// Height at which public key has been obtained.
    public_key_height: HeightDto,
    /// Type of account.
    account_type: AccountTypeDto,
    /// Account format.
    format: AccountStateFormatDto,
    /// Mask of supplemental public key flags.
    supplemental_public_keys_mask: Vec<AccountKeyTypeFlagsDto>,
    /// Linked account public key.
    linked_public_key: Option<KeyDto>,
    /// Node public key.
    node_public_key: Option<KeyDto>,
    /// Vrf public key.
    vrf_public_key: Option<KeyDto>,
    /// Voting public keys.
    voting_public_keys: Vec<PinnedVotingKeyBuilder>,
    /// Current importance snapshot of the account.
    importance_snapshots: Option<ImportanceSnapshotBuilder>,
    /// Activity buckets of the account.
    activity_buckets: Option<HeightActivityBucketsBuilder>,
    /// Balances of account.
    balances: Vec<MosaicBuilder>,
}


impl AccountStateBuilder {
    /// Creates an instance of AccountStateBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A AccountStateBuilder.
    pub fn from_binary(bytes_: &[u8]) -> Self {
        let super_object = StateHeaderBuilder::from_binary(bytes_);
        let mut bytes_ = bytes_[super_object.get_size()..].to_vec();
        let address = AddressDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[address.get_size()..].to_vec();
        let address_height = HeightDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[address_height.get_size()..].to_vec();
        let public_key = KeyDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[public_key.get_size()..].to_vec();
        let public_key_height = HeightDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[public_key_height.get_size()..].to_vec();
        let account_type = AccountTypeDto::from_binary(&bytes_); // kind:CUSTOM2
        let mut bytes_ = bytes_[account_type.get_size()..].to_vec();
        let format = AccountStateFormatDto::from_binary(&bytes_); // kind:CUSTOM2
        let mut bytes_ = bytes_[format.get_size()..].to_vec();
        let supplemental_public_keys_mask = AccountKeyTypeFlagsDto::bytes_to_flags(&bytes_[..1]); // kind:FLAGS
        let mut bytes_ = (&bytes_[1..]).to_vec();
        let mut buf = [0x0u8; 1];
        buf.copy_from_slice(&bytes_[..1]);
        let votingPublicKeysCount = u8::from_le_bytes(buf); // kind:SIZE_FIELD
        let mut bytes_ = (&bytes_[1..]).to_vec();
        let mut linked_public_key = None;
        if supplemental_public_keys_mask.iter().any(|&i| i == AccountKeyTypeFlagsDto::LINKED) {
            let raw_linked_public_key = KeyDto::from_binary(&bytes_);
            bytes_ = (&bytes_[raw_linked_public_key.get_size()..]).to_vec();
            linked_public_key = Some(raw_linked_public_key); // kind:CUSTOM1
        }
        let mut node_public_key = None;
        if supplemental_public_keys_mask.iter().any(|&i| i == AccountKeyTypeFlagsDto::NODE) {
            let raw_node_public_key = KeyDto::from_binary(&bytes_);
            bytes_ = (&bytes_[raw_node_public_key.get_size()..]).to_vec();
            node_public_key = Some(raw_node_public_key); // kind:CUSTOM1
        }
        let mut vrf_public_key = None;
        if supplemental_public_keys_mask.iter().any(|&i| i == AccountKeyTypeFlagsDto::VRF) {
            let raw_vrf_public_key = KeyDto::from_binary(&bytes_);
            bytes_ = (&bytes_[raw_vrf_public_key.get_size()..]).to_vec();
            vrf_public_key = Some(raw_vrf_public_key); // kind:CUSTOM1
        }
        let mut voting_public_keys: Vec<PinnedVotingKeyBuilder> = vec![]; // kind:ARRAY
        let mut bytes_ = bytes_.to_vec();
        for _ in 0..votingPublicKeysCount {
            let item = PinnedVotingKeyBuilder::from_binary(&bytes_);
            voting_public_keys.push(item.clone());
            bytes_ = (&bytes_[item.get_size()..]).to_vec();
        }
        let mut importance_snapshots = None;
        if format == AccountStateFormatDto::HIGH_VALUE {
            let raw_importance_snapshots = ImportanceSnapshotBuilder::from_binary(&bytes_);
            bytes_ = (&bytes_[raw_importance_snapshots.get_size()..]).to_vec();
            importance_snapshots = Some(raw_importance_snapshots); // kind:CUSTOM1
        }
        let mut activity_buckets = None;
        if format == AccountStateFormatDto::HIGH_VALUE {
            let raw_activity_buckets = HeightActivityBucketsBuilder::from_binary(&bytes_);
            bytes_ = (&bytes_[raw_activity_buckets.get_size()..]).to_vec();
            activity_buckets = Some(raw_activity_buckets); // kind:CUSTOM1
        }
        let mut buf = [0x0u8; 2];
        buf.copy_from_slice(&bytes_[..2]);
        let balancesCount = u16::from_le_bytes(buf); // kind:SIZE_FIELD
        let mut bytes_ = (&bytes_[2..]).to_vec();
        let mut balances: Vec<MosaicBuilder> = vec![]; // kind:ARRAY
        let mut bytes_ = bytes_.to_vec();
        for _ in 0..balancesCount {
            let item = MosaicBuilder::from_binary(&bytes_);
            balances.push(item.clone());
            bytes_ = (&bytes_[item.get_size()..]).to_vec();
        }
        AccountStateBuilder { super_object, address, address_height, public_key, public_key_height, account_type, format, supplemental_public_keys_mask, linked_public_key, node_public_key, vrf_public_key, voting_public_keys, importance_snapshots, activity_buckets, balances }
    }

    /// Gets address of account.
    ///
    /// # Returns
    /// A Address of account.
    pub fn get_address(&self) -> AddressDto {
        self.address.clone()
    }

    /// Gets height at which address has been obtained.
    ///
    /// # Returns
    /// A Height at which address has been obtained.
    pub fn get_address_height(&self) -> HeightDto {
        self.address_height.clone()
    }

    /// Gets public key of account.
    ///
    /// # Returns
    /// A Public key of account.
    pub fn get_public_key(&self) -> KeyDto {
        self.public_key.clone()
    }

    /// Gets height at which public key has been obtained.
    ///
    /// # Returns
    /// A Height at which public key has been obtained.
    pub fn get_public_key_height(&self) -> HeightDto {
        self.public_key_height.clone()
    }

    /// Gets type of account.
    ///
    /// # Returns
    /// A Type of account.
    pub fn get_account_type(&self) -> AccountTypeDto {
        self.account_type.clone()
    }

    /// Gets account format.
    ///
    /// # Returns
    /// A Account format.
    pub fn get_format(&self) -> AccountStateFormatDto {
        self.format.clone()
    }

    /// Gets mask of supplemental public key flags.
    ///
    /// # Returns
    /// A Mask of supplemental public key flags.
    pub fn get_supplemental_public_keys_mask(&self) -> Vec<AccountKeyTypeFlagsDto> {
        self.supplemental_public_keys_mask.clone()
    }

    /// Gets linked account public key.
    ///
    /// # Returns
    /// A Linked account public key.
    pub fn get_linked_public_key(&self) -> Option<KeyDto> {
        if self.supplemental_public_keys_mask.iter().any(|&i| i == AccountKeyTypeFlagsDto::LINKED) {
            panic!("supplementalPublicKeysMask is not set to LINKED.")
        };
        self.linked_public_key.clone()
    }

    /// Gets node public key.
    ///
    /// # Returns
    /// A Node public key.
    pub fn get_node_public_key(&self) -> Option<KeyDto> {
        if self.supplemental_public_keys_mask.iter().any(|&i| i == AccountKeyTypeFlagsDto::NODE) {
            panic!("supplementalPublicKeysMask is not set to NODE.")
        };
        self.node_public_key.clone()
    }

    /// Gets vrf public key.
    ///
    /// # Returns
    /// A Vrf public key.
    pub fn get_vrf_public_key(&self) -> Option<KeyDto> {
        if self.supplemental_public_keys_mask.iter().any(|&i| i == AccountKeyTypeFlagsDto::VRF) {
            panic!("supplementalPublicKeysMask is not set to VRF.")
        };
        self.vrf_public_key.clone()
    }

    /// Gets voting public keys.
    ///
    /// # Returns
    /// A Voting public keys.
    pub fn get_voting_public_keys(&self) -> Vec<PinnedVotingKeyBuilder> {
        self.voting_public_keys.clone() // ARRAY or FILL_ARRAY
    }

    /// Gets current importance snapshot of the account.
    ///
    /// # Returns
    /// A Current importance snapshot of the account.
    pub fn get_importance_snapshots(&self) -> Option<ImportanceSnapshotBuilder> {
        if self.format != AccountStateFormatDto::HIGH_VALUE {
            panic!("format is not set to HIGH_VALUE.")
        };
        self.importance_snapshots.clone()
    }

    /// Gets activity buckets of the account.
    ///
    /// # Returns
    /// A Activity buckets of the account.
    pub fn get_activity_buckets(&self) -> Option<HeightActivityBucketsBuilder> {
        if self.format != AccountStateFormatDto::HIGH_VALUE {
            panic!("format is not set to HIGH_VALUE.")
        };
        self.activity_buckets.clone()
    }

    /// Gets balances of account.
    ///
    /// # Returns
    /// A Balances of account.
    pub fn get_balances(&self) -> Vec<MosaicBuilder> {
        self.balances.clone() // ARRAY or FILL_ARRAY
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = self.super_object.get_size();
        size += self.address.get_size();
        size += self.address_height.get_size();
        size += self.public_key.get_size();
        size += self.public_key_height.get_size();
        size += self.account_type.get_size();
        size += self.format.get_size();
        size += 1; // supplemental_public_keys_mask;
        size += 1; // voting_public_keys_count;
        if self.supplemental_public_keys_mask.iter().any(|&i| i == AccountKeyTypeFlagsDto::LINKED) {
            size += self.linked_public_key.as_ref().unwrap().get_size();
        }
        if self.supplemental_public_keys_mask.iter().any(|&i| i == AccountKeyTypeFlagsDto::NODE) {
            size += self.node_public_key.as_ref().unwrap().get_size();
        }
        if self.supplemental_public_keys_mask.iter().any(|&i| i == AccountKeyTypeFlagsDto::VRF) {
            size += self.vrf_public_key.as_ref().unwrap().get_size();
        }
        size += self.voting_public_keys.iter().map(|item| item.get_size()).sum::<usize>(); // array or fill_array;
        if self.format == AccountStateFormatDto::HIGH_VALUE {
            size += self.importance_snapshots.as_ref().unwrap().get_size();
        }
        if self.format == AccountStateFormatDto::HIGH_VALUE {
            size += self.activity_buckets.as_ref().unwrap().get_size();
        }
        size += 2; // balances_count;
        size += self.balances.iter().map(|item| item.get_size()).sum::<usize>(); // array or fill_array;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.super_object.serializer());
        buf.append(&mut self.address.serializer()); // kind:CUSTOM
        buf.append(&mut self.address_height.serializer()); // kind:CUSTOM
        buf.append(&mut self.public_key.serializer()); // kind:CUSTOM
        buf.append(&mut self.public_key_height.serializer()); // kind:CUSTOM
        buf.append(&mut self.account_type.serializer()); // kind:CUSTOM
        buf.append(&mut self.format.serializer()); // kind:CUSTOM
        buf.append(&mut AccountKeyTypeFlagsDto::flags_to_int(self.get_supplemental_public_keys_mask()).to_le_bytes().to_vec()); // kind:FLAGS
        buf.append(&mut (self.get_voting_public_keys().len() as u8).to_le_bytes().to_vec()); // kind:SIZE_FIELD
        if self.supplemental_public_keys_mask.iter().any(|&i| i == AccountKeyTypeFlagsDto::LINKED) {
            buf.append(&mut self.linked_public_key.as_ref().unwrap().serializer()); // kind:CUSTOM
        };
        if self.supplemental_public_keys_mask.iter().any(|&i| i == AccountKeyTypeFlagsDto::NODE) {
            buf.append(&mut self.node_public_key.as_ref().unwrap().serializer()); // kind:CUSTOM
        };
        if self.supplemental_public_keys_mask.iter().any(|&i| i == AccountKeyTypeFlagsDto::VRF) {
            buf.append(&mut self.vrf_public_key.as_ref().unwrap().serializer()); // kind:CUSTOM
        };
        for i in &self.voting_public_keys {
            buf.append(&mut i.serializer()); // kind:ARRAY|FILL_ARRAY
        }
        if self.format == AccountStateFormatDto::HIGH_VALUE {
            buf.append(&mut self.importance_snapshots.as_ref().unwrap().serializer()); // kind:CUSTOM
        };
        if self.format == AccountStateFormatDto::HIGH_VALUE {
            buf.append(&mut self.activity_buckets.as_ref().unwrap().serializer()); // kind:CUSTOM
        };
        buf.append(&mut (self.get_balances().len() as u16).to_le_bytes().to_vec()); // kind:SIZE_FIELD
        for i in &self.balances {
            buf.append(&mut i.serializer()); // kind:ARRAY|FILL_ARRAY
        }
        buf
    }
}

