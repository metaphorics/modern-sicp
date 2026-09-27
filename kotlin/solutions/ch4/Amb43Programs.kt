// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.3 solutions: the shared object programs and the
// measurement helpers the exercise solutions drive. Internal to the
// solutions source set; nothing here is an exercise answer by itself.

package sicp.ch4.solutions

import arrow.core.raise.Raise
import arrow.core.raise.either
import sicp.ch4.AmbDriver
import sicp.ch4.AmbEvaluator
import sicp.ch4.AmbExec
import sicp.ch4.ambDriver
import sicp.ch4.printValue
import sicp.runtime.Env
import sicp.runtime.Random
import sicp.runtime.SchemeError

/** The section's base library: requirements, element and integer
 * choices, primality, list membership, and distinctness. */
internal val AMB_BASE_PRELUDE: String =
    """
    (define (require p) (if (not p) (amb)))
    (define (an-element-of items)
      (require (not (null? items)))
      (amb (car items) (an-element-of (cdr items))))
    (define (an-integer-between low high)
      (require (<= low high))
      (amb low (an-integer-between (+ low 1) high)))
    (define (an-integer-starting-from n)
      (amb n (an-integer-starting-from (+ n 1))))
    (define (even? n) (= (remainder n 2) 0))
    (define (append x y)
      (if (null? x) y (cons (car x) (append (cdr x) y))))
    (define (divides? a b) (= (remainder b a) 0))
    (define (find-divisor n test)
      (cond ((> (* test test) n) n)
            ((divides? test n) test)
            (else (find-divisor n (+ test 1)))))
    (define (smallest-divisor n) (find-divisor n 2))
    (define (prime? n) (= n (smallest-divisor n)))
    (define (prime-sum-pair list1 list2)
      (let ((a (an-element-of list1))
            (b (an-element-of list2)))
        (require (prime? (+ a b)))
        (list a b)))
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
    """.trimIndent()

/** Exercise 4.35's triples procedure. */
internal val TRIPLES_PROGRAM: String =
    """
    (define (a-pythagorean-triple-between low high)
      (let ((i (an-integer-between low high)))
        (let ((j (an-integer-between i high)))
          (let ((k (an-integer-between j high)))
            (require (= (+ (* i i) (* j j)) (* k k)))
            (list i j k)))))
    """.trimIndent()

/** Exercise 4.36: the fair unbounded generator and the naive replacement. */
internal val FAIR_TRIPLE_PROGRAM: String =
    """
    (define (a-pythagorean-triple)
      (let ((k (an-integer-starting-from 1)))
        (let ((i (an-integer-between 1 k)))
          (let ((j (an-integer-between i k)))
            (require (= (+ (* i i) (* j j)) (* k k)))
            (list i j k)))))
    """.trimIndent()

internal val NAIVE_TRIPLE_PROGRAM: String =
    """
    (define (a-pythagorean-triple-naive)
      (let ((i (an-integer-starting-from 1)))
        (let ((j (an-integer-starting-from i)))
          (let ((k (an-integer-starting-from j)))
            (require (= (+ (* i i) (* j j)) (* k k)))
            (list i j k)))))
    """.trimIndent()

/** Exercise 4.37: Ben's generator computes the hypotenuse. */
internal val BEN_TRIPLE_PROGRAM: String =
    """
    (define (isqrt-or-false n)
      (define (try k)
        (cond ((> (* k k) n) false)
              ((= (* k k) n) k)
              (else (try (+ k 1)))))
      (try 1))
    (define (a-pythagorean-triple-ben low high)
      (let ((i (an-integer-between low high))
            (hsq (* high high)))
        (let ((j (an-integer-between i high)))
          (let ((ksq (+ (* i i) (* j j))))
            (require (>= hsq ksq))
            (let ((k (isqrt-or-false ksq)))
              (require (not (eq? k false)))
              (list i j k))))))
    """.trimIndent()

/** The multiple dwelling programs: the book's, 4.38's modified, 4.39's
 * reordered, and 4.40's counting and pruned variants. */
internal val DWELLING_PROGRAM: String =
    """
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
    """.trimIndent()

internal val DWELLING_MODIFIED_PROGRAM: String =
    """
    (define (multiple-dwelling-modified)
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
        (require (not (= (abs (- fletcher cooper)) 1)))
        (list (list 'baker baker)
              (list 'cooper cooper)
              (list 'fletcher fletcher)
              (list 'miller miller)
              (list 'smith smith))))
    """.trimIndent()

