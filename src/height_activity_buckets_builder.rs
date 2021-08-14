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
use super::height_activity_bucket_builder::*;

/// Account activity buckets.
#[derive(Debug, Clone)]
pub struct HeightActivityBucketsBuilder {
    /// Account activity buckets.
    buckets: Vec<HeightActivityBucketBuilder>,
}


impl HeightActivityBucketsBuilder {
    /// Creates an instance of HeightActivityBucketsBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A HeightActivityBucketsBuilder.
    pub fn from_binary(bytes_: &[u8]) -> Self {
        let mut buckets: Vec<HeightActivityBucketBuilder> = vec![]; // kind:ARRAY
        let mut bytes_ = bytes_.to_vec();
        for _ in 0..5 {
            let item = HeightActivityBucketBuilder::from_binary(&bytes_);
            buckets.push(item.clone());
            bytes_ = (&bytes_[item.get_size()..]).to_vec();
        }
        HeightActivityBucketsBuilder { buckets }
    }

    /// Gets account activity buckets.
    ///
    /// # Returns
    /// A Account activity buckets.
    pub fn get_buckets(&self) -> Vec<HeightActivityBucketBuilder> {
        self.buckets.clone() // ARRAY or FILL_ARRAY
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += self.buckets.iter().map(|item| item.get_size()).sum::<usize>(); // array or fill_array;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        for i in &self.buckets {
            buf.append(&mut i.serializer()); // kind:ARRAY|FILL_ARRAY
        }
        buf
    }
}

