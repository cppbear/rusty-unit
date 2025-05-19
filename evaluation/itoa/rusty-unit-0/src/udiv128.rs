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
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_0() {
    rusty_monitor::set_test_id(0);
    let mut u128_0: u128 = 3624u128;
    let mut u128_1: u128 = 5184u128;
    let mut u128_2: u128 = 1276u128;
    let mut u128_3: u128 = 3255u128;
    let mut u128_4: u128 = 2846u128;
    let mut u128_5: u128 = 2495u128;
    let mut u128_6: u128 = 1642u128;
    let mut u128_7: u128 = 80u128;
    let mut u128_8: u128 = 8790u128;
    let mut u128_9: u128 = 9785u128;
    let mut u128_10: u128 = 7258u128;
    let mut u128_11: u128 = 6320u128;
    let mut u128_12: u128 = 4393u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1() {
    rusty_monitor::set_test_id(1);
    let mut u128_0: u128 = 6038u128;
    let mut u128_1: u128 = 1557u128;
    let mut u128_2: u128 = 7814u128;
    let mut u128_3: u128 = 1669u128;
    let mut u128_4: u128 = 3197u128;
    let mut u128_5: u128 = 5613u128;
    let mut u128_6: u128 = 5731u128;
    let mut u128_7: u128 = 4731u128;
    let mut u128_8: u128 = 1421u128;
    let mut u128_9: u128 = 6720u128;
    let mut u128_10: u128 = 6710u128;
    let mut u128_11: u128 = 7144u128;
    let mut u128_12: u128 = 3207u128;
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2() {
    rusty_monitor::set_test_id(2);
    let mut u128_0: u128 = 5805u128;
    let mut u128_1: u128 = 2236u128;
    let mut u128_2: u128 = 7565u128;
    let mut u128_3: u128 = 1022u128;
    let mut u128_4: u128 = 9852u128;
    let mut u128_5: u128 = 9498u128;
    let mut u128_6: u128 = 2395u128;
    let mut u128_7: u128 = 4648u128;
    let mut u128_8: u128 = 5576u128;
    let mut u128_9: u128 = 9528u128;
    let mut u128_10: u128 = 8896u128;
    let mut u128_11: u128 = 6741u128;
    let mut u128_12: u128 = 1286u128;
    let mut u128_13: u128 = 7538u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3() {
    rusty_monitor::set_test_id(3);
    let mut u128_0: u128 = 7955u128;
    let mut u128_1: u128 = 9438u128;
    let mut u128_2: u128 = 4120u128;
    let mut u128_3: u128 = 8335u128;
    let mut u128_4: u128 = 9930u128;
    let mut u128_5: u128 = 7182u128;
    let mut u128_6: u128 = 5962u128;
    let mut u128_7: u128 = 5657u128;
    let mut u128_8: u128 = 6576u128;
    let mut u128_9: u128 = 3309u128;
    let mut u128_10: u128 = 9225u128;
    let mut u128_11: u128 = 7435u128;
    let mut u128_12: u128 = 6287u128;
    let mut u128_13: u128 = 4764u128;
    let mut u128_14: u128 = 497u128;
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4() {
    rusty_monitor::set_test_id(4);
    let mut u128_0: u128 = 5590u128;
    let mut u128_1: u128 = 3404u128;
    let mut u128_2: u128 = 2094u128;
    let mut u128_3: u128 = 13u128;
    let mut u128_4: u128 = 1759u128;
    let mut u128_5: u128 = 2683u128;
    let mut u128_6: u128 = 6757u128;
    let mut u128_7: u128 = 7118u128;
    let mut u128_8: u128 = 3790u128;
    let mut u128_9: u128 = 9826u128;
    let mut u128_10: u128 = 7631u128;
    let mut u128_11: u128 = 7293u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5() {
    rusty_monitor::set_test_id(5);
    let mut u128_0: u128 = 9390u128;
    let mut u128_1: u128 = 6456u128;
    let mut u128_2: u128 = 3542u128;
    let mut u128_3: u128 = 3054u128;
    let mut u128_4: u128 = 8106u128;
    let mut u128_5: u128 = 9895u128;
    let mut u128_6: u128 = 6045u128;
    let mut u128_7: u128 = 7986u128;
    let mut u128_8: u128 = 4541u128;
    let mut u128_9: u128 = 3765u128;
    let mut u128_10: u128 = 6002u128;
    let mut u128_11: u128 = 3875u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6() {
    rusty_monitor::set_test_id(6);
    let mut u128_0: u128 = 1674u128;
    let mut u128_1: u128 = 1012u128;
    let mut u128_2: u128 = 5695u128;
    let mut u128_3: u128 = 7604u128;
    let mut u128_4: u128 = 9777u128;
    let mut u128_5: u128 = 1055u128;
    let mut u128_6: u128 = 137u128;
    let mut u128_7: u128 = 3589u128;
    let mut u128_8: u128 = 3466u128;
    let mut u128_9: u128 = 5586u128;
    let mut u128_10: u128 = 7631u128;
    let mut u128_11: u128 = 381u128;
    let mut u128_12: u128 = 8934u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7() {
    rusty_monitor::set_test_id(7);
    let mut u128_0: u128 = 3229u128;
    let mut u128_1: u128 = 7434u128;
    let mut u128_2: u128 = 4663u128;
    let mut u128_3: u128 = 5330u128;
    let mut u128_4: u128 = 7964u128;
    let mut u128_5: u128 = 1740u128;
    let mut u128_6: u128 = 3927u128;
    let mut u128_7: u128 = 8622u128;
    let mut u128_8: u128 = 5026u128;
    let mut u128_9: u128 = 1456u128;
    let mut u128_10: u128 = 8764u128;
    let mut u128_11: u128 = 7763u128;
    let mut u128_12: u128 = 6381u128;
    let mut u128_13: u128 = 8561u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8() {
    rusty_monitor::set_test_id(8);
    let mut u128_0: u128 = 151u128;
    let mut u128_1: u128 = 8453u128;
    let mut u128_2: u128 = 9077u128;
    let mut u128_3: u128 = 951u128;
    let mut u128_4: u128 = 7405u128;
    let mut u128_5: u128 = 4621u128;
    let mut u128_6: u128 = 2712u128;
    let mut u128_7: u128 = 9639u128;
    let mut u128_8: u128 = 8325u128;
    let mut u128_9: u128 = 645u128;
    let mut u128_10: u128 = 6249u128;
    let mut u128_11: u128 = 5456u128;
    let mut u128_12: u128 = 7773u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_9() {
    rusty_monitor::set_test_id(9);
    let mut u128_0: u128 = 2079u128;
    let mut u128_1: u128 = 1170u128;
    let mut u128_2: u128 = 8903u128;
    let mut u128_3: u128 = 259u128;
    let mut u128_4: u128 = 986u128;
    let mut u128_5: u128 = 8395u128;
    let mut u128_6: u128 = 482u128;
    let mut u128_7: u128 = 7437u128;
    let mut u128_8: u128 = 7460u128;
    let mut u128_9: u128 = 7664u128;
    let mut u128_10: u128 = 285u128;
    let mut u128_11: u128 = 9636u128;
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_10() {
    rusty_monitor::set_test_id(10);
    let mut u128_0: u128 = 9480u128;
    let mut u128_1: u128 = 5790u128;
    let mut u128_2: u128 = 543u128;
    let mut u128_3: u128 = 8401u128;
    let mut u128_4: u128 = 4909u128;
    let mut u128_5: u128 = 6092u128;
    let mut u128_6: u128 = 3345u128;
    let mut u128_7: u128 = 4101u128;
    let mut u128_8: u128 = 9516u128;
    let mut u128_9: u128 = 5132u128;
    let mut u128_10: u128 = 9912u128;
    let mut u128_11: u128 = 7626u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_11() {
    rusty_monitor::set_test_id(11);
    let mut u128_0: u128 = 7394u128;
    let mut u128_1: u128 = 7791u128;
    let mut u128_2: u128 = 6073u128;
    let mut u128_3: u128 = 9012u128;
    let mut u128_4: u128 = 9388u128;
    let mut u128_5: u128 = 4236u128;
    let mut u128_6: u128 = 3846u128;
    let mut u128_7: u128 = 2410u128;
    let mut u128_8: u128 = 465u128;
    let mut u128_9: u128 = 7206u128;
    let mut u128_10: u128 = 4346u128;
    let mut u128_11: u128 = 1136u128;
    let mut u128_12: u128 = 9575u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_12() {
    rusty_monitor::set_test_id(12);
    let mut u128_0: u128 = 2819u128;
    let mut u128_1: u128 = 8106u128;
    let mut u128_2: u128 = 6584u128;
    let mut u128_3: u128 = 8971u128;
    let mut u128_4: u128 = 2054u128;
    let mut u128_5: u128 = 6788u128;
    let mut u128_6: u128 = 7397u128;
    let mut u128_7: u128 = 4675u128;
    let mut u128_8: u128 = 4199u128;
    let mut u128_9: u128 = 8271u128;
    let mut u128_10: u128 = 7938u128;
    let mut u128_11: u128 = 7327u128;
    let mut u128_12: u128 = 8750u128;
    let mut u128_13: u128 = 3132u128;
    let mut u128_14: u128 = 2371u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_13() {
    rusty_monitor::set_test_id(13);
    let mut u128_0: u128 = 9647u128;
    let mut u128_1: u128 = 2050u128;
    let mut u128_2: u128 = 9535u128;
    let mut u128_3: u128 = 1595u128;
    let mut u128_4: u128 = 2715u128;
    let mut u128_5: u128 = 1365u128;
    let mut u128_6: u128 = 914u128;
    let mut u128_7: u128 = 7972u128;
    let mut u128_8: u128 = 7288u128;
    let mut u128_9: u128 = 4392u128;
    let mut u128_10: u128 = 4187u128;
    let mut u128_11: u128 = 541u128;
    let mut u128_12: u128 = 4403u128;
    let mut u128_13: u128 = 2678u128;
    let mut u128_14: u128 = 7384u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_14() {
    rusty_monitor::set_test_id(14);
    let mut u128_0: u128 = 5364u128;
    let mut u128_1: u128 = 8420u128;
    let mut u128_2: u128 = 5829u128;
    let mut u128_3: u128 = 4924u128;
    let mut u128_4: u128 = 7740u128;
    let mut u128_5: u128 = 9455u128;
    let mut u128_6: u128 = 203u128;
    let mut u128_7: u128 = 6996u128;
    let mut u128_8: u128 = 6728u128;
    let mut u128_9: u128 = 6068u128;
    let mut u128_10: u128 = 7367u128;
    let mut u128_11: u128 = 3923u128;
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_15() {
    rusty_monitor::set_test_id(15);
    let mut u128_0: u128 = 296u128;
    let mut u128_1: u128 = 4356u128;
    let mut u128_2: u128 = 4982u128;
    let mut u128_3: u128 = 6358u128;
    let mut u128_4: u128 = 5774u128;
    let mut u128_5: u128 = 1302u128;
    let mut u128_6: u128 = 5412u128;
    let mut u128_7: u128 = 1490u128;
    let mut u128_8: u128 = 7073u128;
    let mut u128_9: u128 = 7853u128;
    let mut u128_10: u128 = 2919u128;
    let mut u128_11: u128 = 3927u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_16() {
    rusty_monitor::set_test_id(16);
    let mut u128_0: u128 = 2196u128;
    let mut u128_1: u128 = 2542u128;
    let mut u128_2: u128 = 7636u128;
    let mut u128_3: u128 = 1451u128;
    let mut u128_4: u128 = 9986u128;
    let mut u128_5: u128 = 303u128;
    let mut u128_6: u128 = 1674u128;
    let mut u128_7: u128 = 805u128;
    let mut u128_8: u128 = 62u128;
    let mut u128_9: u128 = 3137u128;
    let mut u128_10: u128 = 1450u128;
    let mut u128_11: u128 = 9045u128;
    let mut u128_12: u128 = 6610u128;
    let mut u128_13: u128 = 4719u128;
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_17() {
    rusty_monitor::set_test_id(17);
    let mut u128_0: u128 = 1518u128;
    let mut u128_1: u128 = 6751u128;
    let mut u128_2: u128 = 5797u128;
    let mut u128_3: u128 = 9106u128;
    let mut u128_4: u128 = 8180u128;
    let mut u128_5: u128 = 5059u128;
    let mut u128_6: u128 = 5297u128;
    let mut u128_7: u128 = 9216u128;
    let mut u128_8: u128 = 2640u128;
    let mut u128_9: u128 = 9287u128;
    let mut u128_10: u128 = 9945u128;
    let mut u128_11: u128 = 1147u128;
    let mut u128_12: u128 = 7237u128;
    let mut u128_13: u128 = 9441u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_18() {
    rusty_monitor::set_test_id(18);
    let mut u128_0: u128 = 4247u128;
    let mut u128_1: u128 = 490u128;
    let mut u128_2: u128 = 8106u128;
    let mut u128_3: u128 = 3978u128;
    let mut u128_4: u128 = 7539u128;
    let mut u128_5: u128 = 6370u128;
    let mut u128_6: u128 = 5646u128;
    let mut u128_7: u128 = 4142u128;
    let mut u128_8: u128 = 9270u128;
    let mut u128_9: u128 = 4212u128;
    let mut u128_10: u128 = 5335u128;
    let mut u128_11: u128 = 4522u128;
    let mut u128_12: u128 = 3524u128;
    let mut u128_13: u128 = 2253u128;
    let mut u128_14: u128 = 9053u128;
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_19() {
    rusty_monitor::set_test_id(19);
    let mut u128_0: u128 = 7316u128;
    let mut u128_1: u128 = 2292u128;
    let mut u128_2: u128 = 663u128;
    let mut u128_3: u128 = 2735u128;
    let mut u128_4: u128 = 2899u128;
    let mut u128_5: u128 = 6154u128;
    let mut u128_6: u128 = 5634u128;
    let mut u128_7: u128 = 4949u128;
    let mut u128_8: u128 = 4049u128;
    let mut u128_9: u128 = 5280u128;
    let mut u128_10: u128 = 9226u128;
    let mut u128_11: u128 = 6418u128;
    let mut u128_12: u128 = 2156u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_20() {
    rusty_monitor::set_test_id(20);
    let mut u128_0: u128 = 2207u128;
    let mut u128_1: u128 = 9072u128;
    let mut u128_2: u128 = 9164u128;
    let mut u128_3: u128 = 4008u128;
    let mut u128_4: u128 = 7589u128;
    let mut u128_5: u128 = 7277u128;
    let mut u128_6: u128 = 7465u128;
    let mut u128_7: u128 = 4343u128;
    let mut u128_8: u128 = 5467u128;
    let mut u128_9: u128 = 6209u128;
    let mut u128_10: u128 = 8060u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut u128_11: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_21() {
    rusty_monitor::set_test_id(21);
    let mut u128_0: u128 = 6154u128;
    let mut u128_1: u128 = 1120u128;
    let mut u128_2: u128 = 7024u128;
    let mut u128_3: u128 = 7140u128;
    let mut u128_4: u128 = 7220u128;
    let mut u128_5: u128 = 4899u128;
    let mut u128_6: u128 = 5233u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut u128_7: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut buffer_11: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_12: crate::Buffer = crate::Buffer::new();
    let mut buffer_13: crate::Buffer = crate::Buffer::default();
    let mut buffer_14: crate::Buffer = crate::Buffer::new();
    let mut buffer_15: crate::Buffer = crate::Buffer::default();
    let mut buffer_16: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut buffer_17: crate::Buffer = crate::Buffer::new();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_22() {
    rusty_monitor::set_test_id(22);
    let mut u128_0: u128 = 5008u128;
    let mut u128_1: u128 = 6287u128;
    let mut u128_2: u128 = 9004u128;
    let mut u128_3: u128 = 2206u128;
    let mut u128_4: u128 = 3434u128;
    let mut u128_5: u128 = 9605u128;
    let mut u128_6: u128 = 4999u128;
    let mut u128_7: u128 = 3842u128;
    let mut u128_8: u128 = 3141u128;
    let mut u128_9: u128 = 7584u128;
    let mut u128_10: u128 = 2754u128;
    let mut u128_11: u128 = 5463u128;
    let mut u128_12: u128 = 5230u128;
    let mut u128_13: u128 = 9945u128;
    let mut u128_14: u128 = 2469u128;
    let mut u128_15: u128 = 5662u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_15);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut u128_21: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_23() {
    rusty_monitor::set_test_id(23);
    let mut u128_0: u128 = 375u128;
    let mut u128_1: u128 = 118u128;
    let mut u128_2: u128 = 4262u128;
    let mut u128_3: u128 = 1719u128;
    let mut u128_4: u128 = 4833u128;
    let mut u128_5: u128 = 2589u128;
    let mut u128_6: u128 = 332u128;
    let mut u128_7: u128 = 5950u128;
    let mut u128_8: u128 = 5219u128;
    let mut u128_9: u128 = 3662u128;
    let mut u128_10: u128 = 4220u128;
    let mut u128_11: u128 = 2383u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut buffer_10: crate::Buffer = crate::Buffer::default();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_24() {
    rusty_monitor::set_test_id(24);
    let mut u128_0: u128 = 9487u128;
    let mut u128_1: u128 = 4844u128;
    let mut u128_2: u128 = 9915u128;
    let mut u128_3: u128 = 5880u128;
    let mut u128_4: u128 = 2337u128;
    let mut u128_5: u128 = 3200u128;
    let mut u128_6: u128 = 2124u128;
    let mut u128_7: u128 = 4840u128;
    let mut u128_8: u128 = 9930u128;
    let mut u128_9: u128 = 7873u128;
    let mut u128_10: u128 = 7436u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut u128_11: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut buffer_10: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_25() {
    rusty_monitor::set_test_id(25);
    let mut u128_0: u128 = 1864u128;
    let mut u128_1: u128 = 4184u128;
    let mut u128_2: u128 = 106u128;
    let mut u128_3: u128 = 3101u128;
    let mut u128_4: u128 = 3023u128;
    let mut u128_5: u128 = 6858u128;
    let mut u128_6: u128 = 5279u128;
    let mut u128_7: u128 = 3830u128;
    let mut u128_8: u128 = 9329u128;
    let mut u128_9: u128 = 6233u128;
    let mut u128_10: u128 = 5972u128;
    let mut u128_11: u128 = 7629u128;
    let mut u128_12: u128 = 1087u128;
    let mut u128_13: u128 = 487u128;
    let mut u128_14: u128 = 7477u128;
    let mut u128_15: u128 = 3275u128;
    let mut u128_16: u128 = 5713u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_16);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_15);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut u128_21: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut u128_22: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut u128_23: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_26() {
    rusty_monitor::set_test_id(26);
    let mut u128_0: u128 = 2742u128;
    let mut u128_1: u128 = 2394u128;
    let mut u128_2: u128 = 9641u128;
    let mut u128_3: u128 = 1987u128;
    let mut u128_4: u128 = 4722u128;
    let mut u128_5: u128 = 7053u128;
    let mut u128_6: u128 = 2406u128;
    let mut u128_7: u128 = 1483u128;
    let mut u128_8: u128 = 5638u128;
    let mut u128_9: u128 = 5428u128;
    let mut u128_10: u128 = 3886u128;
    let mut u128_11: u128 = 5772u128;
    let mut u128_12: u128 = 2563u128;
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_27() {
    rusty_monitor::set_test_id(27);
    let mut u128_0: u128 = 7579u128;
    let mut u128_1: u128 = 4661u128;
    let mut u128_2: u128 = 8366u128;
    let mut u128_3: u128 = 5978u128;
    let mut u128_4: u128 = 6703u128;
    let mut u128_5: u128 = 9046u128;
    let mut u128_6: u128 = 1821u128;
    let mut u128_7: u128 = 9910u128;
    let mut u128_8: u128 = 4865u128;
    let mut u128_9: u128 = 9841u128;
    let mut u128_10: u128 = 3624u128;
    let mut u128_11: u128 = 9749u128;
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_28() {
    rusty_monitor::set_test_id(28);
    let mut u128_0: u128 = 6229u128;
    let mut u128_1: u128 = 6408u128;
    let mut u128_2: u128 = 1610u128;
    let mut u128_3: u128 = 9006u128;
    let mut u128_4: u128 = 2561u128;
    let mut u128_5: u128 = 7996u128;
    let mut u128_6: u128 = 5293u128;
    let mut u128_7: u128 = 3419u128;
    let mut u128_8: u128 = 612u128;
    let mut u128_9: u128 = 7543u128;
    let mut u128_10: u128 = 8048u128;
    let mut u128_11: u128 = 214u128;
    let mut u128_12: u128 = 6659u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_29() {
    rusty_monitor::set_test_id(29);
    let mut u128_0: u128 = 5975u128;
    let mut u128_1: u128 = 1988u128;
    let mut u128_2: u128 = 7050u128;
    let mut u128_3: u128 = 30u128;
    let mut u128_4: u128 = 8716u128;
    let mut u128_5: u128 = 5878u128;
    let mut u128_6: u128 = 3327u128;
    let mut u128_7: u128 = 2850u128;
    let mut u128_8: u128 = 2067u128;
    let mut u128_9: u128 = 1511u128;
    let mut u128_10: u128 = 1783u128;
    let mut u128_11: u128 = 7023u128;
    let mut u128_12: u128 = 3593u128;
    let mut u128_13: u128 = 9995u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_30() {
    rusty_monitor::set_test_id(30);
    let mut u128_0: u128 = 1854u128;
    let mut u128_1: u128 = 2827u128;
    let mut u128_2: u128 = 7070u128;
    let mut u128_3: u128 = 1507u128;
    let mut u128_4: u128 = 613u128;
    let mut u128_5: u128 = 8664u128;
    let mut u128_6: u128 = 5945u128;
    let mut u128_7: u128 = 9167u128;
    let mut u128_8: u128 = 9440u128;
    let mut u128_9: u128 = 1859u128;
    let mut u128_10: u128 = 2340u128;
    let mut u128_11: u128 = 8323u128;
    let mut u128_12: u128 = 9430u128;
    let mut u128_13: u128 = 1673u128;
    let mut u128_14: u128 = 173u128;
    let mut u128_15: u128 = 5469u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_15);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_31() {
    rusty_monitor::set_test_id(31);
    let mut u128_0: u128 = 5667u128;
    let mut u128_1: u128 = 2461u128;
    let mut u128_2: u128 = 5786u128;
    let mut u128_3: u128 = 756u128;
    let mut u128_4: u128 = 5920u128;
    let mut u128_5: u128 = 3489u128;
    let mut u128_6: u128 = 5675u128;
    let mut u128_7: u128 = 5173u128;
    let mut u128_8: u128 = 5800u128;
    let mut u128_9: u128 = 3808u128;
    let mut u128_10: u128 = 8562u128;
    let mut u128_11: u128 = 1151u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_32() {
    rusty_monitor::set_test_id(32);
    let mut u128_0: u128 = 7802u128;
    let mut u128_1: u128 = 1000u128;
    let mut u128_2: u128 = 406u128;
    let mut u128_3: u128 = 2365u128;
    let mut u128_4: u128 = 1866u128;
    let mut u128_5: u128 = 3304u128;
    let mut u128_6: u128 = 3747u128;
    let mut u128_7: u128 = 2195u128;
    let mut u128_8: u128 = 9497u128;
    let mut u128_9: u128 = 469u128;
    let mut u128_10: u128 = 4457u128;
    let mut u128_11: u128 = 7922u128;
    let mut u128_12: u128 = 9747u128;
    let mut u128_13: u128 = 3734u128;
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_33() {
    rusty_monitor::set_test_id(33);
    let mut u128_0: u128 = 3592u128;
    let mut u128_1: u128 = 3895u128;
    let mut u128_2: u128 = 7531u128;
    let mut u128_3: u128 = 2891u128;
    let mut u128_4: u128 = 2326u128;
    let mut u128_5: u128 = 5746u128;
    let mut u128_6: u128 = 3028u128;
    let mut u128_7: u128 = 1575u128;
    let mut u128_8: u128 = 6582u128;
    let mut u128_9: u128 = 588u128;
    let mut u128_10: u128 = 9274u128;
    let mut u128_11: u128 = 7188u128;
    let mut u128_12: u128 = 1805u128;
    let mut u128_13: u128 = 6242u128;
    let mut u128_14: u128 = 6335u128;
    let mut u128_15: u128 = 3024u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_15, u128_14);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut u128_21: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut u128_22: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_34() {
    rusty_monitor::set_test_id(34);
    let mut u128_0: u128 = 1742u128;
    let mut u128_1: u128 = 858u128;
    let mut u128_2: u128 = 1936u128;
    let mut u128_3: u128 = 6195u128;
    let mut u128_4: u128 = 88u128;
    let mut u128_5: u128 = 7982u128;
    let mut u128_6: u128 = 5187u128;
    let mut u128_7: u128 = 2033u128;
    let mut u128_8: u128 = 2593u128;
    let mut u128_9: u128 = 5987u128;
    let mut u128_10: u128 = 6344u128;
    let mut u128_11: u128 = 4668u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut buffer_10: crate::Buffer = crate::Buffer::default();
    let mut buffer_11: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_35() {
    rusty_monitor::set_test_id(35);
    let mut u128_0: u128 = 8729u128;
    let mut u128_1: u128 = 3153u128;
    let mut u128_2: u128 = 5697u128;
    let mut u128_3: u128 = 7215u128;
    let mut u128_4: u128 = 3117u128;
    let mut u128_5: u128 = 6341u128;
    let mut u128_6: u128 = 4238u128;
    let mut u128_7: u128 = 954u128;
    let mut u128_8: u128 = 3485u128;
    let mut u128_9: u128 = 4738u128;
    let mut u128_10: u128 = 9253u128;
    let mut u128_11: u128 = 2296u128;
    let mut u128_12: u128 = 5313u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_36() {
    rusty_monitor::set_test_id(36);
    let mut u128_0: u128 = 7226u128;
    let mut u128_1: u128 = 8660u128;
    let mut u128_2: u128 = 1264u128;
    let mut u128_3: u128 = 5265u128;
    let mut u128_4: u128 = 8452u128;
    let mut u128_5: u128 = 1382u128;
    let mut u128_6: u128 = 4752u128;
    let mut u128_7: u128 = 3947u128;
    let mut u128_8: u128 = 4433u128;
    let mut u128_9: u128 = 1354u128;
    let mut u128_10: u128 = 5575u128;
    let mut u128_11: u128 = 2283u128;
    let mut u128_12: u128 = 5212u128;
    let mut u128_13: u128 = 7739u128;
    let mut u128_14: u128 = 4593u128;
    let mut u128_15: u128 = 6948u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_15);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_37() {
    rusty_monitor::set_test_id(37);
    let mut u128_0: u128 = 5223u128;
    let mut u128_1: u128 = 5473u128;
    let mut u128_2: u128 = 6492u128;
    let mut u128_3: u128 = 5248u128;
    let mut u128_4: u128 = 7628u128;
    let mut u128_5: u128 = 9104u128;
    let mut u128_6: u128 = 3891u128;
    let mut u128_7: u128 = 2990u128;
    let mut u128_8: u128 = 1546u128;
    let mut u128_9: u128 = 9375u128;
    let mut u128_10: u128 = 7495u128;
    let mut u128_11: u128 = 1420u128;
    let mut u128_12: u128 = 9204u128;
    let mut u128_13: u128 = 6479u128;
    let mut u128_14: u128 = 3829u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_38() {
    rusty_monitor::set_test_id(38);
    let mut u128_0: u128 = 8978u128;
    let mut u128_1: u128 = 8739u128;
    let mut u128_2: u128 = 1509u128;
    let mut u128_3: u128 = 4923u128;
    let mut u128_4: u128 = 1947u128;
    let mut u128_5: u128 = 1505u128;
    let mut u128_6: u128 = 4279u128;
    let mut u128_7: u128 = 7523u128;
    let mut u128_8: u128 = 3440u128;
    let mut u128_9: u128 = 2850u128;
    let mut u128_10: u128 = 1665u128;
    let mut u128_11: u128 = 625u128;
    let mut u128_12: u128 = 1278u128;
    let mut u128_13: u128 = 7963u128;
    let mut u128_14: u128 = 9205u128;
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_39() {
    rusty_monitor::set_test_id(39);
    let mut u128_0: u128 = 9522u128;
    let mut u128_1: u128 = 508u128;
    let mut u128_2: u128 = 2141u128;
    let mut u128_3: u128 = 855u128;
    let mut u128_4: u128 = 6413u128;
    let mut u128_5: u128 = 3404u128;
    let mut u128_6: u128 = 5730u128;
    let mut u128_7: u128 = 1088u128;
    let mut u128_8: u128 = 3934u128;
    let mut u128_9: u128 = 4897u128;
    let mut u128_10: u128 = 7695u128;
    let mut u128_11: u128 = 9279u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_40() {
    rusty_monitor::set_test_id(40);
    let mut u128_0: u128 = 8941u128;
    let mut u128_1: u128 = 6235u128;
    let mut u128_2: u128 = 1795u128;
    let mut u128_3: u128 = 8476u128;
    let mut u128_4: u128 = 4993u128;
    let mut u128_5: u128 = 188u128;
    let mut u128_6: u128 = 5607u128;
    let mut u128_7: u128 = 8112u128;
    let mut u128_8: u128 = 9058u128;
    let mut u128_9: u128 = 287u128;
    let mut u128_10: u128 = 170u128;
    let mut u128_11: u128 = 9848u128;
    let mut u128_12: u128 = 9298u128;
    let mut u128_13: u128 = 7366u128;
    let mut u128_14: u128 = 267u128;
    let mut u128_15: u128 = 854u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_15);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut u128_21: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut u128_22: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_41() {
    rusty_monitor::set_test_id(41);
    let mut u128_0: u128 = 5965u128;
    let mut u128_1: u128 = 2600u128;
    let mut u128_2: u128 = 534u128;
    let mut u128_3: u128 = 950u128;
    let mut u128_4: u128 = 6428u128;
    let mut u128_5: u128 = 1041u128;
    let mut u128_6: u128 = 5990u128;
    let mut u128_7: u128 = 8581u128;
    let mut u128_8: u128 = 1826u128;
    let mut u128_9: u128 = 7370u128;
    let mut u128_10: u128 = 9810u128;
    let mut u128_11: u128 = 6821u128;
    let mut u128_12: u128 = 5674u128;
    let mut u128_13: u128 = 9570u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_42() {
    rusty_monitor::set_test_id(42);
    let mut u128_0: u128 = 7464u128;
    let mut u128_1: u128 = 5219u128;
    let mut u128_2: u128 = 6395u128;
    let mut u128_3: u128 = 2945u128;
    let mut u128_4: u128 = 338u128;
    let mut u128_5: u128 = 9266u128;
    let mut u128_6: u128 = 2275u128;
    let mut u128_7: u128 = 2422u128;
    let mut u128_8: u128 = 4831u128;
    let mut u128_9: u128 = 2901u128;
    let mut u128_10: u128 = 7157u128;
    let mut u128_11: u128 = 9271u128;
    let mut u128_12: u128 = 9766u128;
    let mut u128_13: u128 = 2856u128;
    let mut u128_14: u128 = 6120u128;
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_43() {
    rusty_monitor::set_test_id(43);
    let mut u128_0: u128 = 9412u128;
    let mut u128_1: u128 = 4951u128;
    let mut u128_2: u128 = 2350u128;
    let mut u128_3: u128 = 3571u128;
    let mut u128_4: u128 = 1499u128;
    let mut u128_5: u128 = 3878u128;
    let mut u128_6: u128 = 4973u128;
    let mut u128_7: u128 = 2227u128;
    let mut u128_8: u128 = 844u128;
    let mut u128_9: u128 = 3607u128;
    let mut u128_10: u128 = 8836u128;
    let mut u128_11: u128 = 3121u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_44() {
    rusty_monitor::set_test_id(44);
    let mut u128_0: u128 = 3682u128;
    let mut u128_1: u128 = 6938u128;
    let mut u128_2: u128 = 5019u128;
    let mut u128_3: u128 = 5555u128;
    let mut u128_4: u128 = 3281u128;
    let mut u128_5: u128 = 3814u128;
    let mut u128_6: u128 = 9456u128;
    let mut u128_7: u128 = 6482u128;
    let mut u128_8: u128 = 4111u128;
    let mut u128_9: u128 = 7142u128;
    let mut u128_10: u128 = 8293u128;
    let mut u128_11: u128 = 7456u128;
    let mut u128_12: u128 = 1948u128;
    let mut u128_13: u128 = 8912u128;
    let mut u128_14: u128 = 8524u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_45() {
    rusty_monitor::set_test_id(45);
    let mut u128_0: u128 = 894u128;
    let mut u128_1: u128 = 7704u128;
    let mut u128_2: u128 = 8705u128;
    let mut u128_3: u128 = 3834u128;
    let mut u128_4: u128 = 829u128;
    let mut u128_5: u128 = 4637u128;
    let mut u128_6: u128 = 3794u128;
    let mut u128_7: u128 = 722u128;
    let mut u128_8: u128 = 6125u128;
    let mut u128_9: u128 = 6259u128;
    let mut u128_10: u128 = 6335u128;
    let mut u128_11: u128 = 1162u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut buffer_10: crate::Buffer = crate::Buffer::default();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_46() {
    rusty_monitor::set_test_id(46);
    let mut u128_0: u128 = 7677u128;
    let mut u128_1: u128 = 3726u128;
    let mut u128_2: u128 = 8330u128;
    let mut u128_3: u128 = 1803u128;
    let mut u128_4: u128 = 5845u128;
    let mut u128_5: u128 = 1743u128;
    let mut u128_6: u128 = 4842u128;
    let mut u128_7: u128 = 9477u128;
    let mut u128_8: u128 = 8378u128;
    let mut u128_9: u128 = 4609u128;
    let mut u128_10: u128 = 3877u128;
    let mut u128_11: u128 = 2641u128;
    let mut u128_12: u128 = 6385u128;
    let mut u128_13: u128 = 5959u128;
    let mut u128_14: u128 = 3747u128;
    let mut u128_15: u128 = 6300u128;
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_15, u128_14);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut u128_21: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_47() {
    rusty_monitor::set_test_id(47);
    let mut u128_0: u128 = 4440u128;
    let mut u128_1: u128 = 408u128;
    let mut u128_2: u128 = 6053u128;
    let mut u128_3: u128 = 2795u128;
    let mut u128_4: u128 = 6626u128;
    let mut u128_5: u128 = 2596u128;
    let mut u128_6: u128 = 6089u128;
    let mut u128_7: u128 = 9483u128;
    let mut u128_8: u128 = 6450u128;
    let mut u128_9: u128 = 9452u128;
    let mut u128_10: u128 = 2617u128;
    let mut u128_11: u128 = 9340u128;
    let mut u128_12: u128 = 7671u128;
    let mut u128_13: u128 = 1956u128;
    let mut u128_14: u128 = 2354u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_48() {
    rusty_monitor::set_test_id(48);
    let mut u128_0: u128 = 6931u128;
    let mut u128_1: u128 = 1892u128;
    let mut u128_2: u128 = 1360u128;
    let mut u128_3: u128 = 7658u128;
    let mut u128_4: u128 = 4645u128;
    let mut u128_5: u128 = 6954u128;
    let mut u128_6: u128 = 8393u128;
    let mut u128_7: u128 = 7497u128;
    let mut u128_8: u128 = 2412u128;
    let mut u128_9: u128 = 5130u128;
    let mut u128_10: u128 = 5106u128;
    let mut u128_11: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut buffer_11: crate::Buffer = crate::Buffer::default();
    let mut buffer_12: crate::Buffer = crate::Buffer::default();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut buffer_13: crate::Buffer = crate::Buffer::default();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_49() {
    rusty_monitor::set_test_id(49);
    let mut u128_0: u128 = 6963u128;
    let mut u128_1: u128 = 1181u128;
    let mut u128_2: u128 = 5591u128;
    let mut u128_3: u128 = 7315u128;
    let mut u128_4: u128 = 1414u128;
    let mut u128_5: u128 = 6013u128;
    let mut u128_6: u128 = 2673u128;
    let mut u128_7: u128 = 228u128;
    let mut u128_8: u128 = 8748u128;
    let mut u128_9: u128 = 7758u128;
    let mut u128_10: u128 = 6931u128;
    let mut u128_11: u128 = 6402u128;
    let mut u128_12: u128 = 6291u128;
    let mut u128_13: u128 = 410u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}
}