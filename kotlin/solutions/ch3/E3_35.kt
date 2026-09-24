// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.35

package sicp.ch3.exercises

import kotlin.math.round
import kotlin.math.sqrt

/**
 * Exercise 3.35: Ben's squarer as a primitive constraint. The object
 * expression plays the book's `me`, its two methods are
 * `process-new-value` and `process-forget-value`, and the factory wires
 * the constraint to both connectors and returns it.
 */
public fun squarer(
    a: Connector,
    b: Connector,
): Constraint {
    val me =
        object : Constraint {
            override fun newValue() {
                // the book's process-new-value
                val v = b.value()
                if (v == null) {
                    val r = a.value() ?: return
                    b.setValue(r * r, this)
                    return
                }
                if (v < 0) {
                    error("square less than 0: SQUARER $v")
                }
                a.setValue(round(sqrt(v.toDouble())).toLong(), this)
            }

            override fun forgetValue() {
                // the book's process-forget-value
                a.forgetValue(this)
                b.forgetValue(this)
                newValue()
            }
        }
    a.connect(me)
    b.connect(me)
    return me
}
