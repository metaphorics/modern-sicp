# The Scheme subset grammar

;; SPDX-License-Identifier: GPL-3.0-only
;; Adapted from SICP sections 4.1 to 5.2 and 5.5

This document fixes the object language that the chapter 4 and 5 evaluators of
all four editions run. The language is the Scheme subset used by SICP: the
core forms of 1.1 and 4.1, the lazy extension of 4.2, the nondeterministic
extension of 4.3, the query language of 4.4, the register-machine
language of 5.1 and 5.2, and the compiler of 5.5. One heading per form gives its syntax and its
evaluation rule and cites the SICP section that defines it. The printed form
of every value is fixed by `printer.md` beside this file. Source files use LF
line endings.

An evaluator that declares a capability must run every program of that
capability in `manifest.txt`. The programs use only the required parts of the
grammar; parts marked optional exist for exercises and never appear in a
required program.

## Notation

Syntax is shown as Scheme forms with slots in angle brackets, as in
`(if ⟨predicate⟩ ⟨consequent⟩ ⟨alternative⟩)`. A bracketed slot is optional.
`⟨expression⟩` is any expression, `⟨name⟩` a symbol, `⟨number⟩` an integer or
float literal, `⟨body⟩` a sequence of one or more expressions. The word
"error" means the behavior fixed in `printer.md`: print one `Error:` line and
stop the program.

## Data

### Integers

An integer literal is an optional sign followed by one or more decimal
digits. Integers are exact. Each edition states its integer range and its
overflow rule in its Chapter 0; the corpus keeps every integer inside
`[-2^62, 2^62]` so all editions agree.

### Floats

A float literal is an optional sign, digits, a decimal point, digits, and an
optional exponent `e` or `e-` followed by digits. Floats are IEEE 754 double
precision values. Every operation that involves a float produces a float.

### Booleans

The reader accepts `#t` and `#f`. Every evaluator binds the names `true` and
`false` to these values in its global environment, because the book's programs
use the names (4.1.4). Only `#f` counts as false; every other value counts as
true (1.1.6).

### Strings

A string literal is a sequence of characters between double quotes. Inside a
literal, `\"` denotes a double quote and `\\` denotes a backslash. No other
escape is recognized. A string is an atomic value; `eq?` on two equal strings
is unspecified and corpus programs never test it.

### Symbols

A symbol is a sequence of letters, digits, and the characters
`- ? ! * + / < > = _ .` that does not parse as a number, written without
delimiters. Symbols are case sensitive, and every symbol in the corpus is
lowercase. A quoted symbol evaluates to itself.

### Pairs and the empty list

`(cons ⟨a⟩ ⟨b⟩)` builds a pair with car `⟨a⟩` and cdr `⟨b⟩` (2.1.3, 3.3). The
pair of this subset is mutable: `set-car!` and `set-cdr!` replace its fields
(3.3). The empty list is written `'()`, the quote of no elements (2.2.1); the
subset has no `nil` variable. A chain of pairs whose last cdr is the empty
list is a list.

### Procedures

A procedure is either a primitive, supplied by the evaluator, or a compound
procedure built by `lambda` or `define`, which pairs formal parameters, a
body, and the environment of definition (1.1.4, 3.2). Procedures are first
class values. `eq?` on two compound procedures is true exactly when they are
the same object.

## Primitive procedures

The core primitive procedures are exactly:

```
car cdr cons list null? pair? eq? equal?
+ - * / = < > <= >= remainder quotient abs not
display newline error number? symbol? string? apply
```

`car`, `cdr`, `cons`, `list`, `null?`, `pair?` work on pairs and lists;
`eq?` is identity, `equal?` is structural equality over pairs, numbers,
symbols, strings, and booleans; the arithmetic names follow their usual
meaning, where `/` always returns a float and `remainder` and `quotient` are
integer operations; `display` and `newline` write output as `printer.md`
fixes; `(error ⟨message⟩ ⟨irritant⟩...)` raises an error; `apply` applies a
procedure to a list of arguments. This list is stated and stopped: evaluators
may add more primitives, and `programs/core/metacircular.scm` additionally
requires `set-car!` and `set-cdr!` for its environment frames.

## Core forms

### quote