internal val DWELLING_REORDERED_PROGRAM: String =
    """
    (define (multiple-dwelling-reordered)
      (let ((baker (amb 1 2 3 4 5))
            (cooper (amb 1 2 3 4 5))
            (fletcher (amb 1 2 3 4 5))
            (miller (amb 1 2 3 4 5))
            (smith (amb 1 2 3 4 5)))
        (require (not (= fletcher 5)))
        (require (not (= fletcher 1)))
        (require (not (= (abs (- fletcher cooper)) 1)))
        (require (not (= baker 5)))
        (require (not (= cooper 1)))
        (require (> miller cooper))
        (require (distinct? (list baker cooper fletcher miller smith)))
        (require (not (= (abs (- smith fletcher)) 1)))
        (list (list 'baker baker)
              (list 'cooper cooper)
              (list 'fletcher fletcher)
              (list 'miller miller)
              (list 'smith smith))))
    """.trimIndent()

internal val DWELLING_SPACE_PROGRAM: String =
    """
    (define (dwellings-all)
      (let ((baker (amb 1 2 3 4 5))
            (cooper (amb 1 2 3 4 5))
            (fletcher (amb 1 2 3 4 5))
            (miller (amb 1 2 3 4 5))
            (smith (amb 1 2 3 4 5)))
        (list baker cooper fletcher miller smith)))
    (define (dwellings-distinct)
      (let ((baker (amb 1 2 3 4 5))
            (cooper (amb 1 2 3 4 5))
            (fletcher (amb 1 2 3 4 5))
            (miller (amb 1 2 3 4 5))
            (smith (amb 1 2 3 4 5)))
        (require (distinct? (list baker cooper fletcher miller smith)))
        (list baker cooper fletcher miller smith)))
    """.trimIndent()

internal val DWELLING_PRUNED_PROGRAM: String =
    """
    (define (multiple-dwelling-faster)
      (let ((baker (amb 1 2 3 4 5)))
        (require (not (= baker 5)))
        (let ((cooper (amb 1 2 3 4 5)))
          (require (not (= cooper 1)))
          (let ((fletcher (amb 1 2 3 4 5)))
            (require (not (= fletcher 5)))
            (require (not (= fletcher 1)))
            (require (not (= (abs (- fletcher cooper)) 1)))
            (let ((miller (amb 1 2 3 4 5)))
              (require (> miller cooper))
              (let ((smith (amb 1 2 3 4 5)))
                (require (distinct? (list baker cooper fletcher miller smith)))
                (require (not (= (abs (- smith fletcher)) 1)))
                (list (list 'baker baker)
                      (list 'cooper cooper)
                      (list 'fletcher fletcher)
                      (list 'miller miller)
                      (list 'smith smith))))))))
    """.trimIndent()

/** Exercise 4.42: the Liars puzzle; one true and one untrue statement
 * per girl, as a parity requirement over each pair of claims. */
internal val LIARS_PROGRAM: String =
    """
    (define (one-true p q) (not (= (if p 1 0) (if q 1 0))))
    (define (liars)
      (let ((betty (amb 1 2 3 4 5))
            (ethel (amb 1 2 3 4 5))
            (joan (amb 1 2 3 4 5))
            (kitty (amb 1 2 3 4 5))
            (mary (amb 1 2 3 4 5)))
        (require (distinct? (list betty ethel joan kitty mary)))
        (require (one-true (= kitty 2) (= betty 3)))
        (require (one-true (= ethel 1) (= joan 2)))
        (require (one-true (= joan 3) (= ethel 5)))
        (require (one-true (= kitty 2) (= mary 4)))
        (require (one-true (= mary 4) (= betty 1)))
        (list (list 'betty betty)
              (list 'ethel ethel)
              (list 'joan joan)
              (list 'kitty kitty)
              (list 'mary mary))))
    """.trimIndent()

