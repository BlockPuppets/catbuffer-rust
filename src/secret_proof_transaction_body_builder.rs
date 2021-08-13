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

use super::hash256_dto::*;
use super::lock_hash_algorithm_dto::*;
use super::unresolved_address_dto::*;

/// Binary layout for a secret proof transaction.
#[derive(Debug, Clone)]
pub struct SecretProofTransactionBodyBuilder {
    /// Locked mosaic recipient address.
    pub recipient_address: UnresolvedAddressDto,
    /// Secret.
    pub secret: Hash256Dto,
    /// Hash algorithm.
    pub hash_algorithm: LockHashAlgorithmDto,
    /// Proof data.
    pub proof: Vec<u8>,
}

impl SecretProofTransactionBodyBuilder {



    /// Creates an instance of SecretProofTransactionBodyBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A SecretProofTransactionBodyBuilder.
    pub fn from_binary(payload: &[u8]) -> Self {
        let mut bytes_ = payload.to_vec();
        let recipient_address = UnresolvedAddressDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[recipient_address.get_size()..].to_vec();
        let secret = Hash256Dto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[secret.get_size()..].to_vec();
        let mut buf = [0x0u8; 2];
        buf.copy_from_slice(&bytes_[..2]);
        let proof_size = u16::from_le_bytes(buf); // kind:SIZE_FIELD
        let mut bytes_ = (&bytes_[2..]).to_vec();
        let hash_algorithm = LockHashAlgorithmDto::from_binary(&bytes_); // kind:CUSTOM2
        let mut bytes_ = bytes_[hash_algorithm.get_size()..].to_vec();
        let proof = (&bytes_[..proof_size as usize]).to_vec(); // kind:BUFFER
        let bytes_ = (&bytes_[proof_size as usize..]).to_vec();
        // create object and call.
        SecretProofTransactionBodyBuilder{ recipient_address, secret, hash_algorithm, proof } // TransactionBody
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
   pub fn get_size(&self) -> usize {
       let mut size = 0;
        size += self.recipient_address.get_size(); // recipient_address_size;
        size += self.secret.get_size(); // secret_size;
        size += 2;  // proof_size;
        size += self.hash_algorithm.get_size(); // hash_algorithm_size;
        size += self.proof.len();
        size
   }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.recipient_address.serializer()); // kind:CUSTOM
        buf.append(&mut self.secret.serializer()); // kind:CUSTOM
        let size_value: u16 = self.proof.len() as u16;
        buf.append(&mut size_value.to_le_bytes().to_vec()); // kind:SIZE_FIELD
        buf.append(&mut self.hash_algorithm.serializer()); // kind:CUSTOM
        buf.append(&mut self.proof.clone()); // kind:BUFFER
        buf
    }
}