Syntax: `(quote ⟨datum⟩)`, almost always written `'<datum>` (2.3.1). The
value is the datum itself, unevaluated. `'<a b>` evaluates to the list of the
symbols `a` and `b`.

### define

Syntax: `(define ⟨name⟩ ⟨expression⟩)` or
`(define (⟨name⟩ ⟨parameter⟩...) ⟨body⟩)` (1.1.2, 1.1.4). The second form is
sugar for `(define ⟨name⟩ (lambda (⟨parameter⟩...) ⟨body⟩))`. A define binds
`⟨name⟩` in the current environment frame and yields no printed value at the
top level. Defines that appear at the start of a body are scanned out per
4.1.6: every internal define is visible in every expression of the body, so
internal procedures can be mutually recursive, and a reference before the
defining define runs sees the unassigned marker and is an error.

### set!

Syntax: `(set! ⟨name⟩ ⟨expression⟩)` (3.1.1, 4.1.2). Evaluate the expression,
then rebind the nearest enclosing binding of `⟨name⟩` to the value. Setting a
name with no binding is an error.

### if

Syntax: `(if ⟨predicate⟩ ⟨consequent⟩ [⟨alternative⟩])` (1.1.6). Evaluate the
predicate; if its value is not `#f`, evaluate the consequent; otherwise
evaluate the alternative. Only the chosen branch is evaluated. When the
alternative is omitted and the predicate is false, the value of the `if` is
`#f`; the one-armed form exists because `require` in 4.3.1 is written with
it.

### cond

Syntax: `(cond (⟨p⟩ ⟨e⟩...)... [(else ⟨e⟩...)])`; derived form (1.1.6, 4.1.2).
Evaluate each predicate in order; the first that is not `#f` selects its
clause, whose expressions are evaluated as a sequence. `else` must be the last
clause and always matches. The arrow clause of exercise 4.5,
`(⟨p⟩ => ⟨f⟩)`, is optional: when the evaluator implements it, a true `⟨p⟩`
selects the clause and the value of applying `⟨f⟩` to the value of `⟨p⟩` is
the value of the cond. No required program uses `=>`. When no predicate is
true and there is no `else`, the value is undefined; corpus programs never do
this.

### lambda

Syntax: `(lambda (⟨parameter⟩...) ⟨body⟩)` (1.3.1, 4.1.2). The value is a
compound procedure whose formal parameters are the symbols of the parameter
list, whose body is `⟨body⟩`, and whose environment is the environment of
definition. Applying it binds the parameters to the arguments in a new frame
and evaluates the body as a sequence.

### let

Syntax: `(let ((⟨name⟩ ⟨expression⟩)...) ⟨body⟩)`; derived form, exercise 4.6.
It is equivalent to `((lambda (⟨name⟩...) ⟨body⟩) ⟨expression⟩...)`: the
expressions are evaluated in the outer environment, then bound in one new
frame. The named `let` of exercise 4.8 is optional and no required program
uses it.

### begin

Syntax: `(begin ⟨expression⟩...)` (3.1.1, 4.1.2). The expressions are
evaluated in order; the value of the last one is the value of the begin.

### and

Syntax: `(and ⟨expression⟩...)` (1.1.6). The expressions are evaluated in
order from left to right; the first `#f` stops the evaluation and is the
value; otherwise the value of the last expression is the value. `(and)` is
`#t`.

### or

Syntax: `(or ⟨expression⟩...)` (1.1.6). The expressions are evaluated in
order from left to right; the first value that is not `#f` stops the
evaluation and is the value; otherwise the value is `#f`. `(or)` is `#f`.

## The lazy subgrammar

Programs of capability `lazy` run under the lazy evaluator of 4.2.

### Lazy argument evaluation

The application rule is semi-lazy (4.2.2): the operator is evaluated before
the application, operands of a primitive procedure are evaluated, and
operands of a compound procedure are delayed into thunks. A thunk is
memoized: forcing it evaluates its expression once and caches the value
(exercise 4.27 relies on the memoized default).

### delay and force

`(delay ⟨expression⟩)` builds a memoized thunk for the expression in the
current environment. `(force ⟨thunk⟩)` returns the thunk's value, evaluating
it at most once. `force` is an ordinary procedure: the operator-forcing rule
above makes `(force x)` evaluate the thunk bound to `x` even though the
application delays the argument again.

