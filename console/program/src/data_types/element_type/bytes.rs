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

impl<N: Network> FromBytes for ElementType<N> {
    /// Reads an element type from a buffer.
    fn read_le<R: Read>(mut reader: R) -> IoResult<Self> {
        let variant = u8::read_le(&mut reader)?;
        match variant {
            0 => Ok(Self::Literal(LiteralType::read_le(&mut reader)?)),
            1 => Ok(Self::Struct(Identifier::read_le(&mut reader)?)),
            2.. => Err(error(format!("Failed to deserialize annotation variant {variant}"))),
        }
    }
}

impl<N: Network> ToBytes for ElementType<N> {
    /// Writes an element type to a buffer.
    fn write_le<W: Write>(&self, mut writer: W) -> IoResult<()> {
        match self {
            Self::Literal(literal_type) => {
                u8::write_le(&0u8, &mut writer)?;
                literal_type.write_le(&mut writer)
            }
            Self::Struct(identifier) => {
                u8::write_le(&1u8, &mut writer)?;
                identifier.write_le(&mut writer)
            }
        }
    }
}