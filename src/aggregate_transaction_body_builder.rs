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

use super::cosignature_builder::*;
use super::embedded_transaction_builder::*;
use super::embedded_transaction_helper::*;
use super::generator_utils::*;
use super::hash256_dto::*;

/// Binary layout for an aggregate transaction.
#[derive(Debug, Clone)]
pub struct AggregateTransactionBodyBuilder {
    /// Aggregate hash of an aggregate's transactions.
    pub transactions_hash: Hash256Dto,
    /// Sub-transaction data (transactions are variable sized and payload size is in bytes).
    pub transactions: Vec<Box<dyn EmbeddedTransactionHelper + 'static>>,
    /// Cosignatures data (fills remaining body space after transactions).
    pub cosignatures: Vec<CosignatureBuilder>,
}

impl AggregateTransactionBodyBuilder {
    fn load_embedded_transactions(transactions: &mut Vec<Box<dyn EmbeddedTransactionHelper + 'static>>, mut payload: Vec<u8>, payload_size: u32) -> Vec<u8> {
        let mut remaining_byte_sizes = payload_size as usize;
        while remaining_byte_sizes > 0 {
            let item = load_from_binary(&payload);
            transactions.push(item.clone());
            let size = item.get_size();
            let item_size = size + Self::get_padding_size(item.get_size(), 8);
            remaining_byte_sizes -= item_size;
            payload = (&payload[item_size..]).to_vec();
        }
        payload
    }

    fn load_cosignatures(transactions: &mut Vec<CosignatureBuilder>, mut payload: Vec<u8>, payload_size: usize) -> Vec<u8> {
        let mut remaining_byte_sizes = payload_size;
        while remaining_byte_sizes > 0 {
            let item = CosignatureBuilder::from_binary(&payload);
            transactions.push(item.clone());
            let size = item.get_size();
            let item_size = size + Self::get_padding_size(item.get_size(), 8);
            remaining_byte_sizes -= item_size;
            payload = (&payload[item_size..]).to_vec();
        }
        payload
    }

    /// Creates an instance of AggregateTransactionBodyBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A AggregateTransactionBodyBuilder.
    pub fn from_binary(payload: &[u8]) -> Self {
        let mut _bytes = payload.to_vec();
        let transactions_hash = Hash256Dto::from_binary(&_bytes); // kind:CUSTOM1
        _bytes = _bytes[transactions_hash.get_size()..].to_vec();
        let buf = fixed_bytes::<4>(&_bytes);
        let payload_size = u32::from_le_bytes(buf); // kind:SIZE_FIELD
        _bytes = (&_bytes[4..]).to_vec();
        let buf = fixed_bytes::<4>(&_bytes);
        let _ = u32::from_le_bytes(buf); // kind:SIMPLE
        _bytes = (&_bytes[4..]).to_vec();
        let mut transactions: Vec<Box<dyn EmbeddedTransactionHelper + 'static>> = vec![];
        _bytes = AggregateTransactionBodyBuilder::load_embedded_transactions(&mut transactions, _bytes, payload_size);
        let mut cosignatures: Vec<CosignatureBuilder> = vec![];
        let _ = Self::load_cosignatures(&mut cosignatures, _bytes.clone(), _bytes.clone().len());
        // create object and call.
        AggregateTransactionBodyBuilder { transactions_hash, transactions, cosignatures } // TransactionBody
    }

    /// Serializes an embeded transaction with correct padding.
    /// # Returns
    /// A Serialized embedded transaction.
    pub fn serialize_aligned(transaction: &Box<dyn EmbeddedTransactionHelper>) -> Vec<u8> {
        let txn_bytes = transaction.serializer();
        let padding = vec![0u8; Self::get_padding_size(txn_bytes.len(), 8)];
        [txn_bytes, padding].concat()
    }

    /// Serializes an embeded transaction with correct padding.
    /// # Returns
    /// A Serialized embedded transaction.
    pub fn size_aligned(transaction: &Box<dyn EmbeddedTransactionHelper>) -> usize {
        let txn_size = transaction.get_size();
        let padding_size = Self::get_padding_size(txn_size, 8);
        txn_size + padding_size
    }

    fn get_padding_size(size: usize, alignment: usize) -> usize {
        if alignment == 0 {
            return 0;
        }

        if size % alignment == 0 {
            return 0;
        }
        alignment - size % alignment
    }
    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += self.transactions_hash.get_size(); // transactions_hash_size;
        size += 4;  // payload_size;
        size += 4;  // aggregate_transaction_header__reserved1;
        for i in &self.transactions {
            size += Self::size_aligned(i); // VAR_ARRAY
        };
        for i in &self.cosignatures {
            size += i.get_size(); // FILL_ARRAY
        };
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.transactions_hash.serializer()); // kind:CUSTOM
        // calculate payload size
        let mut size_value: u32 = 0;
        for i in &self.transactions {
            size_value += Self::size_aligned(i) as u32;
        };
        buf.append(&mut size_value.to_le_bytes().to_vec()); // kind:SIZE_FIELD
        buf.append(&mut [0u8; 4].to_vec()); // kind:SIMPLE and is_reserved
        for i in &self.transactions {
            buf.append(&mut Self::serialize_aligned(i)); // kind:VAR_ARRAY
        }
        for i in &self.cosignatures {
            buf.append(&mut i.serializer()); // kind:ARRAY|FILL_ARRAY
        }
        buf
    }
}

