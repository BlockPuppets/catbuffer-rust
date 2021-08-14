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

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use strum_macros::EnumIter;

/// Enumeration of entity types.
#[allow(non_camel_case_types)]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive, EnumIter)]
pub enum EntityTypeDto {
    /// Reserved entity type.
    RESERVED = 0,

    /// Nemesis block header.
    NEMESIS_BLOCK_HEADER = 32835,

    /// Normal block header.
    NORMAL_BLOCK_HEADER = 33091,

    /// Importance block header.
    IMPORTANCE_BLOCK_HEADER = 33347,

    /// Account key link transaction.
    ACCOUNT_KEY_LINK_TRANSACTION = 16716,

    // /// Embedded account key link transaction.
    // EMBEDDED_ACCOUNT_KEY_LINK_TRANSACTION = 16716,

    /// Node key link transaction.
    NODE_KEY_LINK_TRANSACTION = 16972,

    // /// Embedded node key link transaction.
    // EMBEDDED_NODE_KEY_LINK_TRANSACTION = 16972,

    /// Aggregate complete transaction.
    AGGREGATE_COMPLETE_TRANSACTION = 16705,

    /// Aggregate bonded transaction.
    AGGREGATE_BONDED_TRANSACTION = 16961,

    /// Voting key link transaction.
    VOTING_KEY_LINK_TRANSACTION = 16707,

    // /// Embedded voting key link transaction.
    // EMBEDDED_VOTING_KEY_LINK_TRANSACTION = 16707,

    /// Vrf key link transaction.
    VRF_KEY_LINK_TRANSACTION = 16963,

    // /// Embedded vrf key link transaction.
    // EMBEDDED_VRF_KEY_LINK_TRANSACTION = 16963,

    /// Hash lock transaction.
    HASH_LOCK_TRANSACTION = 16712,

    // /// Embedded hash lock transaction.
    // EMBEDDED_HASH_LOCK_TRANSACTION = 16712,

    /// Secret lock transaction.
    SECRET_LOCK_TRANSACTION = 16722,

    // /// Embedded secret lock transaction.
    // EMBEDDED_SECRET_LOCK_TRANSACTION = 16722,

    /// Secret proof transaction.
    SECRET_PROOF_TRANSACTION = 16978,

    // /// Embedded secret proof transaction.
    // EMBEDDED_SECRET_PROOF_TRANSACTION = 16978,

    /// Account metadata transaction.
    ACCOUNT_METADATA_TRANSACTION = 16708,

    // /// Embedded account metadata transaction.
    // EMBEDDED_ACCOUNT_METADATA_TRANSACTION = 16708,

    /// Mosaic metadata transaction.
    MOSAIC_METADATA_TRANSACTION = 16964,

    // /// Embedded mosaic metadata transaction.
    // EMBEDDED_MOSAIC_METADATA_TRANSACTION = 16964,

    /// Namespace metadata transaction.
    NAMESPACE_METADATA_TRANSACTION = 17220,

    // /// Embedded namespace metadata transaction.
    // EMBEDDED_NAMESPACE_METADATA_TRANSACTION = 17220,

    /// Mosaic definition transaction.
    MOSAIC_DEFINITION_TRANSACTION = 16717,

    // /// Embedded mosaic definition transaction.
    // EMBEDDED_MOSAIC_DEFINITION_TRANSACTION = 16717,

    /// Mosaic supply change transaction.
    MOSAIC_SUPPLY_CHANGE_TRANSACTION = 16973,

    // /// Embedded mosaic supply change transaction.
    // EMBEDDED_MOSAIC_SUPPLY_CHANGE_TRANSACTION = 16973,

    /// Multisig account modification transaction.
    MULTISIG_ACCOUNT_MODIFICATION_TRANSACTION = 16725,

    // /// Embedded multisig account modification transaction.
    // EMBEDDED_MULTISIG_ACCOUNT_MODIFICATION_TRANSACTION = 16725,

    /// Address alias transaction.
    ADDRESS_ALIAS_TRANSACTION = 16974,

    // /// Embedded address alias transaction.
    // EMBEDDED_ADDRESS_ALIAS_TRANSACTION = 16974,

    /// Mosaic alias transaction.
    MOSAIC_ALIAS_TRANSACTION = 17230,

    // /// Embedded mosaic alias transaction.
    // EMBEDDED_MOSAIC_ALIAS_TRANSACTION = 17230,

    /// Namespace registration transaction.
    NAMESPACE_REGISTRATION_TRANSACTION = 16718,

    // /// Embedded namespace registration transaction.
    // EMBEDDED_NAMESPACE_REGISTRATION_TRANSACTION = 16718,

    /// Account address restriction transaction.
    ACCOUNT_ADDRESS_RESTRICTION_TRANSACTION = 16720,

    // /// Embedded account address restriction transaction.
    // EMBEDDED_ACCOUNT_ADDRESS_RESTRICTION_TRANSACTION = 16720,

    /// Account mosaic restriction transaction.
    ACCOUNT_MOSAIC_RESTRICTION_TRANSACTION = 16976,

    // /// Embedded account mosaic restriction transaction.
    // EMBEDDED_ACCOUNT_MOSAIC_RESTRICTION_TRANSACTION = 16976,

    /// Account operation restriction transaction.
    ACCOUNT_OPERATION_RESTRICTION_TRANSACTION = 17232,

    // /// Embedded account operation restriction transaction.
    // EMBEDDED_ACCOUNT_OPERATION_RESTRICTION_TRANSACTION = 17232,

    /// Mosaic address restriction transaction.
    MOSAIC_ADDRESS_RESTRICTION_TRANSACTION = 16977,

    // /// Embedded mosaic address restriction transaction.
    // EMBEDDED_MOSAIC_ADDRESS_RESTRICTION_TRANSACTION = 16977,

    /// Mosaic global restriction transaction.
    MOSAIC_GLOBAL_RESTRICTION_TRANSACTION = 16721,

    // /// Embedded mosaic global restriction transaction.
    // EMBEDDED_MOSAIC_GLOBAL_RESTRICTION_TRANSACTION = 16721,

    /// Transfer transaction.
    TRANSFER_TRANSACTION = 16724,

    // /// Embedded transfer transaction.
    // EMBEDDED_TRANSFER_TRANSACTION = 16724,
}

impl EntityTypeDto {
    pub const LENGTH: usize = std::mem::size_of::<Self>();

    /// Gets the size of the type.
    ///
    /// # Returns
    ///
    /// A usize.
    pub fn get_size(&self) -> usize {
        Self::LENGTH
    }

    /// Gets the value of the enum.
    ///
    /// # Returns
    ///
    /// A u16
    pub fn get_value(&self) -> u16 {
        self.to_u16().unwrap()
    }


    /// Creates an `EntityTypeDto` from a slice.
    ///
    /// # Returns
    ///
    /// A `EntityTypeDto`.
    pub fn from_binary(src: &[u8]) -> Self {
        // assert_eq!(src.len(), Self::LENGTH);
        let mut buf = [0x0u8; Self::LENGTH];
        buf.copy_from_slice(&src[..Self::LENGTH]);
        Self::from_u16(u16::from_le_bytes(buf)).unwrap()
    }

    /// Serializes an type to bytes.
    ///
    /// # Returns
    ///
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        self.get_value().to_le_bytes().to_vec()
    }
}
