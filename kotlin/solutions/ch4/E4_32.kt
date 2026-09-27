// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.32

package sicp.ch4.solutions

import sicp.ch4.Evaluator
import sicp.ch4.LazyEvaluator
import sicp.ch4.lazyTranscriptOn

// Exercise 4.32: the extra laziness. Under the procedural pairs of the
// section -- `cons` delaying both slots -- `(car (cons 7 (/ 1 0)))`
// answers 7 because the delayed second slot is never demanded, and asking
// for the `cdr` is the moment the danger finally fires. The chapter-3
// streams cannot do this: a stream node computes its head eagerly and only
// delays the tail (the runtime's `LStream.Cons` holds a computed `head`),
// so the chapter-3 analog -- the strict constructor -- dies at
// construction, exactly what the base evaluator's strict `cons` primitive
// shows. The self-referential `ones` pins the payoff from the other side:
// under delayed construction the definition closes in one step; under the
// strict primitive the same definition reads `ones` before the frame
// binds it.

/** The section's procedural pairs, as object-language definitions. */
private const val PROCEDURAL_PAIRS: String =
    """(define (cons x y) (lambda (m) (m x y)))
(define (car z) (z (lambda (p q) p)))
(define (cdr z) (z (lambda (p q) q)))"""

/** Both slots delayed: the armed second slot is skipped. => "7\nError:
 * division by zero\n" */
public fun lazyPairSlotsTranscript(): String =
    lazyTranscriptOn(
        ::LazyEvaluator,
        """
        ${PROCEDURAL_PAIRS}
        (car (cons 7 (/ 1 0)))
        (cdr (cons 7 (/ 1 0)))
        """.trimIndent(),
    )

/** The chapter-3 shape on this substrate: the strict constructor forces
 * its slot at construction. => "Error: division by zero\n" */
public fun eagerConstructorTranscript(): String =
    transcriptOn(
        ::Evaluator,
        """
        ${PROCEDURAL_PAIRS}
        (car (cons 7 (/ 1 0)))
        """.trimIndent(),
    )

/** Delayed construction closes the self-reference in one step and `car`
 * forces only the head slot. => "1\n" */
public fun onesOneStepTranscript(): String =
    lazyTranscriptOn(
        ::LazyEvaluator,
        """
        ${PROCEDURAL_PAIRS}
        (define ones (cons 1 ones))
        (car ones)
        """.trimIndent(),
    )

/** The same definition under the strict constructor reads `ones` before
 * the frame binds it. => "Error: unbound variable: ones\n" */
public fun strictOnesTranscript(): String =
    transcriptOn(
        ::Evaluator,
        """
        ${PROCEDURAL_PAIRS}
        (define ones (cons 1 ones))
        """.trimIndent(),
    )
