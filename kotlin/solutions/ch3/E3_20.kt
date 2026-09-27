// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.20

package sicp.ch3.exercises

import sicp.runtime.Value

/**
 * The procedural pair of 3.3.1: four operations over the two slots
 * captured by `proceduralCons`.
 */
public interface PairProc {
    public fun car(): Value

    public fun cdr(): Value

    public fun setCar(v: Value)

    public fun setCdr(v: Value)
}

/**
 * The book's mutable `cons` as a closure: two captured `var` slots, read
 * by the selectors and written by the mutators, exactly the way the bank
 * account of 3.1.1 kept its balance in a captured local.
 */
public fun proceduralCons(
    x: Value,
    y: Value,
): PairProc {
    var carCell = x
    var cdrCell = y
    return object : PairProc {
        override fun car(): Value = carCell

        override fun cdr(): Value = cdrCell

        override fun setCar(v: Value) {
            carCell = v
        }

        override fun setCdr(v: Value) {
            cdrCell = v
        }
    }
}
