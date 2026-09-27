// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.63: the Genesis genealogy rules.

package sicp.ch4.solutions

import sicp.ch4.QuerySystem

private const val GENESIS =
    """
    (assert! (son Adam Cain))
    (assert! (son Cain Enoch))
    (assert! (son Enoch Irad))
    (assert! (son Irad Mehujael))
    (assert! (son Mehujael Methushael))
    (assert! (son Methushael Lamech))
    (assert! (wife Lamech Ada))
    (assert! (son Ada Jabal))
    (assert! (son Ada Jubal))
    (assert! (rule (grandson ?g ?s)
                   (and (son ?f ?s) (son ?g ?f))))
    (assert! (rule (son ?m ?s)
                   (and (wife ?m ?w) (son ?w ?s))))
    """

public fun genesisSystem(): QuerySystem {
    val system = QuerySystem()
    system.load(GENESIS)
    return system
}

/** The grandson rule chains two son facts; the wife rule moves a son
 * from the wife to the husband, so the sons of Lamech include the ones
 * the data base already asserts -- the deduction re-derives them. */
public fun genesisQueries(system: QuerySystem): List<String> {
    val out = mutableListOf<String>()
    for (
    query in
    listOf(
        "(grandson Cain ?x)",
        "(son Lamech ?x)",
        "(grandson ?x Methushael)",
    )
    ) {
        out.add("query: $query")
        out.addAll(answersUpto(system, query, 6))
    }
    return out
}

public fun genesisQueries(): List<String> = genesisQueries(genesisSystem())
