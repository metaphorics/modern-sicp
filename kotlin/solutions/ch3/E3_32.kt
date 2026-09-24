// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.32

package sicp.ch3.exercises

/**
 * Exercise 3.32's pinned order: two actions added to one agenda segment
 * at the same time, then one `propagate`. The segment's `ActionQueue`
 * is first in, first out, so the actions run in insertion order and the
 * answer is `["first", "second"]`.
 */
public fun sameSegmentRunOrder(): List<String> {
    val sim = Simulation()
    val ran = mutableListOf<String>()
    sim.agenda.addToAgenda(5) { ran.add("first") }
    sim.agenda.addToAgenda(5) { ran.add("second") }
    sim.propagate()
    return ran
}