/** Exercise 4.43: the yacht puzzle, told and untold. */
internal val YACHT_TOLD_PROGRAM: String =
    """
    (define (yacht-puzzle)
      (let ((moores-daughter 'mary-ann)
            (downings-daughter (amb 'lorna 'gabrielle 'rosalind 'melissa))
            (halls-daughter (amb 'lorna 'gabrielle 'rosalind 'melissa))
            (barnacles-daughter 'melissa)
            (parkers-daughter (amb 'lorna 'gabrielle 'rosalind 'melissa)))
        (require (distinct? (list moores-daughter downings-daughter halls-daughter barnacles-daughter parkers-daughter)))
        (require (not (eq? halls-daughter 'rosalind)))
        (require (not (eq? downings-daughter 'melissa)))
        (require
         (cond ((eq? moores-daughter 'gabrielle) (eq? parkers-daughter 'lorna))
               ((eq? halls-daughter 'gabrielle) (eq? parkers-daughter 'rosalind))
               ((eq? downings-daughter 'gabrielle) (eq? parkers-daughter 'melissa))
               (else false)))
        (list (list 'lornas-father
                    (if (eq? downings-daughter 'lorna) 'downing
                        (if (eq? halls-daughter 'lorna) 'hall
                            (if (eq? parkers-daughter 'lorna) 'parker 'moore)))))))
    """.trimIndent()

internal val YACHT_UNTOLD_PROGRAM: String =
    """
    (define (yacht-puzzle-untold)
      (let ((moores-daughter (amb 'gabrielle 'rosalind 'melissa 'mary-ann))
            (downings-daughter (amb 'lorna 'gabrielle 'rosalind 'melissa 'mary-ann))
            (halls-daughter (amb 'lorna 'gabrielle 'rosalind 'melissa 'mary-ann))
            (barnacles-daughter 'melissa)
            (parkers-daughter (amb 'lorna 'gabrielle 'rosalind 'melissa 'mary-ann)))
        (require (distinct? (list moores-daughter downings-daughter halls-daughter barnacles-daughter parkers-daughter)))
        (require (not (eq? halls-daughter 'rosalind)))
        (require (not (eq? downings-daughter 'melissa)))
        (require
         (cond ((eq? moores-daughter 'gabrielle) (eq? parkers-daughter 'lorna))
               ((eq? halls-daughter 'gabrielle) (eq? parkers-daughter 'rosalind))
               ((eq? downings-daughter 'gabrielle) (eq? parkers-daughter 'melissa))
               (else false)))
        (list (list 'lornas-father
                    (if (eq? downings-daughter 'lorna) 'downing
                        (if (eq? halls-daughter 'lorna) 'hall
                            (if (eq? parkers-daughter 'lorna) 'parker 'moore)))))))
    """.trimIndent()

/** Exercise 4.44: queens, newest column first. */
internal val QUEENS_PROGRAM: String =
    """
    (define (queens board-size)
      (define (safe? positions)
        (define (check rest distance)
          (cond ((null? rest) true)
                ((= (car rest) (car positions)) false)
                ((= (abs (- (car rest) (car positions))) distance) false)
                (else (check (cdr rest) (+ distance 1)))))
        (check (cdr positions) 1))
      (define (queen-cols k)
        (if (= k 0)
            '()
            (let ((previous (queen-cols (- k 1))))
              (let ((row (an-integer-between 1 board-size)))
                (let ((candidate (cons row previous)))
                  (require (safe? candidate))
                  candidate)))))
      (queen-cols board-size))
    """.trimIndent()

