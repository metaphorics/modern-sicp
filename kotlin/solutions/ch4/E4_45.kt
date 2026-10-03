// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.45

package sicp.ch4.solutions

// Exercise 4.45: the five parses of an ambiguous sentence. The section's
// grammar is word lists and search: `parseWord` consumes the input from a
// mutable position that unwinds with each backtrack, `maybe-extend` is a
// choice between stopping and extending with a prepositional phrase, and
// every complete parse that spends the whole input is an answer. The
// sentence "the professor lectures to the student in the class with the
// cat" has five.

/** The section's word lists, as guest source. */
internal val PARSER_WORDS_SOURCE: String =
    """
var unparsed: List<String> = emptyList()

val nouns: List<String> = listOf("student", "professor", "cat", "class")

val verbs: List<String> = listOf("studies", "lectures", "eats", "sleeps")

val articles: List<String> = listOf("the", "a")

val prepositions: List<String> = listOf("for", "to", "in", "by", "with")
    """.trimIndent()

/** The consuming `parseWord` of the parser. */
internal val PARSE_WORD_CONSUMING_SOURCE: String =
    """
fun parseWord(kind: String, words: List<String>): String {
    requireThat(unparsed.size > 0)
    val head = unparsed.get(0)
    requireThat(containsText(words, head))
    unparsed = unparsed.drop(1)
    return "(" + kind + " " + head + ")"
}
    """.trimIndent()

/** The phrase machinery of the section's grammar. */
internal val PARSER_PHRASES_SOURCE: String =
    """
fun parseSimpleNounPhrase(): String =
    "(simple-noun-phrase " + parseWord("article", articles) + " " + parseWord("noun", nouns) + ")"

fun parsePrepositionalPhrase(): String =
    "(prep-phrase " + parseWord("prep", prepositions) + " " + parseNounPhrase() + ")"

fun maybeExtendNoun(nounPhrase: String): String =
    choose(nounPhrase, maybeExtendNoun("(noun-phrase " + nounPhrase + " " + parsePrepositionalPhrase() + ")"))

fun parseNounPhrase(): String = maybeExtendNoun(parseSimpleNounPhrase())

fun maybeExtendVerb(verbPhrase: String): String =
    choose(verbPhrase, maybeExtendVerb("(verb-phrase " + verbPhrase + " " + parsePrepositionalPhrase() + ")"))

fun parseVerbPhrase(): String = maybeExtendVerb(parseWord("verb", verbs))

fun parseSentence(): String = "(sentence " + parseNounPhrase() + " " + parseVerbPhrase() + ")"

fun parseInput(input: List<String>): String {
    unparsed = input
    val sent = parseSentence()
    requireThat(unparsed.size == 0)
    return sent
}
    """.trimIndent()

/** The section's natural-language parser as guest source. */
internal val PARSER_SOURCE: String =
    AMB_BASE_PRELUDE + "\n" + PARSER_WORDS_SOURCE + "\n" + PARSE_WORD_CONSUMING_SOURCE + "\n" + PARSER_PHRASES_SOURCE

/** The ambiguous sentence's input words. */
internal val AMBIGUOUS_INPUT: String =
    """
fun main() {
    budgetCap = 1000000L
    val input = listOf("the", "professor", "lectures", "to", "the", "student", "in", "the", "class", "with", "the", "cat")
    println(parseInput(input))
}
    """.trimIndent()

/** Exactly five parses, then exhaustion. parses[0] and parses[4] are the
 * extreme groupings of the prepositional phrases. */
public fun ambiguousParses(): List<String> = searchLines(PARSER_SOURCE + "\n" + AMBIGUOUS_INPUT)
