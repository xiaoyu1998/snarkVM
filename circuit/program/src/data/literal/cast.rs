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

impl<A: Aleo> Literal<A> {
    /// Casts the literal to the given literal type.
    ///
    /// This method checks that the cast does not lose any bits of information,
    /// and returns an error if it does.
    ///
    /// The hierarchy of downcasting is as follows:
    ///  - (`Address`, `Group`) -> `Field` -> `Scalar` -> `Integer` -> `Boolean`
    ///  - `String` (not supported)
    ///
    /// The hierarchy of upcasting is the inverse of the downcasting hierarchy.
    pub fn cast(&self, to_type: LiteralType) -> Result<Self> {
        match self {
            Self::Address(address) => cast_group_to_type(address.to_group(), to_type),
            Self::Boolean(..) => bail!("Cannot cast a boolean literal to another type (yet)."),
            Self::Field(field) => cast_field_to_type(field.clone(), to_type),
            Self::Group(group) => cast_group_to_type(group.clone(), to_type),
            Self::I8(..) => bail!("Cannot cast an i8 literal to another type (yet)."),
            Self::I16(..) => bail!("Cannot cast an i16 literal to another type (yet)."),
            Self::I32(..) => bail!("Cannot cast an i32 literal to another type (yet)."),
            Self::I64(..) => bail!("Cannot cast an i64 literal to another type (yet)."),
            Self::I128(..) => bail!("Cannot cast an i128 literal to another type (yet)."),
            Self::U8(..) => bail!("Cannot cast a u8 literal to another type (yet)."),
            Self::U16(..) => bail!("Cannot cast a u16 literal to another type (yet)."),
            Self::U32(..) => bail!("Cannot cast a u32 literal to another type (yet)."),
            Self::U64(..) => bail!("Cannot cast a u64 literal to another type (yet)."),
            Self::U128(..) => bail!("Cannot cast a u128 literal to another type (yet)."),
            Self::Scalar(..) => bail!("Cannot cast a scalar literal to another type (yet)."),
            Self::String(..) => bail!("Cannot cast a string literal to another type."),
        }
    }

    /// Casts the literal to the given literal type, with lossy truncation.
    ///
    /// This method makes a *best-effort* attempt to preserve upcasting back
    /// to the original literal type, but it is not guaranteed to do so.
    ///
    /// The hierarchy of downcasting is as follows:
    ///  - (`Address`, `Group`) -> `Field` -> `Scalar` -> `Integer` -> `Boolean`
    ///  - `String` (not supported)
    ///
    /// The hierarchy of upcasting is the inverse of the downcasting hierarchy.
    pub fn cast_lossy(&self, to_type: LiteralType) -> Result<Self> {
        match self {
            Self::Address(address) => cast_lossy_group_to_type(address.to_group(), to_type),
            Self::Boolean(..) => bail!("Cannot lossily cast a boolean literal to another type (yet)."),
            Self::Field(field) => cast_lossy_field_to_type(field.clone(), to_type),
            Self::Group(group) => cast_lossy_group_to_type(group.clone(), to_type),
            Self::I8(..) => bail!("Cannot lossily cast an i8 literal to another type (yet)."),
            Self::I16(..) => bail!("Cannot lossily cast an i16 literal to another type (yet)."),
            Self::I32(..) => bail!("Cannot lossily cast an i32 literal to another type (yet)."),
            Self::I64(..) => bail!("Cannot lossily cast an i64 literal to another type (yet)."),
            Self::I128(..) => bail!("Cannot lossily cast an i128 literal to another type (yet)."),
            Self::U8(..) => bail!("Cannot lossily cast a u8 literal to another type (yet)."),
            Self::U16(..) => bail!("Cannot lossily cast a u16 literal to another type (yet)."),
            Self::U32(..) => bail!("Cannot lossily cast a u32 literal to another type (yet)."),
            Self::U64(..) => bail!("Cannot lossily cast a u64 literal to another type (yet)."),
            Self::U128(..) => bail!("Cannot lossily cast a u128 literal to another type (yet)."),
            Self::Scalar(..) => bail!("Cannot lossily cast a scalar literal to another type (yet)."),
            Self::String(..) => bail!("Cannot lossily cast a string literal to another type."),
        }
    }
}

