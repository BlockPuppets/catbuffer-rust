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

/// Binary layout for address based account restriction.
#[derive(Debug, Clone)]
pub struct AccountRestrictionAddressValueBuilder {
    /// Restriction values.
    restriction_values: Vec<AddressDto>,
}


impl AccountRestrictionAddressValueBuilder {

    /// Creates an instance of AccountRestrictionAddressValueBuilder from binary payload.
    /// payload: Byte payload to use to serialize the object.
    /// # Returns
    /// A AccountRestrictionAddressValueBuilder.
    pub fn from_binary(bytes_: &[u8]) -> Self {
        let mut buf = [0x0u8; 8];
        buf.copy_from_slice(&bytes_[..8]);
        let restrictionValuesCount = u64::from_le_bytes(buf); // kind:SIZE_FIELD
        let mut bytes_ = (&bytes_[8..]).to_vec();
        let mut restriction_values: Vec<AddressDto> = vec![]; // kind:ARRAY
        let mut bytes_= bytes_.to_vec();
        for _ in 0..restrictionValuesCount {
            let item = AddressDto::from_binary(&bytes_);
            restriction_values.push(item.clone());
            bytes_ = (&bytes_[item.get_size()..]).to_vec();
        }
        AccountRestrictionAddressValueBuilder{restriction_values}
    }

    /// Gets restriction values.
    ///
    /// # Returns
    /// A Restriction values.
    pub fn get_restriction_values(&self) -> Vec<AddressDto> {
        self.restriction_values.clone() // ARRAY or FILL_ARRAY
    }

    /// Gets the size of the type.
    ///
    /// Returns:
    /// A size in bytes.
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += 8; // restriction_values_count;
        size += self.restriction_values.iter().map(|item| item.get_size()).sum::<usize>(); // array or fill_array;
        size
   }

    /// Serializes self to bytes.
    ///
    /// # Returns
    /// A Serialized bytes.
    pub fn serializer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.append(&mut (self.get_restriction_values().len() as u64).to_le_bytes().to_vec()); // kind:SIZE_FIELD
        for i in &self.restriction_values {
            buf.append(&mut i.serializer()); // kind:ARRAY|FILL_ARRAY
        }
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value() {
        let payload = "02000000000000006826D27E1D0A26CA4E316F901E23E55C8711DB20DF250DEF6826D27E1D0A26CA4E316F901E23E55C8711DB20DF250DFE";
        let binary = hex::decode(payload).unwrap();
        let dto = AccountRestrictionAddressValueBuilder::from_binary(&binary);
        // println!("{:?}", binary);
        println!("{:#?}", dto);

        assert_eq!(binary, dto.serializer());
    }
}
