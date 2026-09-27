// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.66: Cy's accumulation scheme and Ben's salvage.

package sicp.ch4.solutions

import sicp.ch4.QuerySystem

private fun amounts(
    system: QuerySystem,
    query: String,
): List<Long> {
    val salaries = answersOf(system, query)
    val digits = Regex("(\\d+)[)\\s]*$")
    return salaries.map { line ->
        digits
            .find(line)
            ?.groupValues
            ?.get(1)
            ?.toLong()
            ?: throw AssertionError("no salary in $line")
    }
}

/** The generalization: accumulation-function fed the values of
 * ?variable. On the wheel query the raw frames pay Warbucks four times;
 * the salvage keeps distinct answers only, the true payroll. */
public fun salarySums(system: QuerySystem): List<String> {
    val out = mutableListOf<String>()
    val programmers =
        amounts(system, "(and (job ?person (computer programmer)) (salary ?person ?amount))").sum()
    out.add("sum over the book's query = $programmers")
    val wheelRaw =
        amounts(system, "(and (wheel ?who) (salary ?who ?amount))").sum()
    out.add("Ben's scheme on the wheel query = $wheelRaw (duplicate frames count)")
    val wheelDistinct =
        amounts(system, "(and (wheel ?who) (salary ?who ?amount))").distinct().sum()
    out.add("salvage, distinct answers only = $wheelDistinct")
    return out
}

public fun salarySums(): List<String> = salarySums(microshaftSystem())
