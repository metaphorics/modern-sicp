# Kotlin guest subset: grammar and runtime contract

Anchor: Tony Hoare, Otl Aicher

Status: Phase 1 contract for the Kotlin edition guest language. This document
is normative for guest source. The native oracle results for the witnesses are
recorded in local://modern-sicp-kotlin-contract-native-results.json and marked
VERIFIED below; all guest-engine obligations (Section 5.2, Task 7/10) are
UNRUN. This document claims no full Kotlin compiler.
It defines one finite subset and its rejection categories.

## 1. Scope and terms

The Kotlin edition has two source populations.

- **Host code** is the edition's implementation: the teaching evaluator, the
  register-machine simulator, the compiler, the experimental engines, and the
  test code under `kotlin/`. Host code is ordinary Kotlin under the pinned
  toolchain and the repository style rules. Nothing in this document restricts
  host code except where a rule says so.
- **Guest source** is the language of programs those engines parse, type-check,
  and execute: the teaching programs of the book, the guest evaluator kernel
  (Section 5), and every program a chapter 4 or 5 exercise feeds to an engine.
  Guest source is exactly the admitted grammar of Section 2. A core execution
  path MUST require successful source type checking before any guest effect.

A program that is not guest source is not executed, quoted, or partially
interpreted. The subset gate classifies every rejected form before execution.

Three admission classes:

| Class | Meaning | Enforcement point |
|---|---|---|
| Core | admitted everywhere (Section 2-3) | type checking, then execution |
| Experimental | admitted only inside a named module and execution mode (Section 4) | module admission rules |
| Rejected | host-invalid or host-valid-but-unsupported (Section 6) | type checker or subset gate, before execution |

A **guest evaluation** observes exactly: the ordered stream of `print` /
`println` output (Section 3.7), the program's typed guest errors (Section 3.8),
and the value of `main`'s completion. Two executions agree when these agree.
Internal stack counts are never compared across implementations.

## 2. Finite grammar

Anything not produced by the productions below is not guest source. The
grammar is closed: no unlisted keyword, modifier, operator, library function,
or type constructor is admitted.

### 2.1 Lexical

```ebnf
Ident       = letter { letter | digit | "_" } .          (* Kotlin identifier *)
IntLit      = [ "-" ] digit { digit | "_" } .
LongLit     = [ "-" ] digit { digit | "_" } "L" .
DoubleLit   = [ "-" ] digit { digit | "_" }
              ( "." digit { digit | "_" } [ exponent ] | exponent ) .
exponent    = ( "e" | "E" ) [ "-" ] digit { digit } .
BoolLit     = "true" | "false" .
StringLit   = '"' { string-char | escape | template } '"' .
escape      = "\n" | "\t" | "\r" | "\\" | "\"" | "\$" .
template    = "$" Ident | "${" Expr "}" .
Comment     = "//" to-end-of-line | "/*" to "*/" .
```

Underscores group digits in any numeric literal. A signed literal takes the
type of Section 3.1. Hexadecimal, binary, unsigned, and character literals are
rejected. Raw (triple-quoted) strings are rejected. String interpolation of a
non-primitive value is rejected (Section 3.7).

### 2.2 Declarations

```ebnf
CompilationUnit = { Declaration } .
Declaration     = FunDecl | DataClass | SealedInterface | DataObject
                | PlainClass | TypeAlias | TopProperty .
FunDecl         = [ "public" ] [ "tailrec" ] "fun" Ident "(" [ ParamList ] ")"
                  [ ":" Type ] ( "=" Expr | Block ) .
ParamList       = Param { "," Param } [ "," ] .
Param           = Ident ":" Type .
TopProperty     = [ "public" ] ( "val" | "var" ) Ident ":" Type "=" Expr .
TypeAlias       = [ "public" ] "typealias" Ident "=" Type .
DataClass       = [ "public" ] "data" "class" Ident "(" [ DataPropList ] ")"
                  [ ":" SealedParent ] .
DataPropList    = DataProp { "," DataProp } [ "," ] .
DataProp        = "val" Ident ":" Type .
PropList        = Prop { "," Prop } [ "," ] .
Prop            = ( "val" | "var" ) Ident ":" Type .
PlainClass      = [ "public" ] "class" Ident "(" [ PropList ] ")"
                  [ ":" SealedParent ] "{" { MemberFun } "}" .
SealedInterface = [ "public" ] "sealed" "interface" Ident .
DataObject      = [ "public" ] "data" "object" Ident [ ":" SealedParent ] .
SealedParent    = Ident .
MemberFun       = FunDecl .
```

A guest compilation unit has no package declaration and no imports. The
stdlib surface of Section 2.4 is admitted by name. The complete modifier set
is `public` (optional and redundant), `tailrec`, `data`, and `sealed`;
annotations are rejected. A unit that runs natively contains exactly one
`fun main()`: no parameters, block body, and no declared return type. That
`main` is the single exception to the explicit-return-type rule. User-defined
overloads, default arguments, named arguments outside generated data-class
`copy`, `vararg`, secondary constructors, `init` blocks, `open`, plain
(unsealed) interfaces, and inheritance outside a sealed hierarchy are rejected.

### 2.3 Types, statements, expressions

```ebnf
Type        = "Int" | "Long" | "Double" | "Boolean" | "String" | "Unit"
            | "List" "<" Type ">" | "MutableList" "<" Type ">"
            | "Set" "<" Type ">" | "Collection" "<" Type ">"
            | "Map" "<" Type "," Type ">" | "MutableMap" "<" Type "," Type ">"
            | "Pair" "<" Type "," Type ">"
            | Type "?" | "(" [ TypeList ] ")" "->" Type
            | Ident .                                   (* declared data types *)
TypeList    = Type { "," Type } .

Block       = "{" { Statement } [ Expr ] "}" .
Statement   = LocalProperty | DestructuringDecl | FunDecl | Assignment
            | WhileStmt | ForStmt | ReturnStmt | BreakStmt | ContinueStmt
            | Expr .
LocalProperty = ( "val" | "var" ) Ident [ ":" Type ] "=" Expr .
DestructuringDecl = "val" "(" Ident { "," Ident } [ "," ] ")" "=" Expr .
Assignment  = LValue ( "=" | "+=" | "-=" | "*=" | "/=" | "%=" ) Expr .
LValue      = Ident | Member | Index .
WhileStmt   = "while" "(" Expr ")" Block .
ForStmt     = "for" "(" Ident "in" ( Expr | Expr ".." Expr ) ")" Block .
ReturnStmt  = "return" [ Expr ] .
BreakStmt   = "break" .
ContinueStmt = "continue" .

Expr        = IfExpr | WhenExpr | Lambda | Binary | Elvis | IsTest | Unary
            | Call | Member | Index | Literal | Ident | CallableRef | ThisRef
            | ReturnStmt | Block | "(" Expr ")" .
IfExpr      = "if" "(" Expr ")" ( Block | Expr )
              [ "else" ( Block | Expr | IfExpr ) ] .
WhenExpr    = "when" [ "(" Expr ")" ] "{" { WhenBranch } [ ElseBranch ] "}" .
WhenBranch  = ( [ "is" ] Pattern | Expr ) "->" ( Expr | Block ) .
Pattern     = Ident .                                   (* sealed variant or literal *)
ElseBranch  = "else" "->" ( Expr | Block ) .
Lambda      = "{" [ LambdaParams "->" ] { Statement } [ Expr ] "}" .
LambdaParams = LambdaParam { "," LambdaParam } [ "," ] .
LambdaParam = Ident [ ":" Type ] .
Binary      = Expr ( "+" | "-" | "*" | "/" | "%" | "<" | "<=" | ">" | ">="
            | "==" | "!=" | "===" | "!==" | "&&" | "||" | "to" ) Expr .
Elvis       = Expr "?:" Expr .
IsTest      = Expr ( "is" | "!is" ) Type .
Unary       = ( "-" | "!" ) Expr .
Call        = Expr [ TypeArgs ] "(" [ ArgList ] ")" [ Lambda ] | Expr Lambda .
TypeArgs    = "<" Type { "," Type } ">" .
ArgList     = Argument { "," Argument } [ "," ] .
Argument    = [ Ident "=" ] Expr .
Member      = Expr ( "." | "?." ) Ident .
Index       = Expr "[" Expr "]" .
CallableRef = "::" Ident .
ThisRef     = "this" .
Literal     = IntLit | LongLit | DoubleLit | BoolLit | StringLit | "null" .
```

