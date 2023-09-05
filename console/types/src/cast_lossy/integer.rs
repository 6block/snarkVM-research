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

impl<E: Environment, I: IntegerType> CastLossy<Address<E>> for Integer<E, I> {
    #[inline]
    fn cast_lossy(&self) -> Result<Address<E>> {
        self.cast()
    }
}

impl<E: Environment, I: IntegerType> CastLossy<Boolean<E>> for Integer<E, I> {
    #[inline]
    fn cast_lossy(&self) -> Result<Boolean<E>> {
        match self.to_bits_be().pop() {
            Some(bit) => Ok(Boolean::new(bit)),
            None => bail!("Failed to retrieve the LSB from the integer."),
        }
    }
}

impl<E: Environment, I: IntegerType> CastLossy<Field<E>> for Integer<E, I> {
    #[inline]
    fn cast_lossy(&self) -> Result<Field<E>> {
        self.cast()
    }
}

impl<E: Environment, I: IntegerType> CastLossy<Group<E>> for Integer<E, I> {
    #[inline]
    fn cast_lossy(&self) -> Result<Group<E>> {
        self.cast()
    }
}

impl<E: Environment, I0: IntegerType, I1: IntegerType + TryFrom<I0>> CastLossy<Integer<E, I1>> for Integer<E, I0> {
    #[inline]
    fn cast_lossy(&self) -> Result<Integer<E, I1>> {
        match I0::BITS <= I1::BITS {
            // If the target integer type is larger or the same size as the source integer type, then use the default cast.
            true => Ok(Integer::<E, I1>::new(match I1::try_from(**self) {
                Ok(value) => value,
                Err(_) => bail!("Failed to convert '{}' into '{}'", I0::type_name(), I1::type_name()),
            })),
            // Otherwise, use the lower bits of the source integer type.
            false => Integer::<E, I1>::from_bits_le(&self.to_bits_le()[0..usize::try_from(I1::BITS)?]),
        }
    }
}

impl<E: Environment, I: IntegerType> CastLossy<Scalar<E>> for Integer<E, I> {
    #[inline]
    fn cast_lossy(&self) -> Result<Scalar<E>> {
        self.cast()
    }
}
