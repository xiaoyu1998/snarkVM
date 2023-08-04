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

#[cfg(console)]
impl<A: Aleo> ToBits for Signature<A> {
    type Boolean = Boolean<A>;

    /// Outputs the little-endian bit representation of the signature *without* trailing zeros.
    fn to_bits_le(&self) -> Vec<Self::Boolean> {
        (&self).to_bits_le()
    }

    /// Outputs the big-endian bit representation of the signature *without* leading zeros.
    fn to_bits_be(&self) -> Vec<Self::Boolean> {
        (&self).to_bits_be()
    }
}

#[cfg(console)]
impl<A: Aleo> ToBits for &Signature<A> {
    type Boolean = Boolean<A>;

    /// Outputs the little-endian bit representation of the signature *without* trailing zeros.
    fn to_bits_le(&self) -> Vec<Self::Boolean> {
        // Allocate the `bits_le` vector.
        let mut bits_le = Vec::with_capacity(console::Signature::<A::Network>::size_in_bits());
        // Write the challenge bits.
        bits_le.extend(self.challenge.to_bits_le());
        // Write the response bits.
        bits_le.extend(self.response.to_bits_le());
        // Write the compute key bits.
        bits_le.extend(self.compute_key.to_bits_le());
        // Return the `bits_le` vector.
        bits_le
    }

    /// Outputs the big-endian bit representation of the signature *without* leading zeros.
    fn to_bits_be(&self) -> Vec<Self::Boolean> {
        // Allocate the `bits_be` vector.
        let mut bits_be = Vec::with_capacity(console::Signature::<A::Network>::size_in_bits());
        // Write the challenge bits.
        bits_be.extend(self.challenge.to_bits_be());
        // Write the response bits.
        bits_be.extend(self.response.to_bits_be());
        // Write the compute key bits.
        bits_be.extend(self.compute_key.to_bits_be());
        // Return the `bits_be` vector.
        bits_be
    }
}

#[cfg(all(test, console))]
mod tests {
    use super::*;
    use crate::Circuit;
    use snarkvm_circuit_network::AleoV0;
    use snarkvm_utilities::TestRng;

    type CurrentAleo = AleoV0;

    const ITERATIONS: u64 = 100;

    fn check_to_bits_le(mode: Mode, num_constants: u64, num_public: u64, num_private: u64, num_constraints: u64) {
        let expected_number_of_bits = console::Signature::<<CurrentAleo as Environment>::Network>::size_in_bits();

        let rng = &mut TestRng::default();

        for i in 0..ITERATIONS {
            // Sample a random signature.
            let expected = crate::helpers::generate_signature(i, rng);
            let candidate = Signature::<CurrentAleo>::new(mode, expected);

            CurrentAleo::scope(&format!("{mode} {i}"), || {
                let candidate = candidate.to_bits_le();
                assert_eq!(expected_number_of_bits, candidate.len());

                // Construct the expected bits.
                let mut expected_bits = Vec::new();
                expected_bits.extend(expected.challenge().to_bits_le());
                expected_bits.extend(expected.response().to_bits_le());
                expected_bits.extend(expected.compute_key().to_bits_le());

                for (expected_bit, candidate_bit) in expected_bits.iter().zip_eq(candidate.iter()) {
                    assert_eq!(*expected_bit, candidate_bit.eject_value());
                }
                assert_scope!(num_constants, num_public, num_private, num_constraints);
            });
        }
    }

    fn check_to_bits_be(mode: Mode, num_constants: u64, num_public: u64, num_private: u64, num_constraints: u64) {
        let expected_number_of_bits = console::Signature::<<CurrentAleo as Environment>::Network>::size_in_bits();

        let rng = &mut TestRng::default();

        for i in 0..ITERATIONS {
            // Sample a random signature.
            let expected = crate::helpers::generate_signature(i, rng);
            let candidate = Signature::<CurrentAleo>::new(mode, expected);

            CurrentAleo::scope(&format!("{mode} {i}"), || {
                let candidate = candidate.to_bits_be();
                assert_eq!(expected_number_of_bits, candidate.len());

                // Construct the expected bits.
                let mut expected_bits = Vec::new();
                expected_bits.extend(expected.challenge().to_bits_be());
                expected_bits.extend(expected.response().to_bits_be());
                expected_bits.extend(expected.compute_key().to_bits_be());

                for (expected_bit, candidate_bit) in expected_bits.iter().zip_eq(candidate.iter()) {
                    assert_eq!(*expected_bit, candidate_bit.eject_value());
                }
                assert_scope!(num_constants, num_public, num_private, num_constraints);
            });
        }
    }

    #[test]
    fn test_to_bits_le_constant() {
        check_to_bits_le(Mode::Constant, 1008, 0, 0, 0);
    }

    #[test]
    fn test_to_bits_le_public() {
        check_to_bits_le(Mode::Public, 0, 0, 2012, 2020);
    }

    #[test]
    fn test_to_bits_le_private() {
        check_to_bits_le(Mode::Private, 0, 0, 2012, 2020);
    }

    #[test]
    fn test_to_bits_be_constant() {
        check_to_bits_be(Mode::Constant, 1008, 0, 0, 0);
    }

    #[test]
    fn test_to_bits_be_public() {
        check_to_bits_be(Mode::Public, 0, 0, 2012, 2020);
    }

    #[test]
    fn test_to_bits_be_private() {
        check_to_bits_be(Mode::Private, 0, 0, 2012, 2020);
    }
}