Operator precedence and associativity are Kotlin's. `to` constructs a `Pair`.
Trailing commas are admitted in parameter, property, and argument lists.
`break` and `continue` apply only to an enclosing `while` or `for`. A call or
member selection takes at most one trailing lambda, with or without
parentheses; a bare lambda after a callable expression invokes it. `this` is
admitted only inside member functions and property initializers. `return` in
expression position has type `Nothing`, so the elvis idiom
`gEval(x) ?: return null` is admitted. A lambda with parameters names them;
the implicit `it` parameter is rejected. An omitted arrow denotes a
zero-parameter lambda, as determined by the expected function type.
Named call arguments are admitted only for generated data-class `copy`.

A lambda parameter type comes from the expected function type; where no
expected type exists, every parameter carries an explicit annotation. Every
named function declares its return type explicitly, except `main` (Section
2.2). Local properties infer their type; top-level properties and class
properties declare theirs.

### 2.4 Admitted library surface

The surface is finite. Calls outside it are rejected.

| Type | Admitted members and constructors |
|---|---|
| `Int`, `Long`, `Double` | Section 3.1 arithmetic and comparison; `toInt()`, `toLong()`, `toDouble()`; `toString()` via templates |
| `Boolean` | `&&`, `\|\|`, `!`, `==`, `!=` |
| `String` | `+` (String, String), `==`, `!=`, `length`, `toLongOrNull()`, `toDoubleOrNull()` |
| `List<T>` | `listOf(vararg)`, `emptyList()`, `get(Int)`, `[]`, `size`, `isEmpty()`, `contains(T)`, `plus(List<T>)` (`+`), `map`, `filter`, `fold`, `any`, `all`, `firstOrNull()`, `take(Int)`, `drop(Int)`, `sorted()` |
| `Collection<T>`, `Set<T>` | `size`, `isEmpty()`, `contains(T)`, `map`, `filter`, `fold`, `any`, `all`, `firstOrNull()`, `toList()`; `Set` also `setOf(vararg)`, `emptySet()`, `plus(T)`, `minus(T)` |
| `MutableList<T>` | `mutableListOf()`, `add(T)`, `get(Int)`, `set(Int, T)`, `[]`, `[]=`, `size`, `isEmpty()`, `contains(T)` |
| `Map<K, V>` | `mapOf(vararg Pair)`, `emptyMap()`, `get(K)`, `[]`, `containsKey(K)`, `size`, `isEmpty()`, `keys: Set<K>`, `values: Collection<V>`, `plus(Pair<K, V>)` |
| `MutableMap<K, V>` | `mutableMapOf()`, `get(K)`, `put(K, V)`, `[]`, `[]=`, `containsKey(K)`, `remove(K)`, `size`, `isEmpty()` |
| `Pair<A, B>` | `a to b`, `.first`, `.second`, destructuring |
| functions | `map`, `filter`, `fold`, `any`, `all` as listed above; invocation `f(x)` |
| `kotlin.math` | `abs`, `min`, `max`, `sqrt`, `floor`, `ceil`, `pow` |
| `java.lang.Math` | `addExact`, `subtractExact`, `multiplyExact`, `negateExact` |
| output | `print`, `println` (Section 3.7) |

Destructuring `val (a, b) = e` is admitted for `Pair` and for data classes.
A data class admits exactly: its primary constructor call, `copy` with named
properties, the derived `component1` through `componentN` selectors behind
destructuring, structural `==`/`!=`, and `===`/`!==` identity. User-declared
generic functions and classes, extension
functions, `operator fun`, `infix`, annotations, delegation (`by`), `lateinit`,
`Sequence`, coroutines, reflection, and the remaining stdlib are rejected.

Collection builders infer their element and key/value types from an expected
type or compatible arguments; otherwise the call requires explicit type
arguments. `map` takes `(T) -> R` and returns `List<R>`; `filter` takes
`(T) -> Boolean`; `fold` takes an initial `R` and `(R, T) -> R`.
`any` and `all` take `(T) -> Boolean`. `sorted` admits only `Int`, `Long`,
`Double`, or `String` elements and uses their native ordering.
Read-only collection interfaces do not guarantee deep immutability.

## 3. Type and runtime rules

### 3.1 Numeric types, literals, results, conversions, overflow, division

Admitted numeric types are `Int`, `Long`, `Double`. `Byte`, `Short`, `Char`,
`Float`, unsigned types, and arbitrary-precision types are rejected.

**Literal inference.** The pinned compiler determines the type of each
literal and its unary minus. An unsuffixed integer normally infers `Int`
when it fits, otherwise `Long`; expected `Int` or `Long` types can determine
a representable literal's type. An `L` suffix requires `Long`.
`val x: Long = 1` is valid; an `Int` variable does not widen implicitly.
The native special case `-2147483648` can have type `Int`.
The pinned compiler rejects `-9223372036854775808L` as out of range.
Express the minimum `Long` as `-9223372036854775807L - 1L`.
A floating literal requires a decimal point or exponent and has type
`Double`; `f` and `F` suffixes are rejected.

**Result types.** Arithmetic is same-type only: for `T` in `Int`, `Long`,
`Double`, `T op T` yields `T` for `+ - * /`, and `Int`/`Long` `%` yields the
operand type. Mixed-width arithmetic is rejected even though Kotlin widens
(`1 + 2L` is host-valid Kotlin): the subset requires an explicit conversion
wherever widths differ. Unary `-` preserves its operand type. Comparisons
`< <= > >=` require same-type `Int`/`Long`/`Double` operands and yield
`Boolean`. Equality `== !=` requires the same type on both sides; data classes
compare structurally, `===`/`!==` ask reference identity on class instances.

**Conversions.** `Int.toLong()` is exact. `Long.toInt()` retains the low
32 bits and interprets them as a signed `Int`; it does not saturate.
`Double.toInt()` and `Double.toLong()` truncate toward zero, saturate
out-of-range finite values and infinities, and return zero for NaN.
`Int.toDouble()` and `Long.toDouble()` use native IEEE-754 conversion.
Same-type conversions return the same value. No conversion is implicit.

