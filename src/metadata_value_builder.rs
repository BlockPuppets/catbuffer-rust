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


/// Binary layout of a metadata entry value.
#[derive(Debug, Clone)]
pub struct MetadataValueBuilder {
    /// Data of the value.
    data: Vec<u8>,
}


impl MetadataValueBuilder {

    /// Creates an instance of MetadataValueBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A MetadataValueBuilder.
    pub fn from_binary(bytes_: &[u8]) -> Self {
        let mut buf = [0x0u8; 2];
        buf.copy_from_slice(&bytes_[..2]);
        let size = u16::from_le_bytes(buf); // kind:SIZE_FIELD
        let mut bytes_ = (&bytes_[2..]).to_vec();
        let data = (&bytes_[..size as usize]).to_vec(); // kind:BUFFER
        let bytes_ = (&bytes_[size as usize..]).to_vec();
        MetadataValueBuilder{data}
    }

    /// Gets data of the value.
    ///
    /// # Returns
    /// A Data of the value.
    pub fn get_data(&self) -> Vec<u8> {
        self.data.clone() // ARRAY or FILL_ARRAY
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += 2; // size;
        size += self.data.len();
        size
   }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut (self.get_data().len() as u16).to_le_bytes().to_vec()); // kind:SIZE_FIELD
        buf.append(&mut self.data.clone()); // kind:BUFFER
        buf
    }
}