/// Casts a field literal to the given literal type.
fn cast_field_to_type<A: Aleo>(field: Field<A>, to_type: LiteralType) -> Result<Literal<A>> {
    match to_type {
        LiteralType::Address => bail!("Cannot cast a field literal to an address type (yet)."),
        LiteralType::Boolean => bail!("Cannot cast a field literal to a boolean type (yet)."),
        LiteralType::Field => Ok(Literal::Field(field)),
        LiteralType::Group => bail!("Cannot cast a field literal to a group type (yet)."),
        LiteralType::I8 => Ok(Literal::I8(I8::from_field(field))),
        LiteralType::I16 => Ok(Literal::I16(I16::from_field(field))),
        LiteralType::I32 => Ok(Literal::I32(I32::from_field(field))),
        LiteralType::I64 => Ok(Literal::I64(I64::from_field(field))),
        LiteralType::I128 => Ok(Literal::I128(I128::from_field(field))),
        LiteralType::U8 => Ok(Literal::U8(U8::from_field(field))),
        LiteralType::U16 => Ok(Literal::U16(U16::from_field(field))),
        LiteralType::U32 => Ok(Literal::U32(U32::from_field(field))),
        LiteralType::U64 => Ok(Literal::U64(U64::from_field(field))),
        LiteralType::U128 => Ok(Literal::U128(U128::from_field(field))),
        LiteralType::Scalar => Ok(Literal::Scalar(Scalar::from_field(field))),
        LiteralType::String => bail!("Cannot cast a field literal to a string type."),
    }
}

/// Casts a field literal to the given literal type, with lossy truncation.
fn cast_lossy_field_to_type<A: Aleo>(field: Field<A>, to_type: LiteralType) -> Result<Literal<A>> {
    match to_type {
        LiteralType::Address => bail!("Cannot lossily cast a field literal to an address type (yet)."),
        LiteralType::Boolean => bail!("Cannot lossily cast a field literal to a boolean type (yet)."),
        LiteralType::Field => Ok(Literal::Field(field)),
        LiteralType::Group => bail!("Cannot lossily cast a field literal to a group type (yet)."),
        LiteralType::I8 => Ok(Literal::I8(I8::from_field_lossy(field))),
        LiteralType::I16 => Ok(Literal::I16(I16::from_field_lossy(field))),
        LiteralType::I32 => Ok(Literal::I32(I32::from_field_lossy(field))),
        LiteralType::I64 => Ok(Literal::I64(I64::from_field_lossy(field))),
        LiteralType::I128 => Ok(Literal::I128(I128::from_field_lossy(field))),
        LiteralType::U8 => Ok(Literal::U8(U8::from_field_lossy(field))),
        LiteralType::U16 => Ok(Literal::U16(U16::from_field_lossy(field))),
        LiteralType::U32 => Ok(Literal::U32(U32::from_field_lossy(field))),
        LiteralType::U64 => Ok(Literal::U64(U64::from_field_lossy(field))),
        LiteralType::U128 => Ok(Literal::U128(U128::from_field_lossy(field))),
        LiteralType::Scalar => Ok(Literal::Scalar(Scalar::from_field_lossy(field))),
        LiteralType::String => bail!("Cannot lossily cast a field literal to a string type."),
    }
}

/// Casts a group literal to the given literal type.
fn cast_group_to_type<A: Aleo>(group: Group<A>, to_type: LiteralType) -> Result<Literal<A>> {
    match to_type {
        LiteralType::Address => Ok(Literal::Address(Address::from_group(group))),
        LiteralType::Group => Ok(Literal::Group(group)),
        _ => cast_field_to_type(group.to_x_coordinate(), to_type),
    }
}

/// Casts a group literal to the given literal type, with lossy truncation.
fn cast_lossy_group_to_type<A: Aleo>(group: Group<A>, to_type: LiteralType) -> Result<Literal<A>> {
    match to_type {
        LiteralType::Address => Ok(Literal::Address(Address::from_group(group))),
        LiteralType::Group => Ok(Literal::Group(group)),
        _ => cast_lossy_field_to_type(group.to_x_coordinate(), to_type),
    }
}