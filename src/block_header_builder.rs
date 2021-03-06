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
use super::block_fee_multiplier_dto::*;
use super::difficulty_dto::*;
use super::entity_type_dto::*;
use super::generator_utils::*;
use super::hash256_dto::*;
use super::height_dto::*;
use super::key_dto::*;
use super::network_type_dto::*;
use super::signature_dto::*;
use super::timestamp_dto::*;
use super::vrf_proof_builder::*;

/// Binary layout for a block header.
#[derive(Debug, Clone)]
pub struct BlockHeaderBuilder {
    /// Entity signature.
    signature: SignatureDto,
    /// Entity signer's public key.
    signer_public_key: KeyDto,
    /// Entity version.
    version: u8,
    /// Entity network.
    network: NetworkTypeDto,
    /// Entity type.
    _type: EntityTypeDto,
    /// Block height.
    height: HeightDto,
    /// Number of milliseconds elapsed since creation of nemesis block.
    timestamp: TimestampDto,
    /// Block difficulty.
    difficulty: DifficultyDto,
    /// Generation hash proof.
    generation_hash_proof: VrfProofBuilder,
    /// Previous block hash.
    previous_block_hash: Hash256Dto,
    /// Hash of the transactions in this block.
    transactions_hash: Hash256Dto,
    /// Hash of the receipts generated by this block.
    receipts_hash: Hash256Dto,
    /// Hash of the global chain state at this block.
    state_hash: Hash256Dto,
    /// Beneficiary address designated by harvester.
    beneficiary_address: AddressDto,
    /// Fee multiplier applied to block transactions.
    fee_multiplier: BlockFeeMultiplierDto,
}


impl BlockHeaderBuilder {
    /// Creates an instance of BlockHeaderBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A BlockHeaderBuilder.
    pub fn from_binary(_bytes: &[u8]) -> Self {
        let signature = SignatureDto::from_binary(&_bytes); // kind:CUSTOM1
        let mut _bytes = _bytes[signature.get_size()..].to_vec();
        let signer_public_key = KeyDto::from_binary(&_bytes); // kind:CUSTOM1
        let mut _bytes = _bytes[signer_public_key.get_size()..].to_vec();
        let buf = fixed_bytes::<1>(&_bytes);
        let version = u8::from_le_bytes(buf); // kind:SIMPLE
        let _bytes = (&_bytes[1..]).to_vec();
        let network = NetworkTypeDto::from_binary(&_bytes); // kind:CUSTOM2
        let mut _bytes = _bytes[network.get_size()..].to_vec();
        let _type = EntityTypeDto::from_binary(&_bytes); // kind:CUSTOM2
        let _bytes = (&_bytes[_type.get_size()..]).to_vec();
        let height = HeightDto::from_binary(&_bytes); // kind:CUSTOM1
        let mut _bytes = _bytes[height.get_size()..].to_vec();
        let timestamp = TimestampDto::from_binary(&_bytes); // kind:CUSTOM1
        let mut _bytes = _bytes[timestamp.get_size()..].to_vec();
        let difficulty = DifficultyDto::from_binary(&_bytes); // kind:CUSTOM1
        let mut _bytes = _bytes[difficulty.get_size()..].to_vec();
        let generation_hash_proof = VrfProofBuilder::from_binary(&_bytes); // kind:CUSTOM1
        let mut _bytes = _bytes[generation_hash_proof.get_size()..].to_vec();
        let previous_block_hash = Hash256Dto::from_binary(&_bytes); // kind:CUSTOM1
        let mut _bytes = _bytes[previous_block_hash.get_size()..].to_vec();
        let transactions_hash = Hash256Dto::from_binary(&_bytes); // kind:CUSTOM1
        let mut _bytes = _bytes[transactions_hash.get_size()..].to_vec();
        let receipts_hash = Hash256Dto::from_binary(&_bytes); // kind:CUSTOM1
        let mut _bytes = _bytes[receipts_hash.get_size()..].to_vec();
        let state_hash = Hash256Dto::from_binary(&_bytes); // kind:CUSTOM1
        let mut _bytes = _bytes[state_hash.get_size()..].to_vec();
        let beneficiary_address = AddressDto::from_binary(&_bytes); // kind:CUSTOM1
        let mut _bytes = _bytes[beneficiary_address.get_size()..].to_vec();
        let fee_multiplier = BlockFeeMultiplierDto::from_binary(&_bytes); // kind:CUSTOM1
        let mut _bytes = _bytes[fee_multiplier.get_size()..].to_vec();
        BlockHeaderBuilder { signature, signer_public_key, version, network, _type, height, timestamp, difficulty, generation_hash_proof, previous_block_hash, transactions_hash, receipts_hash, state_hash, beneficiary_address, fee_multiplier }
    }

    /// Gets entity signature.
    ///
    /// # Returns
    /// A Entity signature.
    pub fn get_signature(&self) -> SignatureDto {
        self.signature.clone()
    }

    /// Gets entity signer's public key.
    ///
    /// # Returns
    /// A Entity signer's public key.
    pub fn get_signer_public_key(&self) -> KeyDto {
        self.signer_public_key.clone()
    }