**Overflow.** `Int` and `Long` `+ - *` and unary `-` wrap in two's-complement
arbitration of the operand width (`Int.MIN_VALUE` negation wraps to itself;
`Long.MIN_VALUE / -1` wraps to `Long.MIN_VALUE`). Wrapping is the defined
default and matches native JVM execution. The checked forms
`Math.addExact`, `Math.subtractExact`, `Math.multiplyExact`, `Math.negateExact`
raise guest error `Overflow` instead of wrapping.

**Division.** `Int`/`Long` `/` truncates toward zero; `%` takes the sign of
the dividend. Integer `/` or `%` by zero raises guest error `DivisionByZero`.
`Double / Double` follows IEEE 754: division by `0.0` produces `Infinity` or
`NaN`, never an error. `%` on `Double` is rejected.

### 3.2 Boolean-only conditions

`if`, subject-less `when` branches, `while`, and `for` termination rely only
on `Boolean`. There is no truthiness: `0`, `""`, and `null` are not conditions.
`&&` and `||` require `Boolean` operands, short-circuit left to right, and
their right operand is a `Boolean` expression. `!` requires `Boolean`. A
nullable `Boolean` is not a condition (Section 3.3). This rule is enforced at
source type checking and inside every guest evaluator kernel: a guest program
whose conditional holds a non-Boolean value is rejected before execution.

`when` rules, split by form. Every admitted `when` is exhaustive in its form.
A `when` expression over a non-null sealed subject lists every variant by
`is` (or by the object name for a `data object`) and uses no `else`; a
non-exhaustive `when` expression is a Kotlin compile error and therefore
host-invalid. A `when` statement may be partial in Kotlin; the subset rejects
a partial `when` of any form as `NonExhaustiveWhen`, and rejects an `else` on
a non-null sealed subject as `SealedWhenElse` (Section 6.2). A subject-less
`when` expression uses Boolean guard branches and requires `else`
(host-invalid without it). A `when` over any other subject admits value and
`is` branches and requires `else`. Exhaustiveness is by construction, so
adding a variant is a compile error at every dispatch site.

### 3.3 Nullability

Nullable types `T?` admit exactly: safe call `?.`, elvis `?:`, comparisons
`== null` and `!= null`, and function results of `Map.get`, `firstOrNull`,
`remove`, `toLongOrNull`, `toDoubleOrNull`. Force unwrap `!!` is rejected
(Section 6.2), as are unchecked casts. An `is`/`!is` test or comparison with
`null` narrows a stable binding on the branch where its non-null type is known;
`&&` on its true path, `||` on its false path, and `!` preserve that flow.
Smart casts apply to `val` locals, parameters, and `var` locals with no lambda
captures; they never apply to properties or captured `var` bindings, which
must go through `?.` or `?:`.

### 3.4 `val`, `var`, capture, shadowing

`val` introduces an immutable binding; reassignment is host-invalid and guest
evaluators must reject it at type checking. `var` introduces a mutable
binding. Binding or property reassignment requires `var`. An indexed write
requires a mutable collection, even when its binding is `val`.

Closures capture the binding, not a copy. A lambda that captures a `var`
observes later assignments from any closure sharing that binding, and its own
assignments are observed by the others; the binding lives until no closure
can reach it. Escaping closures over `var` are admitted and MUST behave this
way in interpreted and compiled execution alike. A captured `val` cannot be
rebound, but mutations of an object it references remain visible.

Lexical scope is block structure. An inner binding shadows an outer binding of
the same name; the inner binding is the one referenced from its scope, and a
closure created in the inner scope captures the inner binding.

### 3.5 Structured and recursive data

`data class` declares immutable positional properties, structural equality, a
`copy` constructor call, and destructuring. `sealed interface` declares a
closed variant family whose implementations are `data class` or `data object`
members of the same compilation unit. Variants may reference their own family,
so recursive data is admitted directly. A plain `class` with `val`/`var`
constructor properties and member functions is admitted for stateful records
(environment frames, accumulators); it has no inheritance. Mutable properties
and collections may form object cycles, including a recursive closure's
environment. Preserve object identity and aliases. Read-only collection
interfaces prohibit writes through that interface, not mutation through a
mutable alias. A `Map` lookup of an absent key returns `null`.

### 3.6 Functions and recursion

Named functions are first-class values of function type `(T1, ...) -> R`;
lambdas are anonymous functions with the capture rules of Section 3.4.
Application is call-by-value with left-to-right operand evaluation. Arity and
parameter types are checked before execution. Recursion is expressed with
named functions: self-recursion and mutual recursion are admitted among
top-level functions and among local functions. A `val` lambda that references
its own name is rejected; use a named function. `tailrec` is admitted on a
named function whose self-calls are in tail position and changes no observable
behavior. Recursion depth is a resource, not a contract: a stack overflow is
not a guest error category.

### 3.7 Output

`print(x)` writes, `println(x)` writes and appends `"\n"`, `println()` writes
one `"\n"`. Arguments and template interpolations admit exactly `String`,
`Long`, `Double`, `Boolean`; printing a structured value is rejected, and
programs render structured data with their own functions returning `String`.
Output is emitted in program order, and it is the only observable effect.

Rendering is pinned so native and engine executions agree:

| Value | Rendering |
|---|---|
| `Long` | base 10, `-` for negative, no grouping; `Long.MIN_VALUE` is `-9223372036854775808` |
| `Double` | `Double.toString()` on the pinned JDK: shortest round-trip form, with `NaN`, `Infinity`, `-Infinity` |
| `Boolean` | `true`, `false` |
| `String` | its characters verbatim; no quotes, no escaping |

The string escape set of Section 2.1 is the only escaping; a rendered string
is written back as its characters, not as a quoted literal.

### 3.8 Guest runtime errors

Type errors, unbound names, and shape violations are rejected before
execution. The admitted runtime categories are:

| Guest error | Raised by | Native surface |
|---|---|---|
| `DivisionByZero` | integer `/` or `%` by zero | `java.lang.ArithmeticException` |
| `Overflow` | `Math.*Exact` overflow | `java.lang.ArithmeticException` |
| `IndexOutOfBounds` | `List.get`/`[]` with a bad index | `java.lang.IndexOutOfBoundsException` |
| `UnassignedRead` | kernel-level read of an unassigned recursive binding | none; native typing rejects the program |
| `UnassignedRegister` | machine read of a never-assigned register | none; engine category |
| `ShapeFault` | machine value of the wrong shape for an operand (jump target, test flag, procedure, address) | none; engine category |

Engine implementations report the category, never raw host exceptions.
Diagnostic *wording* is never compared across implementations; the category
and the position (Section 10 note on token locations) are the observable.

## 4. Experimental modules

Experimental forms are excluded from core. Each experiment has a named
module, a named execution mode, explicit admission rules, and finite
behavioral invariants. A guest program mixing core and experimental forms is
admitted only when every experimental form is reachable in its own mode.

### 4.1 Lazy module (mode `Lazy`)

Admitted constructors: `thunk { ... } : Thunk<T>`, `force(t: Thunk<T>): T`,
`lazyPair(head: T, tail: Thunk<List<T>>): List<T>`, `lazyEnd: List<T>`.

