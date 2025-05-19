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
    let mut u128_0: u128 = 3084u128;
    let mut u128_1: u128 = 7527u128;
    let mut u128_2: u128 = 1016u128;
    let mut u128_3: u128 = 9128u128;
    let mut u128_4: u128 = 1765u128;
    let mut u128_5: u128 = 6223u128;
    let mut u128_6: u128 = 9871u128;
    let mut u128_7: u128 = 9396u128;
    let mut u128_8: u128 = 3383u128;
    let mut u128_9: u128 = 9990u128;
    let mut u128_10: u128 = 9194u128;
    let mut u128_11: u128 = 8577u128;
    let mut u128_12: u128 = 2518u128;
    let mut u128_13: u128 = 1686u128;
    let mut u128_14: u128 = 7393u128;
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1() {
    rusty_monitor::set_test_id(1);
    let mut u128_0: u128 = 338u128;
    let mut u128_1: u128 = 8164u128;
    let mut u128_2: u128 = 1727u128;
    let mut u128_3: u128 = 962u128;
    let mut u128_4: u128 = 944u128;
    let mut u128_5: u128 = 7317u128;
    let mut u128_6: u128 = 7802u128;
    let mut u128_7: u128 = 2758u128;
    let mut u128_8: u128 = 8137u128;
    let mut u128_9: u128 = 9403u128;
    let mut u128_10: u128 = 1840u128;
    let mut u128_11: u128 = 7445u128;
    let mut u128_12: u128 = 686u128;
    let mut u128_13: u128 = 7177u128;
    let mut u128_14: u128 = 686u128;
    let mut u128_15: u128 = 8969u128;
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_15, u128_14);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut u128_21: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2() {
    rusty_monitor::set_test_id(2);
    let mut u128_0: u128 = 312u128;
    let mut u128_1: u128 = 3260u128;
    let mut u128_2: u128 = 2658u128;
    let mut u128_3: u128 = 8730u128;
    let mut u128_4: u128 = 2281u128;
    let mut u128_5: u128 = 4818u128;
    let mut u128_6: u128 = 8777u128;
    let mut u128_7: u128 = 4616u128;
    let mut u128_8: u128 = 6738u128;
    let mut u128_9: u128 = 5653u128;
    let mut u128_10: u128 = 380u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut u128_11: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_10: crate::Buffer = crate::Buffer::default();
    let mut buffer_11: crate::Buffer = crate::Buffer::default();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3() {
    rusty_monitor::set_test_id(3);
    let mut u128_0: u128 = 4051u128;
    let mut u128_1: u128 = 4135u128;
    let mut u128_2: u128 = 1188u128;
    let mut u128_3: u128 = 4696u128;
    let mut u128_4: u128 = 116u128;
    let mut u128_5: u128 = 1772u128;
    let mut u128_6: u128 = 3434u128;
    let mut u128_7: u128 = 7473u128;
    let mut u128_8: u128 = 1693u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut u128_9: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut u128_10: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut buffer_11: crate::Buffer = crate::Buffer::default();
    let mut buffer_12: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_13: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut buffer_14: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut u128_11: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4() {
    rusty_monitor::set_test_id(4);
    let mut u128_0: u128 = 5064u128;
    let mut u128_1: u128 = 6205u128;
    let mut u128_2: u128 = 6434u128;
    let mut u128_3: u128 = 8057u128;
    let mut u128_4: u128 = 8526u128;
    let mut u128_5: u128 = 4586u128;
    let mut u128_6: u128 = 470u128;
    let mut u128_7: u128 = 8780u128;
    let mut u128_8: u128 = 8841u128;
    let mut u128_9: u128 = 5290u128;
    let mut u128_10: u128 = 148u128;
    let mut u128_11: u128 = 3372u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5() {
    rusty_monitor::set_test_id(5);
    let mut u128_0: u128 = 1699u128;
    let mut u128_1: u128 = 7247u128;
    let mut u128_2: u128 = 710u128;
    let mut u128_3: u128 = 1087u128;
    let mut u128_4: u128 = 6758u128;
    let mut u128_5: u128 = 5935u128;
    let mut u128_6: u128 = 5289u128;
    let mut u128_7: u128 = 397u128;
    let mut u128_8: u128 = 2232u128;
    let mut u128_9: u128 = 9316u128;
    let mut u128_10: u128 = 6383u128;
    let mut u128_11: u128 = 4173u128;
    let mut u128_12: u128 = 83u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6() {
    rusty_monitor::set_test_id(6);
    let mut u128_0: u128 = 7666u128;
    let mut u128_1: u128 = 8864u128;
    let mut u128_2: u128 = 6921u128;
    let mut u128_3: u128 = 5047u128;
    let mut u128_4: u128 = 6921u128;
    let mut u128_5: u128 = 4708u128;
    let mut u128_6: u128 = 8928u128;
    let mut u128_7: u128 = 6443u128;
    let mut u128_8: u128 = 7518u128;
    let mut u128_9: u128 = 3300u128;
    let mut u128_10: u128 = 8243u128;
    let mut u128_11: u128 = 755u128;
    let mut u128_12: u128 = 7757u128;
    let mut u128_13: u128 = 6499u128;
    let mut u128_14: u128 = 6359u128;
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7() {
    rusty_monitor::set_test_id(7);
    let mut u128_0: u128 = 3581u128;
    let mut u128_1: u128 = 9115u128;
    let mut u128_2: u128 = 2944u128;
    let mut u128_3: u128 = 4079u128;
    let mut u128_4: u128 = 8983u128;
    let mut u128_5: u128 = 382u128;
    let mut u128_6: u128 = 3573u128;
    let mut u128_7: u128 = 3760u128;
    let mut u128_8: u128 = 1918u128;
    let mut u128_9: u128 = 4935u128;
    let mut u128_10: u128 = 8083u128;
    let mut u128_11: u128 = 5079u128;
    let mut u128_12: u128 = 1094u128;
    let mut u128_13: u128 = 8556u128;
    let mut u128_14: u128 = 3170u128;
    let mut u128_15: u128 = 905u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_15);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut u128_21: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8() {
    rusty_monitor::set_test_id(8);
    let mut u128_0: u128 = 9347u128;
    let mut u128_1: u128 = 6467u128;
    let mut u128_2: u128 = 1488u128;
    let mut u128_3: u128 = 3730u128;
    let mut u128_4: u128 = 487u128;
    let mut u128_5: u128 = 1298u128;
    let mut u128_6: u128 = 3443u128;
    let mut u128_7: u128 = 3451u128;
    let mut u128_8: u128 = 9510u128;
    let mut u128_9: u128 = 4231u128;
    let mut u128_10: u128 = 8860u128;
    let mut u128_11: u128 = 8538u128;
    let mut u128_12: u128 = 8470u128;
    let mut u128_13: u128 = 4883u128;
    let mut u128_14: u128 = 2181u128;
    let mut u128_15: u128 = 4261u128;
    let mut u128_16: u128 = 4572u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_16);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_15, u128_14);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut u128_21: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut u128_22: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_9() {
    rusty_monitor::set_test_id(9);
    let mut u128_0: u128 = 3708u128;
    let mut u128_1: u128 = 9372u128;
    let mut u128_2: u128 = 4671u128;
    let mut u128_3: u128 = 7706u128;
    let mut u128_4: u128 = 9208u128;
    let mut u128_5: u128 = 5490u128;
    let mut u128_6: u128 = 5000u128;
    let mut u128_7: u128 = 3755u128;
    let mut u128_8: u128 = 5598u128;
    let mut u128_9: u128 = 5407u128;
    let mut u128_10: u128 = 2969u128;
    let mut u128_11: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    let mut buffer_10: crate::Buffer = crate::Buffer::default();
    let mut buffer_11: crate::Buffer = crate::Buffer::default();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_10() {
    rusty_monitor::set_test_id(10);
    let mut u128_0: u128 = 7415u128;
    let mut u128_1: u128 = 4200u128;
    let mut u128_2: u128 = 1644u128;
    let mut u128_3: u128 = 7871u128;
    let mut u128_4: u128 = 2173u128;
    let mut u128_5: u128 = 7450u128;
    let mut u128_6: u128 = 7471u128;
    let mut u128_7: u128 = 4179u128;
    let mut u128_8: u128 = 5696u128;
    let mut u128_9: u128 = 2932u128;
    let mut u128_10: u128 = 9710u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut u128_11: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_11() {
    rusty_monitor::set_test_id(11);
    let mut u128_0: u128 = 7631u128;
    let mut u128_1: u128 = 8558u128;
    let mut u128_2: u128 = 8913u128;
    let mut u128_3: u128 = 7097u128;
    let mut u128_4: u128 = 57u128;
    let mut u128_5: u128 = 6911u128;
    let mut u128_6: u128 = 9526u128;
    let mut u128_7: u128 = 7243u128;
    let mut u128_8: u128 = 3611u128;
    let mut u128_9: u128 = 9626u128;
    let mut u128_10: u128 = 2964u128;
    let mut u128_11: u128 = 2126u128;
    let mut u128_12: u128 = 9369u128;
    let mut u128_13: u128 = 7299u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_12() {
    rusty_monitor::set_test_id(12);
    let mut u128_0: u128 = 729u128;
    let mut u128_1: u128 = 7355u128;
    let mut u128_2: u128 = 2551u128;
    let mut u128_3: u128 = 9200u128;
    let mut u128_4: u128 = 5882u128;
    let mut u128_5: u128 = 8412u128;
    let mut u128_6: u128 = 5794u128;
    let mut u128_7: u128 = 716u128;
    let mut u128_8: u128 = 4955u128;
    let mut u128_9: u128 = 9300u128;
    let mut u128_10: u128 = 2061u128;
    let mut u128_11: u128 = 8395u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_13() {
    rusty_monitor::set_test_id(13);
    let mut u128_0: u128 = 5159u128;
    let mut u128_1: u128 = 8582u128;
    let mut u128_2: u128 = 2227u128;
    let mut u128_3: u128 = 6303u128;
    let mut u128_4: u128 = 9497u128;
    let mut u128_5: u128 = 8030u128;
    let mut u128_6: u128 = 3261u128;
    let mut u128_7: u128 = 7101u128;
    let mut u128_8: u128 = 8853u128;
    let mut u128_9: u128 = 9058u128;
    let mut u128_10: u128 = 435u128;
    let mut u128_11: u128 = 8972u128;
    let mut u128_12: u128 = 8529u128;
    let mut u128_13: u128 = 1810u128;
    let mut u128_14: u128 = 7603u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_14() {
    rusty_monitor::set_test_id(14);
    let mut u128_0: u128 = 9112u128;
    let mut u128_1: u128 = 6375u128;
    let mut u128_2: u128 = 8957u128;
    let mut u128_3: u128 = 5943u128;
    let mut u128_4: u128 = 6097u128;
    let mut u128_5: u128 = 8326u128;
    let mut u128_6: u128 = 7516u128;
    let mut u128_7: u128 = 8641u128;
    let mut u128_8: u128 = 6820u128;
    let mut u128_9: u128 = 9605u128;
    let mut u128_10: u128 = 9505u128;
    let mut u128_11: u128 = 4613u128;
    let mut u128_12: u128 = 1702u128;
    let mut u128_13: u128 = 7652u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_15() {
    rusty_monitor::set_test_id(15);
    let mut u128_0: u128 = 678u128;
    let mut u128_1: u128 = 9515u128;
    let mut u128_2: u128 = 2922u128;
    let mut u128_3: u128 = 6544u128;
    let mut u128_4: u128 = 5400u128;
    let mut u128_5: u128 = 4575u128;
    let mut u128_6: u128 = 5335u128;
    let mut u128_7: u128 = 5474u128;
    let mut u128_8: u128 = 6428u128;
    let mut u128_9: u128 = 5791u128;
    let mut u128_10: u128 = 5603u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut u128_11: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut buffer_10: crate::Buffer = crate::Buffer::default();
    let mut buffer_11: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_16() {
    rusty_monitor::set_test_id(16);
    let mut u128_0: u128 = 6896u128;
    let mut u128_1: u128 = 4422u128;
    let mut u128_2: u128 = 9607u128;
    let mut u128_3: u128 = 879u128;
    let mut u128_4: u128 = 2025u128;
    let mut u128_5: u128 = 6229u128;
    let mut u128_6: u128 = 9376u128;
    let mut u128_7: u128 = 5912u128;
    let mut u128_8: u128 = 6280u128;
    let mut u128_9: u128 = 7166u128;
    let mut u128_10: u128 = 3062u128;
    let mut u128_11: u128 = 4161u128;
    let mut u128_12: u128 = 1070u128;
    let mut u128_13: u128 = 7958u128;
    let mut u128_14: u128 = 8333u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_17() {
    rusty_monitor::set_test_id(17);
    let mut u128_0: u128 = 3287u128;
    let mut u128_1: u128 = 155u128;
    let mut u128_2: u128 = 2755u128;
    let mut u128_3: u128 = 507u128;
    let mut u128_4: u128 = 2429u128;
    let mut u128_5: u128 = 8250u128;
    let mut u128_6: u128 = 8942u128;
    let mut u128_7: u128 = 6473u128;
    let mut u128_8: u128 = 7540u128;
    let mut u128_9: u128 = 7617u128;
    let mut u128_10: u128 = 2329u128;
    let mut u128_11: u128 = 566u128;
    let mut u128_12: u128 = 8579u128;
    let mut u128_13: u128 = 3010u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_18() {
    rusty_monitor::set_test_id(18);
    let mut u128_0: u128 = 7811u128;
    let mut u128_1: u128 = 5307u128;
    let mut u128_2: u128 = 6218u128;
    let mut u128_3: u128 = 4303u128;
    let mut u128_4: u128 = 4484u128;
    let mut u128_5: u128 = 5480u128;
    let mut u128_6: u128 = 6018u128;
    let mut u128_7: u128 = 4834u128;
    let mut u128_8: u128 = 2988u128;
    let mut u128_9: u128 = 7675u128;
    let mut u128_10: u128 = 4495u128;
    let mut u128_11: u128 = 257u128;
    let mut u128_12: u128 = 9618u128;
    let mut u128_13: u128 = 5814u128;
    let mut u128_14: u128 = 5780u128;
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_19() {
    rusty_monitor::set_test_id(19);
    let mut u128_0: u128 = 6766u128;
    let mut u128_1: u128 = 3794u128;
    let mut u128_2: u128 = 4248u128;
    let mut u128_3: u128 = 3071u128;
    let mut u128_4: u128 = 4104u128;
    let mut u128_5: u128 = 4810u128;
    let mut u128_6: u128 = 489u128;
    let mut u128_7: u128 = 592u128;
    let mut u128_8: u128 = 5781u128;
    let mut u128_9: u128 = 2115u128;
    let mut u128_10: u128 = 6527u128;
    let mut u128_11: u128 = 9843u128;
    let mut u128_12: u128 = 3273u128;
    let mut u128_13: u128 = 9348u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_20() {
    rusty_monitor::set_test_id(20);
    let mut u128_0: u128 = 8773u128;
    let mut u128_1: u128 = 4870u128;
    let mut u128_2: u128 = 4163u128;
    let mut u128_3: u128 = 7627u128;
    let mut u128_4: u128 = 170u128;
    let mut u128_5: u128 = 8831u128;
    let mut u128_6: u128 = 2907u128;
    let mut u128_7: u128 = 8148u128;
    let mut u128_8: u128 = 1744u128;
    let mut u128_9: u128 = 611u128;
    let mut u128_10: u128 = 6405u128;
    let mut u128_11: u128 = 3371u128;
    let mut u128_12: u128 = 9233u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_21() {
    rusty_monitor::set_test_id(21);
    let mut u128_0: u128 = 3561u128;
    let mut u128_1: u128 = 4692u128;
    let mut u128_2: u128 = 194u128;
    let mut u128_3: u128 = 9657u128;
    let mut u128_4: u128 = 1130u128;
    let mut u128_5: u128 = 1155u128;
    let mut u128_6: u128 = 8847u128;
    let mut u128_7: u128 = 615u128;
    let mut u128_8: u128 = 7102u128;
    let mut u128_9: u128 = 5843u128;
    let mut u128_10: u128 = 7233u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut u128_11: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_22() {
    rusty_monitor::set_test_id(22);
    let mut u128_0: u128 = 8100u128;
    let mut u128_1: u128 = 4254u128;
    let mut u128_2: u128 = 2455u128;
    let mut u128_3: u128 = 7289u128;
    let mut u128_4: u128 = 9062u128;
    let mut u128_5: u128 = 3452u128;
    let mut u128_6: u128 = 4231u128;
    let mut u128_7: u128 = 5042u128;
    let mut u128_8: u128 = 900u128;
    let mut u128_9: u128 = 396u128;
    let mut u128_10: u128 = 8639u128;
    let mut u128_11: u128 = 1541u128;
    let mut u128_12: u128 = 8504u128;
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut buffer_10: crate::Buffer = crate::Buffer::default();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut buffer_11: crate::Buffer = crate::Buffer::default();
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_23() {
    rusty_monitor::set_test_id(23);
    let mut u128_0: u128 = 8507u128;
    let mut u128_1: u128 = 8565u128;
    let mut u128_2: u128 = 8862u128;
    let mut u128_3: u128 = 5312u128;
    let mut u128_4: u128 = 8044u128;
    let mut u128_5: u128 = 7211u128;
    let mut u128_6: u128 = 147u128;
    let mut u128_7: u128 = 571u128;
    let mut u128_8: u128 = 5947u128;
    let mut u128_9: u128 = 8002u128;
    let mut u128_10: u128 = 6732u128;
    let mut u128_11: u128 = 1458u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_24() {
    rusty_monitor::set_test_id(24);
    let mut u128_0: u128 = 4894u128;
    let mut u128_1: u128 = 5882u128;
    let mut u128_2: u128 = 4695u128;
    let mut u128_3: u128 = 2330u128;
    let mut u128_4: u128 = 4951u128;
    let mut u128_5: u128 = 3547u128;
    let mut u128_6: u128 = 1199u128;
    let mut u128_7: u128 = 4693u128;
    let mut u128_8: u128 = 3146u128;
    let mut u128_9: u128 = 5162u128;
    let mut u128_10: u128 = 2701u128;
    let mut u128_11: u128 = 7919u128;
    let mut u128_12: u128 = 8430u128;
    let mut u128_13: u128 = 5377u128;
    let mut u128_14: u128 = 2858u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut tuple_8: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_25() {
    rusty_monitor::set_test_id(25);
    let mut u128_0: u128 = 2423u128;
    let mut u128_1: u128 = 1531u128;
    let mut u128_2: u128 = 5862u128;
    let mut u128_3: u128 = 5199u128;
    let mut u128_4: u128 = 5509u128;
    let mut u128_5: u128 = 8805u128;
    let mut u128_6: u128 = 9964u128;
    let mut u128_7: u128 = 1845u128;
    let mut u128_8: u128 = 3784u128;
    let mut u128_9: u128 = 8914u128;
    let mut u128_10: u128 = 2018u128;
    let mut u128_11: u128 = 118u128;
    let mut u128_12: u128 = 6524u128;
    let mut u128_13: u128 = 6349u128;
    let mut u128_14: u128 = 1688u128;
    let mut u128_15: u128 = 1485u128;
    let mut u128_16: u128 = 4827u128;
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_16, u128_15);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut u128_21: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut u128_22: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut u128_23: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut u128_24: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_26() {
    rusty_monitor::set_test_id(26);
    let mut u128_0: u128 = 8579u128;
    let mut u128_1: u128 = 3967u128;
    let mut u128_2: u128 = 8147u128;
    let mut u128_3: u128 = 5417u128;
    let mut u128_4: u128 = 3218u128;
    let mut u128_5: u128 = 8333u128;
    let mut u128_6: u128 = 5515u128;
    let mut u128_7: u128 = 5231u128;
    let mut u128_8: u128 = 436u128;
    let mut u128_9: u128 = 3873u128;
    let mut u128_10: u128 = 4934u128;
    let mut u128_11: u128 = 3063u128;
    let mut u128_12: u128 = 1744u128;
    let mut u128_13: u128 = 7995u128;
    let mut u128_14: u128 = 7731u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_27() {
    rusty_monitor::set_test_id(27);
    let mut u128_0: u128 = 4973u128;
    let mut u128_1: u128 = 7754u128;
    let mut u128_2: u128 = 3709u128;
    let mut u128_3: u128 = 6534u128;
    let mut u128_4: u128 = 4118u128;
    let mut u128_5: u128 = 6977u128;
    let mut u128_6: u128 = 4749u128;
    let mut u128_7: u128 = 3136u128;
    let mut u128_8: u128 = 8886u128;
    let mut u128_9: u128 = 5425u128;
    let mut u128_10: u128 = 2366u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut u128_11: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut buffer_11: crate::Buffer = crate::Buffer::default();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_28() {
    rusty_monitor::set_test_id(28);
    let mut u128_0: u128 = 5457u128;
    let mut u128_1: u128 = 7754u128;
    let mut u128_2: u128 = 884u128;
    let mut u128_3: u128 = 2743u128;
    let mut u128_4: u128 = 2056u128;
    let mut u128_5: u128 = 1640u128;
    let mut u128_6: u128 = 9706u128;
    let mut u128_7: u128 = 2692u128;
    let mut u128_8: u128 = 4442u128;
    let mut u128_9: u128 = 8471u128;
    let mut u128_10: u128 = 4848u128;
    let mut u128_11: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut buffer_11: crate::Buffer = crate::Buffer::new();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_29() {
    rusty_monitor::set_test_id(29);
    let mut u128_0: u128 = 9944u128;
    let mut u128_1: u128 = 5073u128;
    let mut u128_2: u128 = 3061u128;
    let mut u128_3: u128 = 5924u128;
    let mut u128_4: u128 = 5510u128;
    let mut u128_5: u128 = 6376u128;
    let mut u128_6: u128 = 3933u128;
    let mut u128_7: u128 = 2089u128;
    let mut u128_8: u128 = 6174u128;
    let mut u128_9: u128 = 6094u128;
    let mut u128_10: u128 = 134u128;
    let mut u128_11: u128 = 8024u128;
    let mut u128_12: u128 = 1027u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_30() {
    rusty_monitor::set_test_id(30);
    let mut u128_0: u128 = 9919u128;
    let mut u128_1: u128 = 2262u128;
    let mut u128_2: u128 = 6005u128;
    let mut u128_3: u128 = 9172u128;
    let mut u128_4: u128 = 3605u128;
    let mut u128_5: u128 = 8042u128;
    let mut u128_6: u128 = 2023u128;
    let mut u128_7: u128 = 4733u128;
    let mut u128_8: u128 = 5499u128;
    let mut u128_9: u128 = 5113u128;
    let mut u128_10: u128 = 8960u128;
    let mut u128_11: u128 = 6713u128;
    let mut u128_12: u128 = 4351u128;
    let mut u128_13: u128 = 2523u128;
    let mut u128_14: u128 = 4693u128;
    let mut u128_15: u128 = 3613u128;
    let mut u128_16: u128 = 3256u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_16, u128_15);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut u128_21: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut u128_22: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut u128_23: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_31() {
    rusty_monitor::set_test_id(31);
    let mut u128_0: u128 = 6519u128;
    let mut u128_1: u128 = 4862u128;
    let mut u128_2: u128 = 9811u128;
    let mut u128_3: u128 = 3288u128;
    let mut u128_4: u128 = 30u128;
    let mut u128_5: u128 = 2305u128;
    let mut u128_6: u128 = 378u128;
    let mut u128_7: u128 = 5333u128;
    let mut u128_8: u128 = 5037u128;
    let mut u128_9: u128 = 8357u128;
    let mut u128_10: u128 = 3990u128;
    let mut u128_11: u128 = 803u128;
    let mut u128_12: u128 = 8219u128;
    let mut u128_13: u128 = 586u128;
    let mut u128_14: u128 = 8084u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_32() {
    rusty_monitor::set_test_id(32);
    let mut u128_0: u128 = 4984u128;
    let mut u128_1: u128 = 6728u128;
    let mut u128_2: u128 = 1973u128;
    let mut u128_3: u128 = 5856u128;
    let mut u128_4: u128 = 8094u128;
    let mut u128_5: u128 = 6292u128;
    let mut u128_6: u128 = 3425u128;
    let mut u128_7: u128 = 3223u128;
    let mut u128_8: u128 = 7707u128;
    let mut u128_9: u128 = 8105u128;
    let mut u128_10: u128 = 2497u128;
    let mut u128_11: u128 = 3177u128;
    let mut u128_12: u128 = 5244u128;
    let mut u128_13: u128 = 6965u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_33() {
    rusty_monitor::set_test_id(33);
    let mut u128_0: u128 = 4002u128;
    let mut u128_1: u128 = 3871u128;
    let mut u128_2: u128 = 8202u128;
    let mut u128_3: u128 = 584u128;
    let mut u128_4: u128 = 6576u128;
    let mut u128_5: u128 = 103u128;
    let mut u128_6: u128 = 7948u128;
    let mut u128_7: u128 = 1076u128;
    let mut u128_8: u128 = 5998u128;
    let mut u128_9: u128 = 9000u128;
    let mut u128_10: u128 = 290u128;
    let mut u128_11: u128 = 9842u128;
    let mut u128_12: u128 = 2854u128;
    let mut u128_13: u128 = 2604u128;
    let mut u128_14: u128 = 715u128;
    let mut u128_15: u128 = 9387u128;
    let mut u128_16: u128 = 699u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_16);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_15, u128_14);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut u128_21: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut u128_22: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut u128_23: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_34() {
    rusty_monitor::set_test_id(34);
    let mut u128_0: u128 = 6475u128;
    let mut u128_1: u128 = 995u128;
    let mut u128_2: u128 = 1619u128;
    let mut u128_3: u128 = 4704u128;
    let mut u128_4: u128 = 9544u128;
    let mut u128_5: u128 = 5278u128;
    let mut u128_6: u128 = 2765u128;
    let mut u128_7: u128 = 6545u128;
    let mut u128_8: u128 = 1479u128;
    let mut u128_9: u128 = 7013u128;
    let mut u128_10: u128 = 5148u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut u128_11: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_35() {
    rusty_monitor::set_test_id(35);
    let mut u128_0: u128 = 3156u128;
    let mut u128_1: u128 = 2384u128;
    let mut u128_2: u128 = 2058u128;
    let mut u128_3: u128 = 6700u128;
    let mut u128_4: u128 = 3820u128;
    let mut u128_5: u128 = 8191u128;
    let mut u128_6: u128 = 3055u128;
    let mut u128_7: u128 = 2054u128;
    let mut u128_8: u128 = 7513u128;
    let mut u128_9: u128 = 8524u128;
    let mut u128_10: u128 = 2180u128;
    let mut u128_11: u128 = 7036u128;
    let mut u128_12: u128 = 2386u128;
    let mut u128_13: u128 = 9140u128;
    let mut u128_14: u128 = 4873u128;
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_36() {
    rusty_monitor::set_test_id(36);
    let mut u128_0: u128 = 9768u128;
    let mut u128_1: u128 = 809u128;
    let mut u128_2: u128 = 6293u128;
    let mut u128_3: u128 = 306u128;
    let mut u128_4: u128 = 9223u128;
    let mut u128_5: u128 = 2474u128;
    let mut u128_6: u128 = 2830u128;
    let mut u128_7: u128 = 9078u128;
    let mut u128_8: u128 = 9746u128;
    let mut u128_9: u128 = 5070u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut u128_10: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut buffer_10: crate::Buffer = crate::Buffer::default();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_37() {
    rusty_monitor::set_test_id(37);
    let mut u128_0: u128 = 1040u128;
    let mut u128_1: u128 = 3438u128;
    let mut u128_2: u128 = 6001u128;
    let mut u128_3: u128 = 38u128;
    let mut u128_4: u128 = 5106u128;
    let mut u128_5: u128 = 9418u128;
    let mut u128_6: u128 = 1100u128;
    let mut u128_7: u128 = 3542u128;
    let mut u128_8: u128 = 9888u128;
    let mut u128_9: u128 = 5753u128;
    let mut u128_10: u128 = 8061u128;
    let mut u128_11: u128 = 619u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_38() {
    rusty_monitor::set_test_id(38);
    let mut u128_0: u128 = 5489u128;
    let mut u128_1: u128 = 6615u128;
    let mut u128_2: u128 = 904u128;
    let mut u128_3: u128 = 9584u128;
    let mut u128_4: u128 = 1431u128;
    let mut u128_5: u128 = 3964u128;
    let mut u128_6: u128 = 908u128;
    let mut u128_7: u128 = 1100u128;
    let mut u128_8: u128 = 8697u128;
    let mut u128_9: u128 = 6828u128;
    let mut u128_10: u128 = 3792u128;
    let mut u128_11: u128 = 3351u128;
    let mut u128_12: u128 = 588u128;
    let mut u128_13: u128 = 8757u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_39() {
    rusty_monitor::set_test_id(39);
    let mut u128_0: u128 = 6305u128;
    let mut u128_1: u128 = 8578u128;
    let mut u128_2: u128 = 7294u128;
    let mut u128_3: u128 = 2682u128;
    let mut u128_4: u128 = 6414u128;
    let mut u128_5: u128 = 9386u128;
    let mut u128_6: u128 = 3018u128;
    let mut u128_7: u128 = 2331u128;
    let mut u128_8: u128 = 7303u128;
    let mut u128_9: u128 = 7543u128;
    let mut u128_10: u128 = 4661u128;
    let mut u128_11: u128 = 6658u128;
    let mut u128_12: u128 = 8631u128;
    let mut u128_13: u128 = 8324u128;
    let mut u128_14: u128 = 3754u128;
    let mut u128_15: u128 = 7863u128;
    let mut u128_16: u128 = 8459u128;
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_16, u128_15);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut u128_21: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut u128_22: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut u128_23: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_40() {
    rusty_monitor::set_test_id(40);
    let mut u128_0: u128 = 4318u128;
    let mut u128_1: u128 = 1835u128;
    let mut u128_2: u128 = 1869u128;
    let mut u128_3: u128 = 3166u128;
    let mut u128_4: u128 = 9315u128;
    let mut u128_5: u128 = 1292u128;
    let mut u128_6: u128 = 5275u128;
    let mut u128_7: u128 = 1937u128;
    let mut u128_8: u128 = 8831u128;
    let mut u128_9: u128 = 8274u128;
    let mut u128_10: u128 = 4512u128;
    let mut u128_11: u128 = 2815u128;
    let mut u128_12: u128 = 9876u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_41() {
    rusty_monitor::set_test_id(41);
    let mut u128_0: u128 = 7787u128;
    let mut u128_1: u128 = 380u128;
    let mut u128_2: u128 = 8732u128;
    let mut u128_3: u128 = 4971u128;
    let mut u128_4: u128 = 9705u128;
    let mut u128_5: u128 = 1333u128;
    let mut u128_6: u128 = 197u128;
    let mut u128_7: u128 = 6092u128;
    let mut u128_8: u128 = 2044u128;
    let mut u128_9: u128 = 181u128;
    let mut u128_10: u128 = 9988u128;
    let mut u128_11: u128 = 9373u128;
    let mut u128_12: u128 = 2053u128;
    let mut u128_13: u128 = 569u128;
    let mut u128_14: u128 = 9678u128;
    let mut u128_15: u128 = 666u128;
    let mut u128_16: u128 = 4030u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_16);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_15);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut tuple_8: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_42() {
    rusty_monitor::set_test_id(42);
    let mut u128_0: u128 = 5132u128;
    let mut u128_1: u128 = 1606u128;
    let mut u128_2: u128 = 1891u128;
    let mut u128_3: u128 = 8151u128;
    let mut u128_4: u128 = 667u128;
    let mut u128_5: u128 = 1751u128;
    let mut u128_6: u128 = 3674u128;
    let mut u128_7: u128 = 8043u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut u128_8: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut u128_9: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut buffer_11: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_12: crate::Buffer = crate::Buffer::new();
    let mut buffer_13: crate::Buffer = crate::Buffer::new();
    let mut buffer_14: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    let mut buffer_15: crate::Buffer = crate::Buffer::default();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_43() {
    rusty_monitor::set_test_id(43);
    let mut u128_0: u128 = 1149u128;
    let mut u128_1: u128 = 2930u128;
    let mut u128_2: u128 = 4102u128;
    let mut u128_3: u128 = 1511u128;
    let mut u128_4: u128 = 9147u128;
    let mut u128_5: u128 = 3936u128;
    let mut u128_6: u128 = 2980u128;
    let mut u128_7: u128 = 5617u128;
    let mut u128_8: u128 = 3442u128;
    let mut u128_9: u128 = 8545u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut u128_10: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut u128_11: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut buffer_11: crate::Buffer = crate::Buffer::default();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_12: crate::Buffer = crate::Buffer::new();
    let mut buffer_13: crate::Buffer = crate::Buffer::new();
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_44() {
    rusty_monitor::set_test_id(44);
    let mut u128_0: u128 = 7144u128;
    let mut u128_1: u128 = 9581u128;
    let mut u128_2: u128 = 1468u128;
    let mut u128_3: u128 = 8929u128;
    let mut u128_4: u128 = 5544u128;
    let mut u128_5: u128 = 2537u128;
    let mut u128_6: u128 = 889u128;
    let mut u128_7: u128 = 9740u128;
    let mut u128_8: u128 = 533u128;
    let mut u128_9: u128 = 2727u128;
    let mut u128_10: u128 = 4612u128;
    let mut u128_11: u128 = 3599u128;
    let mut u128_12: u128 = 6259u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_45() {
    rusty_monitor::set_test_id(45);
    let mut u128_0: u128 = 1648u128;
    let mut u128_1: u128 = 4680u128;
    let mut u128_2: u128 = 8176u128;
    let mut u128_3: u128 = 4595u128;
    let mut u128_4: u128 = 4709u128;
    let mut u128_5: u128 = 1930u128;
    let mut u128_6: u128 = 601u128;
    let mut u128_7: u128 = 3326u128;
    let mut u128_8: u128 = 8428u128;
    let mut u128_9: u128 = 6899u128;
    let mut u128_10: u128 = 2372u128;
    let mut u128_11: u128 = 9145u128;
    let mut u128_12: u128 = 8759u128;
    let mut u128_13: u128 = 3054u128;
    let mut u128_14: u128 = 6880u128;
    let mut u128_15: u128 = 9136u128;
    let mut u128_16: u128 = 6866u128;
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_16, u128_15);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut u128_21: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut u128_22: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_46() {
    rusty_monitor::set_test_id(46);
    let mut u128_0: u128 = 5297u128;
    let mut u128_1: u128 = 6963u128;
    let mut u128_2: u128 = 1950u128;
    let mut u128_3: u128 = 6776u128;
    let mut u128_4: u128 = 44u128;
    let mut u128_5: u128 = 3443u128;
    let mut u128_6: u128 = 9227u128;
    let mut u128_7: u128 = 7218u128;
    let mut u128_8: u128 = 3328u128;
    let mut u128_9: u128 = 514u128;
    let mut u128_10: u128 = 1163u128;
    let mut u128_11: u128 = 3643u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut buffer_10: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_11: crate::Buffer = crate::Buffer::new();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_47() {
    rusty_monitor::set_test_id(47);
    let mut u128_0: u128 = 6035u128;
    let mut u128_1: u128 = 350u128;
    let mut u128_2: u128 = 3976u128;
    let mut u128_3: u128 = 4488u128;
    let mut u128_4: u128 = 1587u128;
    let mut u128_5: u128 = 678u128;
    let mut u128_6: u128 = 2022u128;
    let mut u128_7: u128 = 4125u128;
    let mut u128_8: u128 = 4353u128;
    let mut u128_9: u128 = 2974u128;
    let mut u128_10: u128 = 4986u128;
    let mut u128_11: u128 = 3073u128;
    let mut u128_12: u128 = 7407u128;
    let mut u128_13: u128 = 9394u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_48() {
    rusty_monitor::set_test_id(48);
    let mut u128_0: u128 = 7350u128;
    let mut u128_1: u128 = 6794u128;
    let mut u128_2: u128 = 5206u128;
    let mut u128_3: u128 = 3128u128;
    let mut u128_4: u128 = 7400u128;
    let mut u128_5: u128 = 1831u128;
    let mut u128_6: u128 = 1905u128;
    let mut u128_7: u128 = 6048u128;
    let mut u128_8: u128 = 2511u128;
    let mut u128_9: u128 = 3443u128;
    let mut u128_10: u128 = 2913u128;
    let mut u128_11: u128 = 3464u128;
    let mut u128_12: u128 = 8444u128;
    let mut u128_13: u128 = 3969u128;
    let mut u128_14: u128 = 8652u128;
    let mut u128_15: u128 = 6334u128;
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_15, u128_14);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut u128_21: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_49() {
    rusty_monitor::set_test_id(49);
    let mut u128_0: u128 = 3397u128;
    let mut u128_1: u128 = 7974u128;
    let mut u128_2: u128 = 8687u128;
    let mut u128_3: u128 = 9360u128;
    let mut u128_4: u128 = 7436u128;
    let mut u128_5: u128 = 8156u128;
    let mut u128_6: u128 = 3290u128;
    let mut u128_7: u128 = 9431u128;
    let mut u128_8: u128 = 9362u128;
    let mut u128_9: u128 = 7373u128;
    let mut u128_10: u128 = 3639u128;
    let mut u128_11: u128 = 194u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}
}