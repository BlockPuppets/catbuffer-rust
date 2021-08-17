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

use std::fmt::Debug;

use crate::embedded_transaction_builder::EmbeddedTransactionBuilder;

use super::embedded_account_address_restriction_transaction_builder::EmbeddedAccountAddressRestrictionTransactionBuilder;
use super::embedded_account_key_link_transaction_builder::EmbeddedAccountKeyLinkTransactionBuilder;
use super::embedded_account_metadata_transaction_builder::EmbeddedAccountMetadataTransactionBuilder;
use super::embedded_account_mosaic_restriction_transaction_builder::EmbeddedAccountMosaicRestrictionTransactionBuilder;
use super::embedded_account_operation_restriction_transaction_builder::EmbeddedAccountOperationRestrictionTransactionBuilder;
use super::embedded_address_alias_transaction_builder::EmbeddedAddressAliasTransactionBuilder;
use super::embedded_hash_lock_transaction_builder::EmbeddedHashLockTransactionBuilder;
use super::embedded_mosaic_address_restriction_transaction_builder::EmbeddedMosaicAddressRestrictionTransactionBuilder;
use super::embedded_mosaic_alias_transaction_builder::EmbeddedMosaicAliasTransactionBuilder;
use super::embedded_mosaic_definition_transaction_builder::EmbeddedMosaicDefinitionTransactionBuilder;
use super::embedded_mosaic_global_restriction_transaction_builder::EmbeddedMosaicGlobalRestrictionTransactionBuilder;
use super::embedded_mosaic_metadata_transaction_builder::EmbeddedMosaicMetadataTransactionBuilder;
use super::embedded_mosaic_supply_change_transaction_builder::EmbeddedMosaicSupplyChangeTransactionBuilder;
use super::embedded_multisig_account_modification_transaction_builder::EmbeddedMultisigAccountModificationTransactionBuilder;
use super::embedded_namespace_metadata_transaction_builder::EmbeddedNamespaceMetadataTransactionBuilder;
use super::embedded_namespace_registration_transaction_builder::EmbeddedNamespaceRegistrationTransactionBuilder;
use super::embedded_node_key_link_transaction_builder::EmbeddedNodeKeyLinkTransactionBuilder;
use super::embedded_secret_lock_transaction_builder::EmbeddedSecretLockTransactionBuilder;
use super::embedded_secret_proof_transaction_builder::EmbeddedSecretProofTransactionBuilder;
use super::embedded_transfer_transaction_builder::EmbeddedTransferTransactionBuilder;
use super::embedded_voting_key_link_transaction_builder::EmbeddedVotingKeyLinkTransactionBuilder;
use super::embedded_vrf_key_link_transaction_builder::EmbeddedVrfKeyLinkTransactionBuilder;

/// Helper trait for embedded transaction serialization.
pub trait EmbeddedTransactionHelper: Debug + Send + Sync {
    fn box_clone(&self) -> Box<dyn EmbeddedTransactionHelper>;
    fn get_size(&self) -> usize;
    fn serializer(&self) -> Vec<u8>;
}

impl Clone for Box<dyn EmbeddedTransactionHelper + 'static> {
    fn clone(&self) -> Box<dyn EmbeddedTransactionHelper + 'static> {
        self.box_clone()
    }
}

pub fn load_from_binary(payload: &[u8]) -> Box<dyn EmbeddedTransactionHelper> {
    let header_builder = EmbeddedTransactionBuilder::from_binary(payload);
    let entity_type_version = header_builder.version;
    match header_builder._type.get_value() {
        0x414c if entity_type_version == 1 => Box::new(EmbeddedAccountKeyLinkTransactionBuilder::from_binary(payload)),
        0x424c if entity_type_version == 1 => Box::new(EmbeddedNodeKeyLinkTransactionBuilder::from_binary(payload)),
        0x4143 if entity_type_version == 1 => Box::new(EmbeddedVotingKeyLinkTransactionBuilder::from_binary(payload)),
        0x4243 if entity_type_version == 1 => Box::new(EmbeddedVrfKeyLinkTransactionBuilder::from_binary(payload)),
        0x4148 if entity_type_version == 1 => Box::new(EmbeddedHashLockTransactionBuilder::from_binary(payload)),
        0x4152 if entity_type_version == 1 => Box::new(EmbeddedSecretLockTransactionBuilder::from_binary(payload)),
        0x4252 if entity_type_version == 1 => Box::new(EmbeddedSecretProofTransactionBuilder::from_binary(payload)),
        0x4144 if entity_type_version == 1 => Box::new(EmbeddedAccountMetadataTransactionBuilder::from_binary(payload)),
        0x4244 if entity_type_version == 1 => Box::new(EmbeddedMosaicMetadataTransactionBuilder::from_binary(payload)),
        0x4344 if entity_type_version == 1 => Box::new(EmbeddedNamespaceMetadataTransactionBuilder::from_binary(payload)),
        0x414d if entity_type_version == 1 => Box::new(EmbeddedMosaicDefinitionTransactionBuilder::from_binary(payload)),
        0x424d if entity_type_version == 1 => Box::new(EmbeddedMosaicSupplyChangeTransactionBuilder::from_binary(payload)),
        0x4155 if entity_type_version == 1 => Box::new(EmbeddedMultisigAccountModificationTransactionBuilder::from_binary(payload)),
        0x424e if entity_type_version == 1 => Box::new(EmbeddedAddressAliasTransactionBuilder::from_binary(payload)),
        0x434e if entity_type_version == 1 => Box::new(EmbeddedMosaicAliasTransactionBuilder::from_binary(payload)),
        0x414e if entity_type_version == 1 => Box::new(EmbeddedNamespaceRegistrationTransactionBuilder::from_binary(payload)),
        0x4150 if entity_type_version == 1 => Box::new(EmbeddedAccountAddressRestrictionTransactionBuilder::from_binary(payload)),
        0x4250 if entity_type_version == 1 => Box::new(EmbeddedAccountMosaicRestrictionTransactionBuilder::from_binary(payload)),
        0x4350 if entity_type_version == 1 => Box::new(EmbeddedAccountOperationRestrictionTransactionBuilder::from_binary(payload)),
        0x4251 if entity_type_version == 1 => Box::new(EmbeddedMosaicAddressRestrictionTransactionBuilder::from_binary(payload)),
        0x4151 if entity_type_version == 1 => Box::new(EmbeddedMosaicGlobalRestrictionTransactionBuilder::from_binary(payload)),
        0x4154 if entity_type_version == 1 => Box::new(EmbeddedTransferTransactionBuilder::from_binary(payload)),
        _ => panic!()
    }
}