### cons-stream and the stream operations

No `cons-stream` special form is needed (4.2.3): with lazy application, a
`cons` defined as a compound procedure is non-strict. The corpus programs use
the message-passing pair of exercise 2.4,

```
(define (cons x y) (lambda (m) (m x y)))
(define (car z) (z (lambda (p q) p)))
(define (cdr z) (z (lambda (p q) q)))
```

and then the stream operations are ordinary procedures:
`the-empty-stream` is `'()`, `(stream-null? s)` is `(null? s)`,
`(stream-car s)` is `(car s)`, and `(stream-cdr s)` is `(force (cdr s))`.
A `cons-stream` may also be defined as `(define (cons-stream a b) (cons a
(delay b)))`; no required program does.

## The amb subgrammar

Programs of capability `amb` run under the nondeterministic evaluator of
4.3, whose driver keeps one current problem.

### amb

Syntax: `(amb ⟨expression⟩...)`, including the zero-alternative form `(amb)`
(4.3.1). The evaluator returns the alternatives in textual order, one value
per driver request; a value is presented only if the rest of the computation
succeeds, and a failure backtracks to the most recent `amb` with an
untried alternative. `(amb)` has no alternatives and always fails.

### require

`require` is an ordinary procedure, not a form, defined in every amb program:

```
(define (require p) (if (not p) (amb)))
```

It fails the current branch of the search when `p` is false (4.3.1).

### try-again

`try-again` is a driver command, not a form: a top-level form that is the
bare symbol `try-again` asks the driver for the next value of the current
problem. Any other top-level form starts a new problem (4.3.1). When the
current problem has no more values, the driver prints the exhausted message
fixed by `printer.md`.

### ramb

`(ramb ⟨expression⟩...)` is the random-order `amb` of exercise 4.50. It is
optional; no required program uses it.

### if-fail

`(if-fail ⟨e1⟩ ⟨e2⟩)` is exercise 4.52. Its value is the value of `⟨e1⟩` if
`⟨e1⟩` has one, and otherwise the value of `⟨e2⟩`. It is optional; no
required program uses it.

### permanent-set!

`(permanent-set! ⟨name⟩ ⟨expression⟩)` is exercise 4.51: like `set!`, but the
binding is not undone by backtracking. It is optional; no required program
uses it.

## The query subgrammar

Programs of capability `query` run under the query language of 4.4. A query
program consists of `assert!` forms followed by query forms.

### assert!

Syntax: `(assert! ⟨assertion⟩)` or `(assert! (rule ...))` (4.4.1). The form
adds the assertion or rule to the data base and prints nothing.

### Pattern variables

A pattern variable is a symbol starting with `?`, as in `?x` or
`?person-1`. A pattern ending in a dotted tail, as in `(?town . ?rest)`,
matches the remaining elements of a list into the tail variable. Patterns
match by unification (4.4.2).

### Simple queries

A simple query is a form that is not a special form, such as
`(job ?x (computer programmer))` (4.4.1). The evaluator prints each
instantiation of the query against the data base, one per line, in the order
fixed by `printer.md`.

### and, or, not, lisp-value

`(and ⟨query⟩...)` matches when all subqueries match, processing them left to
right; `(or ⟨query⟩...)` matches when some subquery matches; `(not ⟨query⟩)`
succeeds with no new bindings when the subquery has no match; and
`(lisp-value ⟨procedure⟩ ⟨pattern⟩...)` applies the procedure to the
instantiated patterns and fails when it returns `#f` (4.4.2). The corpus
query programs use `and` and `not`; `or` and `lisp-value` are part of the
subgrammar but appear in no required program.

### rule

Syntax: `(rule ⟨conclusion⟩ [⟨body-query⟩...])` (4.4.2). The body is optional:
a rule with no body, such as `(rule (same ?x ?x))`, matches whenever its
conclusion unifies with the query. Rules are added to the data base with
`assert!` and are applied to queries and to other rules' bodies.

## The machine subgrammar

Programs of capability `machine` are register-machine definitions run by the
simulator of 5.2. A machine program consists of one `make-machine` call
followed by driver calls on the resulting machine.

