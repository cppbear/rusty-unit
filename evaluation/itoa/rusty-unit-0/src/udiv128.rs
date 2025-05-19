#[cfg(feature = "no-panic")]
use no_panic::no_panic;

/// Multiply unsigned 128 bit integers, return upper 128 bits of the result
#[inline]
#[cfg_attr(feature = "no-panic", no_panic)]
fn u128_mulhi(x: u128, y: u128) -> u128 {
    let x_lo = x as u64;
    let x_hi = (x >> 64) as u64;
    let y_lo = y as u64;
    let y_hi = (y >> 64) as u64;

    // handle possibility of overflow
    let carry = (x_lo as u128 * y_lo as u128) >> 64;
    let m = x_lo as u128 * y_hi as u128 + carry;
    let high1 = m >> 64;

    let m_lo = m as u64;
    let high2 = (x_hi as u128 * y_lo as u128 + m_lo as u128) >> 64;

    x_hi as u128 * y_hi as u128 + high1 + high2
}

/// Divide `n` by 1e19 and return quotient and remainder
///
/// Integer division algorithm is based on the following paper:
///
///   T. Granlund and P. Montgomery, “Division by Invariant Integers Using Multiplication”
///   in Proc. of the SIGPLAN94 Conference on Programming Language Design and
///   Implementation, 1994, pp. 61–72
///
#[inline]
#[cfg_attr(feature = "no-panic", no_panic)]
pub fn udivmod_1e19(n: u128) -> (u128, u64) {
    let d = 10_000_000_000_000_000_000_u64; // 10^19

    let quot = if n < 1 << 83 {
        ((n >> 19) as u64 / (d >> 19)) as u128
    } else {
        u128_mulhi(n, 156927543384667019095894735580191660403) >> 62
    };

    let rem = (n - quot * d as u128) as u64;
    debug_assert_eq!(quot, n / d as u128);
    debug_assert_eq!(rem as u128, n % d as u128);

    (quot, rem)
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_0() {
    rusty_monitor::set_test_id(0);
    let mut u128_0: u128 = 4289u128;
    let mut u128_1: u128 = 2042u128;
    let mut u128_2: u128 = 9261u128;
    let mut u128_3: u128 = 8168u128;
    let mut u128_4: u128 = 4184u128;
    let mut u128_5: u128 = 1149u128;
    let mut u128_6: u128 = 1034u128;
    let mut u128_7: u128 = 3u128;
    let mut u128_8: u128 = 1373u128;
    let mut u128_9: u128 = 1393u128;
    let mut u128_10: u128 = 8476u128;
    let mut u128_11: u128 = 7095u128;
    let mut u128_12: u128 = 4778u128;
    let mut u128_13: u128 = 5530u128;
    let mut u128_14: u128 = 4876u128;
    let mut u128_15: u128 = 7359u128;
    let mut u128_16: u128 = 5556u128;
    let mut u128_17: u128 = 7630u128;
    let mut u128_18: u128 = 993u128;
    let mut u128_19: u128 = 8084u128;
    let mut u128_20: u128 = 7095u128;
    let mut u128_21: u128 = 5223u128;
    let mut u128_22: u128 = 8699u128;
    let mut u128_23: u128 = 3677u128;
    let mut u128_24: u128 = 7828u128;
    let mut u128_25: u128 = 1404u128;
    let mut u128_26: u128 = 6774u128;
    let mut u128_27: u128 = 3996u128;
    let mut u128_28: u128 = 4564u128;
    let mut u128_29: u128 = 8440u128;
    let mut u128_30: u128 = 5425u128;
    let mut u128_31: u128 = 6604u128;
    let mut u128_32: u128 = 5374u128;
    let mut u128_33: u128 = 2008u128;
    let mut u128_34: u128 = 6399u128;
    let mut u128_35: u128 = 3408u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut u128_36: u128 = crate::udiv128::u128_mulhi(u128_35, u128_34);
    let mut u128_37: u128 = crate::udiv128::u128_mulhi(u128_33, u128_32);
    let mut u128_38: u128 = crate::udiv128::u128_mulhi(u128_31, u128_30);
    let mut u128_39: u128 = crate::udiv128::u128_mulhi(u128_29, u128_28);
    let mut u128_40: u128 = crate::udiv128::u128_mulhi(u128_27, u128_26);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_25);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_24);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_23);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut u128_41: u128 = crate::udiv128::u128_mulhi(u128_22, u128_21);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_20);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut buffer_10: crate::Buffer = crate::Buffer::default();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_19);
    let mut u128_42: u128 = crate::udiv128::u128_mulhi(u128_18, u128_17);
    let mut u128_43: u128 = crate::udiv128::u128_mulhi(u128_16, u128_15);
    let mut u128_44: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut u128_45: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut u128_46: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_11: crate::Buffer = crate::Buffer::default();
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut tuple_8: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut tuple_9: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_12: crate::Buffer = crate::Buffer::default();
    let mut tuple_10: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut buffer_13: crate::Buffer = crate::Buffer::default();
    let mut buffer_14: crate::Buffer = crate::Buffer::new();
    let mut tuple_11: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut u128_47: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1() {
    rusty_monitor::set_test_id(1);
    let mut u128_0: u128 = 7710u128;
    let mut u128_1: u128 = 2491u128;
    let mut u128_2: u128 = 9138u128;
    let mut u128_3: u128 = 9340u128;
    let mut u128_4: u128 = 6242u128;
    let mut u128_5: u128 = 6691u128;
    let mut u128_6: u128 = 4539u128;
    let mut u128_7: u128 = 9164u128;
    let mut u128_8: u128 = 263u128;
    let mut u128_9: u128 = 4255u128;
    let mut u128_10: u128 = 4551u128;
    let mut u128_11: u128 = 229u128;
    let mut u128_12: u128 = 657u128;
    let mut u128_13: u128 = 4594u128;
    let mut u128_14: u128 = 1785u128;
    let mut u128_15: u128 = 231u128;
    let mut u128_16: u128 = 1289u128;
    let mut u128_17: u128 = 4365u128;
    let mut u128_18: u128 = 3689u128;
    let mut u128_19: u128 = 5879u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_19);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_18);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_17, u128_16);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_15);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut buffer_11: crate::Buffer = crate::Buffer::default();
    let mut u128_21: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut buffer_12: crate::Buffer = crate::Buffer::default();
    let mut buffer_13: crate::Buffer = crate::Buffer::new();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut buffer_14: crate::Buffer = crate::Buffer::new();
    let mut buffer_15: crate::Buffer = crate::Buffer::new();
    let mut buffer_16: crate::Buffer = crate::Buffer::new();
    let mut buffer_17: crate::Buffer = crate::Buffer::new();
    let mut buffer_18: crate::Buffer = crate::Buffer::new();
    let mut buffer_19: crate::Buffer = crate::Buffer::new();
    let mut u128_22: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut buffer_20: crate::Buffer = crate::Buffer::default();
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut tuple_8: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_21: crate::Buffer = crate::Buffer::default();
    let mut u128_23: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut tuple_9: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut tuple_10: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut tuple_11: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    let mut buffer_22: crate::Buffer = crate::Buffer::default();
    let mut buffer_23: crate::Buffer = crate::Buffer::new();
    let mut buffer_24: crate::Buffer = crate::Buffer::default();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2() {
    rusty_monitor::set_test_id(2);
    let mut u128_0: u128 = 4075u128;
    let mut u128_1: u128 = 5224u128;
    let mut u128_2: u128 = 9443u128;
    let mut u128_3: u128 = 5632u128;
    let mut u128_4: u128 = 5506u128;
    let mut u128_5: u128 = 3673u128;
    let mut u128_6: u128 = 4623u128;
    let mut u128_7: u128 = 643u128;
    let mut u128_8: u128 = 7125u128;
    let mut u128_9: u128 = 5515u128;
    let mut u128_10: u128 = 1283u128;
    let mut u128_11: u128 = 8622u128;
    let mut u128_12: u128 = 4447u128;
    let mut u128_13: u128 = 2510u128;
    let mut u128_14: u128 = 4391u128;
    let mut u128_15: u128 = 6429u128;
    let mut u128_16: u128 = 4712u128;
    let mut u128_17: u128 = 9918u128;
    let mut u128_18: u128 = 309u128;
    let mut u128_19: u128 = 4615u128;
    let mut u128_20: u128 = 4988u128;
    let mut u128_21: u128 = 7758u128;
    let mut u128_22: u128 = 1805u128;
    let mut u128_23: u128 = 4030u128;
    let mut u128_24: u128 = 769u128;
    let mut u128_25: u128 = 5392u128;
    let mut u128_26: u128 = 7574u128;
    let mut u128_27: u128 = 7739u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut u128_28: u128 = crate::udiv128::u128_mulhi(u128_27, u128_26);
    let mut u128_29: u128 = crate::udiv128::u128_mulhi(u128_25, u128_24);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_23);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_22);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut u128_30: u128 = crate::udiv128::u128_mulhi(u128_21, u128_20);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_19);
    let mut u128_31: u128 = crate::udiv128::u128_mulhi(u128_18, u128_17);
    let mut buffer_10: crate::Buffer = crate::Buffer::default();
    let mut u128_32: u128 = crate::udiv128::u128_mulhi(u128_16, u128_15);
    let mut buffer_11: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut u128_33: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut u128_34: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_12: crate::Buffer = crate::Buffer::new();
    let mut buffer_13: crate::Buffer = crate::Buffer::default();
    let mut buffer_14: crate::Buffer = crate::Buffer::default();
    let mut buffer_15: crate::Buffer = crate::Buffer::new();
    let mut buffer_16: crate::Buffer = crate::Buffer::default();
    let mut buffer_17: crate::Buffer = crate::Buffer::new();
    let mut buffer_18: crate::Buffer = crate::Buffer::default();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut buffer_19: crate::Buffer = crate::Buffer::new();
    let mut buffer_20: crate::Buffer = crate::Buffer::new();
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut u128_35: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_21: crate::Buffer = crate::Buffer::default();
    let mut buffer_22: crate::Buffer = crate::Buffer::default();
    let mut u128_36: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut u128_37: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut buffer_23: crate::Buffer = crate::Buffer::new();
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    let mut buffer_24: crate::Buffer = crate::Buffer::default();
    let mut buffer_25: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3() {
    rusty_monitor::set_test_id(3);
    let mut u128_0: u128 = 3662u128;
    let mut u128_1: u128 = 4619u128;
    let mut u128_2: u128 = 9239u128;
    let mut u128_3: u128 = 6242u128;
    let mut u128_4: u128 = 1411u128;
    let mut u128_5: u128 = 3095u128;
    let mut u128_6: u128 = 6732u128;
    let mut u128_7: u128 = 2728u128;
    let mut u128_8: u128 = 6689u128;
    let mut u128_9: u128 = 1428u128;
    let mut u128_10: u128 = 5159u128;
    let mut u128_11: u128 = 8043u128;
    let mut u128_12: u128 = 9660u128;
    let mut u128_13: u128 = 5791u128;
    let mut u128_14: u128 = 6212u128;
    let mut u128_15: u128 = 3942u128;
    let mut u128_16: u128 = 3072u128;
    let mut u128_17: u128 = 1694u128;
    let mut u128_18: u128 = 3473u128;
    let mut u128_19: u128 = 8905u128;
    let mut u128_20: u128 = 8842u128;
    let mut u128_21: u128 = 5905u128;
    let mut u128_22: u128 = 6612u128;
    let mut u128_23: u128 = 4763u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_23);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut u128_24: u128 = crate::udiv128::u128_mulhi(u128_22, u128_21);
    let mut u128_25: u128 = crate::udiv128::u128_mulhi(u128_20, u128_19);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_18);
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut u128_26: u128 = crate::udiv128::u128_mulhi(u128_17, u128_16);
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut u128_27: u128 = crate::udiv128::u128_mulhi(u128_15, u128_14);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut u128_28: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut buffer_11: crate::Buffer = crate::Buffer::new();
    let mut u128_29: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_12: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut buffer_13: crate::Buffer = crate::Buffer::new();
    let mut buffer_14: crate::Buffer = crate::Buffer::default();
    let mut buffer_15: crate::Buffer = crate::Buffer::new();
    let mut u128_30: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut buffer_16: crate::Buffer = crate::Buffer::default();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_17: crate::Buffer = crate::Buffer::new();
    let mut buffer_18: crate::Buffer = crate::Buffer::default();
    let mut u128_31: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut buffer_19: crate::Buffer = crate::Buffer::default();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4() {
    rusty_monitor::set_test_id(4);
    let mut u128_0: u128 = 6619u128;
    let mut u128_1: u128 = 952u128;
    let mut u128_2: u128 = 4692u128;
    let mut u128_3: u128 = 7775u128;
    let mut u128_4: u128 = 5057u128;
    let mut u128_5: u128 = 4961u128;
    let mut u128_6: u128 = 1563u128;
    let mut u128_7: u128 = 7634u128;
    let mut u128_8: u128 = 6478u128;
    let mut u128_9: u128 = 8968u128;
    let mut u128_10: u128 = 4594u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut u128_11: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut buffer_11: crate::Buffer = crate::Buffer::new();
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_12: crate::Buffer = crate::Buffer::default();
    let mut buffer_13: crate::Buffer = crate::Buffer::default();
    let mut buffer_14: crate::Buffer = crate::Buffer::new();
    let mut buffer_15: crate::Buffer = crate::Buffer::new();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut buffer_16: crate::Buffer = crate::Buffer::new();
    let mut buffer_17: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    let mut buffer_18: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5() {
    rusty_monitor::set_test_id(5);
    let mut u128_0: u128 = 569u128;
    let mut u128_1: u128 = 3184u128;
    let mut u128_2: u128 = 5942u128;
    let mut u128_3: u128 = 6990u128;
    let mut u128_4: u128 = 3808u128;
    let mut u128_5: u128 = 7818u128;
    let mut u128_6: u128 = 4314u128;
    let mut u128_7: u128 = 2579u128;
    let mut u128_8: u128 = 9678u128;
    let mut u128_9: u128 = 8866u128;
    let mut u128_10: u128 = 388u128;
    let mut u128_11: u128 = 5342u128;
    let mut u128_12: u128 = 5169u128;
    let mut u128_13: u128 = 6519u128;
    let mut u128_14: u128 = 2106u128;
    let mut u128_15: u128 = 323u128;
    let mut u128_16: u128 = 4383u128;
    let mut u128_17: u128 = 8549u128;
    let mut u128_18: u128 = 9296u128;
    let mut u128_19: u128 = 6497u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_19);
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_18, u128_17);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_16);
    let mut u128_21: u128 = crate::udiv128::u128_mulhi(u128_15, u128_14);
    let mut u128_22: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut u128_23: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut u128_24: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_11: crate::Buffer = crate::Buffer::default();
    let mut buffer_12: crate::Buffer = crate::Buffer::default();
    let mut u128_25: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut buffer_13: crate::Buffer = crate::Buffer::default();
    let mut buffer_14: crate::Buffer = crate::Buffer::new();
    let mut u128_26: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut buffer_15: crate::Buffer = crate::Buffer::default();
    let mut buffer_16: crate::Buffer = crate::Buffer::default();
    let mut buffer_17: crate::Buffer = crate::Buffer::default();
    let mut buffer_18: crate::Buffer = crate::Buffer::new();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    let mut buffer_19: crate::Buffer = crate::Buffer::default();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6() {
    rusty_monitor::set_test_id(6);
    let mut u128_0: u128 = 5915u128;
    let mut u128_1: u128 = 1419u128;
    let mut u128_2: u128 = 1721u128;
    let mut u128_3: u128 = 503u128;
    let mut u128_4: u128 = 8594u128;
    let mut u128_5: u128 = 8828u128;
    let mut u128_6: u128 = 5742u128;
    let mut u128_7: u128 = 4616u128;
    let mut u128_8: u128 = 4005u128;
    let mut u128_9: u128 = 2946u128;
    let mut u128_10: u128 = 4559u128;
    let mut u128_11: u128 = 433u128;
    let mut u128_12: u128 = 5886u128;
    let mut u128_13: u128 = 6435u128;
    let mut u128_14: u128 = 8171u128;
    let mut u128_15: u128 = 3673u128;
    let mut u128_16: u128 = 4720u128;
    let mut u128_17: u128 = 4611u128;
    let mut u128_18: u128 = 4636u128;
    let mut u128_19: u128 = 3164u128;
    let mut u128_20: u128 = 1038u128;
    let mut u128_21: u128 = 6904u128;
    let mut u128_22: u128 = 4613u128;
    let mut u128_23: u128 = 6557u128;
    let mut u128_24: u128 = 9709u128;
    let mut u128_25: u128 = 1745u128;
    let mut u128_26: u128 = 6925u128;
    let mut u128_27: u128 = 3102u128;
    let mut u128_28: u128 = 6043u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_28);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_27);
    let mut u128_29: u128 = crate::udiv128::u128_mulhi(u128_26, u128_25);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_24);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_23);
    let mut u128_30: u128 = crate::udiv128::u128_mulhi(u128_22, u128_21);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_20);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_19);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut u128_31: u128 = crate::udiv128::u128_mulhi(u128_18, u128_17);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut u128_32: u128 = crate::udiv128::u128_mulhi(u128_16, u128_15);
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut buffer_11: crate::Buffer = crate::Buffer::new();
    let mut buffer_12: crate::Buffer = crate::Buffer::new();
    let mut tuple_8: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut u128_33: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut u128_34: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut u128_35: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut tuple_9: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_13: crate::Buffer = crate::Buffer::default();
    let mut tuple_10: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_14: crate::Buffer = crate::Buffer::default();
    let mut buffer_15: crate::Buffer = crate::Buffer::default();
    let mut tuple_11: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut buffer_16: crate::Buffer = crate::Buffer::default();
    let mut buffer_17: crate::Buffer = crate::Buffer::default();
    let mut tuple_12: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_18: crate::Buffer = crate::Buffer::default();
    let mut u128_36: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7() {
    rusty_monitor::set_test_id(7);
    let mut u128_0: u128 = 625u128;
    let mut u128_1: u128 = 7869u128;
    let mut u128_2: u128 = 9085u128;
    let mut u128_3: u128 = 2984u128;
    let mut u128_4: u128 = 8549u128;
    let mut u128_5: u128 = 9655u128;
    let mut u128_6: u128 = 9773u128;
    let mut u128_7: u128 = 6292u128;
    let mut u128_8: u128 = 8382u128;
    let mut u128_9: u128 = 7399u128;
    let mut u128_10: u128 = 8418u128;
    let mut u128_11: u128 = 1628u128;
    let mut u128_12: u128 = 1229u128;
    let mut u128_13: u128 = 8713u128;
    let mut u128_14: u128 = 1384u128;
    let mut u128_15: u128 = 6654u128;
    let mut u128_16: u128 = 5564u128;
    let mut u128_17: u128 = 4677u128;
    let mut u128_18: u128 = 1249u128;
    let mut u128_19: u128 = 8015u128;
    let mut u128_20: u128 = 2631u128;
    let mut u128_21: u128 = 1139u128;
    let mut u128_22: u128 = crate::udiv128::u128_mulhi(u128_21, u128_20);
    let mut u128_23: u128 = crate::udiv128::u128_mulhi(u128_19, u128_18);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_17);
    let mut u128_24: u128 = crate::udiv128::u128_mulhi(u128_16, u128_15);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut u128_25: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut u128_26: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut u128_27: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut u128_28: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut u128_29: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut u128_30: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8() {
    rusty_monitor::set_test_id(8);
    let mut u128_0: u128 = 1317u128;
    let mut u128_1: u128 = 7893u128;
    let mut u128_2: u128 = 2242u128;
    let mut u128_3: u128 = 1382u128;
    let mut u128_4: u128 = 4956u128;
    let mut u128_5: u128 = 8674u128;
    let mut u128_6: u128 = 7809u128;
    let mut u128_7: u128 = 6996u128;
    let mut u128_8: u128 = 3193u128;
    let mut u128_9: u128 = 992u128;
    let mut u128_10: u128 = 1278u128;
    let mut u128_11: u128 = 2004u128;
    let mut u128_12: u128 = 1646u128;
    let mut u128_13: u128 = 7849u128;
    let mut u128_14: u128 = 5004u128;
    let mut u128_15: u128 = 8371u128;
    let mut u128_16: u128 = 7369u128;
    let mut u128_17: u128 = 3502u128;
    let mut u128_18: u128 = 9135u128;
    let mut u128_19: u128 = 7529u128;
    let mut u128_20: u128 = 4424u128;
    let mut u128_21: u128 = 6706u128;
    let mut u128_22: u128 = 7496u128;
    let mut u128_23: u128 = 1480u128;
    let mut u128_24: u128 = 8893u128;
    let mut u128_25: u128 = 4623u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_25);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_24);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_23);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_22);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_21);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_20);
    let mut u128_26: u128 = crate::udiv128::u128_mulhi(u128_19, u128_18);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut u128_27: u128 = crate::udiv128::u128_mulhi(u128_17, u128_16);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut u128_28: u128 = crate::udiv128::u128_mulhi(u128_15, u128_14);
    let mut buffer_10: crate::Buffer = crate::Buffer::default();
    let mut buffer_11: crate::Buffer = crate::Buffer::new();
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut buffer_12: crate::Buffer = crate::Buffer::default();
    let mut buffer_13: crate::Buffer = crate::Buffer::new();
    let mut buffer_14: crate::Buffer = crate::Buffer::new();
    let mut u128_29: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut buffer_15: crate::Buffer = crate::Buffer::default();
    let mut buffer_16: crate::Buffer = crate::Buffer::new();
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut buffer_17: crate::Buffer = crate::Buffer::new();
    let mut u128_30: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_18: crate::Buffer = crate::Buffer::default();
    let mut buffer_19: crate::Buffer = crate::Buffer::new();
    let mut tuple_8: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut buffer_20: crate::Buffer = crate::Buffer::new();
    let mut buffer_21: crate::Buffer = crate::Buffer::new();
    let mut buffer_22: crate::Buffer = crate::Buffer::new();
    let mut u128_31: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut u128_32: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut buffer_23: crate::Buffer = crate::Buffer::new();
    let mut u128_33: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut buffer_24: crate::Buffer = crate::Buffer::new();
    let mut tuple_9: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    let mut buffer_25: crate::Buffer = crate::Buffer::new();
    let mut buffer_26: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_9() {
    rusty_monitor::set_test_id(9);
    let mut u128_0: u128 = 1985u128;
    let mut u128_1: u128 = 4243u128;
    let mut u128_2: u128 = 8384u128;
    let mut u128_3: u128 = 5893u128;
    let mut u128_4: u128 = 7764u128;
    let mut u128_5: u128 = 6017u128;
    let mut u128_6: u128 = 2564u128;
    let mut u128_7: u128 = 637u128;
    let mut u128_8: u128 = 1130u128;
    let mut u128_9: u128 = 1619u128;
    let mut u128_10: u128 = 1681u128;
    let mut u128_11: u128 = 3194u128;
    let mut u128_12: u128 = 4463u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_11: crate::Buffer = crate::Buffer::default();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_12: crate::Buffer = crate::Buffer::new();
    let mut buffer_13: crate::Buffer = crate::Buffer::new();
    let mut buffer_14: crate::Buffer = crate::Buffer::new();
    let mut buffer_15: crate::Buffer = crate::Buffer::default();
    let mut buffer_16: crate::Buffer = crate::Buffer::default();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_10() {
    rusty_monitor::set_test_id(10);
    let mut u128_0: u128 = 7853u128;
    let mut u128_1: u128 = 7278u128;
    let mut u128_2: u128 = 1457u128;
    let mut u128_3: u128 = 6767u128;
    let mut u128_4: u128 = 9938u128;
    let mut u128_5: u128 = 5220u128;
    let mut u128_6: u128 = 5602u128;
    let mut u128_7: u128 = 8376u128;
    let mut u128_8: u128 = 4996u128;
    let mut u128_9: u128 = 1816u128;
    let mut u128_10: u128 = 4131u128;
    let mut u128_11: u128 = 3466u128;
    let mut u128_12: u128 = 603u128;
    let mut u128_13: u128 = 4882u128;
    let mut u128_14: u128 = 4939u128;
    let mut u128_15: u128 = 6527u128;
    let mut u128_16: u128 = 9455u128;
    let mut u128_17: u128 = 9510u128;
    let mut u128_18: u128 = 2336u128;
    let mut u128_19: u128 = 9070u128;
    let mut u128_20: u128 = 8819u128;
    let mut u128_21: u128 = 9840u128;
    let mut u128_22: u128 = 4327u128;
    let mut u128_23: u128 = 3714u128;
    let mut u128_24: u128 = 137u128;
    let mut u128_25: u128 = 1719u128;
    let mut u128_26: u128 = 3579u128;
    let mut u128_27: u128 = 9601u128;
    let mut u128_28: u128 = 5027u128;
    let mut u128_29: u128 = 3126u128;
    let mut u128_30: u128 = 1580u128;
    let mut u128_31: u128 = 4487u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_31);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut u128_32: u128 = crate::udiv128::u128_mulhi(u128_30, u128_29);
    let mut u128_33: u128 = crate::udiv128::u128_mulhi(u128_28, u128_27);
    let mut u128_34: u128 = crate::udiv128::u128_mulhi(u128_26, u128_25);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_24);
    let mut u128_35: u128 = crate::udiv128::u128_mulhi(u128_23, u128_22);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_21);
    let mut u128_36: u128 = crate::udiv128::u128_mulhi(u128_20, u128_19);
    let mut u128_37: u128 = crate::udiv128::u128_mulhi(u128_18, u128_17);
    let mut u128_38: u128 = crate::udiv128::u128_mulhi(u128_16, u128_15);
    let mut u128_39: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut u128_40: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut u128_41: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut u128_42: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut u128_43: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut u128_44: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_11() {
    rusty_monitor::set_test_id(11);
    let mut u128_0: u128 = 3794u128;
    let mut u128_1: u128 = 5560u128;
    let mut u128_2: u128 = 1720u128;
    let mut u128_3: u128 = 722u128;
    let mut u128_4: u128 = 8747u128;
    let mut u128_5: u128 = 7346u128;
    let mut u128_6: u128 = 7084u128;
    let mut u128_7: u128 = 7115u128;
    let mut u128_8: u128 = 7381u128;
    let mut u128_9: u128 = 9543u128;
    let mut u128_10: u128 = 3459u128;
    let mut u128_11: u128 = 4558u128;
    let mut u128_12: u128 = 5057u128;
    let mut u128_13: u128 = 8868u128;
    let mut u128_14: u128 = 3233u128;
    let mut u128_15: u128 = 7321u128;
    let mut u128_16: u128 = 667u128;
    let mut u128_17: u128 = 3378u128;
    let mut u128_18: u128 = 8506u128;
    let mut u128_19: u128 = 1349u128;
    let mut u128_20: u128 = 4389u128;
    let mut u128_21: u128 = 7245u128;
    let mut u128_22: u128 = 291u128;
    let mut u128_23: u128 = 186u128;
    let mut u128_24: u128 = 986u128;
    let mut u128_25: u128 = 491u128;
    let mut u128_26: u128 = 4001u128;
    let mut u128_27: u128 = 2031u128;
    let mut u128_28: u128 = 2057u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_28);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut u128_29: u128 = crate::udiv128::u128_mulhi(u128_27, u128_26);
    let mut u128_30: u128 = crate::udiv128::u128_mulhi(u128_25, u128_24);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_23);
    let mut u128_31: u128 = crate::udiv128::u128_mulhi(u128_22, u128_21);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut u128_32: u128 = crate::udiv128::u128_mulhi(u128_20, u128_19);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut u128_33: u128 = crate::udiv128::u128_mulhi(u128_18, u128_17);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_16);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_15);
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut u128_34: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut u128_35: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut u128_36: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut u128_37: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut u128_38: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut u128_39: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut buffer_11: crate::Buffer = crate::Buffer::new();
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_12() {
    rusty_monitor::set_test_id(12);
    let mut u128_0: u128 = 9509u128;
    let mut u128_1: u128 = 3199u128;
    let mut u128_2: u128 = 3289u128;
    let mut u128_3: u128 = 3667u128;
    let mut u128_4: u128 = 2835u128;
    let mut u128_5: u128 = 3243u128;
    let mut u128_6: u128 = 3905u128;
    let mut u128_7: u128 = 7296u128;
    let mut u128_8: u128 = 4145u128;
    let mut u128_9: u128 = 910u128;
    let mut u128_10: u128 = 9582u128;
    let mut u128_11: u128 = 8934u128;
    let mut u128_12: u128 = 9611u128;
    let mut u128_13: u128 = 6035u128;
    let mut u128_14: u128 = 9817u128;
    let mut u128_15: u128 = 5657u128;
    let mut u128_16: u128 = 9421u128;
    let mut u128_17: u128 = 5865u128;
    let mut u128_18: u128 = 1025u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_18, u128_17);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_16, u128_15);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut u128_21: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut u128_22: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut u128_23: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut u128_24: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut u128_25: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_13() {
    rusty_monitor::set_test_id(13);
    let mut u128_0: u128 = 4006u128;
    let mut u128_1: u128 = 6824u128;
    let mut u128_2: u128 = 1329u128;
    let mut u128_3: u128 = 713u128;
    let mut u128_4: u128 = 6165u128;
    let mut u128_5: u128 = 2699u128;
    let mut u128_6: u128 = 2966u128;
    let mut u128_7: u128 = 5713u128;
    let mut u128_8: u128 = 3872u128;
    let mut u128_9: u128 = 5986u128;
    let mut u128_10: u128 = 9440u128;
    let mut u128_11: u128 = 3820u128;
    let mut u128_12: u128 = 6440u128;
    let mut u128_13: u128 = 9571u128;
    let mut u128_14: u128 = 5539u128;
    let mut u128_15: u128 = 7974u128;
    let mut u128_16: u128 = 2214u128;
    let mut u128_17: u128 = 4983u128;
    let mut u128_18: u128 = 7937u128;
    let mut u128_19: u128 = 8127u128;
    let mut u128_20: u128 = 7925u128;
    let mut u128_21: u128 = 520u128;
    let mut u128_22: u128 = 2137u128;
    let mut u128_23: u128 = crate::udiv128::u128_mulhi(u128_22, u128_21);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_20);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_19);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_18);
    let mut u128_24: u128 = crate::udiv128::u128_mulhi(u128_17, u128_16);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_15);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut u128_25: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut u128_26: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut u128_27: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut u128_28: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut u128_29: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut u128_30: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut u128_31: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_14() {
    rusty_monitor::set_test_id(14);
    let mut u128_0: u128 = 1809u128;
    let mut u128_1: u128 = 5649u128;
    let mut u128_2: u128 = 2482u128;
    let mut u128_3: u128 = 3302u128;
    let mut u128_4: u128 = 99u128;
    let mut u128_5: u128 = 1017u128;
    let mut u128_6: u128 = 3110u128;
    let mut u128_7: u128 = 3323u128;
    let mut u128_8: u128 = 4344u128;
    let mut u128_9: u128 = 1214u128;
    let mut u128_10: u128 = 6219u128;
    let mut u128_11: u128 = 7978u128;
    let mut u128_12: u128 = 7352u128;
    let mut u128_13: u128 = 6838u128;
    let mut u128_14: u128 = 902u128;
    let mut u128_15: u128 = 2374u128;
    let mut u128_16: u128 = 8000u128;
    let mut u128_17: u128 = 5667u128;
    let mut u128_18: u128 = 4638u128;
    let mut u128_19: u128 = 6275u128;
    let mut u128_20: u128 = 62u128;
    let mut u128_21: u128 = 9149u128;
    let mut u128_22: u128 = 9024u128;
    let mut u128_23: u128 = 2821u128;
    let mut u128_24: u128 = 6758u128;
    let mut u128_25: u128 = 3970u128;
    let mut u128_26: u128 = 922u128;
    let mut u128_27: u128 = 5103u128;
    let mut u128_28: u128 = 3403u128;
    let mut u128_29: u128 = 1215u128;
    let mut u128_30: u128 = 7654u128;
    let mut u128_31: u128 = crate::udiv128::u128_mulhi(u128_30, u128_29);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_28);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut u128_32: u128 = crate::udiv128::u128_mulhi(u128_27, u128_26);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut u128_33: u128 = crate::udiv128::u128_mulhi(u128_25, u128_24);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut u128_34: u128 = crate::udiv128::u128_mulhi(u128_23, u128_22);
    let mut u128_35: u128 = crate::udiv128::u128_mulhi(u128_21, u128_20);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_19);
    let mut u128_36: u128 = crate::udiv128::u128_mulhi(u128_18, u128_17);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut u128_37: u128 = crate::udiv128::u128_mulhi(u128_16, u128_15);
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut u128_38: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut buffer_11: crate::Buffer = crate::Buffer::default();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut buffer_12: crate::Buffer = crate::Buffer::new();
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut buffer_13: crate::Buffer = crate::Buffer::default();
    let mut buffer_14: crate::Buffer = crate::Buffer::new();
    let mut buffer_15: crate::Buffer = crate::Buffer::default();
    let mut u128_39: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut buffer_16: crate::Buffer = crate::Buffer::new();
    let mut buffer_17: crate::Buffer = crate::Buffer::default();
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut u128_40: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut buffer_18: crate::Buffer = crate::Buffer::default();
    let mut buffer_19: crate::Buffer = crate::Buffer::new();
    let mut buffer_20: crate::Buffer = crate::Buffer::default();
    let mut buffer_21: crate::Buffer = crate::Buffer::new();
    let mut buffer_22: crate::Buffer = crate::Buffer::new();
    let mut buffer_23: crate::Buffer = crate::Buffer::default();
    let mut buffer_24: crate::Buffer = crate::Buffer::default();
    let mut buffer_25: crate::Buffer = crate::Buffer::new();
    let mut tuple_8: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_26: crate::Buffer = crate::Buffer::new();
    let mut u128_41: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_15() {
    rusty_monitor::set_test_id(15);
    let mut u128_0: u128 = 8394u128;
    let mut u128_1: u128 = 217u128;
    let mut u128_2: u128 = 1053u128;
    let mut u128_3: u128 = 2091u128;
    let mut u128_4: u128 = 4885u128;
    let mut u128_5: u128 = 9482u128;
    let mut u128_6: u128 = 2775u128;
    let mut u128_7: u128 = 3218u128;
    let mut u128_8: u128 = 7491u128;
    let mut u128_9: u128 = 9603u128;
    let mut u128_10: u128 = 668u128;
    let mut u128_11: u128 = 2861u128;
    let mut u128_12: u128 = 9966u128;
    let mut u128_13: u128 = 63u128;
    let mut u128_14: u128 = 8716u128;
    let mut u128_15: u128 = 231u128;
    let mut u128_16: u128 = 5425u128;
    let mut u128_17: u128 = 3313u128;
    let mut u128_18: u128 = 5958u128;
    let mut u128_19: u128 = 953u128;
    let mut u128_20: u128 = 8740u128;
    let mut u128_21: u128 = 8938u128;
    let mut u128_22: u128 = 3659u128;
    let mut u128_23: u128 = 3285u128;
    let mut u128_24: u128 = 2714u128;
    let mut u128_25: u128 = 592u128;
    let mut u128_26: u128 = 7502u128;
    let mut u128_27: u128 = 3287u128;
    let mut u128_28: u128 = 8066u128;
    let mut u128_29: u128 = 6855u128;
    let mut u128_30: u128 = 7055u128;
    let mut u128_31: u128 = 4205u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut u128_32: u128 = crate::udiv128::u128_mulhi(u128_31, u128_30);
    let mut u128_33: u128 = crate::udiv128::u128_mulhi(u128_29, u128_28);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_27);
    let mut u128_34: u128 = crate::udiv128::u128_mulhi(u128_26, u128_25);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_24);
    let mut u128_35: u128 = crate::udiv128::u128_mulhi(u128_23, u128_22);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_21);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_20);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_19);
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut u128_36: u128 = crate::udiv128::u128_mulhi(u128_18, u128_17);
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut u128_37: u128 = crate::udiv128::u128_mulhi(u128_16, u128_15);
    let mut u128_38: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut u128_39: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut u128_40: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut tuple_8: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut tuple_9: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut u128_41: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut tuple_10: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut tuple_11: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_16() {
    rusty_monitor::set_test_id(16);
    let mut u128_0: u128 = 8174u128;
    let mut u128_1: u128 = 5487u128;
    let mut u128_2: u128 = 1422u128;
    let mut u128_3: u128 = 1166u128;
    let mut u128_4: u128 = 4181u128;
    let mut u128_5: u128 = 2880u128;
    let mut u128_6: u128 = 1148u128;
    let mut u128_7: u128 = 4330u128;
    let mut u128_8: u128 = 6697u128;
    let mut u128_9: u128 = 1441u128;
    let mut u128_10: u128 = 9545u128;
    let mut u128_11: u128 = 5781u128;
    let mut u128_12: u128 = 2925u128;
    let mut u128_13: u128 = 9727u128;
    let mut u128_14: u128 = 3424u128;
    let mut u128_15: u128 = 1119u128;
    let mut u128_16: u128 = 4052u128;
    let mut u128_17: u128 = 6115u128;
    let mut u128_18: u128 = 4399u128;
    let mut u128_19: u128 = 2603u128;
    let mut u128_20: u128 = 8501u128;
    let mut u128_21: u128 = 6848u128;
    let mut u128_22: u128 = crate::udiv128::u128_mulhi(u128_21, u128_20);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut u128_23: u128 = crate::udiv128::u128_mulhi(u128_19, u128_18);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_17);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut u128_24: u128 = crate::udiv128::u128_mulhi(u128_16, u128_15);
    let mut u128_25: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut u128_26: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut u128_27: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut u128_28: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut buffer_10: crate::Buffer = crate::Buffer::default();
    let mut u128_29: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut buffer_11: crate::Buffer = crate::Buffer::new();
    let mut u128_30: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    let mut buffer_12: crate::Buffer = crate::Buffer::new();
    let mut buffer_13: crate::Buffer = crate::Buffer::new();
    let mut buffer_14: crate::Buffer = crate::Buffer::default();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_17() {
    rusty_monitor::set_test_id(17);
    let mut u128_0: u128 = 5344u128;
    let mut u128_1: u128 = 1499u128;
    let mut u128_2: u128 = 2482u128;
    let mut u128_3: u128 = 1866u128;
    let mut u128_4: u128 = 3459u128;
    let mut u128_5: u128 = 6354u128;
    let mut u128_6: u128 = 5951u128;
    let mut u128_7: u128 = 5669u128;
    let mut u128_8: u128 = 670u128;
    let mut u128_9: u128 = 6136u128;
    let mut u128_10: u128 = 4987u128;
    let mut u128_11: u128 = 4616u128;
    let mut u128_12: u128 = 8524u128;
    let mut u128_13: u128 = 9200u128;
    let mut u128_14: u128 = 9598u128;
    let mut u128_15: u128 = 8745u128;
    let mut u128_16: u128 = 176u128;
    let mut u128_17: u128 = 4695u128;
    let mut u128_18: u128 = 7849u128;
    let mut u128_19: u128 = 1780u128;
    let mut u128_20: u128 = 772u128;
    let mut u128_21: u128 = 1635u128;
    let mut u128_22: u128 = crate::udiv128::u128_mulhi(u128_21, u128_20);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_19);
    let mut u128_23: u128 = crate::udiv128::u128_mulhi(u128_18, u128_17);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_16);
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut u128_24: u128 = crate::udiv128::u128_mulhi(u128_15, u128_14);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut u128_25: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut buffer_11: crate::Buffer = crate::Buffer::default();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut u128_26: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut buffer_12: crate::Buffer = crate::Buffer::new();
    let mut u128_27: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut u128_28: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut buffer_13: crate::Buffer = crate::Buffer::new();
    let mut u128_29: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_18() {
    rusty_monitor::set_test_id(18);
    let mut u128_0: u128 = 239u128;
    let mut u128_1: u128 = 5636u128;
    let mut u128_2: u128 = 9193u128;
    let mut u128_3: u128 = 5995u128;
    let mut u128_4: u128 = 7859u128;
    let mut u128_5: u128 = 3737u128;
    let mut u128_6: u128 = 1620u128;
    let mut u128_7: u128 = 478u128;
    let mut u128_8: u128 = 4917u128;
    let mut u128_9: u128 = 7205u128;
    let mut u128_10: u128 = 8498u128;
    let mut u128_11: u128 = 4872u128;
    let mut u128_12: u128 = 3952u128;
    let mut u128_13: u128 = 9180u128;
    let mut u128_14: u128 = 9533u128;
    let mut u128_15: u128 = 3290u128;
    let mut u128_16: u128 = 3720u128;
    let mut u128_17: u128 = 7549u128;
    let mut u128_18: u128 = 6139u128;
    let mut u128_19: u128 = 5911u128;
    let mut u128_20: u128 = 2488u128;
    let mut u128_21: u128 = 4320u128;
    let mut u128_22: u128 = 6646u128;
    let mut u128_23: u128 = 9514u128;
    let mut u128_24: u128 = 3073u128;
    let mut u128_25: u128 = 5781u128;
    let mut u128_26: u128 = 7760u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_26);
    let mut u128_27: u128 = crate::udiv128::u128_mulhi(u128_25, u128_24);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_23);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_22);
    let mut u128_28: u128 = crate::udiv128::u128_mulhi(u128_21, u128_20);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut u128_29: u128 = crate::udiv128::u128_mulhi(u128_19, u128_18);
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_17);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_16);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_15);
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut u128_30: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut u128_31: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut u128_32: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut buffer_11: crate::Buffer = crate::Buffer::new();
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut buffer_12: crate::Buffer = crate::Buffer::new();
    let mut buffer_13: crate::Buffer = crate::Buffer::new();
    let mut tuple_8: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut buffer_14: crate::Buffer = crate::Buffer::default();
    let mut tuple_9: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut tuple_10: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_15: crate::Buffer = crate::Buffer::default();
    let mut buffer_16: crate::Buffer = crate::Buffer::new();
    let mut u128_33: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut buffer_17: crate::Buffer = crate::Buffer::new();
    let mut tuple_11: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut tuple_12: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_19() {
    rusty_monitor::set_test_id(19);
    let mut u128_0: u128 = 7822u128;
    let mut u128_1: u128 = 5982u128;
    let mut u128_2: u128 = 3754u128;
    let mut u128_3: u128 = 2477u128;
    let mut u128_4: u128 = 7941u128;
    let mut u128_5: u128 = 5456u128;
    let mut u128_6: u128 = 2948u128;
    let mut u128_7: u128 = 7080u128;
    let mut u128_8: u128 = 3179u128;
    let mut u128_9: u128 = 7570u128;
    let mut u128_10: u128 = 1683u128;
    let mut u128_11: u128 = 4157u128;
    let mut u128_12: u128 = 6726u128;
    let mut u128_13: u128 = 2643u128;
    let mut u128_14: u128 = 6475u128;
    let mut u128_15: u128 = 6979u128;
    let mut u128_16: u128 = 6385u128;
    let mut u128_17: u128 = 4171u128;
    let mut u128_18: u128 = 9021u128;
    let mut u128_19: u128 = 7041u128;
    let mut u128_20: u128 = 349u128;
    let mut u128_21: u128 = 6830u128;
    let mut u128_22: u128 = 9756u128;
    let mut u128_23: u128 = 8193u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_23);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_22);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut u128_24: u128 = crate::udiv128::u128_mulhi(u128_21, u128_20);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_19);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_18);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_17);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_16);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_15);
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut tuple_8: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut tuple_9: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut tuple_10: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut buffer_10: crate::Buffer = crate::Buffer::default();
    let mut u128_25: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut u128_26: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_11: crate::Buffer = crate::Buffer::new();
    let mut u128_27: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_12: crate::Buffer = crate::Buffer::default();
    let mut buffer_13: crate::Buffer = crate::Buffer::default();
    let mut buffer_14: crate::Buffer = crate::Buffer::default();
    let mut buffer_15: crate::Buffer = crate::Buffer::new();
    let mut tuple_11: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_16: crate::Buffer = crate::Buffer::default();
    let mut buffer_17: crate::Buffer = crate::Buffer::new();
    let mut buffer_18: crate::Buffer = crate::Buffer::new();
    let mut tuple_12: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut buffer_19: crate::Buffer = crate::Buffer::default();
    let mut buffer_20: crate::Buffer = crate::Buffer::default();
    let mut buffer_21: crate::Buffer = crate::Buffer::new();
    let mut u128_28: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut tuple_13: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_20() {
    rusty_monitor::set_test_id(20);
    let mut u128_0: u128 = 7106u128;
    let mut u128_1: u128 = 769u128;
    let mut u128_2: u128 = 1753u128;
    let mut u128_3: u128 = 2976u128;
    let mut u128_4: u128 = 870u128;
    let mut u128_5: u128 = 8942u128;
    let mut u128_6: u128 = 8420u128;
    let mut u128_7: u128 = 4934u128;
    let mut u128_8: u128 = 5882u128;
    let mut u128_9: u128 = 9522u128;
    let mut u128_10: u128 = 1054u128;
    let mut u128_11: u128 = 6850u128;
    let mut u128_12: u128 = 7453u128;
    let mut u128_13: u128 = 5104u128;
    let mut u128_14: u128 = 7486u128;
    let mut u128_15: u128 = 6238u128;
    let mut u128_16: u128 = 3939u128;
    let mut u128_17: u128 = 6786u128;
    let mut u128_18: u128 = 9999u128;
    let mut u128_19: u128 = 7671u128;
    let mut u128_20: u128 = 438u128;
    let mut u128_21: u128 = 2424u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut u128_22: u128 = crate::udiv128::u128_mulhi(u128_21, u128_20);
    let mut u128_23: u128 = crate::udiv128::u128_mulhi(u128_19, u128_18);
    let mut u128_24: u128 = crate::udiv128::u128_mulhi(u128_17, u128_16);
    let mut u128_25: u128 = crate::udiv128::u128_mulhi(u128_15, u128_14);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut u128_26: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut u128_27: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut u128_28: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut u128_29: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut u128_30: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut u128_31: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_21() {
    rusty_monitor::set_test_id(21);
    let mut u128_0: u128 = 2764u128;
    let mut u128_1: u128 = 4057u128;
    let mut u128_2: u128 = 8450u128;
    let mut u128_3: u128 = 1687u128;
    let mut u128_4: u128 = 7490u128;
    let mut u128_5: u128 = 7036u128;
    let mut u128_6: u128 = 6997u128;
    let mut u128_7: u128 = 2946u128;
    let mut u128_8: u128 = 776u128;
    let mut u128_9: u128 = 4853u128;
    let mut u128_10: u128 = 1636u128;
    let mut u128_11: u128 = 9257u128;
    let mut u128_12: u128 = 6067u128;
    let mut u128_13: u128 = 2604u128;
    let mut u128_14: u128 = 1997u128;
    let mut u128_15: u128 = 2346u128;
    let mut u128_16: u128 = 5341u128;
    let mut u128_17: u128 = 8442u128;
    let mut u128_18: u128 = 3218u128;
    let mut u128_19: u128 = 1886u128;
    let mut u128_20: u128 = 8327u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_20);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_19);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut u128_21: u128 = crate::udiv128::u128_mulhi(u128_18, u128_17);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_16);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_15);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut u128_22: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut u128_23: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut u128_24: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut u128_25: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut buffer_11: crate::Buffer = crate::Buffer::new();
    let mut buffer_12: crate::Buffer = crate::Buffer::new();
    let mut buffer_13: crate::Buffer = crate::Buffer::new();
    let mut buffer_14: crate::Buffer = crate::Buffer::new();
    let mut buffer_15: crate::Buffer = crate::Buffer::new();
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut u128_26: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut buffer_16: crate::Buffer = crate::Buffer::new();
    let mut buffer_17: crate::Buffer = crate::Buffer::default();
    let mut tuple_8: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_22() {
    rusty_monitor::set_test_id(22);
    let mut u128_0: u128 = 3633u128;
    let mut u128_1: u128 = 2567u128;
    let mut u128_2: u128 = 7516u128;
    let mut u128_3: u128 = 2943u128;
    let mut u128_4: u128 = 3450u128;
    let mut u128_5: u128 = 6531u128;
    let mut u128_6: u128 = 9356u128;
    let mut u128_7: u128 = 3609u128;
    let mut u128_8: u128 = 696u128;
    let mut u128_9: u128 = 8585u128;
    let mut u128_10: u128 = 4318u128;
    let mut u128_11: u128 = 3224u128;
    let mut u128_12: u128 = 3764u128;
    let mut u128_13: u128 = 4046u128;
    let mut u128_14: u128 = 2974u128;
    let mut u128_15: u128 = 6036u128;
    let mut u128_16: u128 = 2453u128;
    let mut u128_17: u128 = 3576u128;
    let mut u128_18: u128 = 7677u128;
    let mut u128_19: u128 = 9699u128;
    let mut u128_20: u128 = 9326u128;
    let mut u128_21: u128 = 3979u128;
    let mut u128_22: u128 = 2704u128;
    let mut u128_23: u128 = 9706u128;
    let mut u128_24: u128 = 2119u128;
    let mut u128_25: u128 = 3233u128;
    let mut u128_26: u128 = 2663u128;
    let mut u128_27: u128 = 9536u128;
    let mut u128_28: u128 = 46u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_28);
    let mut u128_29: u128 = crate::udiv128::u128_mulhi(u128_27, u128_26);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut u128_30: u128 = crate::udiv128::u128_mulhi(u128_25, u128_24);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut u128_31: u128 = crate::udiv128::u128_mulhi(u128_23, u128_22);
    let mut u128_32: u128 = crate::udiv128::u128_mulhi(u128_21, u128_20);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut u128_33: u128 = crate::udiv128::u128_mulhi(u128_19, u128_18);
    let mut u128_34: u128 = crate::udiv128::u128_mulhi(u128_17, u128_16);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut u128_35: u128 = crate::udiv128::u128_mulhi(u128_15, u128_14);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut u128_36: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut u128_37: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut buffer_11: crate::Buffer = crate::Buffer::default();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_12: crate::Buffer = crate::Buffer::new();
    let mut buffer_13: crate::Buffer = crate::Buffer::new();
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut u128_38: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut u128_39: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    let mut buffer_14: crate::Buffer = crate::Buffer::new();
    let mut buffer_15: crate::Buffer = crate::Buffer::default();
    let mut buffer_16: crate::Buffer = crate::Buffer::new();
    let mut buffer_17: crate::Buffer = crate::Buffer::default();
    let mut buffer_18: crate::Buffer = crate::Buffer::default();
    let mut buffer_19: crate::Buffer = crate::Buffer::default();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_23() {
    rusty_monitor::set_test_id(23);
    let mut u128_0: u128 = 7662u128;
    let mut u128_1: u128 = 2389u128;
    let mut u128_2: u128 = 7727u128;
    let mut u128_3: u128 = 8552u128;
    let mut u128_4: u128 = 6044u128;
    let mut u128_5: u128 = 9804u128;
    let mut u128_6: u128 = 4362u128;
    let mut u128_7: u128 = 7887u128;
    let mut u128_8: u128 = 1147u128;
    let mut u128_9: u128 = 8514u128;
    let mut u128_10: u128 = 2055u128;
    let mut u128_11: u128 = 8679u128;
    let mut u128_12: u128 = 1070u128;
    let mut u128_13: u128 = 5326u128;
    let mut u128_14: u128 = 4762u128;
    let mut u128_15: u128 = 3294u128;
    let mut u128_16: u128 = 3989u128;
    let mut u128_17: u128 = 1000u128;
    let mut u128_18: u128 = 4437u128;
    let mut u128_19: u128 = 6407u128;
    let mut u128_20: u128 = 3904u128;
    let mut u128_21: u128 = 9668u128;
    let mut u128_22: u128 = 1517u128;
    let mut u128_23: u128 = 3293u128;
    let mut u128_24: u128 = 489u128;
    let mut u128_25: u128 = 1199u128;
    let mut u128_26: u128 = 1533u128;
    let mut u128_27: u128 = 1348u128;
    let mut u128_28: u128 = 6206u128;
    let mut u128_29: u128 = 3915u128;
    let mut u128_30: u128 = 5072u128;
    let mut u128_31: u128 = 7406u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_31);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_30);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut u128_32: u128 = crate::udiv128::u128_mulhi(u128_29, u128_28);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_27);
    let mut u128_33: u128 = crate::udiv128::u128_mulhi(u128_26, u128_25);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut u128_34: u128 = crate::udiv128::u128_mulhi(u128_24, u128_23);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_22);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_21);
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut u128_35: u128 = crate::udiv128::u128_mulhi(u128_20, u128_19);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_18);
    let mut u128_36: u128 = crate::udiv128::u128_mulhi(u128_17, u128_16);
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut buffer_11: crate::Buffer = crate::Buffer::new();
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_15);
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut tuple_8: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut u128_37: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut tuple_9: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut tuple_10: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut u128_38: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_12: crate::Buffer = crate::Buffer::default();
    let mut buffer_13: crate::Buffer = crate::Buffer::default();
    let mut tuple_11: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut tuple_12: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut u128_39: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut buffer_14: crate::Buffer = crate::Buffer::default();
    let mut buffer_15: crate::Buffer = crate::Buffer::default();
    let mut buffer_16: crate::Buffer = crate::Buffer::default();
    let mut u128_40: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut buffer_17: crate::Buffer = crate::Buffer::new();
    let mut buffer_18: crate::Buffer = crate::Buffer::default();
    let mut buffer_19: crate::Buffer = crate::Buffer::default();
    let mut tuple_13: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    let mut buffer_20: crate::Buffer = crate::Buffer::default();
    let mut buffer_21: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_24() {
    rusty_monitor::set_test_id(24);
    let mut u128_0: u128 = 4202u128;
    let mut u128_1: u128 = 3430u128;
    let mut u128_2: u128 = 1158u128;
    let mut u128_3: u128 = 9865u128;
    let mut u128_4: u128 = 8463u128;
    let mut u128_5: u128 = 8543u128;
    let mut u128_6: u128 = 3416u128;
    let mut u128_7: u128 = 9113u128;
    let mut u128_8: u128 = 4026u128;
    let mut u128_9: u128 = 5347u128;
    let mut u128_10: u128 = 5603u128;
    let mut u128_11: u128 = 7734u128;
    let mut u128_12: u128 = 4205u128;
    let mut u128_13: u128 = 6254u128;
    let mut u128_14: u128 = 7319u128;
    let mut u128_15: u128 = 9656u128;
    let mut u128_16: u128 = 1797u128;
    let mut u128_17: u128 = 5892u128;
    let mut u128_18: u128 = 9020u128;
    let mut u128_19: u128 = 857u128;
    let mut u128_20: u128 = 5558u128;
    let mut u128_21: u128 = 9904u128;
    let mut u128_22: u128 = 4940u128;
    let mut u128_23: u128 = crate::udiv128::u128_mulhi(u128_22, u128_21);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_20);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_19);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut u128_24: u128 = crate::udiv128::u128_mulhi(u128_18, u128_17);
    let mut u128_25: u128 = crate::udiv128::u128_mulhi(u128_16, u128_15);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut buffer_10: crate::Buffer = crate::Buffer::default();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut buffer_11: crate::Buffer = crate::Buffer::new();
    let mut buffer_12: crate::Buffer = crate::Buffer::new();
    let mut buffer_13: crate::Buffer = crate::Buffer::new();
    let mut buffer_14: crate::Buffer = crate::Buffer::default();
    let mut buffer_15: crate::Buffer = crate::Buffer::default();
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut buffer_16: crate::Buffer = crate::Buffer::default();
    let mut buffer_17: crate::Buffer = crate::Buffer::new();
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut tuple_8: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut buffer_18: crate::Buffer = crate::Buffer::default();
    let mut buffer_19: crate::Buffer = crate::Buffer::default();
    let mut buffer_20: crate::Buffer = crate::Buffer::default();
    let mut buffer_21: crate::Buffer = crate::Buffer::default();
    let mut buffer_22: crate::Buffer = crate::Buffer::default();
    let mut tuple_9: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut buffer_23: crate::Buffer = crate::Buffer::default();
    let mut u128_26: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut u128_27: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut buffer_24: crate::Buffer = crate::Buffer::new();
    let mut tuple_10: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_25: crate::Buffer = crate::Buffer::default();
    let mut buffer_26: crate::Buffer = crate::Buffer::new();
    let mut tuple_11: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut buffer_27: crate::Buffer = crate::Buffer::new();
    let mut buffer_28: crate::Buffer = crate::Buffer::new();
    let mut buffer_29: crate::Buffer = crate::Buffer::new();
    let mut tuple_12: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_25() {
    rusty_monitor::set_test_id(25);
    let mut u128_0: u128 = 5541u128;
    let mut u128_1: u128 = 2580u128;
    let mut u128_2: u128 = 61u128;
    let mut u128_3: u128 = 7995u128;
    let mut u128_4: u128 = 2951u128;
    let mut u128_5: u128 = 6349u128;
    let mut u128_6: u128 = 3364u128;
    let mut u128_7: u128 = 8168u128;
    let mut u128_8: u128 = 4672u128;
    let mut u128_9: u128 = 445u128;
    let mut u128_10: u128 = 5602u128;
    let mut u128_11: u128 = 2711u128;
    let mut u128_12: u128 = 1490u128;
    let mut u128_13: u128 = 5666u128;
    let mut u128_14: u128 = 6961u128;
    let mut u128_15: u128 = 1743u128;
    let mut u128_16: u128 = 25u128;
    let mut u128_17: u128 = 4215u128;
    let mut u128_18: u128 = 6818u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_18);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_17);
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_16, u128_15);
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut buffer_11: crate::Buffer = crate::Buffer::default();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut buffer_12: crate::Buffer = crate::Buffer::default();
    let mut u128_21: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_13: crate::Buffer = crate::Buffer::default();
    let mut u128_22: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut u128_23: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut buffer_14: crate::Buffer = crate::Buffer::new();
    let mut buffer_15: crate::Buffer = crate::Buffer::new();
    let mut buffer_16: crate::Buffer = crate::Buffer::default();
    let mut buffer_17: crate::Buffer = crate::Buffer::new();
    let mut buffer_18: crate::Buffer = crate::Buffer::default();
    let mut buffer_19: crate::Buffer = crate::Buffer::default();
    let mut u128_24: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    let mut buffer_20: crate::Buffer = crate::Buffer::new();
    let mut buffer_21: crate::Buffer = crate::Buffer::default();
    let mut buffer_22: crate::Buffer = crate::Buffer::default();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_26() {
    rusty_monitor::set_test_id(26);
    let mut u128_0: u128 = 3549u128;
    let mut u128_1: u128 = 3660u128;
    let mut u128_2: u128 = 8695u128;
    let mut u128_3: u128 = 9593u128;
    let mut u128_4: u128 = 6361u128;
    let mut u128_5: u128 = 2641u128;
    let mut u128_6: u128 = 8502u128;
    let mut u128_7: u128 = 9566u128;
    let mut u128_8: u128 = 8070u128;
    let mut u128_9: u128 = 4454u128;
    let mut u128_10: u128 = 6057u128;
    let mut u128_11: u128 = 3102u128;
    let mut u128_12: u128 = 3478u128;
    let mut u128_13: u128 = 1563u128;
    let mut u128_14: u128 = 377u128;
    let mut u128_15: u128 = 9079u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_15, u128_14);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_11: crate::Buffer = crate::Buffer::new();
    let mut buffer_12: crate::Buffer = crate::Buffer::new();
    let mut buffer_13: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut buffer_14: crate::Buffer = crate::Buffer::new();
    let mut buffer_15: crate::Buffer = crate::Buffer::default();
    let mut buffer_16: crate::Buffer = crate::Buffer::new();
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_27() {
    rusty_monitor::set_test_id(27);
    let mut u128_0: u128 = 2814u128;
    let mut u128_1: u128 = 6632u128;
    let mut u128_2: u128 = 6704u128;
    let mut u128_3: u128 = 3093u128;
    let mut u128_4: u128 = 6346u128;
    let mut u128_5: u128 = 5355u128;
    let mut u128_6: u128 = 6605u128;
    let mut u128_7: u128 = 4304u128;
    let mut u128_8: u128 = 4214u128;
    let mut u128_9: u128 = 1923u128;
    let mut u128_10: u128 = 2357u128;
    let mut u128_11: u128 = 9205u128;
    let mut u128_12: u128 = 4479u128;
    let mut u128_13: u128 = 8079u128;
    let mut u128_14: u128 = 5587u128;
    let mut u128_15: u128 = 2550u128;
    let mut u128_16: u128 = 531u128;
    let mut u128_17: u128 = 42u128;
    let mut u128_18: u128 = 7776u128;
    let mut u128_19: u128 = 4586u128;
    let mut u128_20: u128 = 9427u128;
    let mut u128_21: u128 = 9369u128;
    let mut u128_22: u128 = 3736u128;
    let mut u128_23: u128 = 9253u128;
    let mut u128_24: u128 = 8405u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_24);
    let mut u128_25: u128 = crate::udiv128::u128_mulhi(u128_23, u128_22);
    let mut u128_26: u128 = crate::udiv128::u128_mulhi(u128_21, u128_20);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut u128_27: u128 = crate::udiv128::u128_mulhi(u128_19, u128_18);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut u128_28: u128 = crate::udiv128::u128_mulhi(u128_17, u128_16);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_15);
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut buffer_10: crate::Buffer = crate::Buffer::default();
    let mut buffer_11: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut buffer_12: crate::Buffer = crate::Buffer::new();
    let mut u128_29: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut buffer_13: crate::Buffer = crate::Buffer::new();
    let mut u128_30: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut buffer_14: crate::Buffer = crate::Buffer::new();
    let mut buffer_15: crate::Buffer = crate::Buffer::new();
    let mut buffer_16: crate::Buffer = crate::Buffer::default();
    let mut buffer_17: crate::Buffer = crate::Buffer::new();
    let mut buffer_18: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut u128_31: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_19: crate::Buffer = crate::Buffer::default();
    let mut buffer_20: crate::Buffer = crate::Buffer::default();
    let mut buffer_21: crate::Buffer = crate::Buffer::new();
    let mut buffer_22: crate::Buffer = crate::Buffer::default();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut u128_32: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut u128_33: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_28() {
    rusty_monitor::set_test_id(28);
    let mut u128_0: u128 = 1913u128;
    let mut u128_1: u128 = 8311u128;
    let mut u128_2: u128 = 2333u128;
    let mut u128_3: u128 = 8925u128;
    let mut u128_4: u128 = 5690u128;
    let mut u128_5: u128 = 5935u128;
    let mut u128_6: u128 = 6727u128;
    let mut u128_7: u128 = 3654u128;
    let mut u128_8: u128 = 3270u128;
    let mut u128_9: u128 = 7829u128;
    let mut u128_10: u128 = 8657u128;
    let mut u128_11: u128 = 769u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut buffer_11: crate::Buffer = crate::Buffer::default();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_12: crate::Buffer = crate::Buffer::default();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    let mut buffer_13: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_29() {
    rusty_monitor::set_test_id(29);
    let mut u128_0: u128 = 75u128;
    let mut u128_1: u128 = 5581u128;
    let mut u128_2: u128 = 6676u128;
    let mut u128_3: u128 = 969u128;
    let mut u128_4: u128 = 274u128;
    let mut u128_5: u128 = 5064u128;
    let mut u128_6: u128 = 213u128;
    let mut u128_7: u128 = 9088u128;
    let mut u128_8: u128 = 3572u128;
    let mut u128_9: u128 = 404u128;
    let mut u128_10: u128 = 6089u128;
    let mut u128_11: u128 = 9766u128;
    let mut u128_12: u128 = 5293u128;
    let mut u128_13: u128 = 3660u128;
    let mut u128_14: u128 = 6303u128;
    let mut u128_15: u128 = 5181u128;
    let mut u128_16: u128 = 9838u128;
    let mut u128_17: u128 = 7781u128;
    let mut u128_18: u128 = 43u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_18);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_17);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_16);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_15);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut buffer_10: crate::Buffer = crate::Buffer::default();
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_11: crate::Buffer = crate::Buffer::default();
    let mut u128_21: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut tuple_8: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_12: crate::Buffer = crate::Buffer::new();
    let mut buffer_13: crate::Buffer = crate::Buffer::new();
    let mut tuple_9: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut u128_22: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut tuple_10: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    let mut buffer_14: crate::Buffer = crate::Buffer::default();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_30() {
    rusty_monitor::set_test_id(30);
    let mut u128_0: u128 = 269u128;
    let mut u128_1: u128 = 4813u128;
    let mut u128_2: u128 = 5743u128;
    let mut u128_3: u128 = 2162u128;
    let mut u128_4: u128 = 5394u128;
    let mut u128_5: u128 = 2993u128;
    let mut u128_6: u128 = 9592u128;
    let mut u128_7: u128 = 7952u128;
    let mut u128_8: u128 = 538u128;
    let mut u128_9: u128 = 7122u128;
    let mut u128_10: u128 = 4650u128;
    let mut u128_11: u128 = 3464u128;
    let mut u128_12: u128 = 1982u128;
    let mut u128_13: u128 = 3836u128;
    let mut u128_14: u128 = 2791u128;
    let mut u128_15: u128 = 2514u128;
    let mut u128_16: u128 = 5128u128;
    let mut u128_17: u128 = 5656u128;
    let mut u128_18: u128 = 6831u128;
    let mut u128_19: u128 = 8599u128;
    let mut u128_20: u128 = 6952u128;
    let mut u128_21: u128 = 5568u128;
    let mut u128_22: u128 = 2710u128;
    let mut u128_23: u128 = 956u128;
    let mut u128_24: u128 = crate::udiv128::u128_mulhi(u128_23, u128_22);
    let mut u128_25: u128 = crate::udiv128::u128_mulhi(u128_21, u128_20);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_19);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_18);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_17);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut u128_26: u128 = crate::udiv128::u128_mulhi(u128_16, u128_15);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut u128_27: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut u128_28: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut u128_29: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut tuple_8: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut buffer_10: crate::Buffer = crate::Buffer::default();
    let mut tuple_9: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_11: crate::Buffer = crate::Buffer::new();
    let mut u128_30: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_31() {
    rusty_monitor::set_test_id(31);
    let mut u128_0: u128 = 1046u128;
    let mut u128_1: u128 = 1006u128;
    let mut u128_2: u128 = 6577u128;
    let mut u128_3: u128 = 80u128;
    let mut u128_4: u128 = 9428u128;
    let mut u128_5: u128 = 764u128;
    let mut u128_6: u128 = 7585u128;
    let mut u128_7: u128 = 231u128;
    let mut u128_8: u128 = 4983u128;
    let mut u128_9: u128 = 4761u128;
    let mut u128_10: u128 = 7151u128;
    let mut u128_11: u128 = 5030u128;
    let mut u128_12: u128 = 3731u128;
    let mut u128_13: u128 = 1300u128;
    let mut u128_14: u128 = 9076u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut buffer_10: crate::Buffer = crate::Buffer::default();
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut buffer_11: crate::Buffer = crate::Buffer::default();
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut buffer_12: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_32() {
    rusty_monitor::set_test_id(32);
    let mut u128_0: u128 = 2914u128;
    let mut u128_1: u128 = 9047u128;
    let mut u128_2: u128 = 1537u128;
    let mut u128_3: u128 = 8646u128;
    let mut u128_4: u128 = 5669u128;
    let mut u128_5: u128 = 3199u128;
    let mut u128_6: u128 = 3352u128;
    let mut u128_7: u128 = 2737u128;
    let mut u128_8: u128 = 9592u128;
    let mut u128_9: u128 = 1472u128;
    let mut u128_10: u128 = 934u128;
    let mut u128_11: u128 = 2320u128;
    let mut u128_12: u128 = 2699u128;
    let mut u128_13: u128 = 2146u128;
    let mut u128_14: u128 = 515u128;
    let mut u128_15: u128 = 2269u128;
    let mut u128_16: u128 = 2731u128;
    let mut u128_17: u128 = 4237u128;
    let mut u128_18: u128 = 4919u128;
    let mut u128_19: u128 = 976u128;
    let mut u128_20: u128 = 3708u128;
    let mut u128_21: u128 = 7039u128;
    let mut u128_22: u128 = 1873u128;
    let mut u128_23: u128 = 7861u128;
    let mut u128_24: u128 = 7429u128;
    let mut u128_25: u128 = 2115u128;
    let mut u128_26: u128 = 9595u128;
    let mut u128_27: u128 = 1069u128;
    let mut u128_28: u128 = 1278u128;
    let mut u128_29: u128 = 640u128;
    let mut u128_30: u128 = 8872u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_30);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_29);
    let mut u128_31: u128 = crate::udiv128::u128_mulhi(u128_28, u128_27);
    let mut u128_32: u128 = crate::udiv128::u128_mulhi(u128_26, u128_25);
    let mut u128_33: u128 = crate::udiv128::u128_mulhi(u128_24, u128_23);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_22);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_21);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut u128_34: u128 = crate::udiv128::u128_mulhi(u128_20, u128_19);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut u128_35: u128 = crate::udiv128::u128_mulhi(u128_18, u128_17);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_16);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_15);
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut u128_36: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut u128_37: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_10: crate::Buffer = crate::Buffer::default();
    let mut u128_38: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_11: crate::Buffer = crate::Buffer::new();
    let mut u128_39: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut tuple_8: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut tuple_9: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut tuple_10: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut u128_40: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_33() {
    rusty_monitor::set_test_id(33);
    let mut u128_0: u128 = 253u128;
    let mut u128_1: u128 = 3083u128;
    let mut u128_2: u128 = 4485u128;
    let mut u128_3: u128 = 2724u128;
    let mut u128_4: u128 = 9206u128;
    let mut u128_5: u128 = 7933u128;
    let mut u128_6: u128 = 7092u128;
    let mut u128_7: u128 = 6973u128;
    let mut u128_8: u128 = 1978u128;
    let mut u128_9: u128 = 2577u128;
    let mut u128_10: u128 = 9643u128;
    let mut u128_11: u128 = 1827u128;
    let mut u128_12: u128 = 628u128;
    let mut u128_13: u128 = 5403u128;
    let mut u128_14: u128 = 8412u128;
    let mut u128_15: u128 = 878u128;
    let mut u128_16: u128 = 772u128;
    let mut u128_17: u128 = 1869u128;
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_17, u128_16);
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_15, u128_14);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut u128_21: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut u128_22: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut u128_23: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut u128_24: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut buffer_11: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut buffer_12: crate::Buffer = crate::Buffer::new();
    let mut buffer_13: crate::Buffer = crate::Buffer::default();
    let mut buffer_14: crate::Buffer = crate::Buffer::new();
    let mut buffer_15: crate::Buffer = crate::Buffer::default();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    let mut buffer_16: crate::Buffer = crate::Buffer::default();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_34() {
    rusty_monitor::set_test_id(34);
    let mut u128_0: u128 = 6883u128;
    let mut u128_1: u128 = 3210u128;
    let mut u128_2: u128 = 7318u128;
    let mut u128_3: u128 = 7698u128;
    let mut u128_4: u128 = 1643u128;
    let mut u128_5: u128 = 3450u128;
    let mut u128_6: u128 = 1969u128;
    let mut u128_7: u128 = 7304u128;
    let mut u128_8: u128 = 5060u128;
    let mut u128_9: u128 = 4264u128;
    let mut u128_10: u128 = 208u128;
    let mut u128_11: u128 = 1599u128;
    let mut u128_12: u128 = 6801u128;
    let mut u128_13: u128 = 7720u128;
    let mut u128_14: u128 = 7235u128;
    let mut u128_15: u128 = 8542u128;
    let mut u128_16: u128 = 8482u128;
    let mut u128_17: u128 = 3275u128;
    let mut u128_18: u128 = 9131u128;
    let mut u128_19: u128 = 9952u128;
    let mut u128_20: u128 = 3479u128;
    let mut u128_21: u128 = 240u128;
    let mut u128_22: u128 = 4576u128;
    let mut u128_23: u128 = 8935u128;
    let mut u128_24: u128 = 4785u128;
    let mut u128_25: u128 = 4828u128;
    let mut u128_26: u128 = 5563u128;
    let mut u128_27: u128 = 2121u128;
    let mut u128_28: u128 = 3223u128;
    let mut u128_29: u128 = crate::udiv128::u128_mulhi(u128_28, u128_27);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut u128_30: u128 = crate::udiv128::u128_mulhi(u128_26, u128_25);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut u128_31: u128 = crate::udiv128::u128_mulhi(u128_24, u128_23);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_22);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut u128_32: u128 = crate::udiv128::u128_mulhi(u128_21, u128_20);
    let mut u128_33: u128 = crate::udiv128::u128_mulhi(u128_19, u128_18);
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_17);
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut buffer_11: crate::Buffer = crate::Buffer::default();
    let mut buffer_12: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_16);
    let mut u128_34: u128 = crate::udiv128::u128_mulhi(u128_15, u128_14);
    let mut buffer_13: crate::Buffer = crate::Buffer::new();
    let mut buffer_14: crate::Buffer = crate::Buffer::default();
    let mut buffer_15: crate::Buffer = crate::Buffer::default();
    let mut buffer_16: crate::Buffer = crate::Buffer::default();
    let mut u128_35: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut buffer_17: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut u128_36: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_18: crate::Buffer = crate::Buffer::new();
    let mut u128_37: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut u128_38: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_19: crate::Buffer = crate::Buffer::new();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut u128_39: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_35() {
    rusty_monitor::set_test_id(35);
    let mut u128_0: u128 = 9981u128;
    let mut u128_1: u128 = 9605u128;
    let mut u128_2: u128 = 284u128;
    let mut u128_3: u128 = 6208u128;
    let mut u128_4: u128 = 2742u128;
    let mut u128_5: u128 = 8826u128;
    let mut u128_6: u128 = 5489u128;
    let mut u128_7: u128 = 7656u128;
    let mut u128_8: u128 = 8236u128;
    let mut u128_9: u128 = 5585u128;
    let mut u128_10: u128 = 21u128;
    let mut u128_11: u128 = 5792u128;
    let mut u128_12: u128 = 186u128;
    let mut u128_13: u128 = 1192u128;
    let mut u128_14: u128 = 3722u128;
    let mut u128_15: u128 = 4296u128;
    let mut u128_16: u128 = 9492u128;
    let mut u128_17: u128 = 3043u128;
    let mut u128_18: u128 = 6978u128;
    let mut u128_19: u128 = 8911u128;
    let mut u128_20: u128 = 7949u128;
    let mut u128_21: u128 = 7812u128;
    let mut u128_22: u128 = 8708u128;
    let mut u128_23: u128 = 5023u128;
    let mut u128_24: u128 = 1074u128;
    let mut u128_25: u128 = 4623u128;
    let mut u128_26: u128 = 9861u128;
    let mut u128_27: u128 = 3980u128;
    let mut u128_28: u128 = 5883u128;
    let mut u128_29: u128 = 6980u128;
    let mut u128_30: u128 = 6095u128;
    let mut u128_31: u128 = 5005u128;
    let mut u128_32: u128 = 1310u128;
    let mut u128_33: u128 = 6731u128;
    let mut u128_34: u128 = 4495u128;
    let mut u128_35: u128 = 8420u128;
    let mut u128_36: u128 = 2155u128;
    let mut u128_37: u128 = 5953u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_37);
    let mut u128_38: u128 = crate::udiv128::u128_mulhi(u128_36, u128_35);
    let mut u128_39: u128 = crate::udiv128::u128_mulhi(u128_34, u128_33);
    let mut u128_40: u128 = crate::udiv128::u128_mulhi(u128_32, u128_31);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut u128_41: u128 = crate::udiv128::u128_mulhi(u128_30, u128_29);
    let mut u128_42: u128 = crate::udiv128::u128_mulhi(u128_28, u128_27);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_26);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_25);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_24);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_23);
    let mut u128_43: u128 = crate::udiv128::u128_mulhi(u128_22, u128_21);
    let mut u128_44: u128 = crate::udiv128::u128_mulhi(u128_20, u128_19);
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_18);
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_17);
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_16);
    let mut u128_45: u128 = crate::udiv128::u128_mulhi(u128_15, u128_14);
    let mut tuple_8: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut u128_46: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut tuple_9: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut u128_47: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut u128_48: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut u128_49: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut u128_50: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut buffer_11: crate::Buffer = crate::Buffer::new();
    let mut buffer_12: crate::Buffer = crate::Buffer::new();
    let mut buffer_13: crate::Buffer = crate::Buffer::new();
    let mut buffer_14: crate::Buffer = crate::Buffer::new();
    let mut buffer_15: crate::Buffer = crate::Buffer::new();
    let mut buffer_16: crate::Buffer = crate::Buffer::new();
    let mut u128_51: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_36() {
    rusty_monitor::set_test_id(36);
    let mut u128_0: u128 = 6458u128;
    let mut u128_1: u128 = 4961u128;
    let mut u128_2: u128 = 2294u128;
    let mut u128_3: u128 = 2613u128;
    let mut u128_4: u128 = 9841u128;
    let mut u128_5: u128 = 1523u128;
    let mut u128_6: u128 = 3006u128;
    let mut u128_7: u128 = 1438u128;
    let mut u128_8: u128 = 4346u128;
    let mut u128_9: u128 = 10u128;
    let mut u128_10: u128 = 8713u128;
    let mut u128_11: u128 = 6877u128;
    let mut u128_12: u128 = 8646u128;
    let mut u128_13: u128 = 4962u128;
    let mut u128_14: u128 = 7426u128;
    let mut u128_15: u128 = 2809u128;
    let mut u128_16: u128 = 8395u128;
    let mut u128_17: u128 = 6480u128;
    let mut u128_18: u128 = 7878u128;
    let mut u128_19: u128 = 9880u128;
    let mut u128_20: u128 = 3320u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_20);
    let mut u128_21: u128 = crate::udiv128::u128_mulhi(u128_19, u128_18);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut u128_22: u128 = crate::udiv128::u128_mulhi(u128_17, u128_16);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_15);
    let mut u128_23: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut u128_24: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut u128_25: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut tuple_8: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut u128_26: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_37() {
    rusty_monitor::set_test_id(37);
    let mut u128_0: u128 = 9109u128;
    let mut u128_1: u128 = 1601u128;
    let mut u128_2: u128 = 4168u128;
    let mut u128_3: u128 = 4324u128;
    let mut u128_4: u128 = 347u128;
    let mut u128_5: u128 = 1878u128;
    let mut u128_6: u128 = 810u128;
    let mut u128_7: u128 = 6132u128;
    let mut u128_8: u128 = 3033u128;
    let mut u128_9: u128 = 3309u128;
    let mut u128_10: u128 = 8638u128;
    let mut u128_11: u128 = 8829u128;
    let mut u128_12: u128 = 9007u128;
    let mut u128_13: u128 = 1149u128;
    let mut u128_14: u128 = 9194u128;
    let mut u128_15: u128 = 5196u128;
    let mut u128_16: u128 = 9715u128;
    let mut u128_17: u128 = 8804u128;
    let mut u128_18: u128 = 7773u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_18, u128_17);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_16, u128_15);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut u128_21: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut u128_22: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut u128_23: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut buffer_11: crate::Buffer = crate::Buffer::default();
    let mut buffer_12: crate::Buffer = crate::Buffer::default();
    let mut buffer_13: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_14: crate::Buffer = crate::Buffer::new();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_15: crate::Buffer = crate::Buffer::default();
    let mut u128_24: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut buffer_16: crate::Buffer = crate::Buffer::new();
    let mut buffer_17: crate::Buffer = crate::Buffer::default();
    let mut buffer_18: crate::Buffer = crate::Buffer::default();
    let mut u128_25: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_38() {
    rusty_monitor::set_test_id(38);
    let mut u128_0: u128 = 4592u128;
    let mut u128_1: u128 = 4287u128;
    let mut u128_2: u128 = 618u128;
    let mut u128_3: u128 = 5277u128;
    let mut u128_4: u128 = 3495u128;
    let mut u128_5: u128 = 6181u128;
    let mut u128_6: u128 = 3376u128;
    let mut u128_7: u128 = 5756u128;
    let mut u128_8: u128 = 4281u128;
    let mut u128_9: u128 = 2029u128;
    let mut u128_10: u128 = 6988u128;
    let mut u128_11: u128 = 130u128;
    let mut u128_12: u128 = 7210u128;
    let mut u128_13: u128 = 4034u128;
    let mut u128_14: u128 = 4483u128;
    let mut u128_15: u128 = 7278u128;
    let mut u128_16: u128 = 198u128;
    let mut u128_17: u128 = 4210u128;
    let mut u128_18: u128 = 9616u128;
    let mut u128_19: u128 = 4401u128;
    let mut u128_20: u128 = 1362u128;
    let mut u128_21: u128 = 5596u128;
    let mut u128_22: u128 = 3477u128;
    let mut u128_23: u128 = 3827u128;
    let mut u128_24: u128 = 5287u128;
    let mut u128_25: u128 = 6246u128;
    let mut u128_26: u128 = 5016u128;
    let mut u128_27: u128 = 4396u128;
    let mut u128_28: u128 = 5739u128;
    let mut u128_29: u128 = 5142u128;
    let mut u128_30: u128 = crate::udiv128::u128_mulhi(u128_29, u128_28);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut u128_31: u128 = crate::udiv128::u128_mulhi(u128_27, u128_26);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_25);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_24);
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_23);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_22);
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_21);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_20);
    let mut buffer_10: crate::Buffer = crate::Buffer::default();
    let mut u128_32: u128 = crate::udiv128::u128_mulhi(u128_19, u128_18);
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_17);
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_16);
    let mut u128_33: u128 = crate::udiv128::u128_mulhi(u128_15, u128_14);
    let mut buffer_11: crate::Buffer = crate::Buffer::new();
    let mut buffer_12: crate::Buffer = crate::Buffer::default();
    let mut buffer_13: crate::Buffer = crate::Buffer::default();
    let mut tuple_8: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut buffer_14: crate::Buffer = crate::Buffer::default();
    let mut buffer_15: crate::Buffer = crate::Buffer::new();
    let mut tuple_9: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut buffer_16: crate::Buffer = crate::Buffer::default();
    let mut tuple_10: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut buffer_17: crate::Buffer = crate::Buffer::default();
    let mut buffer_18: crate::Buffer = crate::Buffer::default();
    let mut buffer_19: crate::Buffer = crate::Buffer::default();
    let mut u128_34: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_20: crate::Buffer = crate::Buffer::default();
    let mut buffer_21: crate::Buffer = crate::Buffer::default();
    let mut buffer_22: crate::Buffer = crate::Buffer::new();
    let mut tuple_11: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut tuple_12: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut u128_35: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut tuple_13: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_23: crate::Buffer = crate::Buffer::default();
    let mut u128_36: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut u128_37: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_39() {
    rusty_monitor::set_test_id(39);
    let mut u128_0: u128 = 4502u128;
    let mut u128_1: u128 = 2417u128;
    let mut u128_2: u128 = 7344u128;
    let mut u128_3: u128 = 597u128;
    let mut u128_4: u128 = 1333u128;
    let mut u128_5: u128 = 1831u128;
    let mut u128_6: u128 = 4050u128;
    let mut u128_7: u128 = 7158u128;
    let mut u128_8: u128 = 3346u128;
    let mut u128_9: u128 = 7730u128;
    let mut u128_10: u128 = 7769u128;
    let mut u128_11: u128 = 3500u128;
    let mut u128_12: u128 = 8485u128;
    let mut u128_13: u128 = 9690u128;
    let mut u128_14: u128 = 1447u128;
    let mut u128_15: u128 = 1259u128;
    let mut u128_16: u128 = 4086u128;
    let mut u128_17: u128 = 9701u128;
    let mut u128_18: u128 = 6757u128;
    let mut u128_19: u128 = 2426u128;
    let mut u128_20: u128 = 3764u128;
    let mut u128_21: u128 = 885u128;
    let mut u128_22: u128 = 5116u128;
    let mut u128_23: u128 = 2554u128;
    let mut u128_24: u128 = 7089u128;
    let mut u128_25: u128 = 1610u128;
    let mut u128_26: u128 = 3225u128;
    let mut u128_27: u128 = 4375u128;
    let mut u128_28: u128 = 3404u128;
    let mut u128_29: u128 = 6459u128;
    let mut u128_30: u128 = 3862u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_30);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_29);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_28);
    let mut u128_31: u128 = crate::udiv128::u128_mulhi(u128_27, u128_26);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_25);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut u128_32: u128 = crate::udiv128::u128_mulhi(u128_24, u128_23);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut u128_33: u128 = crate::udiv128::u128_mulhi(u128_22, u128_21);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_20);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_19);
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut u128_34: u128 = crate::udiv128::u128_mulhi(u128_18, u128_17);
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_16);
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_15);
    let mut u128_35: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut u128_36: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut buffer_11: crate::Buffer = crate::Buffer::default();
    let mut buffer_12: crate::Buffer = crate::Buffer::default();
    let mut tuple_8: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut u128_37: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut tuple_9: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut buffer_13: crate::Buffer = crate::Buffer::new();
    let mut u128_38: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut u128_39: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut buffer_14: crate::Buffer = crate::Buffer::new();
    let mut u128_40: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut tuple_10: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_40() {
    rusty_monitor::set_test_id(40);
    let mut u128_0: u128 = 438u128;
    let mut u128_1: u128 = 2749u128;
    let mut u128_2: u128 = 5374u128;
    let mut u128_3: u128 = 9973u128;
    let mut u128_4: u128 = 8465u128;
    let mut u128_5: u128 = 7640u128;
    let mut u128_6: u128 = 7135u128;
    let mut u128_7: u128 = 6634u128;
    let mut u128_8: u128 = 4097u128;
    let mut u128_9: u128 = 5820u128;
    let mut u128_10: u128 = 1487u128;
    let mut u128_11: u128 = 490u128;
    let mut u128_12: u128 = 9802u128;
    let mut u128_13: u128 = 1386u128;
    let mut u128_14: u128 = 6262u128;
    let mut u128_15: u128 = 6882u128;
    let mut u128_16: u128 = 1115u128;
    let mut u128_17: u128 = 5767u128;
    let mut u128_18: u128 = 8297u128;
    let mut u128_19: u128 = 6097u128;
    let mut u128_20: u128 = 3336u128;
    let mut u128_21: u128 = 4727u128;
    let mut u128_22: u128 = 6502u128;
    let mut u128_23: u128 = 7288u128;
    let mut u128_24: u128 = 954u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_24);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_23);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut u128_25: u128 = crate::udiv128::u128_mulhi(u128_22, u128_21);
    let mut u128_26: u128 = crate::udiv128::u128_mulhi(u128_20, u128_19);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_18);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_17);
    let mut u128_27: u128 = crate::udiv128::u128_mulhi(u128_16, u128_15);
    let mut u128_28: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut u128_29: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut u128_30: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut u128_31: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut buffer_10: crate::Buffer = crate::Buffer::default();
    let mut buffer_11: crate::Buffer = crate::Buffer::default();
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_12: crate::Buffer = crate::Buffer::default();
    let mut u128_32: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut buffer_13: crate::Buffer = crate::Buffer::default();
    let mut buffer_14: crate::Buffer = crate::Buffer::new();
    let mut tuple_8: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    let mut buffer_15: crate::Buffer = crate::Buffer::default();
    let mut buffer_16: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_41() {
    rusty_monitor::set_test_id(41);
    let mut u128_0: u128 = 6060u128;
    let mut u128_1: u128 = 6557u128;
    let mut u128_2: u128 = 3168u128;
    let mut u128_3: u128 = 7061u128;
    let mut u128_4: u128 = 7598u128;
    let mut u128_5: u128 = 2429u128;
    let mut u128_6: u128 = 7341u128;
    let mut u128_7: u128 = 2703u128;
    let mut u128_8: u128 = 1622u128;
    let mut u128_9: u128 = 3602u128;
    let mut u128_10: u128 = 11u128;
    let mut u128_11: u128 = 5525u128;
    let mut u128_12: u128 = 3223u128;
    let mut u128_13: u128 = 8660u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut buffer_11: crate::Buffer = crate::Buffer::new();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    let mut buffer_12: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_42() {
    rusty_monitor::set_test_id(42);
    let mut u128_0: u128 = 6390u128;
    let mut u128_1: u128 = 5432u128;
    let mut u128_2: u128 = 2837u128;
    let mut u128_3: u128 = 5188u128;
    let mut u128_4: u128 = 1593u128;
    let mut u128_5: u128 = 5666u128;
    let mut u128_6: u128 = 6535u128;
    let mut u128_7: u128 = 8501u128;
    let mut u128_8: u128 = 1707u128;
    let mut u128_9: u128 = 836u128;
    let mut u128_10: u128 = 6742u128;
    let mut u128_11: u128 = 9537u128;
    let mut u128_12: u128 = 8342u128;
    let mut u128_13: u128 = 488u128;
    let mut u128_14: u128 = 2941u128;
    let mut u128_15: u128 = 1886u128;
    let mut u128_16: u128 = 5795u128;
    let mut u128_17: u128 = 9643u128;
    let mut u128_18: u128 = 4927u128;
    let mut u128_19: u128 = 7234u128;
    let mut u128_20: u128 = 3912u128;
    let mut u128_21: u128 = 8665u128;
    let mut u128_22: u128 = 4267u128;
    let mut u128_23: u128 = 9583u128;
    let mut u128_24: u128 = 7579u128;
    let mut u128_25: u128 = 5170u128;
    let mut u128_26: u128 = 9611u128;
    let mut u128_27: u128 = 1814u128;
    let mut u128_28: u128 = 1991u128;
    let mut u128_29: u128 = 7743u128;
    let mut u128_30: u128 = 646u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_30);
    let mut u128_31: u128 = crate::udiv128::u128_mulhi(u128_29, u128_28);
    let mut u128_32: u128 = crate::udiv128::u128_mulhi(u128_27, u128_26);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_25);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_24);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_23);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_22);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_21);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_20);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_19);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut u128_33: u128 = crate::udiv128::u128_mulhi(u128_18, u128_17);
    let mut tuple_8: (u128, u64) = crate::udiv128::udivmod_1e19(u128_16);
    let mut u128_34: u128 = crate::udiv128::u128_mulhi(u128_15, u128_14);
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut tuple_9: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut tuple_10: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut tuple_11: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut tuple_12: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut tuple_13: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut tuple_14: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut tuple_15: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut tuple_16: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut u128_35: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut u128_36: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut u128_37: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    let mut buffer_10: crate::Buffer = crate::Buffer::default();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_43() {
    rusty_monitor::set_test_id(43);
    let mut u128_0: u128 = 2537u128;
    let mut u128_1: u128 = 4866u128;
    let mut u128_2: u128 = 2855u128;
    let mut u128_3: u128 = 6635u128;
    let mut u128_4: u128 = 8244u128;
    let mut u128_5: u128 = 8239u128;
    let mut u128_6: u128 = 2492u128;
    let mut u128_7: u128 = 163u128;
    let mut u128_8: u128 = 6226u128;
    let mut u128_9: u128 = 4829u128;
    let mut u128_10: u128 = 2204u128;
    let mut u128_11: u128 = 6873u128;
    let mut u128_12: u128 = 1109u128;
    let mut u128_13: u128 = 3848u128;
    let mut u128_14: u128 = 7157u128;
    let mut u128_15: u128 = 6793u128;
    let mut u128_16: u128 = 2421u128;
    let mut u128_17: u128 = 5701u128;
    let mut u128_18: u128 = 4658u128;
    let mut u128_19: u128 = 2141u128;
    let mut u128_20: u128 = 1504u128;
    let mut u128_21: u128 = 7647u128;
    let mut u128_22: u128 = 189u128;
    let mut u128_23: u128 = 4652u128;
    let mut u128_24: u128 = 3364u128;
    let mut u128_25: u128 = 292u128;
    let mut u128_26: u128 = 7236u128;
    let mut u128_27: u128 = 2094u128;
    let mut u128_28: u128 = 8069u128;
    let mut u128_29: u128 = 2228u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_29);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_28);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_27);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_26);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_25);
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_24);
    let mut u128_30: u128 = crate::udiv128::u128_mulhi(u128_23, u128_22);
    let mut u128_31: u128 = crate::udiv128::u128_mulhi(u128_21, u128_20);
    let mut u128_32: u128 = crate::udiv128::u128_mulhi(u128_19, u128_18);
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_17);
    let mut u128_33: u128 = crate::udiv128::u128_mulhi(u128_16, u128_15);
    let mut u128_34: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut u128_35: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut u128_36: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut u128_37: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut tuple_8: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut tuple_9: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut tuple_10: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut buffer_11: crate::Buffer = crate::Buffer::default();
    let mut u128_38: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut buffer_12: crate::Buffer = crate::Buffer::default();
    let mut buffer_13: crate::Buffer = crate::Buffer::default();
    let mut buffer_14: crate::Buffer = crate::Buffer::new();
    let mut tuple_11: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    let mut buffer_15: crate::Buffer = crate::Buffer::new();
    let mut buffer_16: crate::Buffer = crate::Buffer::new();
    let mut buffer_17: crate::Buffer = crate::Buffer::default();
    let mut buffer_18: crate::Buffer = crate::Buffer::new();
    let mut buffer_19: crate::Buffer = crate::Buffer::new();
    let mut buffer_20: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_44() {
    rusty_monitor::set_test_id(44);
    let mut u128_0: u128 = 9176u128;
    let mut u128_1: u128 = 5743u128;
    let mut u128_2: u128 = 5844u128;
    let mut u128_3: u128 = 5383u128;
    let mut u128_4: u128 = 4504u128;
    let mut u128_5: u128 = 3306u128;
    let mut u128_6: u128 = 4035u128;
    let mut u128_7: u128 = 6239u128;
    let mut u128_8: u128 = 6559u128;
    let mut u128_9: u128 = 6211u128;
    let mut u128_10: u128 = 288u128;
    let mut u128_11: u128 = 8374u128;
    let mut u128_12: u128 = 7844u128;
    let mut u128_13: u128 = 3028u128;
    let mut u128_14: u128 = 3589u128;
    let mut u128_15: u128 = 8582u128;
    let mut u128_16: u128 = 981u128;
    let mut u128_17: u128 = 3621u128;
    let mut u128_18: u128 = 1223u128;
    let mut u128_19: u128 = 2312u128;
    let mut u128_20: u128 = 9795u128;
    let mut u128_21: u128 = 8885u128;
    let mut u128_22: u128 = 6803u128;
    let mut u128_23: u128 = 7226u128;
    let mut u128_24: u128 = 7450u128;
    let mut u128_25: u128 = 1764u128;
    let mut u128_26: u128 = 6100u128;
    let mut u128_27: u128 = 3518u128;
    let mut u128_28: u128 = 7086u128;
    let mut u128_29: u128 = 2972u128;
    let mut u128_30: u128 = 8787u128;
    let mut u128_31: u128 = 622u128;
    let mut u128_32: u128 = 5454u128;
    let mut u128_33: u128 = 6846u128;
    let mut u128_34: u128 = 3590u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_34);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_33);
    let mut u128_35: u128 = crate::udiv128::u128_mulhi(u128_32, u128_31);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_30);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_29);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut u128_36: u128 = crate::udiv128::u128_mulhi(u128_28, u128_27);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_26);
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_25);
    let mut u128_37: u128 = crate::udiv128::u128_mulhi(u128_24, u128_23);
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut u128_38: u128 = crate::udiv128::u128_mulhi(u128_22, u128_21);
    let mut u128_39: u128 = crate::udiv128::u128_mulhi(u128_20, u128_19);
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_18);
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_17);
    let mut buffer_11: crate::Buffer = crate::Buffer::default();
    let mut tuple_8: (u128, u64) = crate::udiv128::udivmod_1e19(u128_16);
    let mut buffer_12: crate::Buffer = crate::Buffer::default();
    let mut u128_40: u128 = crate::udiv128::u128_mulhi(u128_15, u128_14);
    let mut tuple_9: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut u128_41: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut u128_42: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_13: crate::Buffer = crate::Buffer::new();
    let mut tuple_10: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut buffer_14: crate::Buffer = crate::Buffer::new();
    let mut tuple_11: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut buffer_15: crate::Buffer = crate::Buffer::default();
    let mut tuple_12: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut tuple_13: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_16: crate::Buffer = crate::Buffer::new();
    let mut u128_43: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut tuple_14: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut u128_44: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    let mut buffer_17: crate::Buffer = crate::Buffer::new();
    let mut buffer_18: crate::Buffer = crate::Buffer::new();
    let mut buffer_19: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_45() {
    rusty_monitor::set_test_id(45);
    let mut u128_0: u128 = 9370u128;
    let mut u128_1: u128 = 2362u128;
    let mut u128_2: u128 = 5859u128;
    let mut u128_3: u128 = 8946u128;
    let mut u128_4: u128 = 5750u128;
    let mut u128_5: u128 = 7189u128;
    let mut u128_6: u128 = 6055u128;
    let mut u128_7: u128 = 8420u128;
    let mut u128_8: u128 = 3690u128;
    let mut u128_9: u128 = 9783u128;
    let mut u128_10: u128 = 436u128;
    let mut u128_11: u128 = 1284u128;
    let mut u128_12: u128 = 8502u128;
    let mut u128_13: u128 = 8827u128;
    let mut u128_14: u128 = 5500u128;
    let mut u128_15: u128 = 8151u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_15);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_11: crate::Buffer = crate::Buffer::default();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_46() {
    rusty_monitor::set_test_id(46);
    let mut u128_0: u128 = 832u128;
    let mut u128_1: u128 = 2250u128;
    let mut u128_2: u128 = 3043u128;
    let mut u128_3: u128 = 4274u128;
    let mut u128_4: u128 = 8465u128;
    let mut u128_5: u128 = 6282u128;
    let mut u128_6: u128 = 8133u128;
    let mut u128_7: u128 = 2930u128;
    let mut u128_8: u128 = 344u128;
    let mut u128_9: u128 = 7319u128;
    let mut u128_10: u128 = 1803u128;
    let mut u128_11: u128 = 7269u128;
    let mut u128_12: u128 = 768u128;
    let mut u128_13: u128 = 6189u128;
    let mut u128_14: u128 = 85u128;
    let mut u128_15: u128 = 110u128;
    let mut u128_16: u128 = 4642u128;
    let mut u128_17: u128 = 4778u128;
    let mut u128_18: u128 = 5804u128;
    let mut u128_19: u128 = 5169u128;
    let mut u128_20: u128 = 2683u128;
    let mut u128_21: u128 = 749u128;
    let mut u128_22: u128 = 7417u128;
    let mut u128_23: u128 = 441u128;
    let mut u128_24: u128 = 8852u128;
    let mut u128_25: u128 = 6726u128;
    let mut u128_26: u128 = 9594u128;
    let mut u128_27: u128 = 9365u128;
    let mut u128_28: u128 = 4480u128;
    let mut u128_29: u128 = 7868u128;
    let mut u128_30: u128 = 4813u128;
    let mut u128_31: u128 = 9538u128;
    let mut u128_32: u128 = 618u128;
    let mut u128_33: u128 = 4501u128;
    let mut u128_34: u128 = 6522u128;
    let mut u128_35: u128 = 9299u128;
    let mut u128_36: u128 = 2326u128;
    let mut u128_37: u128 = 9976u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_37);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut u128_38: u128 = crate::udiv128::u128_mulhi(u128_36, u128_35);
    let mut u128_39: u128 = crate::udiv128::u128_mulhi(u128_34, u128_33);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut u128_40: u128 = crate::udiv128::u128_mulhi(u128_32, u128_31);
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_30);
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut buffer_11: crate::Buffer = crate::Buffer::default();
    let mut buffer_12: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_29);
    let mut u128_41: u128 = crate::udiv128::u128_mulhi(u128_28, u128_27);
    let mut u128_42: u128 = crate::udiv128::u128_mulhi(u128_26, u128_25);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_24);
    let mut u128_43: u128 = crate::udiv128::u128_mulhi(u128_23, u128_22);
    let mut u128_44: u128 = crate::udiv128::u128_mulhi(u128_21, u128_20);
    let mut u128_45: u128 = crate::udiv128::u128_mulhi(u128_19, u128_18);
    let mut u128_46: u128 = crate::udiv128::u128_mulhi(u128_17, u128_16);
    let mut u128_47: u128 = crate::udiv128::u128_mulhi(u128_15, u128_14);
    let mut buffer_13: crate::Buffer = crate::Buffer::new();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut buffer_14: crate::Buffer = crate::Buffer::new();
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut u128_48: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut tuple_8: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut buffer_15: crate::Buffer = crate::Buffer::default();
    let mut buffer_16: crate::Buffer = crate::Buffer::new();
    let mut tuple_9: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut u128_49: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut u128_50: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut u128_51: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    let mut buffer_17: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_47() {
    rusty_monitor::set_test_id(47);
    let mut u128_0: u128 = 3717u128;
    let mut u128_1: u128 = 5888u128;
    let mut u128_2: u128 = 4078u128;
    let mut u128_3: u128 = 6163u128;
    let mut u128_4: u128 = 2179u128;
    let mut u128_5: u128 = 4016u128;
    let mut u128_6: u128 = 3816u128;
    let mut u128_7: u128 = 1488u128;
    let mut u128_8: u128 = 3948u128;
    let mut u128_9: u128 = 6004u128;
    let mut u128_10: u128 = 4254u128;
    let mut u128_11: u128 = 3434u128;
    let mut u128_12: u128 = 5701u128;
    let mut u128_13: u128 = 2745u128;
    let mut u128_14: u128 = 1516u128;
    let mut u128_15: u128 = 897u128;
    let mut u128_16: u128 = 5356u128;
    let mut u128_17: u128 = 6492u128;
    let mut u128_18: u128 = 312u128;
    let mut u128_19: u128 = 9029u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_19);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_18, u128_17);
    let mut u128_21: u128 = crate::udiv128::u128_mulhi(u128_16, u128_15);
    let mut u128_22: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut u128_23: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut u128_24: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut buffer_10: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut buffer_11: crate::Buffer = crate::Buffer::default();
    let mut buffer_12: crate::Buffer = crate::Buffer::new();
    let mut u128_25: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_13: crate::Buffer = crate::Buffer::default();
    let mut u128_26: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut buffer_14: crate::Buffer = crate::Buffer::new();
    let mut u128_27: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut buffer_15: crate::Buffer = crate::Buffer::new();
    let mut buffer_16: crate::Buffer = crate::Buffer::new();
    let mut buffer_17: crate::Buffer = crate::Buffer::new();
    let mut buffer_18: crate::Buffer = crate::Buffer::default();
    let mut buffer_19: crate::Buffer = crate::Buffer::new();
    let mut buffer_20: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    let mut buffer_21: crate::Buffer = crate::Buffer::default();
    let mut buffer_22: crate::Buffer = crate::Buffer::new();
    let mut buffer_23: crate::Buffer = crate::Buffer::new();
    let mut buffer_24: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_48() {
    rusty_monitor::set_test_id(48);
    let mut u128_0: u128 = 6713u128;
    let mut u128_1: u128 = 3427u128;
    let mut u128_2: u128 = 8080u128;
    let mut u128_3: u128 = 6672u128;
    let mut u128_4: u128 = 993u128;
    let mut u128_5: u128 = 3701u128;
    let mut u128_6: u128 = 1669u128;
    let mut u128_7: u128 = 4828u128;
    let mut u128_8: u128 = 3291u128;
    let mut u128_9: u128 = 6692u128;
    let mut u128_10: u128 = 3368u128;
    let mut u128_11: u128 = 1600u128;
    let mut u128_12: u128 = 4226u128;
    let mut u128_13: u128 = 3697u128;
    let mut u128_14: u128 = 2754u128;
    let mut u128_15: u128 = 3576u128;
    let mut u128_16: u128 = 3677u128;
    let mut u128_17: u128 = 4171u128;
    let mut u128_18: u128 = 9497u128;
    let mut u128_19: u128 = 1402u128;
    let mut u128_20: u128 = 4342u128;
    let mut u128_21: u128 = 3832u128;
    let mut u128_22: u128 = 8422u128;
    let mut u128_23: u128 = 7359u128;
    let mut u128_24: u128 = 1690u128;
    let mut u128_25: u128 = 276u128;
    let mut u128_26: u128 = crate::udiv128::u128_mulhi(u128_25, u128_24);
    let mut u128_27: u128 = crate::udiv128::u128_mulhi(u128_23, u128_22);
    let mut u128_28: u128 = crate::udiv128::u128_mulhi(u128_21, u128_20);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_19);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_18);
    let mut u128_29: u128 = crate::udiv128::u128_mulhi(u128_17, u128_16);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut u128_30: u128 = crate::udiv128::u128_mulhi(u128_15, u128_14);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut u128_31: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut u128_32: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut u128_33: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut u128_34: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut buffer_11: crate::Buffer = crate::Buffer::new();
    let mut u128_35: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_49() {
    rusty_monitor::set_test_id(49);
    let mut u128_0: u128 = 3559u128;
    let mut u128_1: u128 = 8541u128;
    let mut u128_2: u128 = 2141u128;
    let mut u128_3: u128 = 6941u128;
    let mut u128_4: u128 = 5582u128;
    let mut u128_5: u128 = 5815u128;
    let mut u128_6: u128 = 8793u128;
    let mut u128_7: u128 = 7807u128;
    let mut u128_8: u128 = 1138u128;
    let mut u128_9: u128 = 7105u128;
    let mut u128_10: u128 = 3367u128;
    let mut u128_11: u128 = 2175u128;
    let mut u128_12: u128 = 5701u128;
    let mut u128_13: u128 = 6176u128;
    let mut u128_14: u128 = 9430u128;
    let mut u128_15: u128 = 2031u128;
    let mut u128_16: u128 = 2093u128;
    let mut u128_17: u128 = 8585u128;
    let mut u128_18: u128 = 7875u128;
    let mut u128_19: u128 = 6393u128;
    let mut u128_20: u128 = 9817u128;
    let mut u128_21: u128 = 5194u128;
    let mut u128_22: u128 = 356u128;
    let mut u128_23: u128 = 4886u128;
    let mut u128_24: u128 = 384u128;
    let mut u128_25: u128 = 3066u128;
    let mut u128_26: u128 = 247u128;
    let mut u128_27: u128 = 8180u128;
    let mut u128_28: u128 = 4763u128;
    let mut u128_29: u128 = 1634u128;
    let mut u128_30: u128 = 5620u128;
    let mut u128_31: u128 = 7673u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_31);
    let mut u128_32: u128 = crate::udiv128::u128_mulhi(u128_30, u128_29);
    let mut u128_33: u128 = crate::udiv128::u128_mulhi(u128_28, u128_27);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_26);
    let mut u128_34: u128 = crate::udiv128::u128_mulhi(u128_25, u128_24);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut u128_35: u128 = crate::udiv128::u128_mulhi(u128_23, u128_22);
    let mut u128_36: u128 = crate::udiv128::u128_mulhi(u128_21, u128_20);
    let mut u128_37: u128 = crate::udiv128::u128_mulhi(u128_19, u128_18);
    let mut u128_38: u128 = crate::udiv128::u128_mulhi(u128_17, u128_16);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_15);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut u128_39: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut u128_40: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut u128_41: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut u128_42: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut buffer_10: crate::Buffer = crate::Buffer::default();
    let mut buffer_11: crate::Buffer = crate::Buffer::new();
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut u128_43: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut buffer_12: crate::Buffer = crate::Buffer::new();
    let mut buffer_13: crate::Buffer = crate::Buffer::default();
    let mut buffer_14: crate::Buffer = crate::Buffer::new();
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    let mut buffer_15: crate::Buffer = crate::Buffer::default();
    panic!("From RustyUnit with love");
}
}