// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.50

package sicp.ch4.solutions

import sicp.ch4.SearchModule

// Exercise 4.50: `ramb`, random choice. The search experiment's
// `chooseRandom` permutes its alternatives by the program's explicit
// seeded stream and is reproducible from its seed, so the enumeration of
// 1 to 5 under seed 20260925 is a fixed order and the sentence generator
// escapes the plain generator's boring first words. The pins are the
// experiment's own permutation, derived from its shuffle; the removed
// engine's ramb drew from a different generator and its orders are
// provenance in the rationale.

/** The seeded enumeration of five alternatives. */
internal val RAMB_ENUM_PROGRAM: String =
    AMB_BASE_PRELUDE + "\n" +
        """
fun main() {
    budgetCap = 1000000L
    seededRandom(20260925L)
    val x = chooseRandom(1L, 2L, 3L, 4L, 5L)
    println("(" + showLong(x) + ")")
}
        """.trimIndent()

/** The random word pickers of the exercise. */
internal val RAMB_PICKERS_SOURCE: String =
    """
fun rambPick(kind: String): String =
    if (kind == "article") {
        chooseRandom("the", "a")
    } else if (kind == "noun") {
        chooseRandom("student", "professor", "cat", "class")
    } else if (kind == "verb") {
        chooseRandom("studies", "lectures", "eats", "sleeps")
    } else {
        chooseRandom("for", "to", "in", "by", "with")
    }

fun parseWord(kind: String, words: List<String>): String = "(" + kind + " " + rambPick(kind) + ")"
    """.trimIndent()

/** The generator with random word choice. */
internal val RAMB_GENERATOR_PROGRAM: String =
    AMB_BASE_PRELUDE + "\n" + PARSER_WORDS_SOURCE + "\n" + RAMB_PICKERS_SOURCE + "\n" + PARSER_PHRASES_SOURCE

/** The seeded shuffle enumerates 4, 2, 3, 5, 1.
 * => [(4), (2), (3), (5), (1)] */
public fun rambEnumeration(): List<String> = searchLines(RAMB_ENUM_PROGRAM)

/** The ramb generator escapes the boring first words.
 * => "(sentence (simple-noun-phrase (article a) (noun class)) (verb lectures))" */
public fun rambGeneratedFirst(): String {
    val source =
        RAMB_GENERATOR_PROGRAM + "\n" +
            """
fun main() {
    budgetCap = 1000000L
    seededRandom(20260925L)
    println(parseSentence())
}
            """.trimIndent()
    return SearchModule.run(source, 1).fold(
        { error -> throw AssertionError(error.toString()) },
        { run ->
            check(run.result.error == null) { run.result.error.toString() }
            run.result.output
                .lines()
                .filter { line -> line.isNotEmpty() }
                .first()
        },
    )
}