Invariants: a thunk computes at most once and memoizes; `force` on a
memoized thunk repeats no work and no effect; primitive operations remain
strict; sequencing forces its non-final actions; lazy procedure application
delays compound arguments and forces the operator before application.
A per-parameter strictness annotation in lazy programs chooses strict or
delayed arguments. The lazy-list printer writes at most ten elements and then
`...`. Reference model: a finite forcing instrument counts force attempts,
distinct computations, and effect order per experiment.

### 4.2 Search module (mode `Search`)

Admitted constructors: `choose(vararg alternatives: T): T`,
`demand(condition: Boolean)`, `chooseRandom(vararg alternatives: T): T`,
`setPermanent { ... }`, `ifFail({ ... }, { ... })`, and a seeded generator
`seededRandom(seed: Long): Random`.

Invariants: `choose` commits left to right; failure of a later `demand`
backtracks to the most recent untried alternative; `chooseRandom` permutes
alternatives by the seeded generator and is reproducible from its seed;
`setPermanent` assignments survive backtracking; `ifFail` answers its second
block exactly when the first fails. The choice counter counts one choice per
entered alternative. Default core never backtracks.

The host driver `SearchModule.run(source)` explores to exhaustion.
`SearchModule.run(source, maxAnswers)` with a positive limit stops after that
many successful `main` results without forcing further alternatives.
`SearchModule.run(source, maxAnswers, maxChoices)` also stops before entering
an alternative beyond its positive choice horizon. Bounded runs preserve
effects already emitted and count only alternatives entered.

### 4.3 Query DSL (host constructors)

Query programs are domain data built from host constructors, not new host
syntax. The DSL vocabulary is:

```kotlin
sealed interface QTerm
data class QSym(val name: String) : QTerm
data class QVar(val name: String) : QTerm
data class QList(val items: List<QTerm>, val tail: QTerm?) : QTerm

sealed interface QQuery
data class QPattern(val term: QTerm) : QQuery
data class QAnd(val parts: List<QQuery>) : QQuery
data class QOr(val parts: List<QQuery>) : QQuery
data class QNot(val part: QQuery) : QQuery
data class QGuard(val predicate: (List<QTerm>) -> Boolean, val args: List<QTerm>) : QQuery
data class QUnique(val part: QQuery) : QQuery

data class QRule(val conclusion: QTerm, val body: QQuery)
data class QFact(val term: QTerm)
data class QFrame(val bindings: Map<QVar, QTerm>)
```

Semantics: unification over `QTerm`; a `QFrame` is an immutable map of
variable to term with no sentinel for unbound; failed matching answers `null`
frames, never a marker value. `QGuard` runs a host predicate when its
arguments are bound. `QUnique` succeeds for exactly one match. Answer
rendering is pinned: `QSym` renders its name, `QVar` renders `?name`, a proper
`QList` renders `[a, b, c]`, an improper one `[a, b | rest]`; an answer line is
`?name = <rendered term>`. Database insertion is chronological; the data
base indexes facts by head symbol. The module exposes an explicit loop
detector and a deduplicating answer view as constructors of its driver.

### 4.4 Machine DSL (host constructors)

Register machines are domain data built from the sealed constructor family
already shaped for this edition:

```kotlin
sealed interface Stmt
data class Label(val name: String) : Stmt
data class Assign(val reg: String, val src: Source) : Stmt
data class Test(val cond: Cond) : Stmt
data class Branch(val label: String) : Stmt
data class Goto(val to: GotoTarget) : Stmt
data class Save(val reg: String) : Stmt
data class Restore(val reg: String) : Stmt
data class Perform(val act: Action) : Stmt
```

with operand forms `Source` = register, constant, or label reference;
`Cond` = named operation over sources; `Action` = named operation run for
effect; `GotoTarget` = label or register. A machine is
`Machine(registers, ops, controller)` where `ops` maps operation names to
host functions over guest values. Instructions cover exactly the summary
forms of 5.1.5. A label used where an operation operand belongs is a typed
machine-program error (the lesson of 5.9), as is a duplicate label
definition or an unknown label target. Registers start unassigned; a read
before any write raises `UnassignedRegister`. Stack save/restore discipline
is the machine's own state; monitored counters (pushes, depth, high-water,
instruction count) are machine outputs. Trace rendering is pinned to one line
per instruction: `label: <instruction text>` with instruction text
`assign r <- s`, `test c`, `branch l`, `goto t`, `save r`, `restore r`,
`perform a` over the constructor fields.

### 4.5 Module composition and boundaries

Each guest unit selects an explicit mode before admission. Core constructs
remain available in every mode. The matrix omits the implicit `Core` from
combined experiment rows. Reject an unlisted experiment combination as
`UnsupportedComposition`.

| Mode set | Admitted | Lesson |
|---|---|---|
| `Core` | yes | default strict semantics |
| `Core` + `Lazy` | yes | 4.2, 4.2.3 |
| `Core` + `Search` | yes | 4.3 |
| `Core` + `Query` | yes | 4.4 |
| `Query` + `Lazy` | yes | answer streams force lazily (4.71-4.74) |
| `Query` + `Search` | yes | 4.78 query as nondeterministic program |
| `Lazy` + `Search` | no | memoized forcing and backtracking have no lesson-pinned interaction |
| `Lazy` + `Search` + `Query` | no | inherits the rejected pair |

Per-module invariants compose in program order: output effects follow the
program, the choice counter counts only choices, a `Lazy` thunk computes at
most once per run, and a `Search` attempt re-runs its effects on backtracking
unless carried by `setPermanent`.

The query implementation row in Section 7 groups separate exercise programs.
It does not require one program to combine `Lazy` and `Search`: 4.71-4.74
use `Query` with `Lazy`; 4.78 uses `Query` with `Search`.

In `Lazy` mode, parameter annotations `@Strict` and `@Delayed` are the only
admitted annotations. Unannotated compound-procedure parameters are delayed;
primitive arguments and `@Strict` parameters are eager. `Thunk<T>` is a
mode-only type. `thunk` takes one zero-argument body; `force` takes one thunk;
`lazyPair` takes a head and a delayed tail; `lazyEnd` takes no arguments.

In `Search` mode, `choose` and `chooseRandom` are intrinsic syntax nodes,
not eager ordinary calls. They accept zero or more alternatives of one type.
Zero alternatives fail; otherwise evaluate one alternative per attempt.
`demand` takes one Boolean expression and fails when false. `setPermanent`
takes one zero-argument body whose writes survive rollback. `ifFail` takes
two zero-argument bodies and enters the second only when the first exhausts
its answers. `seededRandom` takes one `Long` and returns the mode-only
`Random` type. Randomized choices use the program's explicit seeded stream.
Intrinsic variable arity does not admit user-defined `vararg` declarations.

Typed data and guest source are different planes. The constructors of 4.3 and
4.4 build typed domain values (`QTerm`, `QQuery`, `QRule`, `QFrame`, `Stmt`
and friends); that data is never re-parsed as guest source and is admitted
exactly as the constructor calls that build it. The C text produced by
5.51/5.52 is a generated artifact string, never guest source. Only Section 2
compilation units are guest source, and only they receive the Section 3 type
rules and Section 6 admission checks.

