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
use super::proof_gamma_dto::*;
use super::proof_scalar_dto::*;
use super::proof_verification_hash_dto::*;

/// Verfiable random function proof.
#[derive(Debug, Clone)]
pub struct VrfProofBuilder {
    /// Gamma.
    gamma: ProofGammaDto,
    /// Verification hash.
    verification_hash: ProofVerificationHashDto,
    /// Scalar.
    scalar: ProofScalarDto,
}


impl VrfProofBuilder {
    /// Creates an instance of VrfProofBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A VrfProofBuilder.
    pub fn from_binary(bytes_: &[u8]) -> Self {
        let gamma = ProofGammaDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[gamma.get_size()..].to_vec();
        let verification_hash = ProofVerificationHashDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[verification_hash.get_size()..].to_vec();
        let scalar = ProofScalarDto::from_binary(&bytes_); // kind:CUSTOM1
        let mut bytes_ = bytes_[scalar.get_size()..].to_vec();
        VrfProofBuilder { gamma, verification_hash, scalar }
    }

    /// Gets gamma.
    ///
    /// # Returns
    /// A Gamma.
    pub fn get_gamma(&self) -> ProofGammaDto {
        self.gamma.clone()
    }

    /// Gets verification hash.
    ///
    /// # Returns
    /// A Verification hash.
    pub fn get_verification_hash(&self) -> ProofVerificationHashDto {
        self.verification_hash.clone()
    }

    /// Gets scalar.
    ///
    /// # Returns
    /// A Scalar.
    pub fn get_scalar(&self) -> ProofScalarDto {
        self.scalar.clone()
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += self.gamma.get_size(); // gamma;
        size += self.verification_hash.get_size(); // verification_hash;
        size += self.scalar.get_size(); // scalar;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.gamma.serializer()); // kind:CUSTOM
        buf.append(&mut self.verification_hash.serializer()); // kind:CUSTOM
        buf.append(&mut self.scalar.serializer()); // kind:CUSTOM
        buf
    }
}