    /// Gets entity version.
    ///
    /// # Returns
    /// A Entity version.
    pub fn get_version(&self) -> u8 {
        self.version.clone()
    }

    /// Gets entity network.
    ///
    /// # Returns
    /// A Entity network.
    pub fn get_network(&self) -> NetworkTypeDto {
        self.network.clone()
    }

    /// Gets entity type.
    ///
    /// # Returns
    /// A Entity type.
    pub fn get_type(&self) -> EntityTypeDto {
        self._type
    }

    /// Gets block height.
    ///
    /// # Returns
    /// A Block height.
    pub fn get_height(&self) -> HeightDto {
        self.height.clone()
    }

    /// Gets number of milliseconds elapsed since creation of nemesis block.
    ///
    /// # Returns
    /// A Number of milliseconds elapsed since creation of nemesis block.
    pub fn get_timestamp(&self) -> TimestampDto {
        self.timestamp.clone()
    }

    /// Gets block difficulty.
    ///
    /// # Returns
    /// A Block difficulty.
    pub fn get_difficulty(&self) -> DifficultyDto {
        self.difficulty.clone()
    }

    /// Gets generation hash proof.
    ///
    /// # Returns
    /// A Generation hash proof.
    pub fn get_generation_hash_proof(&self) -> VrfProofBuilder {
        self.generation_hash_proof.clone()
    }

    /// Gets previous block hash.
    ///
    /// # Returns
    /// A Previous block hash.
    pub fn get_previous_block_hash(&self) -> Hash256Dto {
        self.previous_block_hash.clone()
    }

    /// Gets hash of the transactions in this block.
    ///
    /// # Returns
    /// A Hash of the transactions in this block.
    pub fn get_transactions_hash(&self) -> Hash256Dto {
        self.transactions_hash.clone()
    }

    /// Gets hash of the receipts generated by this block.
    ///
    /// # Returns
    /// A Hash of the receipts generated by this block.
    pub fn get_receipts_hash(&self) -> Hash256Dto {
        self.receipts_hash.clone()
    }

    /// Gets hash of the global chain state at this block.
    ///
    /// # Returns
    /// A Hash of the global chain state at this block.
    pub fn get_state_hash(&self) -> Hash256Dto {
        self.state_hash.clone()
    }

    /// Gets beneficiary address designated by harvester.
    ///
    /// # Returns
    /// A Beneficiary address designated by harvester.
    pub fn get_beneficiary_address(&self) -> AddressDto {
        self.beneficiary_address.clone()
    }

    /// Gets fee multiplier applied to block transactions.
    ///
    /// # Returns
    /// A Fee multiplier applied to block transactions.
    pub fn get_fee_multiplier(&self) -> BlockFeeMultiplierDto {
        self.fee_multiplier.clone()
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += 4; // size;
        size += 4; // verifiable_entity_header__reserved1;
        size += self.signature.get_size(); // signature;
        size += self.signer_public_key.get_size(); // signer_public_key;
        size += 4; // entity_body__reserved1;
        size += 1; // version;
        size += self.network.get_size(); // network;
        size += self._type.get_size(); // type;
        size += self.height.get_size(); // height;
        size += self.timestamp.get_size(); // timestamp;
        size += self.difficulty.get_size(); // difficulty;
        size += self.generation_hash_proof.get_size(); // generation_hash_proof;
        size += self.previous_block_hash.get_size(); // previous_block_hash;
        size += self.transactions_hash.get_size(); // transactions_hash;
        size += self.receipts_hash.get_size(); // receipts_hash;
        size += self.state_hash.get_size(); // state_hash;
        size += self.beneficiary_address.get_size(); // beneficiary_address;
        size += self.fee_multiplier.get_size(); // fee_multiplier;
        size
    }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut self.get_size().to_le_bytes().to_vec()); // kind:SIMPLE
        buf.append(&mut 4u16.to_le_bytes().to_vec());
        buf.append(&mut self.signature.serializer()); // kind:CUSTOM
        buf.append(&mut self.signer_public_key.serializer()); // kind:CUSTOM
        buf.append(&mut 4u16.to_le_bytes().to_vec());
        buf.append(&mut self.get_version().to_le_bytes().to_vec()); // kind:SIMPLE
        buf.append(&mut self.network.serializer()); // kind:CUSTOM
        buf.append(&mut self._type.serializer()); // kind:CUSTOM
        buf.append(&mut self.height.serializer()); // kind:CUSTOM
        buf.append(&mut self.timestamp.serializer()); // kind:CUSTOM
        buf.append(&mut self.difficulty.serializer()); // kind:CUSTOM
        buf.append(&mut self.generation_hash_proof.serializer()); // kind:CUSTOM
        buf.append(&mut self.previous_block_hash.serializer()); // kind:CUSTOM
        buf.append(&mut self.transactions_hash.serializer()); // kind:CUSTOM
        buf.append(&mut self.receipts_hash.serializer()); // kind:CUSTOM
        buf.append(&mut self.state_hash.serializer()); // kind:CUSTOM
        buf.append(&mut self.beneficiary_address.serializer()); // kind:CUSTOM
        buf.append(&mut self.fee_multiplier.serializer()); // kind:CUSTOM
        buf
    }
}