## 5. Guest evaluator kernel witness (native VERIFIED; guest obligations UNRUN)

The witness below is guest source: every construct it uses is admitted by
Sections 2-3. It implements evaluation of a guest core language over explicit
syntax-tree values and mutable environment frames. It calls no production
evaluator and imports nothing. Its role is a native feasibility witness: it
proves the admitted forms can express an eval/apply kernel in real Kotlin,
and it is native VERIFIED (Section 8.1). It is not the full-source
self-interpreter of the completed edition. The self-interpretation
obligation of Section 5.2 binds the production guest evaluator kernel that
Task 7/10 builds over the full admitted grammar, and the teaching evaluator
MUST interpret that kernel as guest source and reproduce its direct run.

```kotlin
// BEGIN WITNESS guest-kernel
// Guest evaluator kernel. Guest source wholly in admitted forms.
// No imports; no call to any production evaluator.

sealed interface GExpr

data class GNum(val n: Long) : GExpr
data class GBool(val b: Boolean) : GExpr
data class GVar(val name: String) : GExpr
data class GLam(val param: String, val body: GExpr) : GExpr
data class GApp(val fn: GExpr, val arg: GExpr) : GExpr
data class GLet(val name: String, val value: GExpr, val body: GExpr) : GExpr
data class GLetRec(val name: String, val value: GExpr, val body: GExpr) : GExpr
data class GIf(val test: GExpr, val onTrue: GExpr, val onFalse: GExpr) : GExpr
data class GAdd(val left: GExpr, val right: GExpr) : GExpr
data class GMul(val left: GExpr, val right: GExpr) : GExpr
data class GLt(val left: GExpr, val right: GExpr) : GExpr
data class GSet(val name: String, val value: GExpr) : GExpr

sealed interface GValue

data class GNumV(val n: Long) : GValue
data class GBoolV(val b: Boolean) : GValue
data class GClosV(val param: String, val body: GExpr, val env: GFrame) : GValue
data object GUnassigned : GValue

class GFrame(val cells: MutableMap<String, GValue>, val parent: GFrame?) {
    fun lookup(name: String): GValue? {
        var here: GFrame? = this
        while (here != null) {
            val current: GFrame = here ?: return null
            val hit = current.cells[name]
            if (hit != null) {
                return hit
            }
            here = current.parent
        }
        return null
    }

    fun assign(name: String, value: GValue): Boolean {
        var here: GFrame? = this
        while (here != null) {
            val current: GFrame = here ?: return false
            if (current.cells.containsKey(name)) {
                current.cells[name] = value
                return true
            }
            here = current.parent
        }
        return false
    }
}

fun gEval(expr: GExpr, env: GFrame): GValue? =
    when (expr) {
        is GNum -> GNumV(expr.n)
        is GBool -> GBoolV(expr.b)
        is GVar -> {
            val found = env.lookup(expr.name)
            if (found == null || found is GUnassigned) {
                null
            } else {
                found
            }
        }
        is GLam -> GClosV(expr.param, expr.body, env)
        is GApp -> {
            val fn = gEval(expr.fn, env) ?: return null
            if (fn is GClosV) {
                val arg = gEval(expr.arg, env) ?: return null
                val frame = GFrame(mutableMapOf(fn.param to arg), fn.env)
                gEval(fn.body, frame)
            } else {
                null
            }
        }
        is GLet -> {
            val value = gEval(expr.value, env) ?: return null
            val frame = GFrame(mutableMapOf(expr.name to value), env)
            gEval(expr.body, frame)
        }
        is GLetRec -> {
            val frame = GFrame(mutableMapOf(expr.name to GUnassigned), env)
            val value = gEval(expr.value, frame) ?: return null
            frame.cells[expr.name] = value
            gEval(expr.body, frame)
        }
        is GIf -> {
            val test = gEval(expr.test, env) ?: return null
            if (test !is GBoolV) {
                null
            } else if (test.b) {
                gEval(expr.onTrue, env)
            } else {
                gEval(expr.onFalse, env)
            }
        }
        is GAdd -> {
            val left = gEval(expr.left, env) ?: return null
            val right = gEval(expr.right, env) ?: return null
            if (left is GNumV && right is GNumV) {
                GNumV(left.n + right.n)
            } else {
                null
            }
        }
        is GMul -> {
            val left = gEval(expr.left, env) ?: return null
            val right = gEval(expr.right, env) ?: return null
            if (left is GNumV && right is GNumV) {
                GNumV(left.n * right.n)
            } else {
                null
            }
        }
        is GLt -> {
            val left = gEval(expr.left, env) ?: return null
            val right = gEval(expr.right, env) ?: return null
            if (left is GNumV && right is GNumV) {
                GBoolV(left.n < right.n)
            } else {
                null
            }
        }
        is GSet -> {
            val value = gEval(expr.value, env) ?: return null
            if (env.assign(expr.name, value)) {
                GBoolV(true)
            } else {
                null
            }
        }
    }

fun factorial(n: Long): Long = if (n < 2L) 1L else n * factorial(n - 1L)

fun guestFactorial(n: Long): GExpr =
    GLetRec(
        "fact",
        GLam(
            "n",
            GIf(
                GLt(GVar("n"), GNum(2L)),
                GNum(1L),
                GMul(GVar("n"), GApp(GVar("fact"), GAdd(GVar("n"), GNum(-1L)))),
            ),
        ),
        GApp(GVar("fact"), GNum(n)),
    )

fun sharedCellProgram(): GExpr =
    GLet(
        "count",
        GNum(0L),
        GLet(
            "bump",
            GLam("u", GSet("count", GAdd(GVar("count"), GNum(1L)))),
            GLet(
                "read",
                GLam("u", GVar("count")),
                GLet(
                    "unused",
                    GApp(GVar("bump"), GNum(0L)),
                    GApp(GVar("read"), GNum(0L)),
                ),
            ),
        ),
    )

fun main() {
    val grown = gEval(guestFactorial(10L), GFrame(mutableMapOf(), null))
    val observed = gEval(sharedCellProgram(), GFrame(mutableMapOf(), null))
    val factorialOk = grown is GNumV && grown.n == factorial(10L)
    val captureOk = observed is GNumV && observed.n == 1L
    if (factorialOk && captureOk) {
        println("ok")
    } else {
        println("mismatch")
    }
}
// END WITNESS guest-kernel
```

