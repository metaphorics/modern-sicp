# OCaml edition plan input

## 0. Verified baseline

- The edition targets **OCaml 5.5.1** (current stable per [OCaml releases](https://ocaml.org/releases), dated 2026-09-05; 5.5.1 is a bug-fix release over 5.5.0) with **Dune 3.24**. OCaml 5.5.0 added module-dependent functions, polymorphic functions as arguments, generalized local definitions, and about 60 new standard-library functions; none of them change the design below, and one addition is adopted deliberately: `Domain.count` (since 5.5) is used only in a 3.4 margin note.
- All standard-library API claims below are grounded in the **5.5 manual**, read this session: [Effect](https://ocaml.org/manual/5.5/api/Effect.html), [Effect.Deep](https://ocaml.org/manual/5.5/api/Effect.Deep.html), [Domain](https://ocaml.org/manual/5.5/api/Domain.html), [Mutex](https://ocaml.org/manual/5.5/api/Mutex.html), [Lazy](https://ocaml.org/manual/5.5/api/Lazy.html), [Pqueue](https://ocaml.org/manual/5.5/api/Pqueue.html).
- Main text uses Stdlib only, except Alcotest and QCheck in tests. Chapter appendices use **Base v0.17.3** and **Core v0.17.2** ([Base docs](https://ocaml.org/p/base/v0.17.3/doc/index.html), [Core docs](https://ocaml.org/p/core/v0.17.2/doc/index.html)).
- The texinfo source openings for all 22 sections were read this session; the progression procedures, data abstraction, state and streams, evaluators, register machines is confirmed.

A change from self-contained editions, one-to-one section numbering, or the per-edition host-subset contracts for chapters 4 and 5 (`spec/host-subsets/ocaml/grammar.md`) would require revising the module boundaries below. Chapters 4-5 admit only the typed guest source in grammar §§2-4 with the fixed Stdlib surface in §7; anything else is host-valid but subset-unsupported and is rejected before any guest effect (grammar §§1, 8).

## 1. Concept map

| SICP concept | Construct in OCaml | Notes |
|---|---|---|
| Primitive expressions | `int`, `float`, `bool`, strings | Numeric kinds stay explicit. No implicit integer-to-float conversion. |
| Compound procedures | Functions and `let rec` | OCaml closures directly model lexical scope. |
| Higher-order procedures | Function values | Function types make procedure contracts visible. |
| Named data abstraction | Module plus abstract `type t` | Constructor functions enforce invariants. Selectors do not expose representation. |
| Pairs and lists | Tuples and `'a list` | No universal pair type in chapters 1 and 2. |
| Guest program data at run time | Closed typed `expr`/`value` variants | Guest evaluator interprets its explicit guest AST; an internal `value` never makes a source type error executable (grammar §§1, 11). |
| Symbols and quotation | Explicit syntax constructors | Quotation is explicit construction; OCaml identifiers are not run-time symbols. |
| Message passing | Records of closures | Closer to SICP dispatch closures than OCaml objects, with simpler static types. |
| Assignment | `ref` (`ref`/`!`/`:=`) plus host-ordinary mutable records where the state lesson needs them | Guest core mutation is `ref`, `Array.make`/`get`/`set`/`length`, and `Hashtbl.create`/`find_opt`/`replace`/`remove`/`length` only; mutable record fields and record updates are host-valid but subset-unsupported (grammar §§3, 6, 8). |
| Mutable pairs | Host `{ mutable car; mutable cdr }` in 3.3; guest memory uses `word array` cells | Host record is restricted to 3.3; guest heap is parallel car/cdr `word array` storage with `Array.get`/`set` (grammar §§10, 13). |
| Symbolic algebra | Algebraic variants and pattern matching | Differentiation, polynomials, sets, Huffman trees. |
| Data-directed dispatch | Host `Hashtbl` of operation closures in chapters 2-3; guest uses typed `Term`/`Query` and `Instruction` constructors | Main text preserves `put`/`get` mechanics of 2.4 and 2.5 as host teaching; guest query/machine are domain languages of host constructors, not tables of source syntax (grammar §§10, 13). |
| Generic arithmetic appendix | Host-ecosystem functors and first-class modules (Base/Core appendix, outside guest core) | Appendix contrasts typed composition with run-time tags; guest core has no module, functor, or first-class-module declarations (grammar §§2, 8). |
| Picture language | Painter functions producing SVG primitives | SVG is deterministic, inspectable, needs no GUI runtime. |
| Streams | Host-ordinary memoized streams in 3.5; guest lazy answers use explicit thunk cells and fair query streams | Chapter 3 preserves `delay`/`force` sharing as host teaching; guest lazy/search are separately named experiments with explicit thunk and answer events (grammar §§10). |
| Parallel execution | Host-ordinary `Domain.spawn`, `Domain.join`, `Mutex.protect` in 3.4 | Stdlib only; Eio is not used. Guest core has no admitted concurrency primitive; guest search order is the experiment's documented search order over admitted `list` data, never scheduler observation (grammar §10). |
| Serializer | Host-ordinary closure that protects a call with one mutex | Mirrors SICP's serializer in 3.4 as host teaching; guest search keeps reversible and permanent assignment distinct from core `ref` assignment (grammar §§10). |
| Environment | Guest environments as typed bindings with `ref` cells where the lesson needs sharing | Frames use admitted `ref`, array, and hash-table operations with immutable keys; an internal value never bypasses source checking (grammar §§1, 4, 6). |
| Lazy evaluator thunk | Separately named lazy experiment with explicit thunk values | Delayed arguments carry expression, environment, and optional memoized result; primitives stay strict (grammar §§10). |
| Choice and failure | Separately named search experiment with explicit choice/failure/answer events | Documented search order with restartable choice paths; a one-shot continuation is never resumed twice; search forms never enter the core grammar (grammar §§10). |
| Query frames | Immutable bindings with fair delayed answer streams | Unification includes an occurs check; frames are immutable; conjunction/disjunction/negation are closed variants and lists (grammar §§10, 13). |
| Register machine | Instruction variants, register table, arrays, stack | Labels assemble to array positions. |
| Pair memory | Parallel car/cdr arrays | Models `the-cars`, `the-cdrs`, allocation, roots, copying GC. |
| Compiler output | Register-machine instruction sequence | Compiler and explicit-control evaluator share the simulator IR. |

### Hard mapping sketches

#### Guest program values (typed guest AST, not dynamic Scheme)

```ocaml
(* guest program values: closed typed variants interpreting an explicit guest AST per grammar §§11; no dynamic Scheme typing, no host eval or reflection *)
type value = VInt of int | VBool of bool | VUnit | VTuple of value list | VConstructor of string * value list
```

#### Host mutable pairs in 3.3 (guest uses array cells)

```ocaml
(* host-ordinary 3.3 mutable pairs; guest memory uses word arrays per grammar §§10 *)
type 'a mpair = { mutable car : 'a; mutable cdr : 'a }
let set_car pair value = pair.car <- value
let set_cdr pair value = pair.cdr <- value
```

#### Records of closures for message passing

```ocaml
type account = {
  withdraw : int -> (int, error) result;
  deposit : int -> (int, error) result;
  balance : unit -> int;
}
```

#### Symbolic differentiation

```ocaml
type expr =
  | Const of int | Var of string
  | Add of expr * expr | Mul of expr * expr
  | Pow of expr * int
let rec deriv variable = function
  | Const _ -> Const 0 | Var x -> Const (if x = variable then 1 else 0)
  | Add (x, y) -> Add (deriv variable x, deriv variable y)
  | Mul (x, y) -> Add (Mul (deriv variable x, y), Mul (x, deriv variable y))
  | Pow (x, n) -> Mul (Const n, Mul (Pow (x, n - 1), deriv variable x))
```

#### Data-directed dispatch in the main text

```ocaml
type tag = string
type operation = value list -> (value, error) result
module Key = struct type t = string * tag list let equal = ( = ) let hash = Hashtbl.hash end
(* host-ordinary 2.4 dispatch uses only the admitted operations below; Hashtbl.Make and arbitrary paths are host-valid but subset-unsupported per grammar §§3, 8 *)
let put table op tags fn = Hashtbl.replace table (op, tags) fn
let find_operation table op tags = Hashtbl.find_opt table (op, tags)
```

#### Typed arithmetic in the Base/Core appendix

```ocaml
module type Arithmetic = sig
  type t
  val add : t -> t -> t
  val mul : t -> t -> t
  val sexp_of_t : t -> Sexp.t
end
type packed = Pack : (module Arithmetic with type t = 'a) * 'a -> packed
```

#### SVG picture language

```ocaml
type point = { x : float; y : float }
type segment = point * point
type frame = { origin : point; edge1 : point; edge2 : point }
type painter = frame -> segment list
let beside left right frame =
  transform_left frame (left frame) @ transform_right frame (right frame)
```

#### Memoized streams

```ocaml
(* host-ordinary 3.5 streams teach sharing with a memoized Lazy.t tail; the separately named guest lazy/search experiments use explicit thunk cells and fair answer streams per grammar §10 *)
type 'a stream = Cons of 'a * 'a stream Lazy.t
let cons_stream head tail = Cons (head, Lazy.from_fun tail)
let stream_car (Cons (head, _)) = head
let stream_cdr (Cons (_, tail)) = Lazy.force tail
let rec map f (Cons (head, tail)) =
  Cons (f head, lazy (map f (Lazy.force tail)))
```
Host `Lazy.force` computes once and reuses the value ([Lazy](https://ocaml.org/manual/5.5/api/Lazy.html)); `Lazy.Mutexed` (5.5) is a host-only margin note. Sharing check: two cursors over one stream force the tail once, where two fresh traversals force twice, which is what tailored addition 3.59a counts.

#### Serializer and domains

```ocaml
(* host-ordinary 3.4 teaching; guest core admits no concurrency primitive per grammar §§7 *)
let make_serializer () =
  let mutex = Mutex.create () in
  fun procedure argument -> Mutex.protect mutex (fun () -> procedure argument)
(* host-ordinary 3.4 teaching; guest search order is the experiment's documented order over admitted list data per grammar §10 *)
let parallel left right =
  let domain = Domain.spawn left in
  let right_result = right () in
  Domain.join domain, right_result
```

Grounded in [Domain.spawn/join](https://ocaml.org/manual/5.5/api/Domain.html) and [Mutex.protect](https://ocaml.org/manual/5.5/api/Mutex.html) (since 5.1; guaranteed unlock even when `f` raises).

#### Evaluator environment

```ocaml
type frame = (string, value) Hashtbl.t
type env = frame list
let rec find_variable name = function
  | [] -> Error (Unbound_variable name)
  | frame :: outer ->
      match Hashtbl.find_opt frame name with
      | Some value -> Ok value | None -> find_variable name outer
```

#### Lazy evaluator thunk

The sketch uses the runtime's `Value.thunk_state` and memoizes only a
successful force. `eval` takes an environment before an expression.
The section implementation also counts allocations, forces, recomputations
and memo hits.

```ocaml
let delay environment expression = Value.thunk ~expr:expression ~env:environment

let rec force eval value =
  match Value.thunk_state_of value with
  | None -> Ok value
  | Some cell ->
    (match !cell with
     | Value.Forced result -> Ok result
     | Value.Delayed (expression, environment) ->
       let ( let* ) = Result.bind in
       let* value = eval environment expression in
       let* result = force eval value in
       Value.set_thunk_state cell (Value.Forced result);
       Ok result)

let actual_value eval environment expression =
  Result.bind (eval environment expression) (force eval)
```

#### Restartable effect search

The search experiment exposes direct-style `choose` and `fail`. The host search engine must restart the computation for each choice path, because OCaml continuations are one-shot: `Effect.Deep.continue` raises `Continuation_already_resumed` when a continuation is resumed twice ([Effect](https://ocaml.org/manual/5.5/api/Effect.html), [Effect.Deep](https://ocaml.org/manual/5.5/api/Effect.Deep.html)).

```ocaml
(* separately named search experiment only; host implementation may use effects with a restartable choice-path driver without making Effect a core source form per grammar §§7, 10; never resume a one-shot continuation twice *)
type _ Effect.t += Choose : 'a list -> 'a Effect.t | Fail : unit Effect.t
let choose choices = Effect.perform (Choose choices)
let fail () = Effect.perform Fail
let solutions computation =
  Search.restart_with_choice_paths computation  (* replays choice paths *)
```

The engine runs the program under `Effect.Deep.try_with` with a handler whose `effc` field matches `Choose` and `Fail`; on `Choose` it records the tried index and re-runs the whole computation with that path pinned, never re-`continue`-ing one continuation. The restartable choice-path driver stays the main presentation over CPS because guest search programs keep explicit choice/failure/answer events with a documented order; a boxed CPS comparison follows in the text as host teaching.

#### Query language

```ocaml
type term = Atom of string | Variable of string | Compound of string * term list
type frame = (string * term) list
type query = Predicate of term | And of query list | Or of query list | Not of query
type answer_stream = frame Seq.t
val unify : term -> term -> frame -> (frame, error) result
val qeval : database -> query -> answer_stream -> answer_stream
```

`Seq.t` can represent no answers, a finite answer sequence, or an unbounded answer sequence. It suspends the next query step. The query engine must interleave suspended branches fairly; this property does not follow from the sequence type alone. Search uses the same sequence representation with its specified choice order. Neither API uses the nonempty memoized stream sketch from section 3.5.

#### Register-machine instructions

```ocaml
type source = Const of value | Reg of string | Op of string * source list
type instruction =
  | Assign of string * source | Test of source | Branch of string
  | Goto of [ `Label of string | `Reg of string ]
  | Perform of source | Save of string | Restore of string
```

#### Pair memory and copying GC

```ocaml
type address = int
type word = Immediate of value | Pointer of address | Forwarded of address
type heap = { cars : word array; cdrs : word array; mutable free : int }
let allocate heap car cdr =
  let address = heap.free in
  heap.cars.(address) <- car; heap.cdrs.(address) <- cdr;
  heap.free <- address + 1; address
```

## 2. Per-section notes

### 1.1 The elements of programming

- Changes: prefix book forms become infix arithmetic, `let`, `let rec`, `if`, and pattern matching over the typed guest source (grammar §§2-4). The first square-root program shows explicit `int` versus `float` operators with `float_of_int` as the only admitted conversion.
- Hard spot: OCaml has no single numeric tower and no implicit coercion.
- Representative program: Newton square root with nested helpers and a tolerance.
- Tailored additions:
  - **1.7a:** Rewrite `sqrt` so an invalid tolerance returns `result`.
  - **1.8a:** Expose `Sqrt.t` abstractly and preserve its invariant.

### 1.2 Procedures and the processes they generate

- Changes: non-tail recursion versus accumulator-based tail recursion; factorial, Fibonacci, exponentiation, GCD, and primality examples are preserved.
- Hard spot: bounded `int` changes the meaning of asymptotically correct programs.
- Representative program: fast exponentiation, recursive and iterative.
- Tailored additions:
  - **1.19a:** State and test the largest Fibonacci input that fits `int`.
  - **1.28a:** Return primality witnesses through a variant instead of `bool`.

### 1.3 Higher-order procedures

- Changes: function types replace untyped procedure values. Labeled arguments are avoided in first translations, then introduced in one style note.
- Hard spot: one generic `sum` cannot silently mix integer and float arithmetic.
- Representative program: `fixed_point` and `iterative_improve` over `float`.
- Tailored additions:
  - **1.41a:** Compose a function `n` times without an intermediate list.
  - **1.46a:** Give `iterative_improve` a convergence-error result.

### 2.1 Introduction to data abstraction

- Changes: rational and interval representations become modules with abstract `type t`, smart constructors, and accessors. Tuples stay inside implementations, never in interfaces.
- Hard spot: constructor failure for zero denominators and invalid intervals.
- Representative program: normalized `Rational.make : int -> int -> (t, error) result`.
- Tailored additions:
  - **2.1a:** QCheck proof that normalization is idempotent.
  - **2.12a:** Compare endpoint and center-percent abstract interfaces.

### 2.2 Hierarchical data and closure

- Changes: proper book lists map to `'a list` (grammar §§3 admits only the listed `List` operations); heterogeneous trees use closed variants. Sequence pipelines use `List.map`, `List.filter`, and folds. Picture output becomes SVG.
- Hard spot: book pair trees can be heterogeneous and improper; `'a list` cannot, so heterogeneous trees use explicit closed variants.
- Representative program: `square_limit` producing one SVG file.
- Tailored additions:
  - **2.20a:** Implement `same_parity` without repeated append.
  - **2.52a:** Add an SVG view-box transform with a golden-output test.

### 2.3 Symbolic data

- Changes: quotation becomes explicit constructors. Differentiation and Huffman trees use variants. Set implementations remain separate modules behind one signature.
- Hard spot: preserving syntax versus values without stringly typed trees.
- Representative program: derivative simplification over `expr`, then Huffman encode and decode.
- Tailored additions:
  - **2.58a:** Parse infix text into the expression variant with recoverable errors.
  - **2.70a:** QCheck that decoding an encoded symbol list is the identity.

### 2.4 Multiple representations for abstract data

- Changes: rectangular and polar complex packages install closures into a run-time dispatch table. The main text does not collapse the lesson into one closed variant too early.
- Hard spot: heterogeneous dispatch-table values need one common `value list -> result` boundary.
- Representative program: generic `real_part`, `magnitude`, and arithmetic after installing two packages.
- Tailored additions:
  - **2.74a:** Make personnel-file lookup return `option`, not an exception.
  - **2.76a:** Compare dispatch table and closed variant after adding one operation and one representation.

### 2.5 Systems with generic operations

- Changes: the main text keeps type tags, coercion, raising, dropping, and polynomial packages. The appendix rebuilds the key arithmetic packages with first-class modules and functors.
- Hard spot: mixed-type coercion can loop or become ambiguous.
- Representative program: generic arithmetic tower ending with polynomial GCD.
- Tailored additions:
  - **2.85a:** Detect cycles in the coercion graph.
  - **2.97a:** Test polynomial normalization under coefficient coercion.

### 3.1 Assignment and local state

- Changes: local state is a captured `ref`. Account messages are a record of closures. Host random-state examples pass `Random.State.t` explicitly as host teaching; guest programs have no admitted input, clock, or ambient-state primitive (grammar §§9).
- Hard spot: object identity versus structural equality once closures capture refs.
- Representative program: password-protected account with `withdraw`, `deposit`, and balance accessors.
- Tailored additions:
  - **3.7a:** Add a read-only account capability by record projection.
  - **3.11a:** Draw the captured `ref` and explain aliasing between two account handles.

### 3.2 The environment model (re-cut)

- Re-cut: presented as the OCaml closure-plus-`ref` model. Environment diagrams are re-cut to closure captures and shared cells; guest frame-lookup pictures remain only as preparation for the chapter 4-5 typed guest environments (grammar §§13).
- Hard spot: OCaml's internal closure layout is an implementation detail, not a language guarantee.
- Representative program: two counters sharing one captured cell versus counters with separate cells.
- Tailored additions:
  - **3.10a:** Predict values under OCaml's unspecified native argument order by sequencing effectful work with explicit `let` bindings first (guest teaching evaluators and compiler standardize on left-to-right guest operand evaluation per grammar §§6; native runs promise no order).
  - **3.20a:** Translate an environment diagram into explicit frame records.

### 3.3 Modeling with mutable data

- Changes: a dedicated host mutable-pair record is introduced for 3.3. Pair queue, table, agenda, wire, and connector are built before any Stdlib alternative is shown. Guest pair memory instead uses parallel car/cdr `word array` storage with `Array.get`/`set` (grammar §§10, 13).
- Hard spot: cycles make structural equality and naive printers diverge.
- Representative program: ripple-carry adder on the agenda simulator, then Celsius-Fahrenheit constraints.
- Margin note (host-only, outside guest core): OCaml 5.4 added the `Pqueue` module per the [5.5 manual](https://ocaml.org/manual/5.5/api/Pqueue.html). It is shown as a host agenda alternative; the main text keeps the hand-built agenda because SICP's deterministic time-order story is the lesson. Guest order never depends on unspecified iteration or scheduler order (grammar §§9).
- Tailored additions:
  - **3.18a:** Detect cycles using physical identity and a visited table.
  - **3.23a:** Implement a deque with constant-time operations behind an abstract interface.

### 3.4 Concurrency (re-cut)

- Re-cut: host `parallel-execute` becomes host `parallel`, built on `Domain.spawn` and `Domain.join`; host serializers wrap one `Mutex.t` with `Mutex.protect`. Eio is not used: this section studies shared-memory interleavings, not structured I/O concurrency. None of these host concurrency forms enter the guest core, whose fixed surface is `spec/host-subsets/ocaml/grammar.md` §7 (guest search order is the experiment's documented order over admitted `list` data, never scheduler observation). Interleaving analysis becomes schedule analysis; OCaml's scheduler makes some schedules rare, so tests assert invariants over many runs rather than one interleaving. A margin note mentions `Domain.count` (since 5.5) for observing live domains.
- Hard spot: no example may assert one specific nondeterministic outcome.
- Representative program: serialized account exchange with globally ordered account IDs to avoid deadlock.
- Tailored additions:
  - **3.47a:** Implement a bounded semaphore with `Mutex` and `Condition`.
  - **3.48a:** QCheck that ordered two-account exchanges preserve total balance.

### 3.5 Streams

- Changes: a host-ordinary custom `Cons` teaches `delay`/`force` sharing in 3.5; `Seq` appears only after the memoization lesson, because `Seq` is not memoized by default. Guest lazy answers instead use the separately named lazy experiment with explicit thunk cells and fair delayed answer streams (grammar §§10). `Lazy.Mutexed` (5.5) is a host margin note only and never a core source form.
- Hard spot: recursively defined streams and accidental self-forcing.
- Representative program: weighted pairs and Ramanujan numbers, then the signal integrator.
- Tailored additions:
  - **3.59a:** Count forces to prove memoization.
  - **3.77a:** Add a fuel-bounded stream observer returning `result`.

### 4.1 The metacircular evaluator

- Changes: the evaluator implements the typed guest source in `spec/host-subsets/ocaml/grammar.md` §§2-4. Direct, analyzed, explicit-control, and compiler consumers accept the same admitted program forms. `eval` and `apply` return `(value, eval_error) result` over the closed guest `expr`/`value` variants; a complete program is accepted by the subset parser (§§2-4), which rejects host-valid excluded syntax (`while`, `try`) before `ocamlc` type-checks it, and only then do guest effects run, and an internal value never makes a source type error executable (grammar §1). Primitive procedures use the same error channel with explicit cases for every admitted primitive and no generic fallback dispatcher.
- Hard spot: mutually recursive `expr`, `value`, `closure`, and environment types without exposing representations.
- Representative program: the evaluator REPL, then the analyzed evaluator compiling an expression to `env -> result`.
- Tailored additions:
  - **4.14a:** Add source spans and report the span of an unbound variable.
  - **4.22a:** Compare direct evaluation with analyzed evaluation on one recursive program.

### 4.2 Lazy evaluation

- Changes: the separately named lazy experiment (grammar §§10) delays compound-procedure arguments as explicit thunk values carrying expression, environment, and optional memoized result; primitives stay strict and memoization with repeated forcing are explicit observable experiment properties. The strict core gains no implicit delay and ordinary core application does not change.
- Hard spot: delayed expression versus forced value in the experiment engine (host `Lazy` caching behavior is a host-implementation detail and never a core source form per grammar §§7).
- Representative program: a forcing-count demonstration over admitted experiment data with one memoized suspension forced twice past a `ref` probe (exactly one evaluation) beside the same suspension under recompute mode (two evaluations), followed by lazy lists built from explicit thunk data in the experiment.
- Tailored additions:
  - **4.29a:** Instrument thunk creation and forcing counts.
  - **4.31a:** Add strict, lazy, and lazy-memo parameter annotations to the typed AST.

### 4.3 Nondeterministic computing (re-cut)

- Re-cut: the separately named search experiment exposes explicit choice, failure, and answer events with a documented search order (grammar §§10); search forms never enter the core grammar. The host implementation may use OCaml 5 effect handlers with a restartable choice-path driver and never resumes a one-shot continuation twice; reversible and permanent assignment stay distinct from core `ref` assignment. Exercise statements keep their numbers and objectives; only the engine admission notes change.
- Hard spot: one-shot continuations make the naive deep handler incorrect.
- Representative program: prime-sum pairs, then the natural-language parser.
- Tailored additions:
  - **4.35a:** Enumerate Pythagorean triples with a finite fuel bound.
  - **4.45a:** Record and replay the choice path for each parse.

### 4.4 Logic programming

- Changes: the query language is a domain language of ordinary typed constructors (terms, variables, rules, frames, conjunction/disjunction/negation nodes as closed variants and lists), not an additional textual grammar. Frames are immutable bindings, unification includes an occurs check, and delayed answer streams belong to the query experiment with fair interleaving (grammar §§10, 13). The query reader is not a general OCaml-source parser.
- Hard spot: occurs checks, variable renaming, fair disjunction, negation order.
- Representative program: personnel database queries, then recursive `append-to-form` rules.
- Tailored additions:
  - **4.77a:** Add an explicit occurs check and show the rejected cyclic binding.
  - **4.79a:** Use a work queue to make disjunction fair.

### 5.1 Designing register machines

- Changes: controller descriptions are typed OCaml instruction values with labels (grammar §§10, 13), not quoted book lists. Diagrams keep the original registers and data paths. No controller text is accepted as an expression in the OCaml core.
- Hard spot: preserving simultaneous-update reasoning while each instruction mutates one register.
- Representative program: GCD machine, then recursive and iterative factorial machines.
- Tailored additions:
  - **5.2a:** Add a typed builder that rejects duplicate labels.
  - **5.6a:** Compare stack depth of recursive and iterative exponentiation machines.

### 5.2 A register-machine simulator

- Changes: machine state is an abstract mutable record. Assembly resolves labels to instruction-array indices. Operation lookup returns `option`; execution errors return `result`.
- Hard spot: heterogeneous controller operands versus one `value` register type.
- Representative program: assemble and run the GCD machine through `make`, `set_register`, `start`, and `get_register`.
- Tailored additions:
  - **5.15a:** Add instruction counting without changing instruction semantics.
  - **5.19a:** Add breakpoints and a deterministic execution trace.

### 5.3 Storage allocation and garbage collection

- Changes: `Array.get`/`Array.set` implement vector memory. Two semispaces use parallel car and cdr arrays. Roots include registers and stack entries.
- Hard spot: forwarding pointers and root rewriting without unsafe casts.
- Representative program: stop-and-copy collection on a pair graph containing sharing.
- Tailored additions:
  - **5.20a:** Render heap arrays and roots as SVG before and after collection.
  - **5.21a:** Prove collection preserves sharing but removes unreachable pairs.

### 5.4 The explicit-control evaluator

- Changes: reuses the chapter 4 AST/value contracts and the chapter 5 simulator. The controller is a checked instruction list. Registers stay `exp`, `env`, `val`, `continue`, `proc`, `argl`, `unev`.
- Hard spot: every save/restore sequence must preserve stack discipline across errors and tail calls.
- Representative program: the explicit-control evaluator running recursive factorial and an erroneous application.
- Tailored additions:
  - **5.26a:** Measure maximum stack depth for tail-recursive factorial.
  - **5.30a:** Route machine errors through the evaluator's typed error display.

### 5.5 Compilation

- Changes: `compile : Ast.expr -> target -> linkage -> instruction_sequence result`. Instruction sequences keep needed/modified register sets. Compiled and interpreted procedures share `apply-dispatch`.
- Hard spot: lexical addresses, register preservation, linkage, and mixed compiled/interpreted calls must agree exactly.
- Representative program: compile factorial, print instructions, execute them on the simulator, then call between compiled and interpreted procedures.
- Exercise 5.51 keeps its number and objective: translate the evaluator to a standalone C runtime built by an external harness; the OCaml host builds the evaluator source as text and the external harness compiles and runs it (grammar §13). C is a separate build target, never guest source.
- Exercise 5.52 keeps its number and objective: emit C from the typed compiler representation, built as a standalone image by an external harness, and compare its result with direct native execution (grammar §13).
- Tailored additions:
  - **5.39a:** Represent lexical addresses with an abstract smart-constructed type.
  - **5.52a:** Emit a standalone machine image and compare its result with direct native execution. For 5.50/5.52 self-interpretation, the unit must parse and type-check the translated evaluator itself as valid guest source, run it with the teaching evaluator on a translated guest program, then compare the observable result with direct native execution; calling a native helper is not sufficient (grammar §§11, 13, all UNRUN until Phase 2).

## 3. Edition conventions

### Numbers

- `int` for discrete arithmetic, indices, counters, GCD, primality, and exact rational numerators and denominators.
- `float` for square roots, fixed points, integration, intervals, complex polar form, tolerances. Float operators stay explicit: `+.`, `-.`, `*.`, `/.`.
- On a 64-bit OCaml runtime, `int` holds signed 63-bit values, maximum `2^62 - 1`. Consequences used throughout the text:
  - `20!` fits; `21!` overflows.
  - With `F(0)=0`, `F(1)=1`: `F(90)` fits; `F(91)` exceeds the bound.
  - Unbounded exponent, Ackermann, binomial, and polynomial-coefficient experiments overflow before their mathematical examples finish.
- Keep main examples inside the stated bounds and add boundary checks where an exercise invites large inputs. Do not add Zarith to the main edition; an appendix note shows how arbitrary-precision `Z.t` removes those bounds. Zarith 1.14 is the documented arbitrary-precision integer library ([Zarith](https://ocaml.org/p/zarith/latest)).
- Scope: this section preserves the valid host numeric objectives in chapters 0-3 (exact `int` rational numerators and denominators, `float` roots/intervals/tolerances, polynomial-coefficient lessons with explicit boundary checks). The guest contracts in `spec/host-subsets/ocaml/grammar.md` §§4 (distinct `int`/`float`, `float_of_int` as the only admitted conversion, no implicit promotion, no admitted `int_of_float`) and §§6 (target-width wrapping, target-dependent range, division-by-zero and out-of-range array access as runtime failures) govern only the chapter 4-5 teaching engines. They do not narrow or rewrite the host chapter-0/3 rational, polynomial, or Fibonacci objectives; the required change is removal of the old shared-Scheme source claims and correct guest admission, not a rewrite of valid earlier host numeric lessons.

### Data and abstraction

- Tuples for local, transparent products; `'a list` for proper homogeneous sequences; variants for trees and symbolic syntax.
- The guest `value` is a closed typed variant for the evaluator kernel (`spec/host-subsets/ocaml/grammar.md` §§11): it interprets its explicit guest AST and never calls a host evaluator or reflects over OCaml syntax. Host dispatch tables in chapters 2-3 are separate host teaching.
- In 3.3 use the dedicated `{ mutable car; mutable cdr }` representation; ordinary OCaml lists are never redefined as mutable.
- Every public module ships an `.mli`.
- Every public `type t` stays abstract; constructors stay private. Smart constructors build values; observers and, where genuinely needed, a view function expose pattern-matching needs.
- Construction that can reject input is a smart constructor returning `result`.
- `find_*` returns `option`. `get_*` returns the value and carries a documented existence precondition; it is used only where absence is a programming error or statically impossible. There is no `find_exn` in the project API.

### Errors

- Recoverable failures return `('a, error) result`: invalid rational denominator, unbound guest variable, arity mismatch, type error, unknown operation, invalid instruction, exhausted memory, parse failure. The complete program is parsed (grammar §§2-4) and type-checked by `ocamlc` before any guest initializer, print, mutation, machine instruction, or evaluator effect runs; an internal value never makes a source type error executable (grammar §1).
- Each subsystem defines a closed error variant and a `pp_error` printer.
- Exceptions are limited to violated internal invariants and programming errors.
- Never `try ... with _`; never `Obj.magic`.

### Message passing

Records of closures, not OCaml objects: SICP's dispatch-procedure model stays visible, each message is a named typed field, and object types, row polymorphism, and method syntax are avoided.

### Tests

- Alcotest suites use `Alcotest.test_case`, `Alcotest.check`, and `Alcotest.run` ([Alcotest 1.9.1](https://ocaml.org/p/alcotest/latest); full API at <https://mirage.github.io/alcotest/alcotest/Alcotest/index.html>).
- Property tests use **qcheck-core** (module `QCheck`, or `QCheck2` for integrated shrinking; run through `QCheck_base_runner.run_tests_main`) per [qcheck-core docs](https://ocaml.org/p/qcheck-core/latest/doc/index.html). The `qcheck` package is a compatibility shim; the project depends on `qcheck-core` directly.
- High-value properties: rational normalization, Huffman round trips, stream-prefix equivalence, account total preservation, evaluator/compiler equivalence, GC reachability preservation.
- Formatting: `dune fmt`.

### Documentation and listing style

- Documentation comments live in `.mli` files only, in POSIX manual voice: "`make numerator denominator` is ...", "`find_binding env name` returns ...", "`get_register machine name` returns ...".
- Modules are named by concept, not exercise number (`Fixed_point`, `Rational`, `Huffman`, `Register_machine`); one concept module may hold several short listings.
- Runnable listing files: `examples/sNN_description.ml` (for example `examples/s03_newton_sqrt.ml`).
- Exercise and solution modules: `Ex_1_07` in `ex_1_07.ml` / `ex_1_07.mli`; tailored additions take a lowercase suffix (`Ex_2_20a`).

### Interpreter interactions

- Host-language interactions use utop form and keep inferred types (guest observable output is only the exact stdout bytes from the admitted printers plus exit status; there is no admitted input primitive per grammar §9):

```text
# Factorial.compute 6;;
- : int = 720
```

- Chapter 4 and 5 teaching evaluators keep the guest language visually distinct through typed guest constructors, not through a separate object-language prompt. Guest output is the exact byte sequence from `print_string`, `print_endline`, `print_int`, and `print_newline` in evaluation order; evaluator and compiler preserve all specified output and sequencing effects (grammar §9). There is no `;Value:` output form in this edition.

### Base and Core appendix format per chapter

Every chapter ends with one host-ecosystem appendix (outside the guest core; guest expansions require a contract update per grammar §§14) containing the same six subsections:

1. **Base translation:** re-express two or three key programs with Base naming and container conventions.
2. **Core translation:** add Core facilities only where system-dependent I/O, queues, timing, or richer containers help. Core extends Base with system-dependent modules ([Core docs, relationship section](https://ocaml.org/p/core/v0.17.2/doc/index.html)).
3. **Errors:** replace the chapter's principal `result` pipeline with `Or_error` and `let%bind`.
4. **Collections:** one meaningful use of `Map` or `Set` with an explicit comparator.
5. **Data interchange:** derive or write `Sexp` conversion for one central type; print one stable example.
6. **Tests:** one `let%expect_test` via `ppx_expect`, plus a property test where appropriate.

Grounded module names ([Base v0.17.3 docs](https://ocaml.org/p/base/v0.17.3/doc/index.html)): `Base.Or_error` (a `Result` specialization carrying `Error.t`, with `Let_syntax`, `Monad_infix`, and `Applicative_infix` submodules), `Base.Map`, `Base.Set`, `Base.Sexp`, `Base.Hashtbl`, `Base.Queue`; the docs describe "`Result`, `Error`, and `Or_error`, supporting the or-error pattern". Verified top-level Core modules include `Map`, `Hashtbl`, `Int`, `Float`, `List`, `Array`, `Linked_queue` ([Core v0.17.2 docs](https://ocaml.org/p/core/v0.17.2/doc/index.html)); `Core.Queue`, `Core.Set`, `Core.Sexp`, `Core.Or_error` come with Core's extension of Base and were not individually opened this session (marked unverified at top level, verified via the Base tree). `let%bind` and `let%expect_test` are supplied by the `ppx_let` and `ppx_expect` packages (both v0.17, confirmed present in the Base package ecosystem pages; their own syntax pages returned 404 this session, so the exact expansion names are unverified).

Chapter-specific appendix programs:

| Chapter | Base appendix | Core appendix |
|---|---|---|
| 1 | `Or_error` convergence, `let%bind`, S-expression trace of iteration | timing and stable expect output for factorial/fixed point |
| 2 | comparator-backed `Map`/`Set`, S-expression symbolic AST, typed arithmetic modules | priority queue or queue support for Huffman and package-install diagnostics |
| 3 | `Or_error` account operations, maps for environments, S-expression event traces | `Queue` for agendas, timing, richer test diagnostics |
| 4 | `Or_error` evaluator, map-backed frames, S-expression AST and values | reader/driver I/O and expect-tested evaluator sessions |
| 5 | map/set register analysis, S-expression instructions and compiler metadata | trace I/O, queue-backed work lists, expect-tested disassembly |

## 4. Architecture sketches

### Chapter 4 evaluator

```ocaml
(* reader.mli *)
type error
val read : string -> (Ast.expr, error) result

(* ast.mli *)
type expr
type definition
val variable : string -> expr
val lambda : string list -> expr list -> (expr, error) result

(* value.mli *)
type t
type primitive = t list -> (t, Eval_error.t) result
val int : int -> t
val pair : t -> t -> t

(* env.mli *)
type t
val empty : unit -> t
val extend : string list -> Value.t list -> t -> (t, error) result
val find_binding : t -> string -> Value.t option
val define : t -> string -> Value.t -> unit
val set : t -> string -> Value.t -> (unit, error) result

(* eval.mli *)
val eval : Env.t -> Ast.expr -> (Value.t, Eval_error.t) result
val apply : Value.t -> Value.t list -> (Value.t, Eval_error.t) result
val actual_value : Env.t -> Ast.expr -> (Value.t, Eval_error.t) result

(* analyze.mli *)
type executable = Env.t -> (Value.t, Eval_error.t) result
val analyze : Ast.expr -> (executable, Eval_error.t) result

(* amb.mli *)
(* separately named search experiment only; search forms never enter the core grammar per grammar §§10 *)
val choose : 'a list -> 'a
val fail : unit -> 'a
val solutions : (unit -> 'a) -> 'a Seq.t

(* query.mli *)
type database
type query
val evaluate : database -> query -> answer_stream
```

Public interfaces keep the guest `Ast.expr`, `Value.t`, and `Env.t` abstract over the same admitted program forms for direct, analyzed, explicit-control, and compiler consumers (grammar §§1, 13). 4.2 is a separately named lazy experiment that adds explicit thunk cells without changing core evaluation. 4.3 is a separately named search experiment whose forms never enter the core grammar. 4.4 is a sibling query engine over typed term/frame/rule constructors with fair answer streams, not another case in `eval`.

### Chapter 5 register-machine simulator

```ocaml
(* machine_value.mli *)
type t
(* guest machine values are typed host constructors per grammar §§10, 13; no Scheme conversion is admitted *)
val of_guest : Value.t -> t
val to_guest : t -> (Value.t, error) result

(* instruction.mli *)
type register = string
type label = string
type source
type t
val assign : register -> source -> t
val branch : label -> t
val save : register -> t
val restore : register -> t

(* operation.mli *)
type t = Machine_value.t list -> (Machine_value.t, error) result

(* assembler.mli *)
type program
val assemble : Instruction.t list -> (program, error) result

(* stack.mli *)
type t
val create : unit -> t
val push : t -> Machine_value.t -> unit
val pop : t -> (Machine_value.t, error) result

(* machine.mli *)
type t
val make : registers:string list -> operations:(string * Operation.t) list ->
  Instruction.t list -> (t, error) result
val set_register : t -> string -> Machine_value.t -> (unit, error) result
val find_register : t -> string -> Machine_value.t option
val get_register : t -> string -> Machine_value.t
val start : t -> (unit, error) result
val step : t -> ([ `Running | `Halted ], error) result

(* heap.mli *)
type t
val create : capacity:int -> t
val cons : t -> word -> word -> (address, error) result
val collect : t -> roots:root list -> (unit, error) result
```

### Chapter 5 compiler

```ocaml
(* register_set.mli *)
type t
val empty : t
val add : string -> t -> t
val union : t -> t -> t

(* instruction_sequence.mli *)
type t
val make : needs:Register_set.t -> modifies:Register_set.t ->
  Instruction.t list -> t
val append : t -> t -> t
val preserving : Register_set.t -> t -> t -> t
val instructions : t -> Instruction.t list

(* compiler.mli *)
type target = string
type linkage = Next | Return | Label of string
val compile : Ast.expr -> target -> linkage ->
  (Instruction_sequence.t, Compile_error.t) result

(* lexical_address.mli *)
type t
val make : frame:int -> slot:int -> (t, error) result
val frame : t -> int
val slot : t -> int

(* compiled_procedure.mli *)
type t
val make : entry:string -> env:Env.t -> t
val entry : t -> string
val env : t -> Env.t

(* runtime.mli *)
val install_compiled_code : Machine.t -> Instruction_sequence.t ->
  (string, error) result
val apply_dispatch : Machine_value.t -> Machine.t -> (unit, error) result
```

The compiler consumes the chapter 4 AST and emits the chapter 5 simulator's instruction type; no parallel compiler IR exists. The explicit-control evaluator and compiled procedures share `Machine_value.t`, environment operations, linkage labels, and `apply_dispatch`.

## 5. Chapter 0 primer outline (about 30 pages)

The primer teaches exactly the OCaml subset the book uses. Every section ends in utop transcripts the reader can reproduce; every exercise has a stub in `ocaml/ch0/exercises` and a solution in `ocaml/ch0/solutions`.

| Section | Pages | What it teaches |
|---|---|---|
| 0.1 Toolchain | 2 | opam switch on 5.5.1, Dune 3.24, utop, `dune build` / `dune runtest` / `dune fmt`, and how `examples/`, `exercises/`, `solutions/` directories work. |
| 0.2 Expressions, values, types | 3 | `int`/`float`/`bool`/`string`/`unit` in `spec/host-subsets/ocaml/grammar.md` §§2-4; typed toplevel replies; per-type operators (`+`/`-`/`*`/`/`/`mod` versus `+.`/`-.`/`*.`/`/.`); explicit conversion with `float_of_int` as the only admitted numeric conversion; type errors as help. |
| 0.3 Bindings, functions, modules | 4 | `let`, shadowing, `let rec`, partial application, `.ml`/`.mli` pairs, dotted module names as host teaching; translating book definitions. Labels and optional arguments are host-ordinary here and never enter the guest core (grammar §§8). |
| 0.4 Pattern matching and variants | 4 | `match`, exhaustiveness, `option` and `result`, a hand-rolled list type as a warm-up reimplementation, small trees. |
| 0.5 Lists and higher-order functions | 4 | `::`, `@`, `List.map`, `List.filter`, folds; deriving `map`/`fold` by hand before using the library versions; pipeline style. |
| 0.6 Records, mutable fields, refs | 3 | Host record syntax including `{ mutable ... }`, the `ref` cell, aliasing versus copying, when each is appropriate. Guest core admits immutable records with `ref`/array/hash-table mutation only; mutable record fields and record updates are host-valid but subset-unsupported (grammar §§3, 8). |
| 0.7 Closures and lexical scope | 3 | Captured environments, closures as objects warm-up, why function equality does not exist while cell identity does. |
| 0.8 Errors | 3 | `option` versus `result` versus exceptions; chaining with bind; `assert` for invariants; the edition's error contract. |
| 0.9 Modules as values | 2 | Host-ecosystem functors and first-class modules packed in a variant (outside guest core); a promise that 2.4/2.5 host appendices return to this. Guest compilation units contain only type and value declarations (grammar §§2). |
| 0.10 Testing and formatting | 2 | One Alcotest suite, one QCheck property, `dune runtest`, `dune fmt`. |

Primer exercises (numbered 0.1 onward; they do not shift book numbering):

- **0.1:** Construct and run five OCaml interactions covering arithmetic, `let` bindings, named functions, conditionals, and anonymous functions; predict each inferred type first, then check it in utop. Exercise 0.3 covers pattern matching.
- **0.2:** Implement `sum_cubes` over a range twice, once by recursion and once with `List.fold_left`; state in one sentence when the fold version is preferable.
- **0.3:** Define a `shape` variant with an `area` function; add a constructor and let the compiler list every site that breaks.
- **0.4:** Build `make_account` returning a record of closures over one `ref`; then break it by sharing the `ref` between two accounts and explain the observed aliasing.
- **0.5:** Chain three fallible steps with `Result.bind`, then map the error into a printable summary string.

## 6. Re-cut list and numbering divergences

Lesson re-cuts (the language changes how the lesson is taught; exercises keep their numbers):

1. **3.2** becomes the closure and `ref` model: environment diagrams are re-cut to closure captures and shared cells; frame-lookup pictures remain only as preparation for chapters 4 and 5. Exercises 3.9-3.11 keep their numbers with re-cut drawings.
2. **3.4** becomes Domains and Mutex: `parallel-execute` becomes `parallel` over `Domain.spawn`/`Domain.join`, serializers become `Mutex.protect` closures. Exercises 3.38-3.42 and 3.47-3.49 keep their numbers; their statements adapt from interleaving analysis to schedule analysis.
3. **4.3** is a separately named search experiment with explicit choice/failure/answer events and a documented order; the host implementation may use effect handlers with a restartable choice-path driver and never resumes a one-shot continuation twice. Search forms never enter the core grammar. Exercises 4.35-4.54 keep their numbers and objectives; only engine admission notes change (grammar §§10).

Scoped adaptations that are not re-cuts (mappings, margins, or sidebars only): 1.1 typed REPL transcripts; 1.2 integer-bound margins; 2.4/2.5 keep the host dispatch table in the main text with the host-ecosystem typed-module treatment in appendices (outside guest core); 3.3 introduces the host mutable-pair record beside immutable lists (guest memory uses array cells); 3.5 keeps host `delay`/`force` sharing with a `Seq` comparison (guest lazy/search are separate experiments).

Numbering divergences: **none.** No book exercise is renumbered in this edition. Chapter 0 exercises live in their own 0.x sequence and displace nothing.

## 7. Dune layout under `ocaml/`

```text
ocaml/
  dune-project
  ocamlformat
  common/                    library sicp_common
  ch0/                       primer support
    lib/                     library sicp_ch0 (primer running examples)
    exercises/               library sicp_ch0_exercises
    solutions/               library sicp_ch0_solutions
    test/
  ch1/
    lib/                     library sicp_ch1
    examples/                runnable executables
    exercises/               library sicp_ch1_exercises
    solutions/               library sicp_ch1_solutions
    test/                    Alcotest and QCheck runners
    base/                    library sicp_ch1_base
    core/                    library sicp_ch1_core
  ch2/ ... ch5/              same shape
  integration/
    evaluator_compiler/      chapter 4 versus chapter 5 equivalence tests
    full_stack/              reader, evaluator, compiler, machine smoke tests
```

- One main-text library per chapter: `sicp_ch1` through `sicp_ch5`.
- `sicp_common` holds only genuinely shared contracts: SVG primitives, the typed guest AST shared by chapters 4 and 5 per `spec/host-subsets/ocaml/grammar.md` §§2-4 and §13, test helpers. It must not become a utility dumping ground.
- `examples/` are small executables linked to the chapter library; each reproduces one transcript or artifact.
- `exercises/` are reader-facing signatures and starter implementations, kept in a separate library so unfinished work cannot break the main text.
- `solutions/` implement the same conceptual interfaces under distinct module names; chapter tests link against solutions.
- `test/` links `sicp_chN`, `sicp_chN_solutions`, Alcotest, and qcheck-core. Tests never inspect source text or implementation fields.
- Appendix libraries `sicp_chN_base` and `sicp_chN_core` live in `base/` and `core/`; they may depend on the chapter library when wrapping its types, but appendix code never leaks into the main-text dependency graph.
- Every module, including exercises and appendices, has an `.mli`.
- Root aliases run `dune fmt` and chapter-scoped test aliases; each chapter is verified by its own alias before the integration alias.

## 8. Plan changes to record

1. Toolchain pin moved to OCaml 5.5.1 (current stable, 2026-09-05) with Dune 3.24; all manual citations target the 5.5 manual.
2. Bounded `int` and `float` in the main text; oversized examples are bounded or changed; Zarith stays optional.
3. Records of closures for message-passing objects.
4. Table-of-closures dispatch in the main text; functors and first-class modules in the Base/Core appendix.
5. Host `Lazy.t` streams for 3.5 as host teaching; `Seq` only as comparison; `Lazy.Mutexed` host-only margin note. Guest lazy/search are separate experiments with explicit cells and events (grammar §§10).
6. Host Domains and Mutex for 3.4 as host teaching; no Eio; `Domain.count` host-only margin note. None enters the guest core (grammar §§7).
7. Separately named search experiment with explicit `Choose`/`Fail`/answer events and a documented order; host implementation may use effects with a restartable choice-path driver. Search forms never enter the core grammar; multi-shot continuation reuse is explicitly prohibited (grammar §§10).
8. Chapter 4 AST/value contract shared with chapter 5 simulator and compiler.
9. One Base/Core appendix per chapter in the fixed six-subsection format.
10. New Chapter 0 primer (about 30 pages, exercises 0.1-0.5) with `ocaml/ch0/` support directories.
11. Re-cuts recorded for 3.2, 3.4, 4.3; numbering divergence list is empty.

### Anchor files

- Section, listing, and exercise mappings: the checked-in inventories (`docs/exercise-map.md`) with Git provenance; policy in `modern-sicp/docs/plan/host-subsets-specification.md` and `modern-sicp/docs/plan/host-subsets-migration.md`. The old `text/original/sicp-pocket.texi` archive is not source authority.
- `modern-sicp/ocaml/dune-project`: toolchain pin (OCaml 5.5.1, Dune 3.24), dependency families, formatting, workspace aliases.
- `modern-sicp/ocaml/common/ast.mli`: shared typed guest-AST contract for chapters 4 and 5 per `spec/host-subsets/ocaml/grammar.md` §§2-4 (no general OCaml-source parsing; query and controller readers are domain-language readers only).
- `modern-sicp/ocaml/ch4/lib/eval.mli`: evaluator, environment, value, error, lazy, and nondeterministic boundaries.
- `modern-sicp/ocaml/ch5/lib/instruction.mli`: shared register-machine IR for simulator, explicit-control evaluator, and compiler.
