// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.65: the wheel rule's fourfold listing.

package sicp.ch4.solutions

import sicp.ch4.QuerySystem

/** Cy D. Fect's query under the chronological data base: Ben once, then
 * Warbucks four times. The four Warbucks frames are the four deduction
 * routes: middle-manager Ben (3 supervisees) and middle-manager Scrooge
 * (0 supervisees beyond Cratchet? -- exactly: Scrooge's supervisee is
 * Cratchet, Aull's is none), each route a distinct (middle-manager, x)
 * frame the rule body passes. Ben's single frame comes from
 * middle-manager Alyssa with supervisee Louis. */
public fun wheelQuery(system: QuerySystem): List<String> {
    val out = mutableListOf<String>()
    out.add("query: (wheel ?who)")
    val answers = answersOf(system, "(wheel ?who)")
    out.addAll(answers)
    out.add("Warbucks appears ${answers.count { it.contains("(Warbucks Oliver)") }} times")
    out.add("Ben appears ${answers.count { it.contains("(Bitdiddle Ben)") }} time")
    return out
}

public fun wheelQuery(): List<String> = wheelQuery(microshaftSystem())
