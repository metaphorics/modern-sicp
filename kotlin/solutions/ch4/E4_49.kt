// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.49

package sicp.ch4.solutions

import sicp.ch4.SearchModule

// Exercise 4.49: sentences by generation. The generator is the parser
// with one change -- `parseWord` draws a word from its list instead of
// consuming input -- so the same grammar now enumerates sentences. The
// first sentences "bore their way down one recursion": the choices stop
// extending at the first opportunity, so the first answer is the smallest
// sentence and the next one grows exactly one extension.

/** The generating `parseWord` of the exercise. */
internal val PARSE_WORD_GENERATING_SOURCE: String =
    """
fun parseWord(kind: String, words: List<String>): String = "(" + kind + " " + anElementOfString(words) + ")"
    """.trimIndent()

/** The generator program: the section's grammar over drawn words. */
internal val GENERATOR_PROGRAM: String =
    AMB_BASE_PRELUDE + "\n" + PARSER_WORDS_SOURCE + "\n" + PARSE_WORD_GENERATING_SOURCE + "\n" + PARSER_PHRASES_SOURCE

/** The first six generated sentences. sentences[0] is the smallest
 * sentence; sentences[1] grows one extension.
 * => [(sentence (simple-noun-phrase (article the) (noun student)) (verb studies)),
 * (sentence (simple-noun-phrase (article the) (noun student)) (verb-phrase
 * (verb studies) (prep-phrase (prep for) (simple-noun-phrase (article the)
 * (noun student)))))] */
public fun generatedSentences(): List<String> {
    val source =
        GENERATOR_PROGRAM + "\n" +
            """
fun main() {
    budgetCap = 1000000L
    println(parseSentence())
}
            """.trimIndent()
    return SearchModule.run(source, 6).fold(
        { error -> throw AssertionError(error.toString()) },
        { run ->
            check(run.result.error == null) { run.result.error.toString() }
            run.result.output
                .lines()
                .filter { line -> line.isNotEmpty() }
        },
    )
}
