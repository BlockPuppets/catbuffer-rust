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
use super::importance_height_dto::*;

/// Account activity bucket.
#[derive(Debug, Clone)]
pub struct HeightActivityBucketBuilder {
    /// Activity start height.
    start_height: ImportanceHeightDto,
    /// Total fees paid by account.
    total_fees_paid: AmountDto,
    /// Number of times account has been used as a beneficiary.
    beneficiary_count: u32,
    /// Raw importance score.
    raw_score: u64,
}


impl HeightActivityBucketBuilder {
    /// Creates an instance of HeightActivityBucketBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A HeightActivityBucketBuilder.
    pub fn from_binary(bytes_: &[u8]) -> Self {
        let start_height = ImportanceHeightDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[start_height.get_size()..].to_vec();
        let total_fees_paid = AmountDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[total_fees_paid.get_size()..].to_vec();
        let mut buf = fixed_bytes::<4>(&bytes_);
        let beneficiary_count = u32::from_le_bytes(buf); // kind:SIMPLE
        let bytes_ = (&bytes_[4..]).to_vec();
        let mut buf = fixed_bytes::<8>(&bytes_);
        let raw_score = u64::from_le_bytes(buf); // kind:SIMPLE
        let bytes_ = (&bytes_[8..]).to_vec();
        HeightActivityBucketBuilder { start_height, total_fees_paid, beneficiary_count, raw_score }
    }

    /// Gets activity start height.
    ///
    /// # Returns
    /// A Activity start height.
    pub fn get_start_height(&self) -> ImportanceHeightDto {
        self.start_height.clone()
    }

    /// Gets total fees paid by account.
    ///
    /// # Returns
    /// A Total fees paid by account.
    pub fn get_total_fees_paid(&self) -> AmountDto {
        self.total_fees_paid.clone()
    }

    /// Gets number of times account has been used as a beneficiary.
    ///
    /// # Returns
    /// A Number of times account has been used as a beneficiary.
    pub fn get_beneficiary_count(&self) -> u32 {
        self.beneficiary_count.clone()
    }

    /// Gets raw importance score.
    ///
    /// # Returns
    /// A Raw importance score.
    pub fn get_raw_score(&self) -> u64 {
        self.raw_score.clone()
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += self.start_height.get_size(); // start_height;
        size += self.total_fees_paid.get_size(); // total_fees_paid;
        size += 4; // beneficiary_count;
        size += 8; // raw_score;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.start_height.serializer()); // kind:CUSTOM
        buf.append(&mut self.total_fees_paid.serializer()); // kind:CUSTOM
        buf.append(&mut (self.get_beneficiary_count() as u16).to_le_bytes().to_vec()); // kind:SIMPLE
        buf.append(&mut (self.get_raw_score() as u16).to_le_bytes().to_vec()); // kind:SIMPLE
        buf
    }
}