Native result (VERIFIED; evidence
local://modern-sicp-kotlin-contract-native-results.json): compile exit 0, run
exit 0, stdout exactly `ok` followed by one line ending.
`guestFactorial(10L)` interprets guest recursion; `factorial` runs the same
computation directly; `sharedCellProgram` requires two closures over one
mutable binding to observe each other's writes. Guest-engine runs of this file
(Task 7/10) remain UNRUN.

### 5.1 Construct closure

The kernel-in-guest-source direction of the coextension is this table: every
construct the kernel uses maps to an admitted form, so the subset can express
its own evaluator. The converse direction, that the production kernel's
object language covers the full admitted grammar, is the Task 7/10 obligation
of Section 5.2.

| Kernel construct | Admitted form |
|---|---|
| `GExpr`, `GValue` sealed families | 3.5 sealed interface + data class / data object |
| `GFrame` | 3.5 plain class with `MutableMap` property |
| `gEval` recursion over `Expr` | 3.6 named-function recursion, exhaustive `when` |
| closures carrying `GEnv` | 3.4 capture, 3.5 recursive data |
| `GUnassigned` and its rejection | 3.8 `UnassignedRead` discipline |
| mutable cells, tie-the-knot `GLetRec` | 3.4 `var` semantics via mutable maps |
| `?:` on evaluation results, `return null` | 3.3 elvis; 2.3 `ReturnStmt` as expression |
| `cells[k]` reads and `cells[k] = v` writes | 2.3 `Index` read and `LValue` write |
| `is` / `!is` narrowing in `gEval` | 2.3 `IsTest`; 3.3 smart casts |
| `this` inside `GFrame` methods | 2.3 `ThisRef` |
| guest arithmetic and comparisons | 3.1 same-type `Long` rules |
| Boolean-only guest conditionals | 3.2 |
| `main`, `println` | 3.7 |

The full guest grammar (Section 2) extends this core one form at a time: each
extra form desugars to, or is evaluated exactly like, a core form above.

### 5.2 Self-interpretation invariant

For every evaluator the chapters teach (the self-interpreter obligations of
Section 7): the evaluator's source is guest source whose object language is
the full admitted grammar; the teaching engine parses and type-checks that
source, runs it on a translated guest program, and the result must equal
running the evaluator directly on the same program. The same invariant binds
the compiled evaluator (5.50): the compiler input is guest source and the
machine run must agree with direct execution. A call to a production
evaluator, a reflection hook, or an engine escape hatch does not satisfy this
invariant. The witness of Section 5 establishes the construct-coverage half
of this invariant for its core object language and is native VERIFIED; the
full-grammar half is a guest obligation of Task 7/10 and is UNRUN.

Two diagnostic planes stay separate. Ill-typed guest *source* is rejected
statically, before any guest effect, by the pinned compiler or the teaching
engine's checker; Sections 3 and 6 diagnostics fire here, and a dynamic
internal value representation must never route invalid source around this
check. An invalid *represented* program, meaning program data a kernel
interprets, is a runtime outcome of the represented language: the kernel
answers a guest error category (the witness answers `null`), never a Kotlin
compile diagnostic. The two planes must not be conflated in either direction.

## 6. Rejection catalog

### 6.1 Host-invalid (rejected by the compiler before any effect)

| Category | Example |
|---|---|
| Non-Boolean condition | `if (x) ...` with `x: Long` |
| Nullable condition | `if (b) ...` with `b: Boolean?` |
| Reassignment of `val` | `val x = 1L; x = 2L` |
| Implicit width change | `fun f(n: Int): Long { return n }` |
| Mixed `==` types | `someLong == someInt` |
| Literal out of range | `val x = 99999999999999999999` |
| Non-exhaustive `when` expression | sealed dispatch missing a variant (3.2) |
| Undeclared name, wrong arity, wrong argument type | any call outside its declaration |
| Type inference (`TypeInference`) | `val xs = emptyList()` with no expected or explicit element type |

The subset gate reports these as `HostInvalid` with a category and position.
Compiler diagnostic wording is never part of the contract.

### 6.2 Host-valid but unsupported (rejected by the subset gate before execution)

| Category | Form | Why rejected |
|---|---|---|
| `MixedWidthArithmetic` | `1 + 2L` | result types must be determined by one width |
| `ForceUnwrap` | `s!!` | crashes bypass the guest error categories |
| `UncheckedCast` | `v as T` | breaks the sealed exhaustiveness guarantee |
| `Exceptions` | `try`, `catch`, `throw` | errors are values and categories (3.8) |
| `Reflection` | `::class`, `KClass`, `javaClass` | dynamic dispatch outside the grammar |
| `Delegation` | `by lazy`, `by` anything | second lazy mechanism (Section 4.1 owns laziness) |
| `Sequence` | `Sequence`, `yield`, suspending code | lazy data belongs to the lazy module |
| `UserGenerics` | `fun <T>`, generic classes | open-ended types beyond Section 2.3 |
| `ExtensionsOperators` | extension functions, `operator fun`, `infix` | new call syntax outside the grammar |
| `StructuredOutput` | `println(structuredValue)` | rendering is program-defined (3.7) |
| `CharSurface` | char literals, `String.get` | no `Char` type in the subset |
| `HexBinaryLiterals` | `0x...`, `0b...` | literal grammar is decimal (2.1) |
| `NamedDefaultVarargs` | user-declared defaults or varargs; named arguments outside generated `copy` | only the listed library and experiment signatures vary in arity |
| `PropertySmartCast` | smart cast on a property or captured `var` | 3.3 narrowing rules |
| `NonExhaustiveWhen` | partial `when` statement | exhaustiveness is required in every form (3.2) |
| `SealedWhenElse` | `else` over a non-null sealed subject | variant dispatch is closed (3.2) |
| `ImplicitIt` | implicit `it` parameter | lambda parameters are named (2.3) |
| `UnsupportedComposition` | `Lazy` with `Search` (with or without `Query`) | no lesson-pinned interaction (4.5) |
| `RecursiveValLambda` | `val f = { ... f(...) }` | 3.6: recursion is named functions |
| `RawStrings` | triple-quoted strings | string grammar is 2.1 |
| `MiscKotlin` | `object` expressions, enums, annotations, `lateinit`, `this` outside members, `in` outside `for` | not in the closed grammar |

Chapters 0-3 may show host-only Kotlin (`by lazy`, `Sequence`, the runtime
stream type) as host-language teaching; such listings are host code and are
not guest source. Any listing fed to a teaching engine MUST satisfy this
contract instead.

## 7. Chapter 4 and 5 lesson-family map

Every exercise in each range is covered by the row's admitted forms; a row
marked experimental uses only the named module. Exercises added by this
edition are listed with their base exercise. No family may be dropped or
partially migrated.

### Chapter 4

| Family (book sections) | Exercises | Guest forms | Class |
|---|---|---|---|
| Core evaluation: apply, dispatch, sequencing, operand order (4.1.1) | 4.1-4.5 | kernel core forms, `when` dispatch, 3.2 conditions | core |
| Expression representation and derived forms (4.1.2) | 4.6-4.10 | sealed syntax trees; derived forms desugar in guest code; new syntax extends the sealed family, never a foreign parser | core |
| Evaluator data structures: environments, procedures, truth (4.1.3) | 4.11-4.13 | frames as data (3.5), closures (3.4), mutable maps, Boolean truth (3.2) | core |
| Running the evaluator as a program (4.1.4) | 4.14 | primitive table as `Map`; user-defined and primitive mapping side by side | core |
| Data as programs (4.1.5) | 4.15 | explicit syntax-tree values (no eval of text); halting answer is prose | core |
| Internal definitions and recursive bindings (4.1.6) | 4.16-4.20, 4.20a | tie-the-knot `GLetRec`, `UnassignedRead`, shadowing (3.4) | core |
| Analysis separated from execution (4.1.7) | 4.21-4.24 | analysis returns closures; timing measured by counters | core |
| Lazy evaluation semantics (4.2.1-4.2.2) | 4.25-4.26, 4.26a, 4.27-4.31 | lazy module: `thunk`/`force`, strict primitives, memoization, sequencing forces, per-parameter strictness | `Lazy` |
| Lazy lists (4.2.3) | 4.32-4.34 | `lazyPair`/`lazyEnd`, ten-element printer rule | `Lazy` |
| Search: choice and backtracking (4.3.1) | 4.35, 4.35a, 4.36-4.37 | `choose`, `demand`, choice counter, seeded generator | `Search` |
| Search puzzle programs (4.3.2) | 4.38-4.44 | `Search` over `List`/`Map` data; 4.41 is core-only | `Search` + core |
| Parsing and generation by search (4.3.2) | 4.45-4.49 | word grammars as `List<String>` and `Search`; no text parsing | `Search` |
| Search engine extensions (4.3.3) | 4.50-4.54 | `chooseRandom` (seeded), `setPermanent`, `ifFail`; the guard-as-procedure argument is prose | `Search` |
| Query retrieval: facts, rules, patterns (4.4.1) | 4.55-4.63, 4.68-4.69 | query DSL constructors, unification over `QTerm` | `Query` |
| Query semantics: ordering, duplicates, loops, statistics (4.4.2-4.4.3) | 4.60, 4.64-4.67, 4.65a | deduplicating answer view, loop detector, chronology | `Query` |
| Query implementation: driver, matching, unification, database, streams, frames (4.4.4.1-4.4.4.8) | 4.70-4.79 | `QFrame` maps, lazy module streams for interleaving (4.71-4.74), dispatch table as `Map`, search-mode redesign (4.78), environment-model evaluator (4.79) | `Query` + `Lazy` + `Search` + core |

### Chapter 5

| Family (book sections) | Exercises | Guest forms | Class |
|---|---|---|---|
| Machine language and design (5.1.1-5.1.2) | 5.1-5.4 | machine DSL `Stmt` constructors, operands, operations | core |
| Subroutines and stack recursion (5.1.3-5.1.4) | 5.5-5.6 | `Save`/`Restore`/`Goto`, stack discipline | core |
| Machine model and assembler (5.2.1-5.2.3) | 5.7-5.13, 5.12a | `Machine` constructor, label scan, typed machine errors (Section 4.4), `AssemblySummary` census data | core |
| Machine instrumentation and performance (5.2.4) | 5.14-5.19 | monitored stack counters, instruction counting, tracing, breakpoints | core |
| Memory as vectors, allocation, collection (5.3) | 5.20-5.22 | vector/pair memory as `List`/`MutableList` data with explicit indices; allocation statistics | core |
| Explicit-control evaluator (5.4.1-5.4.3) | 5.23-5.25 | machine controller over kernel forms; 5.25 switches to `Lazy` semantics | core + `Lazy` |
| Running and monitoring the evaluator (5.4.4) | 5.26-5.30 | stack counters, tail-recursion comparison, typed guest errors (3.8) | core |
| Compiler structure and code generation (5.5.1-5.5.5) | 5.31-5.38 | compile sealed guest syntax trees to `List<Stmt>`; preserving, open coding | core |
| Lexical addressing (5.5.6) | 5.39-5.44 | compile-time environment as `List<List<String>>`; lexical addresses as data | core |
| Compiler and evaluator interfaces (5.5.7) | 5.45-5.48 | cross-mode runs, nested compiled calls, compile-and-run | core |
| Self-interpreter and compiler endgames (5.4.4, 5.5) | 5.49-5.52 | 5.49 read-compile-execute machine; 5.50 compiled guest evaluator kernel (Section 5); 5.51-5.52 emit C as `String` artifacts | core |

Self-interpreter and compiler obligations, named: 4.1 family (guest evaluator
kernel), 5.23-5.25 (explicit-control evaluator), 5.45-5.50 (compiled
evaluator and its interfaces), 5.51-5.52 (C backends, with C text as a
generated artifact and never guest source). Each must satisfy Section 5.2
where it interprets or compiles guest source.

## 8. Native compile and run oracle

Toolchain pins (`docs/toolchain-pins.md`, `kotlin/gradle/libs.versions.toml`):
Kotlin 2.4.20 (K2), JDK 25, Gradle 9.7.0. The system `kotlinc` on PATH is a
stale 1.3-SNAPSHOT and MUST NOT be used. `./gradlew` is the pin source and the
edition gate, not the per-witness oracle: witness files are standalone and
outside the Gradle source sets. The oracle invokes the pinned compiler
artifact that the Gradle build resolved into the local cache, on the pinned
JDK.

Oracle driver (from the repository root):

```sh
STDLIB=/home/alpha/.gradle/caches/modules-2/files-2.1/org.jetbrains.kotlin/kotlin-stdlib/2.4.20/94c1451890d164aed65d43b3db0b7ecef7c183e7/kotlin-stdlib-2.4.20.jar
KC='java -cp /home/alpha/.gradle/caches/modules-2/files-2.1/org.jetbrains.kotlin/kotlin-compiler-embeddable/2.4.20/23b042894dcabe39edd0f8bd30b7be0f25fe9933/kotlin-compiler-embeddable-2.4.20.jar:/home/alpha/.gradle/caches/modules-2/files-2.1/org.jetbrains.kotlin/kotlin-build-tools-api/2.4.20/846d16ddb9652941ac68fe45dad92524358f0b89/kotlin-build-tools-api-2.4.20.jar:/home/alpha/.gradle/caches/modules-2/files-2.1/org.jetbrains.kotlin/kotlin-stdlib/2.4.20/94c1451890d164aed65d43b3db0b7ecef7c183e7/kotlin-stdlib-2.4.20.jar:/home/alpha/.gradle/caches/modules-2/files-2.1/org.jetbrains.kotlin/kotlin-script-runtime/2.4.20/c5d2fe2964e557067ce970573bbc7ffa74f57cf4/kotlin-script-runtime-2.4.20.jar:/home/alpha/.gradle/caches/modules-2/files-2.1/org.jetbrains.kotlin/kotlin-reflect/1.6.10/1cbe9c92c12a94eea200d23c2bbaedaf3daf5132/kotlin-reflect-1.6.10.jar:/home/alpha/.gradle/caches/modules-2/files-2.1/org.jetbrains.kotlin/kotlin-daemon-embeddable/2.4.20/6216c8108a609251b8314bcc0f0ef7eb48e3b67a/kotlin-daemon-embeddable-2.4.20.jar:/home/alpha/.gradle/caches/modules-2/files-2.1/org.jetbrains.kotlinx/kotlinx-coroutines-core-jvm/1.8.0/ac1dc37a30a93150b704022f8d895ee1bd3a36b3/kotlinx-coroutines-core-jvm-1.8.0.jar:/home/alpha/.gradle/caches/modules-2/files-2.1/org.jetbrains/annotations/23.0.0/8cc20c07506ec18e0834947b84a864bfc094484e/annotations-23.0.0.jar org.jetbrains.kotlin.cli.jvm.K2JVMCompiler'
```

Compile: `$KC -no-stdlib -classpath "$STDLIB" -jvm-target 25 -d OUT FILE.kt`.
Run: `java -cp OUT:$STDLIB MainClass`, where `FILE.kt` with top-level
`main` runs as facade class `FILE` + `Kt` (so `GuestKernel.kt` runs as
`GuestKernelKt`). Native compiler acceptance and runtime output are recorded
separately.

### 8.1 Positive witness (native VERIFIED)

File `GuestKernel.kt` = the block between the `guest-kernel` markers
(Section 5). Acceptance: compile exit 0; run exit 0; stdout exactly
`ok\n`. Native result VERIFIED per
local://modern-sicp-kotlin-contract-native-results.json (compile exit 0,
run exit 0, stdout `ok\n`). Guest-engine obligations (Section 5.2) are UNRUN.

