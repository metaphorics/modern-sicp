// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.21

package sicp.ch4.solutions

import sicp.ch4.Evaluator

// Exercise 4.21: recursion without `define`. Every procedure below receives
// itself (or the pair of itself and its partner) as an ordinary argument
// and performs the recursive step by calling that argument, so the base
// evaluator -- which carries no recursion machinery at all -- runs all
// three demonstrations unchanged. Part (a) is the self-application analog
// for Fibonacci on 10; part (b) fills the statement's blanks: each of the
// mutual pair takes both procedures plus the count, so `even?`'s recursive
// step is `(od? ev? od? (- k 1))` and `odd?`'s is `(ev? ev? od? (- k 1))`.
// No Y combinator is needed for these shapes: self-application hands each
// procedure enough of itself to continue, which is the subtlety Louis's
// intuition in 4.20 missed.

/** The statement's expression: `fact` rebuilt on every call from its own
 * self-application. => 3628800 */
private val FACT: String =
    """
    ((lambda (n)
       ((lambda (fact)
          (fact fact n))
        (lambda (ft k)
          (if (= k 1)
              1
              (* k (ft ft (- k 1)))))))
     10)
    """.trimIndent()

/** The part (a) analog: fib through self-application. => 55 */
private val FIB: String =
    """
    ((lambda (n)
       ((lambda (fib)
          (fib fib n))
        (lambda (fb k)
          (if (< k 2)
              k
              (+ (fb fb (- k 1)) (fb fb (- k 2)))))))
     10)
    """.trimIndent()

/** The part (b) mutual pair with the blanks filled, run on 7 and 10.
 * => #f then #t */
private val EVEN_ODD: String =
    """
    (define (f n)
      ((lambda (ev? od?)
         (ev? ev? od? n))
       (lambda (ev? od? k)
         (if (= k 0) true (od? ev? od? (- k 1))))
       (lambda (ev? od? k)
         (if (= k 0) false (ev? ev? od? (- k 1))))))
    (f 7)
    (f 10)
    """.trimIndent()

/** The book's self-application expression computes 10!. => "3628800\n" */
public fun selfApplicationFactTranscript(): String = transcriptOn(::Evaluator, FACT)

/** The same trick drives fib 10. => "55\n" */
public fun selfApplicationFibTranscript(): String = transcriptOn(::Evaluator, FIB)

/** The filled blanks answer `(f 7)` and `(f 10)`. => "#f\n#t\n" */
public fun mutualEvenOddWithoutDefineTranscript(): String = transcriptOn(::Evaluator, EVEN_ODD)
