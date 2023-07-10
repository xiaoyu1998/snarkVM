// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;

impl<N: Network> FromBytes for ConfirmedTransmissions<N> {
    /// Reads the confirmed transmissions from the buffer.
    #[inline]
    fn read_le<R: Read>(mut reader: R) -> IoResult<Self> {
        // Read the version.
        let version = u8::read_le(&mut reader)?;
        // Ensure the version is valid.
        if version != 0 {
            return Err(error("Invalid confirmed transmissions version"));
        }

        // Read the transactions.
        let transactions = FromBytes::read_le(&mut reader)?;

        // Read the ratifications.
        let num_ratifications = u32::read_le(&mut reader)?;
        let mut ratifications = Vec::with_capacity(num_ratifications as usize);
        for _ in 0..num_ratifications {
            ratifications.push(FromBytes::read_le(&mut reader)?);
        }

        // Read the coinbase.
        let coinbase_variant = u8::read_le(&mut reader)?;
        let coinbase = match coinbase_variant {
            0 => None,
            1 => Some(FromBytes::read_le(&mut reader)?),
            _ => return Err(error("Invalid coinbase variant")),
        };

        // Construct the confirmed transmissions.
        Ok(Self::from(transactions, ratifications, coinbase))
    }
}

impl<N: Network> ToBytes for ConfirmedTransmissions<N> {
    /// Writes the confirmed transmissions to the buffer.
    #[inline]
    fn write_le<W: Write>(&self, mut writer: W) -> IoResult<()> {
        // Write the version.
        0u8.write_le(&mut writer)?;

        // Write the transactions.
        self.transactions.write_le(&mut writer)?;

        // Write the ratifications.
        (u32::try_from(self.ratifications.len()).map_err(|e| error(e.to_string())))?.write_le(&mut writer)?;
        for ratification in &self.ratifications {
            ratification.write_le(&mut writer)?;
        }

        // Write the coinbase solution.
        match self.coinbase {
            None => 0u8.write_le(&mut writer)?,
            Some(ref coinbase) => {
                1u8.write_le(&mut writer)?;
                coinbase.write_le(&mut writer)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use console::network::Testnet3;

    type CurrentNetwork = Testnet3;

    #[test]
    fn test_bytes() -> Result<()> {
        let rng = &mut TestRng::default();

        for expected in [crate::confirmed_transmissions::test_helpers::sample_confirmed_transmissions(rng)].into_iter()
        {
            // Check the byte representation.
            let expected_bytes = expected.to_bytes_le()?;
            assert_eq!(expected, ConfirmedTransmissions::read_le(&expected_bytes[..])?);
            assert!(ConfirmedTransmissions::<CurrentNetwork>::read_le(&expected_bytes[1..]).is_err());
        }
        Ok(())
    }
}
