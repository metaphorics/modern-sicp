// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.15

package sicp.ch2.exercises

// makeInterval is exercise 2.7's; percent is exercise 2.12's; par1 and par2 are exercise 2.14's.

public fun ex_2_15(): Pair<Double, Double> {
    val r1 = makeInterval(6.12, 7.48)
    val r2 = makeInterval(4.465, 4.935)
    return percent(par1(r1, r2)) to percent(par2(r1, r2))
}
