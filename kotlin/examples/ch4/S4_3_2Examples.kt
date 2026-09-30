// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.3.2, examples of nondeterministic programs: the
// multiple-dwelling logic puzzle and the natural-language parser, with
// the session answers the section's prose pins. The parser consumes its
// word list through ordinary writes, which the undo trail rolls back when
// a parse branch dies, so a failed extension leaves the input as it was.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.SearchModule
import sicp.ch4.SearchRun

private val DWELLING: String =
    """
    fun distinct(a: Long, b: Long, c: Long, d: Long, e: Long): Boolean =
        a != b && a != c && a != d && a != e && b != c && b != d && b != e && c != d && c != e && d != e

    fun main() {
        val baker = choose(1L, 2L, 3L, 4L, 5L)
        val cooper = choose(1L, 2L, 3L, 4L, 5L)
        val fletcher = choose(1L, 2L, 3L, 4L, 5L)
        val miller = choose(1L, 2L, 3L, 4L, 5L)
        val smith = choose(1L, 2L, 3L, 4L, 5L)
        demand(distinct(baker, cooper, fletcher, miller, smith))
        demand(baker != 5L)
        demand(cooper != 1L)
        demand(fletcher != 5L)
        demand(fletcher != 1L)
        demand(miller > cooper)
        demand(fletcher - cooper != 1L)
        demand(cooper - fletcher != 1L)
        demand(smith - fletcher != 1L)
        demand(fletcher - smith != 1L)
        println(
            "[[baker, ${'$'}{baker}], [cooper, ${'$'}{cooper}], " +
                "[fletcher, ${'$'}{fletcher}], [miller, ${'$'}{miller}], [smith, ${'$'}{smith}]]",
        )
    }
    """.trimIndent()

private val PARSER: String =
    """
    val articles: List<String> = listOf("the", "a")
    val nouns: List<String> = listOf("student", "professor", "cat", "class")
    val verbs: List<String> = listOf("studies", "lectures", "eats", "sleeps")
    val prepositions: List<String> = listOf("for", "to", "in", "by", "with")

    var unparsed: List<String> = listOf<String>()

    fun parseWord(kind: String, words: List<String>): String {
        demand(!unparsed.isEmpty())
        demand(words.contains(unparsed.get(0)))
        val word = unparsed.get(0)
        unparsed = unparsed.drop(1)
        return "[" + kind + ", " + word + "]"
    }

    fun parsePrepositionalPhrase(): String =
        "[prep-phrase, " + parseWord("prep", prepositions) + ", " + parseNounPhrase() + "]"

    fun parseNounPhrase(): String =
        extendNoun("[simple-noun-phrase, " + parseWord("article", articles) + ", " + parseWord("noun", nouns) + "]")

    fun extendNoun(phrase: String): String =
        choose(phrase, extendNoun("[noun-phrase, " + phrase + ", " + parsePrepositionalPhrase() + "]"))

    fun parseVerbPhrase(): String = extendVerb(parseWord("verb", verbs))

    fun extendVerb(phrase: String): String =
        choose(phrase, extendVerb("[verb-phrase, " + phrase + ", " + parsePrepositionalPhrase() + "]"))

    fun parseSentence(): String =
        "[sentence, " + parseNounPhrase() + ", " + parseVerbPhrase() + "]"

    fun parseThen(input: List<String>): String {
        unparsed = input
        val sentence = parseSentence()
        demand(unparsed.isEmpty())
        println(sentence)
        return sentence
    }

    fun main() {
        ifFail(
            { parseThen(listOf("the", "cat", "eats")) },
            {
                ifFail(
                    { parseThen(listOf("the", "student", "with", "the", "cat", "sleeps", "in", "the", "class")) },
                    { parseThen(listOf("the", "professor", "lectures", "to", "the", "student", "with", "the", "cat")) },
                )
            },
        )
    }
    """.trimIndent()

private fun searchRun(source: String): SearchRun =
    SearchModule.run(source).fold(
        { e -> throw AssertionError("admission rejected the unit: ${e.category}: ${e.message}") },
        { it },
    )

public class S4_3_2ExamplesTest :
    FunSpec({
        test("multiple dwelling has exactly one solution") {
            val run = searchRun(DWELLING)
            run.result.output shouldBe "[[baker, 3], [cooper, 2], [fletcher, 4], [miller, 5], [smith, 1]]\n"
            run.result.error shouldBe null
        }

        test("the simple sentence parses") {
            searchRun(PARSER).result.output.contains(
                "[sentence, [simple-noun-phrase, [article, the], [noun, cat]], [verb, eats]]",
            ) shouldBe true
        }

        test("the nested prepositional phrase parses") {
            searchRun(PARSER).result.output.contains(
                "[sentence, [noun-phrase, [simple-noun-phrase, [article, the], [noun, student]], " +
                    "[prep-phrase, [prep, with], [simple-noun-phrase, [article, the], [noun, cat]]]], " +
                    "[verb-phrase, [verb, sleeps], [prep-phrase, [prep, in], " +
                    "[simple-noun-phrase, [article, the], [noun, class]]]]]",
            ) shouldBe true
        }

        test("the ambiguous sentence has two parses, in the section's order") {
            searchRun(PARSER).result.output shouldBe
                "[sentence, [simple-noun-phrase, [article, the], [noun, cat]], [verb, eats]]\n" +
                "[sentence, [noun-phrase, [simple-noun-phrase, [article, the], [noun, student]], " +
                "[prep-phrase, [prep, with], [simple-noun-phrase, [article, the], [noun, cat]]]], " +
                "[verb-phrase, [verb, sleeps], [prep-phrase, [prep, in], " +
                "[simple-noun-phrase, [article, the], [noun, class]]]]]\n" +
                "[sentence, [simple-noun-phrase, [article, the], [noun, professor]], " +
                "[verb-phrase, [verb-phrase, [verb, lectures], [prep-phrase, [prep, to], " +
                "[simple-noun-phrase, [article, the], [noun, student]]]], " +
                "[prep-phrase, [prep, with], [simple-noun-phrase, [article, the], [noun, cat]]]]]\n" +
                "[sentence, [simple-noun-phrase, [article, the], [noun, professor]], " +
                "[verb-phrase, [verb, lectures], [prep-phrase, [prep, to], [noun-phrase, " +
                "[simple-noun-phrase, [article, the], [noun, student]], [prep-phrase, [prep, with], " +
                "[simple-noun-phrase, [article, the], [noun, cat]]]]]]]\n"
        }
    })
