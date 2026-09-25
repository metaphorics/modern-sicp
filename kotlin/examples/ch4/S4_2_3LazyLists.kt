// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.2.3, streams as lazy lists: the procedural pairs,
// the list operations over them, and the two self-referential sessions the
// section runs -- `integers` demanded at 17 and the `solve` integral
// demanded at element 1000.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.LazyEvaluator
import sicp.ch4.lazyTranscriptOn

/** The section's list language: procedural pairs and the operations over
 * them, with the self-referential `ones` and `integers`. */
private val LIST_LANGUAGE: String =
    """
    (define (cons x y) (lambda (m) (m x y)))
    (define (car z) (z (lambda (p q) p)))
    (define (cdr z) (z (lambda (p q) q)))
    (define (list-ref items n)
      (if (= n 0)
          (car items)
          (list-ref (cdr items) (- n 1))))
    (define (map proc items)
      (if (null? items)
          '()
          (cons (proc (car items))
                (map proc (cdr items)))))
    (define (scale-list items factor)
      (map (lambda (x) (* x factor))
           items))
    (define (add-lists list1 list2)
      (cond ((null? list1) list2)
            ((null? list2) list1)
            (else (cons (+ (car list1)
                           (car list2))
                        (add-lists (cdr list1)
                                   (cdr list2))))))
    (define ones (cons 1 ones))
    (define integers
      (cons 1 (add-lists ones integers)))
    """.trimIndent()

/** The driver's printed form of element 1000 of the solve integral: the
 * host's shortest round-trip decimal for the computed double. */
private const val SOLVE_ELEMENT_1000: String = "2.716923932235896\n"

public class S4_2_3LazyListsTest :
    FunSpec({
        test("the integer stream demanded at element 17 answers 18") {
            lazyTranscriptOn(
                ::LazyEvaluator,
                """
                ${LIST_LANGUAGE}
                (list-ref integers 17)
                """.trimIndent(),
            ) shouldBe "18\n"
        }

        test("the solve integral demanded at element 1000 answers e") {
            // Forcing the 1000th element is non-tail recursion through as
            // many delayed slots; the demand runs on a thread with stack
            // headroom, the host truth the lazy module documents.
            val transcript =
                forcingThread {
                    lazyTranscriptOn(
                        ::LazyEvaluator,
                        """
                        ${LIST_LANGUAGE}
                        (define (integral integrand initial-value dt)
                          (define int
                            (cons initial-value
                                  (add-lists (scale-list integrand dt)
                                             int)))
                          int)
                        (define (solve f y0 dt)
                          (define y (integral dy y0 dt))
                          (define dy (map f y))
                          y)
                        (list-ref (solve (lambda (x) x) 1 0.001) 1000)
                        """.trimIndent(),
                    )
                }
            transcript shouldBe SOLVE_ELEMENT_1000
        }
    })

/** Runs [block] on a thread with stack room for the deep non-tail force
 * chain, and returns its answer. */
private inline fun <T> forcingThread(crossinline block: () -> T): T {
    var answer: Result<T>? = null
    val worker =
        java.lang.Thread(
            null,
            { answer = runCatching(block) },
            "lazy-force",
            256L * 1024L * 1024L,
        )
    worker.start()
    worker.join()
    return checkNotNull(answer).getOrThrow()
}