### 8.2 Negative witnesses (native compile behavior VERIFIED; gate rejection UNRUN)

Host-invalid, each a complete file: compile MUST fail (nonzero exit, no class
output). VERIFIED per local://modern-sicp-kotlin-contract-native-results.json:
each of the four compiled with exit 1. Compare rejection by exit status only,
never by diagnostic wording.

```kotlin
// BEGIN WITNESS bad-nonboolean-cond
fun gate(x: Long): Long {
    if (x) {
        return 1L
    }
    return 0L
}
// END WITNESS bad-nonboolean-cond
```

```kotlin
// BEGIN WITNESS bad-nullable-cond
fun gate(b: Boolean?): Long {
    if (b) {
        return 1L
    }
    return 0L
}
// END WITNESS bad-nullable-cond
```

```kotlin
// BEGIN WITNESS bad-val-reassign
fun bump(): Long {
    val x = 1L
    x = 2L
    return x
}
// END WITNESS bad-val-reassign
```

```kotlin
// BEGIN WITNESS bad-int-widen
fun widen(n: Int): Long {
    return n
}
// END WITNESS bad-int-widen
```

Host-valid but subset-unsupported, each a complete file: compile MUST succeed
(proving the host-valid category). VERIFIED per the evidence file: each of the
four compiled with exit 0, including `1 + 2L` through integer widening, which
settles the mixed-width classification. The future subset gate (Task 7/11)
MUST reject each with the named category of Section 6.2 before execution;
that gate rejection is UNRUN.

