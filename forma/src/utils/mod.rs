// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod extend;
mod order;
mod prefix_scan;
pub mod simd;
mod small_bit_set;

pub use self::{
    extend::{ExtendTuple10, ExtendTuple3, ExtendVec},
    order::{Order, OrderError},
    prefix_scan::{PrefixScanIter, PrefixScanParIter},
    small_bit_set::SmallBitSet,
};

pub trait CanonBits {
    fn to_canon_bits(self) -> u32;
}

impl CanonBits for f32 {
    fn to_canon_bits(self) -> u32 {
        if self.is_nan() {
            return f32::NAN.to_bits();
        }

        if self == 0.0 {
            return 0.0f32.to_bits();
        }

        self.to_bits()
    }
}

pub trait RoundToBit {
    fn round_to_bit<const B: usize>(self) -> f32;
}

impl RoundToBit for f32 {
    fn round_to_bit<const B: usize>(self) -> f32 {
        let shift_left: f32 = (B as f32).exp2();
        let shift_right: f32 = shift_left.recip();

        self.mul_add(shift_left, 0.5).floor() * shift_right
    }
}

pub trait DivCeil {
    fn div_ceil_(self, other: Self) -> Self;
}

impl DivCeil for u32 {
    fn div_ceil_(self, other: Self) -> Self {
        (self + other - 1) / other
    }
}

impl DivCeil for usize {
    fn div_ceil_(self, other: Self) -> Self {
        (self + other - 1) / other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f32_canon_bits_nan() {
        let nan0 = f32::NAN;
        let nan1 = nan0 + 1.0;

        assert_ne!(nan0, nan1);
        assert_eq!(nan0.to_canon_bits(), nan1.to_canon_bits());
    }

    #[test]
    fn f32_canon_bits_zero() {
        let neg_zero = -0.0f32;
        let pos_zero = 0.0;

        assert_eq!(neg_zero, pos_zero);
        assert_ne!(neg_zero.to_bits(), pos_zero.to_bits());
        assert_eq!(neg_zero.to_canon_bits(), pos_zero.to_canon_bits());
    }

    #[test]
    fn round_to_bit_subnormal() {
        let val = 0.000_000_000_000_000_123_456;

        let rounded = val.round_to_bit::<64>();

        assert!(0.000_000_000_000_000_123_4 < rounded);
        assert!(rounded < val);
    }

    #[test]
    fn round_to_bit_large_number() {
        let val = 123.456;

        let rounded = val.round_to_bit::<4>();

        assert!(123.4 < rounded);
        assert!(rounded < val);
    }

    #[test]
    fn round_to_bit_large_number_outside_of_range() {
        let val = 123.456;

        let rounded = val.round_to_bit::<20>();

        assert_eq!(rounded, val);
    }
}
