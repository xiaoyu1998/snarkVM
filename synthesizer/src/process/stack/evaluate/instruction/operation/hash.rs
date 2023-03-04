// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use super::*;

impl<N: Network> Stack<N> {
    /// Evaluates the instruction.
    #[inline]
    pub fn evaluate<A: circuit::Aleo<Network = N>>(
        &self,
        stack: &Stack<N>,
        registers: &mut Registers<N, A>,
    ) -> Result<()> {
        // Ensure the number of operands is correct.
        if self.operands.len() != 1 {
            bail!("Instruction '{}' expects 1 operands, found {} operands", Self::opcode(), self.operands.len())
        }
        // Load the operand.
        let input = registers.load(stack, &self.operands[0])?;
        // Hash the input.
        let output = match VARIANT {
            0 => N::hash_bhp256(&input.to_bits_le())?,
            1 => N::hash_bhp512(&input.to_bits_le())?,
            2 => N::hash_bhp768(&input.to_bits_le())?,
            3 => N::hash_bhp1024(&input.to_bits_le())?,
            4 => N::hash_ped64(&input.to_bits_le())?,
            5 => N::hash_ped128(&input.to_bits_le())?,
            6 => N::hash_psd2(&input.to_fields()?)?,
            7 => N::hash_psd4(&input.to_fields()?)?,
            8 => N::hash_psd8(&input.to_fields()?)?,
            _ => bail!("Invalid 'hash' variant: {VARIANT}"),
        };
        // Store the output.
        registers.store(stack, &self.destination, Value::Plaintext(Plaintext::from(Literal::Field(output))))
    }
}