```kotlin
// BEGIN WITNESS unsupported-mixed-width
fun mixed(): Long {
    return 1 + 2L
}
// END WITNESS unsupported-mixed-width
```

```kotlin
// BEGIN WITNESS unsupported-force-unwrap
fun length(s: String?): Int {
    return s!!.length
}
// END WITNESS unsupported-force-unwrap
```

```kotlin
// BEGIN WITNESS unsupported-exceptions
fun parse(s: String): Long {
    try {
        return s.toLong()
    } catch (e: NumberFormatException) {
        return 0L
    }
}
// END WITNESS unsupported-exceptions
```

```kotlin
// BEGIN WITNESS unsupported-unchecked-cast
fun asNumber(v: Any): Long {
    return v as Long
}
// END WITNESS unsupported-unchecked-cast
```

## 9. Parent verification commands (native reproduction; guest obligations UNRUN)

From `/home/alpha/book/modern-sicp`, after defining `STDLIB` and `KC` as in
Section 8. Marker extraction uses anchored patterns so the command text
cannot collide with the witness markers.

```sh
GRAMMAR=spec/host-subsets/kotlin/grammar.md
OUT=/tmp/kotlin-guest-oracle
rm -rf "$OUT"
mkdir -p "$OUT"

extract() {
  awk -v s="^// BEGIN WITNESS $1$" -v e="^// END WITNESS $1$" \
    '$0 ~ s { f = 1 } f { print } $0 ~ e { f = 0 }' "$GRAMMAR" > "$OUT/$2"
}

extract guest-kernel GuestKernel.kt
extract bad-nonboolean-cond NegCond.kt
extract bad-nullable-cond NegNullable.kt
extract bad-val-reassign NegVal.kt
extract bad-int-widen NegWiden.kt
extract unsupported-mixed-width UnMixed.kt
extract unsupported-force-unwrap UnForce.kt
extract unsupported-exceptions UnExceptions.kt
extract unsupported-unchecked-cast UnCast.kt

$KC -no-stdlib -classpath "$STDLIB" -jvm-target 25 -d "$OUT/ok" "$OUT/GuestKernel.kt"
java -cp "$OUT/ok:$STDLIB" GuestKernelKt
# expect: compile exit 0; run exit 0; stdout exactly "ok" with one line ending

for f in NegCond NegNullable NegVal NegWiden; do
  if $KC -no-stdlib -classpath "$STDLIB" -jvm-target 25 -d "$OUT/$f-out" "$OUT/$f.kt" 2>/dev/null; then
    echo "FAIL: $f unexpectedly accepted"; exit 1
  fi
  echo "ok: $f rejected as host-invalid"
done

$KC -no-stdlib -classpath "$STDLIB" -jvm-target 25 -d "$OUT/UnMixed-out" "$OUT/UnMixed.kt" || { echo "FAIL: UnMixed not host-valid"; exit 1; }
$KC -no-stdlib -classpath "$STDLIB" -jvm-target 25 -d "$OUT/UnForce-out" "$OUT/UnForce.kt" || { echo "FAIL: UnForce not host-valid"; exit 1; }
$KC -no-stdlib -classpath "$STDLIB" -jvm-target 25 -d "$OUT/UnExceptions-out" "$OUT/UnExceptions.kt" || { echo "FAIL: UnExceptions not host-valid"; exit 1; }
$KC -no-stdlib -classpath "$STDLIB" -jvm-target 25 -d "$OUT/UnCast-out" "$OUT/UnCast.kt" || { echo "FAIL: UnCast not host-valid"; exit 1; }
# expect: all four compile with exit 0. The admission gate (Task 7/11) must
# then reject them by category: UnMixed = MixedWidthArithmetic,
# UnForce = ForceUnwrap, UnExceptions = Exceptions, UnCast = UncheckedCast.
```

Compiler acceptance and run output are recorded as separate ledger lines.
The block above reproduces the VERIFIED native runs
(local://modern-sicp-kotlin-contract-native-results.json). Guest-engine
obligations remain UNRUN.

Edition gates for the migration phase, quoted from the approved
specification's Commands section for Kotlin. They are required commands, not
claims that they have passed:

```sh
./gradlew --console=plain ktlintCheck build
./gradlew --console=plain test examplesTest solutionsTest --rerun-tasks
```

Run them from `kotlin/` once the Task 10 migration lands. The orchestrator
owns gate execution.

## 10. Contract limits

- This document defines a subset, not a compiler. The pinned compiler is the
  acceptance oracle for host validity; the subset gate is the authority for
  admission.
- No new dependency is required or admitted: the guest surface is Kotlin
  stdlib plus `kotlin.math` and `java.lang.Math` only.
- Token locations are preserved in every diagnostic; evaluator callers never
  see parser internals.
- The reader, analyzer, evaluator, explicit-control evaluator, and compiler
  of the edition share this single syntax contract within the edition.
- Native status: the witness suite is VERIFIED under the pinned compiler
  (evidence local://modern-sicp-kotlin-contract-native-results.json).
  `unsupported-mixed-width` compiling with exit 0 confirms the host-valid
  classification of mixed-width arithmetic; no native [INFERENCE] remains
  open.
- Guest-engine obligations are UNRUN: Section 5.2 self-interpretation over
  the full admitted grammar (Task 7/10) and gate rejection of the Section
  6.2 categories (Task 7/11).
