// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.3.2, examples of nondeterministic programs: the
// multiple dwelling logic puzzle and the natural-language parser, with
// the session results the section's prose pins.

package sicp.ch4.examples

import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.AmbEvaluator
import sicp.ch4.ambDriver
import sicp.ch4.printValue

private val PRELUDE =
    """
    (define (require p) (if (not p) (amb)))
    (define (an-element-of items)
      (require (not (null? items)))
      (amb (car items) (an-element-of (cdr items))))
    (define (an-integer-between low high)
      (require (<= low high))
      (amb low (an-integer-between (+ low 1) high)))
    (define (member item x)
      (cond ((null? x) false)
            ((equal? item (car x)) true)
            (else (member item (cdr x)))))
    (define (memq item x)
      (cond ((null? x) false)
            ((eq? item (car x)) true)
            (else (memq item (cdr x)))))
    (define (distinct? items)
      (cond ((null? items) true)
            ((null? (cdr items)) true)
            ((member (car items) (cdr items)) false)
            (else (distinct? (cdr items)))))
    (define (multiple-dwelling)
      (let ((baker (amb 1 2 3 4 5))
            (cooper (amb 1 2 3 4 5))
            (fletcher (amb 1 2 3 4 5))
            (miller (amb 1 2 3 4 5))
            (smith (amb 1 2 3 4 5)))
        (require (distinct? (list baker cooper fletcher miller smith)))
        (require (not (= baker 5)))
        (require (not (= cooper 1)))
        (require (not (= fletcher 5)))
        (require (not (= fletcher 1)))
        (require (> miller cooper))
        (require (not (= (abs (- smith fletcher)) 1)))
        (require (not (= (abs (- fletcher cooper)) 1)))
        (list (list 'baker baker)
              (list 'cooper cooper)
              (list 'fletcher fletcher)
              (list 'miller miller)
              (list 'smith smith))))
    (define *unparsed* '())
    (define nouns '(noun student professor cat class))
    (define verbs '(verb studies lectures eats sleeps))
    (define articles '(article the a))
    (define prepositions '(prep for to in by with))
    (define (parse-word word-list)
      (require (not (null? *unparsed*)))
      (require (memq (car *unparsed*) (cdr word-list)))
      (let ((found-word (car *unparsed*)))
        (set! *unparsed* (cdr *unparsed*))
        (list (car word-list) found-word)))
    (define (parse-simple-noun-phrase)
      (list 'simple-noun-phrase
            (parse-word articles)
            (parse-word nouns)))
    (define (parse-prepositional-phrase)
      (list 'prep-phrase
            (parse-word prepositions)
            (parse-noun-phrase)))
    (define (parse-noun-phrase)
      (define (maybe-extend noun-phrase)
        (amb noun-phrase
             (maybe-extend (list 'noun-phrase
                                 noun-phrase
                                 (parse-prepositional-phrase)))))
      (maybe-extend (parse-simple-noun-phrase)))
    (define (parse-verb-phrase)
      (define (maybe-extend verb-phrase)
        (amb verb-phrase
             (maybe-extend (list 'verb-phrase
                                 verb-phrase
                                 (parse-prepositional-phrase)))))
      (maybe-extend (parse-word verbs)))
    (define (parse-sentence)
      (list 'sentence
            (parse-noun-phrase)
            (parse-verb-phrase)))
    (define (parse input)
      (set! *unparsed* input)
      (let ((sent (parse-sentence)))
        (require (null? *unparsed*))
        sent))
    """.trimIndent()

private fun firstAnswers(
    query: String,
    count: Int,
): List<String> =
    either {
        val driver = ambDriver(::AmbEvaluator, PRELUDE)
        val answers = mutableListOf<String>()
        var next = driver.solve(query)
        while (next != null && answers.size < count) {
            answers.add(printValue(next))
            next = driver.tryAgain()
        }
        answers
    }.fold(
        { e -> throw AssertionError(e.toString()) },
        { it },
    )

public class S4_3_2ExamplesTest :
    FunSpec({
        test("multiple dwelling has exactly one solution") {
            firstAnswers("(multiple-dwelling)", 2) shouldBe
                listOf("((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1))")
        }

        test("the simple sentence parses") {
            firstAnswers("(parse '(the cat eats))", 1) shouldBe
                listOf("(sentence (simple-noun-phrase (article the) (noun cat)) (verb eats))")
        }

        test("the nested prepositional phrase parses") {
            firstAnswers("(parse '(the student with the cat sleeps in the class))", 1) shouldBe
                listOf(
                    "(sentence (noun-phrase (simple-noun-phrase (article the) (noun student)) " +
                        "(prep-phrase (prep with) (simple-noun-phrase (article the) (noun cat)))) " +
                        "(verb-phrase (verb sleeps) (prep-phrase (prep in) " +
                        "(simple-noun-phrase (article the) (noun class)))))",
                )
        }

        test("the ambiguous sentence has two parses") {
            firstAnswers("(parse '(the professor lectures to the student with the cat))", 2) shouldBe
                listOf(
                    "(sentence (simple-noun-phrase (article the) (noun professor)) " +
                        "(verb-phrase (verb-phrase (verb lectures) (prep-phrase (prep to) " +
                        "(simple-noun-phrase (article the) (noun student)))) " +
                        "(prep-phrase (prep with) (simple-noun-phrase (article the) (noun cat)))))",
                    "(sentence (simple-noun-phrase (article the) (noun professor)) " +
                        "(verb-phrase (verb lectures) (prep-phrase (prep to) (noun-phrase " +
                        "(simple-noun-phrase (article the) (noun student)) (prep-phrase (prep with) " +
                        "(simple-noun-phrase (article the) (noun cat)))))))",
                )
        }
    })
