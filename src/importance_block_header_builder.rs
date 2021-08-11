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
use super::amount_dto::*;
use super::block_fee_multiplier_dto::*;
use super::block_header_builder::*;
use super::difficulty_dto::*;
use super::entity_type_dto::*;
use super::hash256_dto::*;
use super::height_dto::*;
use super::importance_block_footer_builder::*;
use super::key_dto::*;
use super::network_type_dto::*;
use super::signature_dto::*;
use super::timestamp_dto::*;
use super::vrf_proof_builder::*;

/// Binary layout for an importance block header.
#[derive(Debug, Clone)]
pub struct ImportanceBlockHeaderBuilder {
    /// Block header.
    super_object: BlockHeaderBuilder,
    /// Importance block footer.
    importance_block_footer: ImportanceBlockFooterBuilder,
}


impl ImportanceBlockHeaderBuilder {

    /// Creates an instance of ImportanceBlockHeaderBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A ImportanceBlockHeaderBuilder.
    pub fn from_binary(bytes_: &[u8]) -> Self {
        let super_object = BlockHeaderBuilder::from_binary(bytes_);
        let mut bytes_ = bytes_[super_object.get_size()..].to_vec();
        let importance_block_footer = ImportanceBlockFooterBuilder::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[importance_block_footer.get_size()..].to_vec();
        ImportanceBlockHeaderBuilder{super_object, importance_block_footer}
    }

    /// Gets number of voting eligible accounts.
    ///
    /// # Returns
    /// A Number of voting eligible accounts.
    pub fn get_voting_eligible_accounts_count(&self) -> u32 {
        self.importance_block_footer.get_voting_eligible_accounts_count()
    }

    /// Gets number of harvesting eligible accounts.
    ///
    /// # Returns
    /// A Number of harvesting eligible accounts.
    pub fn get_harvesting_eligible_accounts_count(&self) -> u64 {
        self.importance_block_footer.get_harvesting_eligible_accounts_count()
    }

    /// Gets total balance eligible for voting.
    ///
    /// # Returns
    /// A Total balance eligible for voting.
    pub fn get_total_voting_balance(&self) -> AmountDto {
        self.importance_block_footer.get_total_voting_balance()
    }

    /// Gets previous importance block hash.
    ///
    /// # Returns
    /// A Previous importance block hash.
    pub fn get_previous_importance_block_hash(&self) -> Hash256Dto {
        self.importance_block_footer.get_previous_importance_block_hash()
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = self.super_object.get_size();
        size += self.importance_block_footer.get_size();
        size
   }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.super_object.serializer());
        buf.append(&mut self.importance_block_footer.serializer()); // kind:CUSTOM
        buf
    }
}