/** The natural-language parser: the section's final grammar. */
internal val PARSER_PROGRAM: String =
    """
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

/** Exercise 4.48: the adjective extension of the grammar. */
internal val ADJECTIVE_EXTENSION: String =
    """
    (define adjectives '(adjective quick brown sleepy))
    (define (maybe-adjectives)
      (amb '()
           (cons (parse-word adjectives) (maybe-adjectives))))
    (define (parse-noun-phrase-extended)
      (list 'noun-phrase
            (parse-word articles)
            (maybe-adjectives)
            (parse-word nouns)))
    (define (parse-sentence-extended)
      (list 'sentence
            (parse-noun-phrase-extended)
            (parse-verb-phrase)))
    (define (parse-extended input)
      (set! *unparsed* input)
      (let ((sent (parse-sentence-extended)))
        (require (null? *unparsed*))
        sent))
    """.trimIndent()

/** Exercise 4.47: Louis's verb phrase, and the interchanged order. */
internal val LOUIS_VERB_PHRASE: String =
    """
    (define (parse-verb-phrase)
      (amb (parse-word verbs)
           (list 'verb-phrase
                 (parse-verb-phrase)
                 (parse-prepositional-phrase))))
    """.trimIndent()

internal val LOUIS_INTERCHANGED: String =
    """
    (define (parse-verb-phrase)
      (amb (list 'verb-phrase
                 (parse-verb-phrase)
                 (parse-prepositional-phrase))
           (parse-word verbs)))
    """.trimIndent()

/** Exercises 4.49 and 4.50: the generator versions of `parse-word` and
 * `parse`, plain and ramb-driven. */
internal val GENERATOR_PROGRAM: String =
    """
    (define (parse-word word-list)
      (list (car word-list) (an-element-of (cdr word-list))))
    (define (parse input)
      (set! *unparsed* '())
      (let ((sent (parse-sentence)))
        (require (null? *unparsed*))
        sent))
    """.trimIndent()

internal val RAMB_GENERATOR_PROGRAM: String =
    """
    (define (parse-word word-list)
      (list (car word-list) (ramb-a-word (cdr word-list))))
    (define (ramb-a-word items)
      (require (not (null? items)))
      (ramb (car items) (ramb-a-word (cdr items))))
    (define (parse input)
      (set! *unparsed* '())
      (let ((sent (parse-sentence)))
        (require (null? *unparsed*))
        sent))
    """.trimIndent()

/** Raised by [BudgetAmb] when a search outspends its choice budget. */
internal class BudgetExhausted(
    val cap: Long,
) : RuntimeException("choice budget exhausted after $cap choices")

/** An evaluator that faults a search outspending [cap] choices: the
 * honest budget for the divergent sessions the exercises measure. */
internal class BudgetAmb(
    global: Env,
    val cap: Long,
) : AmbEvaluator(global) {
    override fun analyzedAmb(alternatives: List<AmbExec>): AmbExec {
        val exec = super.analyzedAmb(alternatives)
        return { env, succeed ->
            if (choicesTaken > cap) {
                throw BudgetExhausted(cap)
            }
            exec(env, succeed)
        }
    }
}

/** Runs a fresh driver over [prelude] with the given factory and seed. */
internal fun newDriver(
    factory: (Env, Random?) -> AmbEvaluator,
    prelude: String,
    seed: ULong? = null,
): AmbDriver = ambDriver(factory, prelude, seed)

/** Every answer of [query] as printed strings, at most [limit] of them. */
context(r: Raise<SchemeError>)
internal fun answerLines(
    driver: AmbDriver,
    query: String,
    limit: Int = Int.MAX_VALUE,
): List<String> {
    val lines = mutableListOf<String>()
    var next = driver.solve(query)
    while (next != null && lines.size < limit) {
        lines.add(printValue(next))
        next = driver.tryAgain()
    }
    return lines
}

/** The printed answers of [query], faults rendered as their message. */
internal fun answerLinesFaulted(
    factory: (Env, Random?) -> AmbEvaluator,
    prelude: String,
    query: String,
    limit: Int = Int.MAX_VALUE,
    seed: ULong? = null,
): List<String> =
    either {
        answerLines(newDriver(factory, prelude, seed), query, limit)
    }.fold(
        { e -> listOf(e.toString()) },
        { it },
    )

/** The first answer of [query], or the empty string when exhausted. */
internal fun firstAnswerLine(
    factory: (Env, Random?) -> AmbEvaluator,
    prelude: String,
    query: String,
    seed: ULong? = null,
): String {
    val lines = answerLinesFaulted(factory, prelude, query, limit = 1, seed = seed)
    return lines.firstOrNull() ?: ""
}

/** Backtracks to the first answer of [query]; -1 when nothing survives. */
internal fun backtracksToFirst(
    factory: (Env, Random?) -> AmbEvaluator,
    prelude: String,
    query: String,
): Long =
    either {
        val driver = newDriver(factory, prelude)
        val first = driver.solve(query)
        if (first == null) -1L else driver.evaluator.backtracks
    }.fold(
        { -1L },
        { it },
    )

/** The fault message of a budgeted run of [query]: either the typed
 * budget fault or the search's own result. */
internal fun budgetedFault(
    prelude: String,
    query: String,
    cap: Long,
    seed: ULong? = null,
): String =
    try {
        either {
            val driver = newDriver({ env, _ -> BudgetAmb(env, cap) }, prelude, seed)
            var next = driver.solve(query)
            while (next != null) {
                next = driver.tryAgain()
            }
            "no fault"
        }.fold(
            { e -> e.toString() },
            { it },
        )
    } catch (budget: BudgetExhausted) {
        budget.message ?: "budget exhausted"
    }
