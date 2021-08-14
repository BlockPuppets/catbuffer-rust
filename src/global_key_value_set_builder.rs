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
use super::global_key_value_builder::*;

/// Binary layout for a global restriction key-value set.
#[derive(Debug, Clone)]
pub struct GlobalKeyValueSetBuilder {
    /// Key value array.
    keys: Vec<GlobalKeyValueBuilder>,
}


impl GlobalKeyValueSetBuilder {
    /// Creates an instance of GlobalKeyValueSetBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A GlobalKeyValueSetBuilder.
    pub fn from_binary(bytes_: &[u8]) -> Self {
        let mut buf = fixed_bytes::<1>(&bytes_);
        let keyValueCount = u8::from_le_bytes(buf); // kind:SIZE_FIELD
        let mut bytes_ = (&bytes_[1..]).to_vec();
        let mut keys: Vec<GlobalKeyValueBuilder> = vec![]; // kind:ARRAY
        let mut bytes_ = bytes_.to_vec();
        for _ in 0..keyValueCount {
            let item = GlobalKeyValueBuilder::from_binary(&bytes_);
            keys.push(item.clone());
            bytes_ = (&bytes_[item.get_size()..]).to_vec();
        }
        GlobalKeyValueSetBuilder { keys }
    }

    /// Gets key value array.
    ///
    /// # Returns
    /// A Key value array.
    pub fn get_keys(&self) -> Vec<GlobalKeyValueBuilder> {
        self.keys.clone() // ARRAY or FILL_ARRAY
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += 1; // key_value_count;
        size += self.keys.iter().map(|item| item.get_size()).sum::<usize>(); // array or fill_array;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut (self.get_keys().len() as u8).to_le_bytes().to_vec()); // kind:SIZE_FIELD
        for i in &self.keys {
            buf.append(&mut i.serializer()); // kind:ARRAY|FILL_ARRAY
        }
        buf
    }
}