### make-machine

Syntax: `(make-machine ⟨register-names⟩ ⟨operations⟩ ⟨controller-text⟩)`
(5.2.1). `⟨register-names⟩` is a list of symbols naming the registers.
`⟨operations⟩` is a list of two-element lists, each pairing an operation name
with its implementation; corpus programs pair names with core primitives, as
in `(list (list 'rem remainder) (list '= =))`. `⟨controller-text⟩` is a list
of instructions and label symbols.

### Instructions

The instructions are (5.1):

- `(assign ⟨register⟩ ⟨source⟩)` sets the register from the source;
- `(test ⟨operation-call⟩)` evaluates the call and stores its value in the
  condition flag;
- `(branch (label ⟨label⟩))` continues at the label when the flag is true,
  and otherwise falls through;
- `(goto (label ⟨label⟩))` or `(goto (reg ⟨register⟩))` continues at the
  labeled instruction sequence or at the instruction sequence named by the
  register's value;
- `(save ⟨register⟩)` pushes the register's value on the single stack;
- `(restore ⟨register⟩)` pops the stack into the register;
- `(perform ⟨operation-call⟩)` evaluates the call for its effect.

A label is a symbol standing alone in the controller text. The machine starts
at the first instruction and stops when the program counter moves past the
last instruction.

### Sources

An `assign` source is `(reg ⟨register⟩)`, `(const ⟨value⟩)`, `(label
⟨label⟩)`, or an operation call `(op ⟨name⟩ ⟨source⟩...)`, whose arguments
are `(reg ...)` or `(const ...)` sources (5.1.1). Operation names resolve in
the machine's operation table.

### The machine API

`(set-register-contents! ⟨machine⟩ ⟨register⟩ ⟨value⟩)` sets a register and
returns the symbol `done`; `(get-register-contents ⟨machine⟩ ⟨register⟩)`
returns a register's value; `(start ⟨machine⟩)` runs the machine to a halt
and returns `done` (5.2.2). The printed forms of these calls are fixed by
`printer.md`.

## The compiler subgrammar

Programs of capability `compiler` run the compiler of 5.5 over one
expression, assemble the resulting instruction sequence on the simulator of
5.2, and run it to a halt.

### compile

Syntax: `(compile ⟨expression⟩ ⟨target⟩ ⟨linkage⟩)` (5.5.1). `⟨expression⟩`
is any core expression: self-evaluating, quoted, a variable, `set!`,
`define`, `if`, `lambda`, `begin`, `cond` (compiled through `cond->if`), or
an application. `⟨target⟩` names the register receiving the value (`val`,
or `proc` for an operator); `⟨linkage⟩` is `next` (fall through), `return`
(continue at `(reg continue)`), or a label name. Corpus programs compile
with target `val` and linkage `next`.

### Instruction sequences

A compiled fragment is a triple of needed registers, modified registers,
and statements (5.5.1, 5.5.4). `(statements ⟨sequence⟩)` selects the
instruction list; `append`, `preserving`, `tack-on`, and `parallel`
combine fragments while tracking the two register sets. Linkage `return`
generates tail-recursive code: a call in tail position transfers directly
without saving `continue`.

### assemble and run

`(assemble ⟨controller-text⟩ ⟨machine⟩)` is the assembler of 5.2.2. A
corpus compiler program assembles
`(statements (compile ⟨expression⟩ 'val 'next))` onto a machine with
registers `(env proc val argl continue)` and drives it with the machine
API: `set-register-contents!` loads `env` with the global environment,
`start` runs to a halt, and `get-register-contents` reads `val`. The
machine operations available to compiled code are `list` and `cons` (for
`argl`), `lookup-variable-value`, `define-variable!`,
`set-variable-value!`, `primitive-procedure?`,
`apply-primitive-procedure`, `make-compiled-procedure`,
`compiled-procedure-entry`, `compiled-procedure-env`,
`extend-environment`, and `false?`. Printed forms follow the machine
traces of `printer.md`.

### Output equivalence

For every corpus compiler program, the assembled run prints byte-identical
output to the interpretation of the same expression. Compilation changes
only hidden execution metrics (stack pushes, instruction count), never
values, side-effect order, or errors.
