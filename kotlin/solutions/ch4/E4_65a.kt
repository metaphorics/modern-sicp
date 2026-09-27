// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.65a: the deduplicated wheel listing, produced by
// the same wheel rule.

package sicp.ch4.solutions

import sicp.ch4.QuerySystem

/** The deduplication rule: run the rule once, key each answer frame on
 * the query's bound variables -- here the single ?person binding, read
 * off the instantiated answer -- and keep the first occurrence of each
 * key. Insertion order is the rule's own answer order, so the
 * deduplicated listing is Ben then Warbucks. */
public fun deduplicatedWheel(system: QuerySystem): List<String> {
    val answers = answersOf(system, "(wheel ?who)")
    val seen = LinkedHashSet<String>()
    for (answer in answers) {
        seen.add(answer)
    }
    return seen.toList()
}

public fun deduplicatedWheel(): List<String> = deduplicatedWheel(microshaftSystem())
