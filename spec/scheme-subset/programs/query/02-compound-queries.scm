;; The query language of SICP 4.4, transcribed from sicp-pocket.texi.
;;
;; The implementation comes from the 4.4.4 blocks (42 blocks, 28980-30054;
;; exercise blocks 30060+ excluded): qeval and the and/or/not/lisp-value
;; handlers, unification, the rule system, and the indexed data base (the
;; later indexed `add-assertion!` replaces the simple one, in book order).
;; The interactive query-driver-loop is replaced by a batch driver under
;; book-exact names: `(assert! '...)` adds the quoted assertion or rule and
;; prints nothing; `(run-query '...)` writes one instantiation per line.
;; Both return unspecified, per printer.md (driver chatter is never printed).
;; Host-level scaffolding the book assumes from earlier chapters: 3.5 streams
;; (`the-empty-stream`, `stream-null?/car/cdr`, `cons-stream` macro over Guile
;; memoized delay/force, `stream-map`, `stream-append`) and the 3.3 operation
;; table in minimal assoc form (`put` returns unspecified so the five
;; installations print nothing). `tagged-list?` is assumed from 4.1 (with
;; host-level #f for the object-level `false`). `execute` (lisp-value path)
;; is transcribed but never called: no required program uses `lisp-value`.
;; Result order is data-base order newest-assertion-first, as the cons-built
;; index yields (the 4.4.1 transcript shows insertion order from an earlier
;; stage); the expected file fixes it for all editions.
;; Expected output, fixed before the program was run:
;;   (and (job (Fect Cy D) (computer programmer)) (supervisor (Fect Cy D) (Bitdiddle Ben)))
;;   (and (job (Hacker Alyssa P) (computer programmer)) (supervisor (Hacker Alyssa P) (Bitdiddle Ben)))
;;   (and (supervisor (Aull DeWitt) (Warbucks Oliver)) (not (job (Aull DeWitt) (computer programmer))))
;;   (and (supervisor (Cratchet Robert) (Scrooge Eben)) (not (job (Cratchet Robert) (computer programmer))))
;;   (and (supervisor (Scrooge Eben) (Warbucks Oliver)) (not (job (Scrooge Eben) (computer programmer))))
;;   (and (supervisor (Bitdiddle Ben) (Warbucks Oliver)) (not (job (Bitdiddle Ben) (computer programmer))))
;;   (and (supervisor (Reasoner Louis) (Hacker Alyssa P)) (not (job (Reasoner Louis) (computer programmer))))
;;   (and (supervisor (Tweakit Lem E) (Bitdiddle Ben)) (not (job (Tweakit Lem E) (computer programmer))))
;; The expected file is written only when the run reproduces these lines.
;; Assumed from the 4.1 evaluator (tagged-list? is used by the
;; query syntax predicates but defined in neither 4.4 block).
(define (tagged-list? exp tag)
  (if (pair? exp)
      (eq? (car exp) tag)
      #f))
;; (#f here: at host level there is no `false` binding; the object
;; language binds `false` in its own global environment.)

;; Host-level stream support (SICP 3.5 representation over Guile
;; memoized `delay`/`force`). The book assumes these from earlier chapters;
;; editions implement them equivalently and the gate compares result bytes.
(define the-empty-stream '())
(define (stream-null? s) (null? s))
(define (stream-car s) (car s))
(define (stream-cdr s) (force (cdr s)))
(define-syntax cons-stream
  (syntax-rules () ((cons-stream a b) (cons a (delay b)))))
(define (stream-append s1 s2)
  (if (stream-null? s1)
      s2
      (cons-stream (stream-car s1)
                   (stream-append (stream-cdr s1) s2))))
;; (Assumed from SICP 3.5; `get-indexed-rules` appends the two rule streams.)
(define (stream-map proc s)
  (if (stream-null? s)
      the-empty-stream
      (cons-stream (proc (stream-car s))
                   (stream-map proc (stream-cdr s)))))
;; The operation table of SICP 3.3.3 in minimal form: `put` returns
;; unspecified so the five installations print nothing under the runner.
;; `depends-on?` (unification) tests `true`/`false`, which the book has from
;; the 4.1 global environment; at host level they are bound here.
(define true #t)
(define false #f)
(define *qtable* '())
(define (put key1 key2 val)
  ;; Entries are key-value pairs whose car is the (key1 key2) list, so
  ;; `assoc` compares full keys and the newest entry wins; no overwrite
  ;; dance is needed.
  (set! *qtable* (cons (cons (list key1 key2) val) *qtable*))
  (if #f #f))
(define (get key1 key2)
  (let ((record (assoc (list key1 key2) *qtable*)))
    (if record (cdr record) #f)))

(define input-prompt  ";;; Query input:")

(define output-prompt ";;; Query results:")

(define (instantiate
         exp frame unbound-var-handler)
  (define (copy exp)
    (cond ((var? exp)
           (let ((binding
                  (binding-in-frame
                   exp frame)))
             (if binding
                 (copy
                  (binding-value binding))
                 (unbound-var-handler
                  exp frame))))
          ((pair? exp)
           (cons (copy (car exp))
                 (copy (cdr exp))))
          (else exp)))
  (copy exp))

(define (qeval query frame-stream)
  (let ((qproc (get (type query) 'qeval)))
    (if qproc
        (qproc (contents query) frame-stream)
        (simple-query query frame-stream))))

(define (simple-query query-pattern
                      frame-stream)
  (stream-flatmap
   (lambda (frame)
     (stream-append-delayed
      (find-assertions query-pattern frame)
      (delay
        (apply-rules query-pattern frame))))
   frame-stream))

(define (conjoin conjuncts frame-stream)
  (if (empty-conjunction? conjuncts)
      frame-stream
      (conjoin (rest-conjuncts conjuncts)
               (qeval
                (first-conjunct conjuncts)
                frame-stream))))

(put 'and 'qeval conjoin)

(define (disjoin disjuncts frame-stream)
  (if (empty-disjunction? disjuncts)
      the-empty-stream
      (interleave-delayed
       (qeval (first-disjunct disjuncts)
              frame-stream)
       (delay (disjoin
               (rest-disjuncts disjuncts)
               frame-stream)))))

(put 'or 'qeval disjoin)

(define (negate operands frame-stream)
  (stream-flatmap
   (lambda (frame)
     (if (stream-null?
          (qeval (negated-query operands)
                 (singleton-stream frame)))
         (singleton-stream frame)
         the-empty-stream))
   frame-stream))

(put 'not 'qeval negate)

(define (lisp-value call frame-stream)
  (stream-flatmap
   (lambda (frame)
     (if (execute
          (instantiate
           call
           frame
           (lambda (v f)
             (error
              "Unknown pat var: LISP-VALUE"
              v))))
         (singleton-stream frame)
         the-empty-stream))
   frame-stream))

(put 'lisp-value 'qeval lisp-value)

(define (execute exp)
  (apply (eval (predicate exp)
               user-initial-environment)
         (args exp)))

(define (always-true ignore frame-stream)
  frame-stream)

(put 'always-true 'qeval always-true)

(define (find-assertions pattern frame)
  (stream-flatmap
    (lambda (datum)
      (check-an-assertion datum pattern frame))
    (fetch-assertions pattern frame)))

(define (check-an-assertion
         assertion query-pat query-frame)
  (let ((match-result
         (pattern-match
          query-pat assertion query-frame)))
    (if (eq? match-result 'failed)
        the-empty-stream
        (singleton-stream match-result))))

(define (pattern-match pat dat frame)
  (cond ((eq? frame 'failed) 'failed)
        ((equal? pat dat) frame)
        ((var? pat)
         (extend-if-consistent
          pat dat frame))
        ((and (pair? pat) (pair? dat))
         (pattern-match
          (cdr pat)
          (cdr dat)
          (pattern-match
           (car pat) (car dat) frame)))
        (else 'failed)))

(define (extend-if-consistent var dat frame)
  (let ((binding (binding-in-frame var frame)))
    (if binding
        (pattern-match
         (binding-value binding) dat frame)
        (extend var dat frame))))

(define (apply-rules pattern frame)
  (stream-flatmap
   (lambda (rule)
     (apply-a-rule rule pattern frame))
   (fetch-rules pattern frame)))

(define (apply-a-rule rule
                      query-pattern
                      query-frame)
  (let ((clean-rule
         (rename-variables-in rule)))
    (let ((unify-result
           (unify-match query-pattern
                        (conclusion clean-rule)
                        query-frame)))
      (if (eq? unify-result 'failed)
          the-empty-stream
          (qeval (rule-body clean-rule)
                 (singleton-stream
                  unify-result))))))

(define (rename-variables-in rule)
  (let ((rule-application-id
         (new-rule-application-id)))
    (define (tree-walk exp)
      (cond ((var? exp)
             (make-new-variable
              exp
              rule-application-id))
            ((pair? exp)
             (cons (tree-walk (car exp))
                   (tree-walk (cdr exp))))
            (else exp)))
    (tree-walk rule)))

(define (unify-match p1 p2 frame)
  (cond ((eq? frame 'failed) 'failed)
        ((equal? p1 p2) frame)
        ((var? p1)
         (extend-if-possible p1 p2 frame))
        ((var? p2)
         (extend-if-possible
          p2
          p1
          frame))        ;
        ((and (pair? p1)
              (pair? p2))
         (unify-match
          (cdr p1)
          (cdr p2)
          (unify-match
           (car p1)
           (car p2)
           frame)))
        (else 'failed)))

(define (extend-if-possible var val frame)
  (let ((binding (binding-in-frame var frame)))
    (cond (binding
           (unify-match
            (binding-value binding) val frame))
          ((var? val)
           (let ((binding
                  (binding-in-frame
                   val
                   frame)))
             (if binding
                 (unify-match
                  var
                  (binding-value binding)
                  frame)
                 (extend var val frame))))
          ((depends-on? val var frame)
           'failed)
          (else (extend var val frame)))))

(define (depends-on? exp var frame)
  (define (tree-walk e)
    (cond ((var? e)
           (if (equal? var e)
               true
               (let
                 ((b (binding-in-frame
                      e
                      frame)))
                  (if b
                      (tree-walk
                       (binding-value b))
                      false))))
          ((pair? e)
           (or (tree-walk (car e))
               (tree-walk (cdr e))))
          (else false)))
  (tree-walk exp))

(define THE-ASSERTIONS the-empty-stream)

(define (fetch-assertions pattern frame)
  (if (use-index? pattern)
      (get-indexed-assertions pattern)
      (get-all-assertions)))

(define (get-all-assertions) THE-ASSERTIONS)

(define (get-indexed-assertions pattern)
  (get-stream (index-key-of pattern)
              'assertion-stream))

(define (get-stream key1 key2)
  (let ((s (get key1 key2)))
    (if s s the-empty-stream)))

(define THE-RULES the-empty-stream)

(define (fetch-rules pattern frame)
  (if (use-index? pattern)
      (get-indexed-rules pattern)
      (get-all-rules)))

(define (get-all-rules) THE-RULES)

(define (get-indexed-rules pattern)
  (stream-append
   (get-stream (index-key-of pattern)
               'rule-stream)
   (get-stream '? 'rule-stream)))

(define (add-rule-or-assertion! assertion)
  (if (rule? assertion)
      (add-rule! assertion)
      (add-assertion! assertion)))

(define (add-assertion! assertion)
  (store-assertion-in-index assertion)
  (let ((old-assertions THE-ASSERTIONS))
    (set! THE-ASSERTIONS
          (cons-stream assertion
                       old-assertions))
    'ok))

(define (add-rule! rule)
  (store-rule-in-index rule)
  (let ((old-rules THE-RULES))
    (set! THE-RULES
          (cons-stream rule old-rules))
    'ok))

(define (store-assertion-in-index assertion)
  (if (indexable? assertion)
      (let ((key (index-key-of assertion)))
        (let ((current-assertion-stream
               (get-stream
                key 'assertion-stream)))
          (put key
               'assertion-stream
               (cons-stream
                assertion
                current-assertion-stream))))))

(define (store-rule-in-index rule)
  (let ((pattern (conclusion rule)))
    (if (indexable? pattern)
        (let ((key (index-key-of pattern)))
          (let ((current-rule-stream
                 (get-stream
                  key 'rule-stream)))
            (put key
                 'rule-stream
                 (cons-stream
                  rule
                  current-rule-stream)))))))

(define (indexable? pat)
  (or (constant-symbol? (car pat))
      (var? (car pat))))

(define (index-key-of pat)
  (let ((key (car pat)))
    (if (var? key) '? key)))

(define (use-index? pat)
  (constant-symbol? (car pat)))

(define (add-assertion! assertion)
  (store-assertion-in-index assertion)
  (set! THE-ASSERTIONS
        (cons-stream assertion
                     THE-ASSERTIONS))
  'ok)

(define (stream-append-delayed s1 delayed-s2)
  (if (stream-null? s1)
      (force delayed-s2)
      (cons-stream
       (stream-car s1)
       (stream-append-delayed (stream-cdr s1)
                              delayed-s2))))

(define (interleave-delayed s1 delayed-s2)
  (if (stream-null? s1)
      (force delayed-s2)
      (cons-stream
       (stream-car s1)
       (interleave-delayed
        (force delayed-s2)
        (delay (stream-cdr s1))))))

(define (stream-flatmap proc s)
  (flatten-stream (stream-map proc s)))

(define (flatten-stream stream)
  (if (stream-null? stream)
      the-empty-stream
      (interleave-delayed
       (stream-car stream)
       (delay (flatten-stream
               (stream-cdr stream))))))

(define (singleton-stream x)
  (cons-stream x the-empty-stream))

(define (type exp)
  (if (pair? exp)
      (car exp)
      (error "Unknown expression TYPE"
             exp)))

(define (contents exp)
  (if (pair? exp)
      (cdr exp)
      (error "Unknown expression CONTENTS"
             exp)))

(define (assertion-to-be-added? exp)
  (eq? (type exp) 'assert!))

(define (add-assertion-body exp)
  (car (contents exp)))

(define (empty-conjunction? exps) (null? exps))

(define (first-conjunct exps) (car exps))

(define (rest-conjuncts exps) (cdr exps))

(define (empty-disjunction? exps) (null? exps))

(define (first-disjunct exps) (car exps))

(define (rest-disjuncts exps) (cdr exps))

(define (negated-query exps) (car exps))

(define (predicate exps) (car exps))

(define (args exps) (cdr exps))

(define (rule? statement)
  (tagged-list? statement 'rule))

(define (conclusion rule) (cadr rule))

(define (rule-body rule)
  (if (null? (cddr rule))
      '(always-true)
      (caddr rule)))

(define (query-syntax-process exp)
  (map-over-symbols expand-question-mark exp))

(define (map-over-symbols proc exp)
  (cond ((pair? exp)
         (cons (map-over-symbols
                proc (car exp))
               (map-over-symbols
                proc (cdr exp))))
        ((symbol? exp) (proc exp))
        (else exp)))

(define (expand-question-mark symbol)
  (let ((chars (symbol->string symbol)))
    (if (string=? (substring chars 0 1) "?")
        (list '? (string->symbol
                  (substring
                   chars
                   1
                   (string-length chars))))
        symbol)))

(define (var? exp) (tagged-list? exp '?))

(define (constant-symbol? exp) (symbol? exp))

(define rule-counter 0)

(define (new-rule-application-id)
  (set! rule-counter (+ 1 rule-counter))
  rule-counter)

(define (make-new-variable
         var rule-application-id)
  (cons '? (cons rule-application-id
                 (cdr var))))

(define (contract-question-mark variable)
  (string->symbol
   (string-append "?"
     (if (number? (cadr variable))
         (string-append
          (symbol->string (caddr variable))
          "-"
          (number->string (cadr variable)))
         (symbol->string (cadr variable))))))

(define (make-binding variable value)
  (cons variable value))

(define (binding-variable binding)
  (car binding))

(define (binding-value binding)
  (cdr binding))

(define (binding-in-frame variable frame)
  (assoc variable frame))

(define (extend variable value frame)
  (cons (make-binding variable value) frame))

;; Batch driver replacing the interactive query-driver-loop: `assert!`
;; takes the quoted datum (a rule adds a rule, anything else asserts);
;; `run-query` writes one instantiation per line. Both return unspecified,
;; so assertion forms print nothing and each query prints only its results,
;; per printer.md (driver chatter is never printed).
(define (assert! datum)
  ;; The datum is syntax-processed before storing, exactly as the book's
  ;; driver-loop processes input before adding: stored rules carry (? name)
  ;; variables so `rename-variables-in` sees them.
  (let ((processed (query-syntax-process datum)))
    (if (and (pair? processed) (eq? (car processed) 'rule))
        (add-rule! processed)
        (add-assertion! processed)))
  (if #f #f))
(define (run-query q)
  (let ((processed (query-syntax-process q)))
    (let ((frames (qeval processed (singleton-stream '()))))
      (let loop ((s frames))
        (if (not (stream-null? s))
            (begin
              (write (instantiate processed
                                  (stream-car s)
                                  (lambda (v f) (contract-question-mark v))))
              (newline)
              (loop (stream-cdr s)))))))
  (if #f #f))
(assert! '(address (Bitdiddle Ben) (Slumerville (Ridge Road) 10)))
(assert! '(job (Bitdiddle Ben) (computer wizard)))
(assert! '(salary (Bitdiddle Ben) 60000))
(assert! '(address (Hacker Alyssa P) (Cambridge (Mass Ave) 78)))
(assert! '(job (Hacker Alyssa P) (computer programmer)))
(assert! '(salary (Hacker Alyssa P) 40000))
(assert! '(supervisor (Hacker Alyssa P) (Bitdiddle Ben)))
(assert! '(address (Fect Cy D) (Cambridge (Ames Street) 3)))
(assert! '(job (Fect Cy D) (computer programmer)))
(assert! '(salary (Fect Cy D) 35000))
(assert! '(supervisor (Fect Cy D) (Bitdiddle Ben)))
(assert! '(address (Tweakit Lem E) (Boston (Bay State Road) 22)))
(assert! '(job (Tweakit Lem E) (computer technician)))
(assert! '(salary (Tweakit Lem E) 25000))
(assert! '(supervisor (Tweakit Lem E) (Bitdiddle Ben)))
(assert! '(address (Reasoner Louis) (Slumerville (Pine Tree Road) 80)))
(assert! '(job (Reasoner Louis) (computer programmer trainee)))
(assert! '(salary (Reasoner Louis) 30000))
(assert! '(supervisor (Reasoner Louis) (Hacker Alyssa P)))
(assert! '(supervisor (Bitdiddle Ben) (Warbucks Oliver)))
(assert! '(address (Warbucks Oliver) (Swellesley (Top Heap Road))))
(assert! '(job (Warbucks Oliver) (administration big wheel)))
(assert! '(salary (Warbucks Oliver) 150000))
(assert! '(address (Scrooge Eben) (Weston (Shady Lane) 10)))
(assert! '(job (Scrooge Eben) (accounting chief accountant)))
(assert! '(salary (Scrooge Eben) 75000))
(assert! '(supervisor (Scrooge Eben) (Warbucks Oliver)))
(assert! '(address (Cratchet Robert) (Allston (N Harvard Street) 16)))
(assert! '(job (Cratchet Robert) (accounting scrivener)))
(assert! '(salary (Cratchet Robert) 18000))
(assert! '(supervisor (Cratchet Robert) (Scrooge Eben)))
(assert! '(address (Aull DeWitt) (Slumerville (Onion Square) 5)))
(assert! '(job (Aull DeWitt) (administration secretary)))
(assert! '(salary (Aull DeWitt) 25000))
(assert! '(supervisor (Aull DeWitt) (Warbucks Oliver)))
(assert! '(can-do-job (computer wizard) (computer programmer)))
(assert! '(can-do-job (computer wizard) (computer technician)))
(assert! '(can-do-job (computer programmer) (computer programmer trainee)))
(assert! '(can-do-job (administration secretary) (administration big wheel)))
(run-query '(and (job ?x (computer programmer)) (supervisor ?x (Bitdiddle Ben))))
(run-query '(and (supervisor ?x ?y) (not (job ?x (computer programmer)))))
