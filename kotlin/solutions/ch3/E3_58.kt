// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.58

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.consStream

/**
 * Exercise 3.58: give an interpretation of the stream computed by
 * expand. Each step emits the quotient of num * radix by den and
 * continues with the remainder, which is long division: the elements
 * are the digits of num / den written in [radix]. So expanding 1 by 7
 * at radix 10 produces the decimal digits of 1/7, and expanding 3 by 8
 * produces 3, 7, 5 and then zeros once the division comes out exact.
 */
public fun expand(
    num: Long,
    den: Long,
    radix: Long,
): LStream<Long> = consStream(num * radix / den) { expand(num * radix % den, den, radix) }
