;; The nondeterministic evaluator of SICP 4.3.3, transcribed from sicp-pocket.texi.
;;
;; The analyze base comes from 4.1.7 (blocks 24635-24662, dispatcher only; all
;; other analyze procedures are superseded by the 4.3.3 deltas below) and the
;; amb extensions from 4.3.3 (blocks 26651-26917: CPS analyze procedures,
;; get-args, execute-application, analyze-amb; 26584-26587: amb syntax;
;; 26603-26606: ambeval). The `((amb? exp) (analyze-amb exp))` clause is
;; spliced into the dispatcher (26590-26593). Renames and the filled primitive
;; table are as in core/metacircular.scm; `/` returns a float.
;; The book's interactive driver-loop (26943-26980, reads input with prompt)
;; is replaced by a batch driver: `(run-amb input)` starts a new problem and
;; returns its first value, and the program-defined `(try-again)` returns the
;; next value (a bare `try-again` top-level form evaluates as a call to it,
;; per the runner). On exhaustion both print the two printer.md lines and
;; return unspecified, so the runner prints nothing further.
;; The object language's `apply` re-enters this evaluator with the live
;; succeed/fail continuations: execute-application's primitive branch calls
;; the 4-argument apply-primitive-procedure (the book's 2-argument call cannot
;; re-enter), which dispatches the `apply` entry to amb-apply-implementation.
;; The host's apply cannot take a tagged procedure object. Each program below
;; covers it with `(twice cons 4)` and `(twice add2 21)`.
;; The analyzer has no `let` clause (the book assumes exercise 4.22), so
;; object code desugars `let` to direct `((lambda ...) ...)` application.
;; Expected output, fixed before the program was run:
;;   ok
;;   ok
;;   ok
;;   (3 4 5)
;;   (6 8 10)
;;   ;;; There are no more values of
;;   (a-pythagorean-triple-between 1 10)
;;   ok
;;   (4 . 4)
;;   ok
;;   42
;; The last four lines cover grammar.md's `apply` primitive: `(twice cons 4)`
;; through a primitive procedure and `(twice add2 21)` through a compound
;; procedure, under the amb application rule.
;; The expected file is written only when the run reproduces these lines.
(define true #t)

(define false #f)

(define apply-in-underlying-scheme apply)

(define (list-of-values exps env)
  (if (no-operands? exps)
      '()
      (cons (m-eval (first-operand exps) env)
            (list-of-values 
             (rest-operands exps) 
             env))))

(define (self-evaluating? exp)
  (cond ((number? exp) true)
        ((string? exp) true)
        (else false)))

(define (variable? exp) (symbol? exp))

(define (quoted? exp)
  (tagged-list? exp 'quote))

(define (text-of-quotation exp)
  (cadr exp))

(define (tagged-list? exp tag)
  (if (pair? exp)
      (eq? (car exp) tag)
      false))

(define (assignment? exp)
  (tagged-list? exp 'set!))

(define (assignment-variable exp) 
  (cadr exp))

(define (assignment-value exp) (caddr exp))

(define (definition? exp)
  (tagged-list? exp 'define))

(define (definition-variable exp)
  (if (symbol? (cadr exp))
      (cadr exp)
      (caadr exp)))

(define (definition-value exp)
  (if (symbol? (cadr exp))
      (caddr exp)
      (make-lambda 
       (cdadr exp)   ; formal parameters
       (cddr exp))))

(define (lambda? exp) 
  (tagged-list? exp 'lambda))

(define (lambda-parameters exp) (cadr exp))

(define (lambda-body exp) (cddr exp))

(define (make-lambda parameters body)
  (cons 'lambda (cons parameters body)))

(define (if? exp) (tagged-list? exp 'if))

(define (if-predicate exp) (cadr exp))

(define (if-consequent exp) (caddr exp))

(define (if-alternative exp)
  (if (not (null? (cdddr exp)))
      (cadddr exp)
      'false))

(define (make-if predicate 
                 consequent 
                 alternative)
  (list 'if 
        predicate 
        consequent 
        alternative))

(define (begin? exp) 
  (tagged-list? exp 'begin))

(define (begin-actions exp) (cdr exp))

(define (last-exp? seq) (null? (cdr seq)))

(define (first-exp seq) (car seq))

(define (rest-exps seq) (cdr seq))

(define (sequence->exp seq)
  (cond ((null? seq) seq)
        ((last-exp? seq) (first-exp seq))
        (else (make-begin seq))))

(define (make-begin seq) (cons 'begin seq))

(define (application? exp) (pair? exp))

(define (operator exp) (car exp))

(define (operands exp) (cdr exp))

(define (no-operands? ops) (null? ops))

(define (first-operand ops) (car ops))

(define (rest-operands ops) (cdr ops))

(define (cond? exp) 
  (tagged-list? exp 'cond))

(define (cond-clauses exp) (cdr exp))

(define (cond-else-clause? clause)
  (eq? (cond-predicate clause) 'else))

(define (cond-predicate clause) 
  (car clause))

(define (cond-actions clause) 
  (cdr clause))

(define (cond->if exp)
  (expand-clauses (cond-clauses exp)))

(define (expand-clauses clauses)
  (if (null? clauses)
      'false     ; no else clause
      (let ((first (car clauses))
            (rest (cdr clauses)))
        (if (cond-else-clause? first)
            (if (null? rest)
                (sequence->exp 
                 (cond-actions first))
                (error "ELSE clause isn't last: COND->IF"
                       clauses))
            (make-if (cond-predicate first)
                     (sequence->exp 
                      (cond-actions first))
                     (expand-clauses 
                      rest))))))

(define (true? x)
  (not (eq? x false)))

(define (false? x)
  (eq? x false))

(define (make-procedure parameters body env)
  (list 'procedure parameters body env))

(define (compound-procedure? p)
  (tagged-list? p 'procedure))

(define (procedure-parameters p) (cadr p))

(define (procedure-body p) (caddr p))

(define (procedure-environment p) (cadddr p))

(define (enclosing-environment env) (cdr env))

(define (first-frame env) (car env))

(define the-empty-environment '())

(define (make-frame variables values)
  (cons variables values))

(define (frame-variables frame) (car frame))

(define (frame-values frame) (cdr frame))

(define (add-binding-to-frame! var val frame)
  (set-car! frame (cons var (car frame)))
  (set-cdr! frame (cons val (cdr frame))))

(define (extend-environment vars vals base-env)
  (if (= (length vars) (length vals))
      (cons (make-frame vars vals) base-env)
      (if (< (length vars) (length vals))
          (error "Too many arguments supplied" 
                 vars 
                 vals)
          (error "Too few arguments supplied" 
                 vars 
                 vals))))

(define (lookup-variable-value var env)
  (define (env-loop env)
    (define (scan vars vals)
      (cond ((null? vars)
             (env-loop 
              (enclosing-environment env)))
            ((eq? var (car vars))
             (car vals))
            (else (scan (cdr vars) 
                        (cdr vals)))))
    (if (eq? env the-empty-environment)
        (error "Unbound variable" var)
        (let ((frame (first-frame env)))
          (scan (frame-variables frame)
                (frame-values frame)))))
  (env-loop env))

(define (set-variable-value! var val env)
  (define (env-loop env)
    (define (scan vars vals)
      (cond ((null? vars)
             (env-loop 
              (enclosing-environment env)))
            ((eq? var (car vars))
             (set-car! vals val))
            (else (scan (cdr vars) 
                        (cdr vals)))))
    (if (eq? env the-empty-environment)
        (error "Unbound variable: SET!" var)
        (let ((frame (first-frame env)))
          (scan (frame-variables frame)
                (frame-values frame)))))
  (env-loop env))

(define (define-variable! var val env)
  (let ((frame (first-frame env)))
    (define (scan vars vals)
      (cond ((null? vars)
             (add-binding-to-frame! 
              var val frame))
            ((eq? var (car vars))
             (set-car! vals val))
            (else (scan (cdr vars) 
                        (cdr vals)))))
    (scan (frame-variables frame)
          (frame-values frame))))

(define (primitive-procedure? proc)
  (tagged-list? proc 'primitive))

(define (primitive-implementation proc) 
  (cadr proc))

(define (analyze exp)
  (cond ((self-evaluating? exp)
         (analyze-self-evaluating exp))
        ((quoted? exp)
         (analyze-quoted exp))
        ((variable? exp)
         (analyze-variable exp))
        ((assignment? exp)
         (analyze-assignment exp))
        ((definition? exp)
         (analyze-definition exp))
        ((if? exp)
         (analyze-if exp))
        ((lambda? exp)
         (analyze-lambda exp))
        ((begin? exp)
         (analyze-sequence
          (begin-actions exp)))
        ((cond? exp)
         (analyze (cond->if exp)))
        ((amb? exp) (analyze-amb exp))
        ((application? exp)
         (analyze-application exp))
        (else
         (error "Unknown expression
                 type: ANALYZE"
                exp))))

(define (analyze-self-evaluating exp)
  (lambda (env succeed fail)
    (succeed exp fail)))

(define (analyze-quoted exp)
  (let ((qval (text-of-quotation exp)))
    (lambda (env succeed fail)
      (succeed qval fail))))

(define (analyze-variable exp)
  (lambda (env succeed fail)
    (succeed (lookup-variable-value exp env)
             fail)))

(define (analyze-lambda exp)
  (let ((vars (lambda-parameters exp))
        (bproc (analyze-sequence
                (lambda-body exp))))
    (lambda (env succeed fail)
      (succeed (make-procedure vars bproc env)
               fail))))
(define (analyze-if exp)
  (let ((pproc (analyze (if-predicate exp)))
        (cproc (analyze (if-consequent exp)))
        (aproc (analyze (if-alternative exp))))
    (lambda (env succeed fail)
      (pproc env


             (lambda (pred-value fail2)
               (if (true? pred-value)
                   (cproc env succeed fail2)
                   (aproc env succeed fail2)))


             fail))))
(define (analyze-sequence exps)
  (define (sequentially a b)
    (lambda (env succeed fail)
      (a env

         (lambda (a-value fail2)
           (b env succeed fail2))

         fail)))
  (define (loop first-proc rest-procs)
    (if (null? rest-procs)
        first-proc
        (loop (sequentially first-proc
                            (car rest-procs))
              (cdr rest-procs))))
  (let ((procs (map analyze exps)))
    (if (null? procs)
        (error "Empty sequence: ANALYZE"))
    (loop (car procs) (cdr procs))))
(define (analyze-definition exp)
  (let ((var (definition-variable exp))
        (vproc (analyze
                (definition-value exp))))
    (lambda (env succeed fail)
      (vproc env
             (lambda (val fail2)
               (define-variable! var val env)
               (succeed 'ok fail2))
             fail))))
(define (analyze-assignment exp)
  (let ((var (assignment-variable exp))
        (vproc (analyze
                (assignment-value exp))))
    (lambda (env succeed fail)
      (vproc env
             (lambda (val fail2)
               (let ((old-value
                      (lookup-variable-value
                       var
                       env)))
                 (set-variable-value!
                  var
                  val
                  env)
                 (succeed
                  'ok
                  (lambda ()
                    (set-variable-value!
                     var
                     old-value
                     env)
                    (fail2)))))
               fail))))
(define (analyze-application exp)
  (let ((fproc (analyze (operator exp)))
        (aprocs (map analyze (operands exp))))
    (lambda (env succeed fail)
      (fproc env
             (lambda (proc fail2)
               (get-args
                aprocs
                env
                (lambda (args fail3)
                  (execute-application
                   proc args succeed fail3))
                fail2))
             fail))))
(define (get-args aprocs env succeed fail)
  (if (null? aprocs)
      (succeed '() fail)
      ((car aprocs)
       env
       ;;
       (lambda (arg fail2)
         (get-args
          (cdr aprocs)
          env
          ;;
          ;;
          (lambda (args fail3)
            (succeed (cons arg args)
                     fail3))
          fail2))
       fail)))
(define (execute-application
         proc args succeed fail)
  (cond ((primitive-procedure? proc)
         (apply-primitive-procedure proc args succeed fail))
        ((compound-procedure? proc)
         ((procedure-body proc)
          (extend-environment
           (procedure-parameters proc)
           args
           (procedure-environment proc))
          succeed
          fail))
        (else (error "Unknown procedure type: EXECUTE-APPLICATION"
                     proc))))
(define (analyze-amb exp)
  (let ((cprocs
         (map analyze (amb-choices exp))))
    (lambda (env succeed fail)
      (define (try-next choices)
        (if (null? choices)
            (fail)
            ((car choices)
             env
             succeed
             (lambda ()
               (try-next (cdr choices))))))
      (try-next cprocs))))
(define (amb? exp) (tagged-list? exp 'amb))
(define (amb-choices exp) (cdr exp))
(define (ambeval exp env succeed fail)
  ((analyze exp) env succeed fail))

(define (amb-apply-implementation args succeed fail)
  (let ((procedure (car args))
        (arguments (cadr args)))
    (cond ((primitive-procedure? procedure)
           (succeed (apply-in-underlying-scheme
                     (primitive-implementation procedure) arguments)
                    fail))
          ((compound-procedure? procedure)
           ((procedure-body procedure)
            (extend-environment (procedure-parameters procedure)
                                arguments
                                (procedure-environment procedure))
            succeed fail))
          (else (error "Unknown procedure type: APPLY" procedure)))))

(define primitive-procedures
  (list
        (list 'car car)
        (list 'cdr cdr)
        (list 'cons cons)
        (list 'list list)
        (list 'null? null?)
        (list 'pair? pair?)
        (list 'eq? eq?)
        (list 'equal? equal?)
        (list '+ +)
        (list '- -)
        (list '* *)
        (list '/ (lambda args (exact->inexact (apply / args))))
        (list '= =)
        (list '< <)
        (list '> >)
        (list '<= <=)
        (list '>= >=)
        (list 'remainder remainder)
        (list 'quotient quotient)
        (list 'abs abs)
        (list 'not not)
        (list 'display display)
        (list 'newline newline)
        (list 'error error)
        (list 'number? number?)
        (list 'symbol? symbol?)
        (list 'string? string?)
        (list 'set-car! set-car!)
        (list 'set-cdr! set-cdr!)
        (list 'apply amb-apply-implementation)))

(define (primitive-procedure-names)
  (map car primitive-procedures))

(define (primitive-procedure-objects)
  (map (lambda (proc) 
         (list 'primitive (cadr proc)))
       primitive-procedures))

(define (setup-environment)
  (let ((initial-env
         (extend-environment 
          (primitive-procedure-names)
          (primitive-procedure-objects)
          the-empty-environment)))
    (define-variable! 'true true initial-env)
    (define-variable! 'false false initial-env)
    initial-env))

(define (apply-primitive-procedure proc args succeed fail)
  (let ((impl (primitive-implementation proc)))
    (if (eq? impl amb-apply-implementation)
        (amb-apply-implementation args succeed fail)
        (succeed (apply-in-underlying-scheme impl args) fail))))

(define the-global-environment (setup-environment))

(define *amb-next* #f)
(define *amb-input* #f)
(define (report-amb-exhausted input)
  (display ";;; There are no more values of")
  (newline)
  (write input)
  (newline)
  (if #f #f))
(define (run-amb input)
  (set! *amb-input* input)
  (ambeval input the-global-environment
           (lambda (val next) (set! *amb-next* next) val)
           (lambda () (set! *amb-next* #f) (report-amb-exhausted input))))
(define (try-again)
  (if *amb-next*
      (let ((next *amb-next*)) (set! *amb-next* #f) (next))
      (report-amb-exhausted *amb-input*)))
(run-amb '(define (require p) (if (not p) (amb))))
(run-amb '(define (an-integer-between low high) (require (not (> low high))) (amb low (an-integer-between (+ low 1) high))))
(run-amb '(define (a-pythagorean-triple-between low high) ((lambda (i) ((lambda (j) ((lambda (k) (require (= (+ (* i i) (* j j)) (* k k))) (list i j k)) (an-integer-between j high))) (an-integer-between i high))) (an-integer-between low high))))
(run-amb '(a-pythagorean-triple-between 1 10))
try-again
try-again
(run-amb '(define (twice f x) (apply f (list x x))))
(run-amb '(twice cons 4))
(run-amb '(define (add2 x y) (+ x y)))
(run-amb '(twice add2 21))
