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

use super::amount_dto::*;
use super::generator_utils::*;
use super::hash256_dto::*;

/// Binary layout for an importance block footer.
#[derive(Debug, Clone)]
pub struct ImportanceBlockFooterBuilder {
    /// Number of voting eligible accounts.
    voting_eligible_accounts_count: u32,
    /// Number of harvesting eligible accounts.
    harvesting_eligible_accounts_count: u64,
    /// Total balance eligible for voting.
    total_voting_balance: AmountDto,
    /// Previous importance block hash.
    previous_importance_block_hash: Hash256Dto,
}


impl ImportanceBlockFooterBuilder {
    /// Creates an instance of ImportanceBlockFooterBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A ImportanceBlockFooterBuilder.
    pub fn from_binary(bytes_: &[u8]) -> Self {
        let mut buf = fixed_bytes::<4>(&bytes_);
        let voting_eligible_accounts_count = u32::from_le_bytes(buf); // kind:SIMPLE
        let bytes_ = (&bytes_[4..]).to_vec();
        let mut buf = fixed_bytes::<8>(&bytes_);
        let harvesting_eligible_accounts_count = u64::from_le_bytes(buf); // kind:SIMPLE
        let bytes_ = (&bytes_[8..]).to_vec();
        let total_voting_balance = AmountDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[total_voting_balance.get_size()..].to_vec();
        let previous_importance_block_hash = Hash256Dto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[previous_importance_block_hash.get_size()..].to_vec();
        ImportanceBlockFooterBuilder { voting_eligible_accounts_count, harvesting_eligible_accounts_count, total_voting_balance, previous_importance_block_hash }
    }

    /// Gets number of voting eligible accounts.
    ///
    /// # Returns
    /// A Number of voting eligible accounts.
    pub fn get_voting_eligible_accounts_count(&self) -> u32 {
        self.voting_eligible_accounts_count.clone()
    }

    /// Gets number of harvesting eligible accounts.
    ///
    /// # Returns
    /// A Number of harvesting eligible accounts.
    pub fn get_harvesting_eligible_accounts_count(&self) -> u64 {
        self.harvesting_eligible_accounts_count.clone()
    }

    /// Gets total balance eligible for voting.
    ///
    /// # Returns
    /// A Total balance eligible for voting.
    pub fn get_total_voting_balance(&self) -> AmountDto {
        self.total_voting_balance.clone()
    }

    /// Gets previous importance block hash.
    ///
    /// # Returns
    /// A Previous importance block hash.
    pub fn get_previous_importance_block_hash(&self) -> Hash256Dto {
        self.previous_importance_block_hash.clone()
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += 4; // voting_eligible_accounts_count;
        size += 8; // harvesting_eligible_accounts_count;
        size += self.total_voting_balance.get_size(); // total_voting_balance;
        size += self.previous_importance_block_hash.get_size(); // previous_importance_block_hash;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut (self.get_voting_eligible_accounts_count() as u16).to_le_bytes().to_vec()); // kind:SIMPLE
        buf.append(&mut (self.get_harvesting_eligible_accounts_count() as u16).to_le_bytes().to_vec()); // kind:SIMPLE
        buf.append(&mut self.total_voting_balance.serializer()); // kind:CUSTOM
        buf.append(&mut self.previous_importance_block_hash.serializer()); // kind:CUSTOM
        buf
    }
}

