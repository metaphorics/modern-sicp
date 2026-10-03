// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.48

package sicp.ch4.solutions

// Exercise 4.48: extend the grammar. The noun phrase grows an adjective
// slot: `adjectives` chooses zero or more adjective words, the empty
// choice keeps plain sentences parsing, and the phrase renders its
// modifier list in the answer. Two adjectives attach to "the quick brown
// cat sleeps"; "the cat sleeps" keeps its empty modifier list.

/** The adjective extension of the section's parser. */
internal val ADJECTIVE_PARSER_SOURCE: String =
    PARSER_SOURCE +
        """

val adjectiveWords: List<String> = listOf("quick", "brown", "sleepy")

fun adjectives(): List<String> {
    val more = choose(false, true)
    if (!more) {
        return emptyList()
    }
    val word = parseWord("adjective", adjectiveWords)
    val rest = adjectives()
    return listOf(word) + rest
}

fun renderWords(items: List<String>): String {
    var out = "("
    var index = 0
    while (index < items.size) {
        if (index > 0) {
            out = out + " "
        }
        out = out + items.get(index)
        index = index + 1
    }
    return out + ")"
}

fun parseNounPhraseExtended(): String =
    "(noun-phrase " + parseWord("article", articles) + " " + renderWords(adjectives()) + " " + parseWord("noun", nouns) + ")"

fun parseSentenceExtended(): String = "(sentence " + parseNounPhraseExtended() + " " + parseVerbPhrase() + ")"

fun parseExtended(input: List<String>): String {
    unparsed = input
    val sent = parseSentenceExtended()
    requireThat(unparsed.size == 0)
    return sent
}
        """.trimIndent()

/** Two adjectives attach to the noun phrase.
 * => "(sentence (noun-phrase (article the) ((adjective quick) (adjective brown))
 * (noun cat)) (verb sleeps))" */
public fun adjectiveParse(): String =
    searchLines(
        ADJECTIVE_PARSER_SOURCE + "\n" +
            """
fun main() {
    budgetCap = 1000000L
    println(parseExtended(listOf("the", "quick", "brown", "cat", "sleeps")))
}
            """.trimIndent(),
    ).first()

/** The empty modifier choice keeps plain sentences parsing.
 * => "(sentence (noun-phrase (article the) () (noun cat)) (verb sleeps))" */
public fun noAdjectiveParse(): String =
    searchLines(
        ADJECTIVE_PARSER_SOURCE + "\n" +
            """
fun main() {
    budgetCap = 1000000L
    println(parseExtended(listOf("the", "cat", "sleeps")))
}
            """.trimIndent(),
    ).first()
