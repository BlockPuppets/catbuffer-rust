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

use super::key_dto::*;
use super::signature_dto::*;

/// Cosignature attached to an aggregate transaction.
#[derive(Debug, Clone)]
pub struct CosignatureBuilder {
    /// Version.
    version: u64,
    /// Cosigner public key.
    signer_public_key: KeyDto,
    /// Cosigner signature.
    signature: SignatureDto,
}


impl CosignatureBuilder {

    /// Creates an instance of CosignatureBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A CosignatureBuilder.
    pub fn from_binary(bytes_: &[u8]) -> Self {
        let mut buf = [0x0u8; 8];
        buf.copy_from_slice(&bytes_[..8]);
        let version = u64::from_le_bytes(buf); // kind:SIMPLE
        let bytes_ = (&bytes_[8..]).to_vec();
        let signer_public_key = KeyDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[signer_public_key.get_size()..].to_vec();
        let signature = SignatureDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[signature.get_size()..].to_vec();
        CosignatureBuilder{version, signer_public_key, signature}
    }

    /// Gets version.
    ///
    /// # Returns
    /// A Version.
    pub fn get_version(&self) -> u64 {
        self.version.clone()
    }

    /// Gets cosigner public key.
    ///
    /// # Returns
    /// A Cosigner public key.
    pub fn get_signer_public_key(&self) -> KeyDto {
        self.signer_public_key.clone()
    }

    /// Gets cosigner signature.
    ///
    /// # Returns
    /// A Cosigner signature.
    pub fn get_signature(&self) -> SignatureDto {
        self.signature.clone()
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += 8; // version;
        size += self.signer_public_key.get_size();
        size += self.signature.get_size();
        size
   }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut (self.get_version() as u16).to_le_bytes().to_vec()); // kind:SIMPLE
        buf.append(&mut self.signer_public_key.serializer()); // kind:CUSTOM
        buf.append(&mut self.signature.serializer()); // kind:CUSTOM
        buf
    }
}

