// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.50: compile and run the metacircular evaluator.

package sicp.ch5.solutions

import arrow.core.raise.either
import sicp.ch5.CompilerConfig
import sicp.ch5.CompilerState
import sicp.ch5.compileAndGo
import sicp.ch5.compileBlock

/** The 4.1 metacircular evaluator adapted for the 5.5.7 machine's environment primitives:
 *  the environment procedures ride behind machine words, the list accessors and the
 *  arithmetic the primitive table names ride in the runtime support, and `factorial`
 *  is defined inside the object world by the session driver, not at machine level. */
public val metacircularEvaluatorSource: String =
    """
(define true #t)
(define false #f)

(define (m-eval exp env)
  (cond ((self-evaluating? exp) exp)
        ((variable? exp) (lookup-variable-value exp env))
        ((quoted? exp) (text-of-quotation exp))
        ((assignment? exp) (eval-assignment exp env))
        ((definition? exp) (eval-definition exp env))
        ((if? exp) (eval-if exp env))
        ((lambda? exp) (make-procedure (lambda-parameters exp) (lambda-body exp) env))
        ((begin? exp) (eval-sequence (begin-actions exp) env))
        ((cond? exp) (m-eval (cond->if exp) env))
        ((application? exp)
         (m-apply (m-eval (operator exp) env) (list-of-values (operands exp) env)))
        (else (error "Unknown expression type: EVAL" exp))))

(define (m-apply procedure arguments)
  (cond ((primitive-procedure? procedure)
         (apply-primitive-procedure procedure arguments))
        ((compound-procedure? procedure)
         (eval-sequence (procedure-body procedure)
           (extend-environment (procedure-parameters procedure) arguments
                               (procedure-environment procedure))))
        (else (error "Unknown procedure type: APPLY" procedure))))

(define (list-of-values exps env)
  (if (no-operands? exps) '()
      (cons (m-eval (first-operand exps) env)
            (list-of-values (rest-operands exps) env))))
(define (eval-if exp env)
  (if (true? (m-eval (if-predicate exp) env))
      (m-eval (if-consequent exp) env)
      (m-eval (if-alternative exp) env)))
(define (eval-sequence exps env)
  (cond ((last-exp? exps) (m-eval (first-exp exps) env))
        (else (m-eval (first-exp exps) env)
              (eval-sequence (rest-exps exps) env))))
(define (eval-assignment exp env)
  (set-variable-value! (assignment-variable exp) (m-eval (assignment-value exp) env) env)
  'ok)
(define (eval-definition exp env)
  (define-variable! (definition-variable exp) (m-eval (definition-value exp) env) env)
  'ok)

(define (self-evaluating? exp)
  (cond ((number? exp) true) ((string? exp) true) (else false)))
(define (variable? exp) (symbol? exp))
(define (quoted? exp) (tagged-list? exp 'quote))
(define (text-of-quotation exp) (cadr exp))
(define (tagged-list? exp tag) (if (pair? exp) (eq? (car exp) tag) false))
(define (assignment? exp) (tagged-list? exp 'set!))
(define (assignment-variable exp) (cadr exp))
(define (assignment-value exp) (caddr exp))
(define (definition? exp) (tagged-list? exp 'define))
(define (definition-variable exp) (if (symbol? (cadr exp)) (cadr exp) (caadr exp)))
(define (definition-value exp)
  (if (symbol? (cadr exp)) (caddr exp)
      (make-lambda (cdadr exp) (cddr exp))))
(define (lambda? exp) (tagged-list? exp 'lambda))
(define (lambda-parameters exp) (cadr exp))
(define (lambda-body exp) (cddr exp))
(define (make-lambda parameters body) (cons 'lambda (cons parameters body)))
(define (if? exp) (tagged-list? exp 'if))
(define (if-predicate exp) (cadr exp))
(define (if-consequent exp) (caddr exp))
(define (if-alternative exp) (if (not (null? (cdddr exp))) (cadddr exp) 'false))
(define (make-if predicate consequent alternative)
  (list 'if predicate consequent alternative))
(define (begin? exp) (tagged-list? exp 'begin))
(define (begin-actions exp) (cdr exp))
(define (last-exp? seq) (null? (cdr seq)))
(define (first-exp seq) (car seq))
(define (rest-exps seq) (cdr seq))
(define (sequence->exp seq)
  (cond ((null? seq) seq) ((last-exp? seq) (first-exp seq))
        (else (make-begin seq))))
(define (make-begin seq) (cons 'begin seq))
(define (application? exp) (pair? exp))
(define (operator exp) (car exp))
(define (operands exp) (cdr exp))
(define (no-operands? ops) (null? ops))
(define (first-operand ops) (car ops))
(define (rest-operands ops) (cdr ops))
(define (cond? exp) (tagged-list? exp 'cond))
(define (cond-clauses exp) (cdr exp))
(define (cond-else-clause? clause) (eq? (cond-predicate clause) 'else))
(define (cond-predicate clause) (car clause))
(define (cond-actions clause) (cdr clause))
(define (cond->if exp) (expand-clauses (cond-clauses exp)))
(define (expand-clauses clauses)
  (if (null? clauses) 'false
      (let ((first (car clauses)) (rest (cdr clauses)))
        (if (cond-else-clause? first)
            (if (null? rest) (sequence->exp (cond-actions first))
                (error "ELSE clause isn't last: COND->IF" clauses))
            (make-if (cond-predicate first) (sequence->exp (cond-actions first))
                     (expand-clauses rest))))))
(define (true? x) (not (eq? x false)))
(define (make-procedure parameters body env) (list 'procedure parameters body env))
(define (compound-procedure? p) (tagged-list? p 'procedure))
(define (procedure-parameters p) (cadr p))
(define (procedure-body p) (caddr p))
(define (procedure-environment p) (cadddr p))
(define (primitive-procedure? proc) (tagged-list? proc 'primitive))
(define (primitive-implementation proc) (cadr proc))
(define (map f l)
  (if (null? l) '() (cons (f (car l)) (map f (cdr l)))))
(define primitive-procedures
  (list (list 'car car) (list 'cdr cdr) (list 'cons cons) (list 'list list)
        (list 'null? null?) (list 'pair? pair?) (list 'eq? eq?) (list 'equal? equal?)
        (list '+ +) (list '- -) (list '* *) (list '/ /) (list '= =) (list '< <)
        (list '> >) (list '<= <=) (list '>= >=) (list 'remainder remainder)
        (list 'quotient quotient) (list 'abs abs) (list 'not not)
        (list 'display display) (list 'newline newline) (list 'error error)
        (list 'number? number?) (list 'symbol? symbol?) (list 'string? string?)))
(define (primitive-procedure-names) (map car primitive-procedures))
(define (primitive-procedure-objects)
  (map (lambda (proc) (list 'primitive (cadr proc))) primitive-procedures))
(define (apply-primitive-procedure proc args)
  (apply-in-underlying-scheme (primitive-implementation proc) args))
(define (setup-environment)
  (let ((initial-env (extend-environment (primitive-procedure-names)
                                         (primitive-procedure-objects) 'the-empty)))
    (define-variable! 'true true initial-env)
    (define-variable! 'false false initial-env)
    initial-env))
(define the-global-environment (setup-environment))
(m-eval '(define (factorial n) (if (= n 1) 1 (* n (factorial (- n 1))))) the-global-environment)
    """.trimIndent()

/** The session driver: object-level calls into the compiled evaluator, running
 *  the object world's factorial twice around the book's tick pair. */
private val metacircularDriver: String =
    """
    (m-eval '(factorial 5) the-global-environment)
    (m-eval '((lambda (x) (cons x (list x x))) 'tick) the-global-environment)
    (m-eval '(factorial 5) the-global-environment)
    """.trimIndent()

/** Compile and execute the evaluator on the 5.5.7 register machine. */
public fun compiledMetacircularRuns(): List<String> {
    val result =
        either {
            val state = CompilerState()
            val (entry, block) = compileBlock(CompilerConfig(), state, readForms(metacircularEvaluatorSource))
            val evaluator = compileAndGo(entry, block, metacircularDriver, runtimeSupport = true)
            evaluator.drive()
            evaluator.transcript
        }.fold({ error("compiled metacircular evaluator failed: $it") }, { it })
    val values = valuesOf(result)
    check(values.contains("120")) { "compiled metacircular evaluator did not answer 120: $result" }
    return listOf("answers: ${values.joinToString()}")
}
