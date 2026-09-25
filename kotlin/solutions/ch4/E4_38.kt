// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.38

package sicp.ch4.solutions

import sicp.ch4.AmbEvaluator

/** The modified puzzle's solutions in the search's own order: five of
 * them, with the book's answer fourth. */
public fun modifiedDwellingSolutions(): List<String> =
    answerLinesFaulted(
        ::AmbEvaluator,
        "$AMB_BASE_PRELUDE\n$DWELLING_MODIFIED_PROGRAM",
        "(multiple-dwelling-modified)",
    )

/** An independent enumeration over the same assignment space with the
 * same two remaining restrictions -- the implementation-vs-brute-force
 * cross-check the count rests on. */
public fun modifiedDwellingBruteForce(): List<String> {
    val solutions = mutableListOf<String>()
    for (baker in 1..5) {
        for (cooper in 1..5) {
            for (fletcher in 1..5) {
                for (miller in 1..5) {
                    for (smith in 1..5) {
                        val floors = listOf(baker, cooper, fletcher, miller, smith)
                        if (floors.toSet().size != 5) {
                            continue
                        }
                        if (baker == 5 || cooper == 1 || fletcher == 5 || fletcher == 1) {
                            continue
                        }
                        if (miller <= cooper || kotlin.math.abs(fletcher - cooper) == 1) {
                            continue
                        }
                        solutions.add(
                            "((baker $baker) (cooper $cooper) (fletcher $fletcher) " +
                                "(miller $miller) (smith $smith))",
                        )
                    }
                }
            }
        }
    }
    return solutions
}
