// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.47

package sicp.ch4.solutions

import sicp.ch4.SearchModule
import sicp.ch4.SearchRun

// Exercise 4.47: Louis Reasoner's `parse-verb-phrase`. Louis puts the
// recursion inside the first alternative's construction, so the search
// tries to build the extension before it has a verb phrase to extend.
// Its first parse still arrives, but the next answers recurse without
// consuming input; interchanging the alternatives recurses even before
// that first parse. The host sets a choice horizon rather than forcing
// an unbounded run or fabricating a guest budget line.

/** The section's parser with Louis's verb phrase. */
internal val LOUIS_PARSER_SOURCE: String =
    PARSER_SOURCE +
        """

fun louisVerbPhrase(): String =
    choose(parseWord("verb", verbs), "(verb-phrase " + louisVerbPhrase() + " " + parsePrepositionalPhrase() + ")")
        """.trimIndent()

/** The interchanged order: the recursion is the first alternative. */
internal val LOUIS_INTERCHANGED_SOURCE: String =
    PARSER_SOURCE +
        """

fun interchangedVerbPhrase(): String =
    choose("(verb-phrase " + interchangedVerbPhrase() + " " + parsePrepositionalPhrase() + ")", parseWord("verb", verbs))
        """.trimIndent()

/** Louis's session: the text's input, one complete parse, then the dive. */
internal val LOUIS_PROBE: String =
    """
fun main() {
    unparsed = listOf("the", "cat", "eats")
    val noun = parseNounPhrase()
    val verb = louisVerbPhrase()
    requireThat(unparsed.size == 0)
    println("(sentence " + noun + " " + verb + ")")
}
    """.trimIndent()

/** The interchanged session: the dive comes before any parse. */
internal val INTERCHANGED_PROBE: String =
    """
fun main() {
    unparsed = listOf("the", "cat", "eats")
    val noun = parseNounPhrase()
    val verb = interchangedVerbPhrase()
    requireThat(unparsed.size == 0)
    println("(sentence " + noun + " " + verb + ")")
}
    """.trimIndent()

/** Louis's first parse arrives within the 500-choice horizon. */
public fun louisFirstParse(): String =
    boundedParse(LOUIS_PARSER_SOURCE + "\n" + LOUIS_PROBE, 1, 500)
        .result.output
        .lineSequence()
        .first { it.isNotEmpty() }

/** A further search after Louis's first parse reaches the host horizon. */
public fun louisTryAgainFault(): String = horizon(boundedParse(LOUIS_PARSER_SOURCE + "\n" + LOUIS_PROBE, 2, 500))

/** Recursion-first order reaches the host horizon without a parse. */
public fun interchangedFault(): String = horizon(boundedParse(LOUIS_INTERCHANGED_SOURCE + "\n" + INTERCHANGED_PROBE, 1, 300))

private fun boundedParse(
    source: String,
    maxAnswers: Int,
    maxChoices: Int,
): SearchRun =
    SearchModule.run(source, maxAnswers, maxChoices).fold(
        { error -> throw AssertionError(error.toString()) },
        { run ->
            check(run.result.error == null) { run.result.error.toString() }
            run
        },
    )

private fun horizon(run: SearchRun): String {
    val answers =
        run.result.output
            .lineSequence()
            .count { it.isNotEmpty() }
    return "answers: " + answers + ", choices: " + run.choices
}
