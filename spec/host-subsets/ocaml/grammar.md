# OCaml host-subset contract

Anchor: Tony Hoare, David Parnas

This document defines the OCaml source admitted by the chapter 4 and chapter 5 teaching engines. The contract is specific to the OCaml edition. It does not promise a complete OCaml compiler, parser, or runtime. The target source is ordinary OCaml 5.5.1 source restricted to the grammar and the fixed standard-library surface below.

Witness status: the §10–§12 witness blocks are **NATIVE-VERIFIED** on the pinned OCaml 5.5.1 switch by the parent's oracle (`local://modern-sicp-ocaml-contract-native-results.json`, `local://modern-sicp-ocaml-kernel-corrected-results.json`). Everything about the future teaching evaluators, analyzer, explicit-control evaluator, compiler, self-interpretation conformance, and exercise behavior is **UNRUN** until the Phase 2 migration executes it.

## 1. Conformance boundary

All core consumers use one typed expression representation: the direct evaluator, analyzed evaluator, explicit-control evaluator, and compiler accept the same admitted program forms. The guest evaluator itself is an OCaml subset program; no path may use a host `eval`, reflection, or a production compiler as a substitute for evaluating the guest evaluator.

Admission has three ordered stages:

1. The subset parser accepts only the concrete syntax in §2–§4 and rejects unsupported host syntax before evaluation.
2. The complete program is type-checked by OCaml 5.5.1 before any guest initializer, print, mutation, machine instruction, or evaluator effect runs. `ocamlc` is the typing authority; this contract does not reimplement all OCaml type inference.
3. Only a successfully parsed and type-checked program enters the selected evaluator. An internal dynamic `value` type does not make a source type error executable.

A syntax form accepted by `ocamlc` but absent from this contract is **host-valid, subset-unsupported**. The core must reject it before guest effects. Compiler acceptance alone is not subset acceptance. Every accepted source also compiles under the pinned compiler and flags in §12.

## 2. Lexical forms and declarations

The lexical alphabet is ASCII. A value identifier starts with a lowercase ASCII letter or `_`, followed by ASCII letters, digits, `_`, or `'`. A type identifier starts with a lowercase ASCII letter and follows the same continuation rule; type names and value names share spelling rules but occupy different namespaces. A constructor identifier starts with an uppercase ASCII letter and follows the same continuation rule. A type variable is `'` followed by a lowercase identifier. Keywords are reserved. Identifiers are case-sensitive. A value identifier may shadow an outer value identifier; constructor and type names are unique within one compilation unit.

Whitespace separates tokens. `(* ... *)` comments may nest as in OCaml. The subset has no preprocessor, attributes, extensions, or line directives.

```ebnf
program          ::= { top_item }
top_item         ::= type_item | value_item
value_item       ::= "let" [ "rec" ] binding { "and" binding }   (* parallel; "rec" makes the group mutual *)
binding          ::= value_name { value_name } "=" expr
                   | value_name "=" "fun" value_name "->" expr
                   | "()" "=" expr
                   | "_" "=" expr
type_item        ::= "type" type_binding { "and" type_binding }
type_binding     ::= [ type_parameter | "(" type_parameter { "," type_parameter } ")" ] type_name "=" type_body
type_body        ::= constructor_decl { "|" constructor_decl }
                   | "{" field_decl { ";" field_decl } [ ";" ] "}"
                   | type_expr
constructor_decl ::= constructor_name [ "of" type_expr { "*" type_expr } ]
field_decl       ::= value_name ":" type_expr
type_expr        ::= app_type { "*" app_type } [ "->" type_expr ]
app_type         ::= type_atom { type_name }
type_atom        ::= type_variable | type_name
                   | "(" type_expr ")"
                   | "(" type_expr { "," type_expr } ")" type_name
```

The declaration grammar admits algebraic variants, type abbreviations, recursive and mutually recursive variant declarations, and immutable record declarations. It does not admit mutable record fields. Type parameters are ordinary Hindley–Milner variables. A postfix `type_name` applies to the type on its left (`int list`, `value ref`, `word array`, `(string * value) list`). The built-in type constructors are `unit`, `int`, `float`, `bool`, `string`, `list`, `option`, `result`, `ref`, and `array`. The only abstract module type in the admitted prelude is `('a, 'b) Hashtbl.t`, and `Hashtbl.t` is the one qualified `type_name` the grammar admits.

A compilation unit contains only type declarations and value declarations. It has no module, signature, functor, exception, class, object, external, include, open, or local-module declarations. A top-level initializer runs in source order only after the whole unit has passed parsing and type checking.

## 3. Expression grammar

The following productions are finite and exhaustive. The parser uses the native OCaml precedence and associativity for each listed operator; parentheses may group any expression. The notation `simple_expr` means one identifier, literal, constructor, parenthesized expression, tuple, or list literal, not an arbitrary unparenthesized expression. Bare tuple expressions are admitted at the `tuple_expr` layer, so `match a, b with` parses its scrutinee as one tuple; the tuple comma binds tighter than `:=` and looser than `||`, matching OCaml. A `case_pattern` comma sequence is a tuple pattern without outer parentheses (`match a, b with | PInt x, VInt y -> ...`); tuple patterns elsewhere require parentheses. `:=` is right-associative and binds tighter than `;`.

```ebnf
expr             ::= let_expr | if_expr | function_expr | match_expr | sequence
let_expr         ::= "let" [ "rec" ] binding { "and" binding } "in" expr
if_expr          ::= "if" expr "then" expr "else" expr
function_expr    ::= "fun" value_name { value_name } "->" expr
                   | "function" match_case { match_case }
match_expr       ::= "match" expr "with" match_case { match_case }
match_case       ::= [ "|" ] case_pattern "->" expr
case_pattern     ::= pattern [ "," pattern { "," pattern } ]
sequence         ::= assignment [ ";" expr ]
assignment       ::= tuple_expr [ ":=" assignment ]
tuple_expr       ::= logical_or [ "," logical_or { "," logical_or } ]
logical_or       ::= logical_and { "||" logical_and }
logical_and      ::= comparison { "&&" comparison }
comparison       ::= cons_expr [ compare_op cons_expr ]
cons_expr        ::= concat_expr [ "::" cons_expr ]
concat_expr      ::= additive [ "^" concat_expr ]
additive         ::= multiplicative { ( "+" | "-" | "+." | "-." ) multiplicative }
multiplicative   ::= unary { ( "*" | "/" | "mod" | "*." | "/." ) unary }
unary            ::= ( "-" | "-." | "not" | "!" | "ref" ) unary | application
application      ::= simple_expr { simple_expr }
simple_expr      ::= value_name | constructor_name
                   | int_literal | float_literal | string_literal
                   | "true" | "false" | "()"
                   | "(" expr ")"
                   | "(" expr "," expr { "," expr } ")"
                   | "[]" | "[" [ expr { ";" expr } ] "]"
                   | constructor_name simple_expr
                   | "Array.make" simple_expr simple_expr
                   | "Array.get" simple_expr simple_expr
                   | "Array.set" simple_expr simple_expr simple_expr
                   | "Array.length" simple_expr
                   | "List.map" simple_expr simple_expr
                   | "List.filter" simple_expr simple_expr
                   | "List.fold_left" simple_expr simple_expr simple_expr
                   | "List.fold_right" simple_expr simple_expr simple_expr
                   | "List.length" simple_expr | "List.rev" simple_expr
                   | "List.append" simple_expr simple_expr
                   | "List.sort" simple_expr simple_expr
                   | "Hashtbl.create" simple_expr
                   | "Hashtbl.find_opt" simple_expr simple_expr
                   | "Hashtbl.replace" simple_expr simple_expr simple_expr
                   | "Hashtbl.remove" simple_expr simple_expr
                   | "Hashtbl.length" simple_expr
                   | simple_expr "." value_name
                   | "{" value_name "=" expr { ";" value_name "=" expr } [ ";" ] "}"
compare_op       ::= "=" | "<>" | "<" | "<=" | ">" | ">="
pattern          ::= value_name | "_" | int_literal | float_literal
                   | string_literal | "true" | "false" | "()"
                   | "[]" | pattern "::" pattern
                   | "(" pattern "," pattern { "," pattern } ")"
                   | constructor_name [ pattern | "(" pattern { "," pattern } ")" ]
int_literal      ::= decimal_digit { decimal_digit }
float_literal    ::= decimal_digit { decimal_digit } "." { decimal_digit } [ exponent ]
                   | decimal_digit { decimal_digit } exponent
exponent         ::= ( "e" | "E" ) [ "+" | "-" ] decimal_digit { decimal_digit }
string_literal   ::= '"' { ordinary_string_char | escape } '"'
escape           ::= '\\' | '\\"' | "\\n" | "\\r" | "\\t"
```

The productions for prelude names above are direct applications only; the subset does not permit arbitrary module paths or module aliases. For prelude values used as first-class functions (for example, a map callback), the corresponding name may also appear as a simple expression. A constructor with multiple payload fields uses a tuple payload. Lists and tuples are immutable. A record expression supplies each declared field exactly once. Record field access uses `record.field`; record update syntax is excluded.

The admitted core operators are integer `+`, `-`, `*`, `/`, `mod`; float `+.`, `-.`, `*.`, `/.`; Boolean `not`, `&&`, `||`; comparisons `=`, `<>`, `<`, `<=`, `>`, `>=`; list cons `::`; and string concatenation `^`. `=` and `<>` type-check only on `unit`, `int`, `float`, `bool`, and `string`. Ordered comparisons type-check only on `int`, `float`, and `string`. The subset does not inherit OCaml's polymorphic comparison on functions, references, arrays, records, or arbitrary variants. Programs that need equality on structured data define it by recursion over constructors.

## 4. Static type and binding contract

OCaml 5.5.1 infers value types. The subset uses OCaml's ordinary Hindley–Milner inference, including its value restriction and relaxed value restriction. It adds no dynamic source-language typing and no implicit conversion. Numeric types remain distinct: integer operators require `int`; float operators require `float`. `float_of_int` is the only admitted numeric conversion. It has the pinned Stdlib behavior, including rounding when a machine integer is not exactly representable as a float. There is no implicit promotion and no admitted `int_of_float`.

`let name = e1 in e2` infers `e1` before `e2` and introduces a lexically scoped binding. Shadowing is legal and resolves to the nearest binding. `let _ = e1 in e2` discards the value. A binding is immutable; the grammar has no variable-reassignment form. Mutation requires an admitted reference, array, or hash-table operation.

`let rec` and recursive `and` groups admit function bindings (`let rec f x = e`, `let rec f = fun x -> e`) and statically constructive recursive values: a right-hand side that OCaml 5.5.1 accepts as a tuple, list, or variant construction whose recursive occurrences lie beneath a constructor. This admits the recursive closure knot an evaluator needs, a closure whose environment names the closure itself; the pinned compiler enforces the construction rule. Recursive aliases, recursive application results, and cyclic type abbreviations are rejected. The rule applies at top level and locally. Every name in a `rec` group sees the whole group; a `let ... and ...` group without `rec` is parallel and its members do not see one another. A recursive type group is admitted when every cycle passes through a variant constructor or immutable record field.

`let` generalization follows the pinned OCaml compiler. In particular, an expansive reference, array, or hash-table allocation does not acquire unsafe polymorphism: reference-containing type variables remain weak/monomorphic under the host value restriction. A failed `ocamlc` type check rejects the complete unit before execution. The guest evaluator may use a tagged internal `value`, but it may construct that value only from a successfully checked source program.

Every `match` and `function` case set must be exhaustive. The oracle promotes warning 8 to an error. Duplicate variables in one pattern are rejected. Or-patterns, guards, lazy patterns, exception patterns, and `let` patterns beyond a name, `_`, or `()` are outside the grammar; destructuring happens in `match`. A pattern match checks cases in source order and evaluates only the body of the first matching case.

## 5. Algebraic data and closures

A program may declare closed variant types, including recursive trees and mutually recursive data types. Constructors carry zero or more immutable payload fields. It may use tuple products, proper lists, `option`, `result`, immutable records, and arrays. There are no extensible or polymorphic variants, GADTs, existential constructors, object types, open rows, or first-class modules.

Functions are curried, first-class lexical closures. A closure sees the binding environment at its definition site, not the call site. Escaping closures keep every captured value alive. If a closure captures a `ref`, all closures that hold that reference observe the same cell; assigning through `:=` changes its contents, not the immutable name that points to it. `let rec` closures capture the whole recursive group. A compiler or explicit-control evaluator must preserve these rules even when the closure escapes its defining call.

Patterns include variables, wildcard, unit and scalar literals, tuple patterns, list nil/cons, and declared constructors. Constructor arity and payload types are checked by `ocamlc`. Pattern variables bind immutable names. Ref/array contents are not implicitly destructured or copied by pattern matching.

## 6. Mutation, numeric behavior, and evaluation order

The core mutation surface is `ref e`, dereference `!e`, and assignment `e1 := e2`; reference assignment returns `unit`. Arrays are admitted only through `Array.make`, `Array.get`, `Array.set`, and `Array.length`. `Array.make n initial` stores the same initial value in each slot, so a mutable initial value is aliased across slots. Array access is bounds checked; an invalid index is a runtime failure. Arrays are mutable and reference-like; array identity or structural equality is not admitted. The only hash-table operations are `Hashtbl.create`, `find_opt`, `replace`, `remove`, and `length`; table keys in teaching programs are immutable integers, strings, tuples, or constructor trees. `ref` contents, arrays, and hash tables cannot be compared with `=` or `<>`.

Integer width is target-dependent. The contract records `Sys.word_size` and `Sys.int_size` for each native oracle run; the switch alone does not imply a fixed width. On a target with `Sys.int_size = n`, the representable signed range is `[-2^(n-1), 2^(n-1)-1]`. Integer addition, subtraction, and multiplication use OCaml's target-width wrapping behavior. Integer division truncates toward zero; `mod` has the sign of its left operand. Division or remainder by zero is a runtime failure. Float is the host IEEE-754 binary64 type; float arithmetic, comparisons, infinities, and NaNs follow OCaml's runtime behavior. No exercise or oracle may assume that an integer computation exceeds the target range unless it explicitly tests that target-dependent boundary.

The edition's documented 64-bit runtime bounds (`docs/plan/idiom-ocaml.md`, Numbers) are signed 63-bit values with maximum `2^62 - 1`: `20!` fits and `21!` overflows; with `F(0)=0`, `F(1)=1`, `F(90)` fits and `F(91)` overflows. Main examples stay inside these bounds, and a boundary exercise states its bound in the statement.

The core uses strict, call-by-value evaluation. A `let` right-hand side runs before its body. An `if` evaluates its condition and one branch. `&&` and `||` short-circuit from left to right. `;` evaluates its left expression before its right expression; the left expression's value is discarded, matching OCaml. A match evaluates its scrutinee once, checks alternatives in source order, then evaluates one branch. Top-level initializers execute in source order after successful compilation.

OCaml leaves the evaluation order of function and constructor arguments, tuple components, and record fields unspecified in native execution, so the contract promises no left-to-right or right-to-left order for those subexpressions in native runs. The teaching evaluators and the compiler standardize on left-to-right guest operand evaluation, the book's `list-of-values` rule that exercises 4.1, 4.46, and 5.36 make observable inside the guest language. Because the host leaves the order unspecified (the 4.1 hard spot in `docs/exercise-map.md`), accepted conformance programs must not observe it: effectful work is sequenced with explicit `let` bindings before its values are combined, and native/interpreter/compiler comparisons use only observations invariant across the orders OCaml permits.

## 7. Fixed standard-library surface

Unqualified prelude values admitted by the core are `print_string`, `print_endline`, `print_int`, `print_newline`, `string_of_int`, `string_of_float`, `float_of_int`, and `sqrt`, with their pinned Stdlib types and effects. `Array`, `List`, and `Hashtbl` expose only the direct-call members listed in §3 and §6. `None`, `Some`, `Ok`, and `Error` are the built-in constructors of `option` and `result`. Other `Stdlib` names, other modules, `open`, and aliases are unsupported unless this contract is amended before use. Programs may define ordinary recursive list utilities instead of importing a module function.

The fixed surface is deliberately small and auditable. It does not expose `Obj`, `Marshal`, `Sys`, `Unix`, `Random`, `Buffer`, `Format`, `Lazy`, or `Effect` in core source. The chapter 4 lazy and search experiments use their separately named engines in §10; their host implementation may use the OCaml 5.5.1 `Lazy` or effect APIs without making those APIs core source forms.

## 8. Rejection categories

| Category | Meaning | Required disposition |
|---|---|---|
| Lexical/syntax-invalid | The pinned OCaml parser cannot parse the source, or the subset lexer finds a malformed token, string, comment, or delimiter. | Reject the whole source before any guest effect. |
| Type-invalid | The pinned type checker rejects the source, including unbound names, mismatched types, invalid constructor arity, a cyclic type abbreviation, or a recursive binding the compiler rejects as non-constructive. | Reject the whole source before any guest effect. |
| Contract-invalid | The pinned compiler accepts the form, but it violates a subset rule such as a non-exhaustive match or a forbidden recursive binding. | Reject before evaluation; use `-warn-error +8` for non-exhaustive matches. |
| Host-valid, subset-unsupported | The source is valid OCaml but uses an excluded construct or module. | Reject before evaluation; never lower it through a different dynamic language. |
| Runtime failure | A checked program reaches a host operation failure such as integer division by zero or out-of-range array access. | Report as a runtime failure; do not mislabel it as a type rejection or promise a stable host diagnostic string. |

Host-valid but unsupported syntax includes local/open modules, arbitrary qualified paths, arbitrary custom operators, `while`, `for`, `try`, `raise`, exception declarations, assertions, mutable record fields, record updates, arrays other than the four listed operations, mutable bytes, channels, references to nonlocal evaluation machinery, classes, objects, polymorphic variants, GADTs, first-class modules, labels/optional arguments, extensible constructors, lazy patterns, quotations, attributes, and extensions. The parser rejects the unsupported construct even when `ocamlc` accepts it.

## 9. Observable behavior

A successful native program has exit status zero unless its own runtime operation fails. Its observable output is the exact byte sequence written to stdout by the admitted print functions, plus the exit status. `print_endline` writes its string followed by a newline; `print_string` writes the string; `print_int` writes the decimal form of an integer; `print_newline` writes one newline. The evaluator and compiler must preserve all specified output and sequencing effects. The contract does not pin compiler diagnostic wording, stderr formatting, stack statistics across different machines, or a specific order for OCaml-unspecified evaluation contexts.

There is no admitted input primitive. A guest program cannot read from stdin, access files, query the environment, start processes, or observe the clock. This keeps output and state comparison reproducible. A chapter-specific machine driver may receive explicit input values through typed host constructors; those values are test inputs, not ambient process input.

## 10. Separate lazy and search experiments

The strict core has no implicit delay, thunk forcing, or nondeterminism. The lazy evaluator is a separately named experiment whose delayed arguments are explicit thunk values carrying an expression, environment, and optional memoized result. Primitive arguments remain strict. Memoization and repeated forcing are explicit observable experiment properties; they do not change ordinary core application.

The search evaluator is a separately named experiment with explicit choice, failure, and answer events and a documented search order. The OCaml host implementation may use OCaml 5 effect handlers with a restartable choice-path driver; it must not resume a one-shot continuation more than once. Search forms do not enter the core grammar and are accepted only by that experiment's admission rule. Reversible assignment and permanent assignment remain distinct search semantics; neither is smuggled into core reference assignment.

The query language is a domain language represented by ordinary typed constructors, not by an additional textual grammar. Query terms, variables, rules, frames, and conjunction/disjunction/negation nodes use closed variants and lists. Unification includes an occurs check; frames use immutable bindings; delayed answer streams belong to the query experiment and preserve fair interleaving. The query reader and register-controller reader are not general OCaml-source parsers.

The register machine is also a domain language represented by host constructors. Its instruction and operand trees, labels, words, and controller are ordinary OCaml values. The machine simulator uses arrays for registers and the two car/cdr semispaces, with refs for program counter, free pointer, and stack state. Stop-and-copy collection updates every root and preserves sharing. No controller text is accepted as an expression in the OCaml core.

**NATIVE-VERIFIED host-constructor sketch** (compiled by the parent: exit 0; domain data, not core syntax):

```ocaml
type term = Atom of string | Variable of string | Compound of string * term list
type query = Predicate of term | And of query list | Or of query list | Not of query
type word = Immediate of int | Pair_pointer of int | Label of string
type source = Constant of word | Register of string | Operation of string * source list
type instruction = Assign of string * source | Test of source | Branch of string | Goto of string | Save of string | Restore of string
type semispace = { cars : word array; cdrs : word array }
type heap = { spaces : semispace array; active_space : int ref; free_pointer : int ref }
```

## 11. Guest evaluator-kernel witness

The evaluator must be expressible as an ordinary admitted OCaml program. The following complete, host-valid kernel uses only admitted constructors, recursive functions, lists, pattern matching, closures, references, arrays, scalar operators, and the fixed printer. It interprets its explicit guest AST; it never calls a host evaluator or reflects over OCaml syntax. Its dynamic `VError` cases make the kernel total on malformed hand-built ASTs; the source boundary still type-checks the complete guest program before evaluation. A full evaluator adds explicit cases for every additional admitted primitive in §3 and §7; it has no generic fallback dispatcher.

**NATIVE-VERIFIED evaluator-kernel source**; parent run on OCaml 5.5.1 (`local://modern-sicp-ocaml-kernel-corrected-results.json`): exit 0, stdout `7` followed by a newline:

```ocaml
type expr =
  | EInt of int
  | EBool of bool
  | EUnit
  | EVar of string
  | EIf of expr * expr * expr
  | ELambda of string * expr
  | EApply of expr * expr
  | ELet of string * expr * expr
  | ELetRec of string * string * expr * expr
  | EAdd of expr * expr
  | ESub of expr * expr
  | EEqual of expr * expr
  | ELess of expr * expr
  | ETuple of expr list
  | EConstructor of string * expr list
  | EMatch of expr * (pattern * expr) list
  | EListNil
  | EListCons of expr * expr
  | ERef of expr
  | EDeref of expr
  | EAssign of expr * expr
  | EArrayMake of expr * expr
  | EArrayGet of expr * expr
  | EArraySet of expr * expr * expr
  | ESequence of expr * expr
and pattern =
  | PInt of int
  | PBool of bool
  | PUnit
  | PVar of string
  | PWildcard
  | PTuple of pattern list
  | PConstructor of string * pattern list
  | PListNil
  | PListCons of pattern * pattern

type value =
  | VInt of int
  | VBool of bool
  | VUnit
  | VTuple of value list
  | VConstructor of string * value list
  | VListNil
  | VListCons of value * value
  | VClosure of string * expr * env
  | VRef of value ref
  | VArray of value array
  | VError
and env = (string * value) list

let rec extend bindings environment =
  match bindings with
  | [] -> environment
  | (name, value) :: rest -> (name, value) :: extend rest environment

let rec lookup name environment =
  match environment with
  | [] -> None
  | (bound_name, value) :: rest ->
      if name = bound_name then Some value else lookup name rest

let rec append left right =
  match left with
  | [] -> right
  | item :: rest -> item :: append rest right

let rec bind_patterns patterns values =
  match patterns, values with
  | [], [] -> Some []
  | pattern :: remaining_patterns, value :: remaining_values ->
      (match bind_pattern pattern value with
       | None -> None
       | Some bindings ->
           (match bind_patterns remaining_patterns remaining_values with
            | None -> None
            | Some rest -> Some (append bindings rest)))
  | _, _ -> None
and bind_pattern pattern value =
  match pattern, value with
  | PWildcard, _ -> Some []
  | PVar name, value -> Some [(name, value)]
  | PUnit, VUnit -> Some []
  | PInt expected, VInt actual -> if expected = actual then Some [] else None
  | PBool expected, VBool actual -> if expected = actual then Some [] else None
  | PTuple patterns, VTuple values -> bind_patterns patterns values
  | PConstructor (expected_name, patterns), VConstructor (name, values) ->
      if expected_name = name then bind_patterns patterns values else None
  | PListNil, VListNil -> Some []
  | PListCons (head_pattern, tail_pattern), VListCons (head, tail) ->
      bind_patterns [head_pattern; tail_pattern] [head; tail]
  | _, _ -> None

let rec eval environment expression =
  match expression with
  | EInt n -> VInt n
  | EBool b -> VBool b
  | EUnit -> VUnit
  | EVar name ->
      (match lookup name environment with Some value -> value | None -> VError)
  | EIf (condition, consequent, alternative) ->
      (match eval environment condition with
       | VBool true -> eval environment consequent
       | VBool false -> eval environment alternative
       | _ -> VError)
  | ELambda (parameter, body) -> VClosure (parameter, body, environment)
  | EApply (procedure, argument) ->
      let procedure_value = eval environment procedure in
      let argument_value = eval environment argument in
      (match procedure_value with
       | VClosure (parameter, body, saved_environment) ->
           eval ((parameter, argument_value) :: saved_environment) body
       | _ -> VError)
  | ELet (name, right_hand_side, body) ->
      let value = eval environment right_hand_side in
      eval ((name, value) :: environment) body
  | ELetRec (name, parameter, function_body, body) ->
      let rec recursive_closure =
        VClosure (parameter, function_body, (name, recursive_closure) :: environment)
      in
      eval ((name, recursive_closure) :: environment) body
  | EAdd (left, right) ->
      let left_value = eval environment left in
      let right_value = eval environment right in
      (match left_value, right_value with
       | VInt x, VInt y -> VInt (x + y)
       | _, _ -> VError)
  | ESub (left, right) ->
      let left_value = eval environment left in
      let right_value = eval environment right in
      (match left_value, right_value with
       | VInt x, VInt y -> VInt (x - y)
       | _, _ -> VError)
  | EEqual (left, right) ->
      let left_value = eval environment left in
      let right_value = eval environment right in
      (match left_value, right_value with
       | VInt x, VInt y -> VBool (x = y)
       | VBool x, VBool y -> VBool (x = y)
       | VUnit, VUnit -> VBool true
       | _, _ -> VError)
  | ELess (left, right) ->
      let left_value = eval environment left in
      let right_value = eval environment right in
      (match left_value, right_value with
       | VInt x, VInt y -> VBool (x < y)
       | _, _ -> VError)
  | ETuple expressions -> VTuple (eval_many environment expressions)
  | EConstructor (name, expressions) ->
      VConstructor (name, eval_many environment expressions)
  | EMatch (scrutinee, cases) -> eval_cases environment (eval environment scrutinee) cases
  | EListNil -> VListNil
  | EListCons (head, tail) ->
      let head_value = eval environment head in
      let tail_value = eval environment tail in
      VListCons (head_value, tail_value)
  | ERef initial_expression -> VRef (ref (eval environment initial_expression))
  | EDeref reference ->
      (match eval environment reference with VRef cell -> !cell | _ -> VError)
  | EAssign (reference, right_hand_side) ->
      let reference_value = eval environment reference in
      let assigned_value = eval environment right_hand_side in
      (match reference_value with
       | VRef cell -> cell := assigned_value; VUnit
       | _ -> VError)
  | EArrayMake (length_expression, initial_expression) ->
      let length_value = eval environment length_expression in
      let initial_value = eval environment initial_expression in
      (match length_value with
       | VInt length -> if length < 0 then VError else VArray (Array.make length initial_value)
       | _ -> VError)
  | EArrayGet (array_expression, index_expression) ->
      let array_value = eval environment array_expression in
      let index_value = eval environment index_expression in
      (match array_value, index_value with
       | VArray array, VInt index ->
           if index < 0 || index >= Array.length array then VError else Array.get array index
       | _, _ -> VError)
  | EArraySet (array_expression, index_expression, value_expression) ->
      let array_value = eval environment array_expression in
      let index_value = eval environment index_expression in
      let new_value = eval environment value_expression in
      (match array_value, index_value with
       | VArray array, VInt index ->
           if index < 0 || index >= Array.length array then VError
           else (Array.set array index new_value; VUnit)
       | _, _ -> VError)
  | ESequence (first, second) ->
      let _ = eval environment first in
      eval environment second
and eval_many environment expressions =
  match expressions with
  | [] -> []
  | expression :: rest ->
      let value = eval environment expression in
      value :: eval_many environment rest
and eval_cases environment value cases =
  match cases with
  | [] -> VError
  | (pattern, body) :: rest ->
      (match bind_pattern pattern value with
       | None -> eval_cases environment value rest
       | Some bindings -> eval (extend bindings environment) body)
and show value =
  match value with
  | VInt n -> string_of_int n
  | VBool true -> "true"
  | VBool false -> "false"
  | VUnit -> "unit"
  | VTuple _ -> "tuple"
  | VConstructor (name, _) -> name
  | VListNil -> "list"
  | VListCons _ -> "list"
  | VClosure _ -> "closure"
  | VRef _ -> "ref"
  | VArray _ -> "array"
  | VError -> "error"

let guest_program =
  ELet
    ("cell", ERef (EInt 0),
     ELetRec
       ("count", "n",
        EIf
          (EEqual (EVar "n", EInt 0),
           EMatch
             (EConstructor ("Present", [EDeref (EVar "cell")]),
              [ (PConstructor ("Present", [PVar "answer"]), EVar "answer");
                (PConstructor ("Absent", []), EInt 0) ]),
           ESequence
             (EAssign (EVar "cell", EAdd (EDeref (EVar "cell"), EInt 1)),
              EApply (EVar "count", ESub (EVar "n", EInt 1)))),
        EApply (EVar "count", EInt 7)))

let () = print_endline (show (eval [] guest_program))
```

The kernel is self-describing in the required sense: its syntax, environment, values, closure capture, recursive binding, constructor matching, references, and machine-array operations are all defined with admitted OCaml constructs. The checker for the full grammar must reject an invalid guest type before calling `eval`; the error values above are defensive evaluator outcomes, not a substitute for source type checking. Native feasibility of this kernel is not the full guest-source self-interpretation proof. That proof requires the complete teaching evaluator as guest source, parsed and type-checked by the subset checker, executed by the teaching evaluator on a translated guest program, and compared with direct execution. It is **UNRUN** and lands with the Phase 2 conformance run.

## 12. Native OCaml 5.5.1 oracle

The pinned switch is named `5.5.1`; `ocaml/justfile` creates that switch and `ocaml/dune-project` pins the same compiler. These commands use only `ocamlc` and Stdlib. Save the evaluator-kernel block of §11 as `kernel.ml`, and the three witness blocks below as `positive.ml`, `negative-type.ml`, and `unsupported.ml`, all in the temporary directory the first command creates.

```sh
tmpdir="$(mktemp -d /tmp/mig-ocaml-grammar.XXXXXX)"
opam exec --switch=5.5.1 -- ocamlc -version
# Verified by the parent: 5.5.1

opam exec --switch=5.5.1 -- ocamlc -config-var word_size
# Verified by the parent: prints 64 (exit 0), so Sys.int_size = 63.

opam exec --switch=5.5.1 -- ocamlc -c -warn-error +8 "$tmpdir/kernel.ml"
opam exec --switch=5.5.1 -- ocamlc -warn-error +8 -o "$tmpdir/kernel" "$tmpdir/kernel.ml"
"$tmpdir/kernel"
# Verified by the parent: stdout 7, status 0

opam exec --switch=5.5.1 -- ocamlc -c -warn-error +8 "$tmpdir/positive.ml"
opam exec --switch=5.5.1 -- ocamlc -warn-error +8 -o "$tmpdir/positive" "$tmpdir/positive.ml"
"$tmpdir/positive"
# Verified by the parent: stdout 7, status 0

opam exec --switch=5.5.1 -- ocamlc -c -warn-error +8 "$tmpdir/negative-type.ml"
# Verified by the parent: nonzero compile status (type error); do not run it.

opam exec --switch=5.5.1 -- ocamlc -c -warn-error +8 "$tmpdir/unsupported.ml"
# Verified by the parent: zero compile status, proving host-validity; the subset admission layer must reject it before evaluation.
```

**NATIVE-VERIFIED direct-native guest source** (`positive.ml`); parent run: exit 0, stdout `7`; the recursive closure observes writes to its captured reference, then matches a closed variant:

```ocaml
type marker = Present of int | Absent

let () =
  let cell = ref 0 in
  let rec count n =
    if n = 0 then
      (match Present (!cell) with
       | Present answer -> answer
       | Absent -> 0)
    else
      (cell := !cell + 1;
       count (n - 1))
  in
  print_endline (string_of_int (count 7))
```

**NATIVE-VERIFIED host-invalid type witness** (`negative-type.ml`); parent run: `ocamlc` rejects it with a type error before the print can run:

```ocaml
let () =
  print_endline "must-not-run";
  let _ = 1 + true in
  ()
```

**NATIVE-VERIFIED host-valid, subset-unsupported witness** (`unsupported.ml`): native compilation exited zero. The program was not executed. The future subset checker must reject the loop before any guest effect; that rejection remains **UNRUN**.

```ocaml
let () =
  print_endline "must-not-run";
  while false do () done
```

The witness blocks are parent-verified native oracles (evidence `local://modern-sicp-ocaml-contract-native-results.json` and `local://modern-sicp-ocaml-kernel-corrected-results.json`); this contract task ran no compiler. The invalid-source witnesses check distinct boundaries: host type failure and host-valid unsupported syntax. The latter must be rejected by the subset parser before an interpreter, machine, or native executable starts.

## 13. Chapter 4 and 5 lesson map

Each row identifies the lesson family and the host subset features that preserve it. The exercise ranges are the edition's current numbered ranges; keep every number and objective when consumers migrate.

| Section and exercise range | Lesson family | Admitted representation and execution boundary |
|---|---|---|
| 4.1, 4.1–4.24 | Direct evaluator, data-directed dispatch, special forms, derived forms, environments, internal definitions, recursive bindings, analyzed evaluator | Recursive `expr`/`value` variants, typed functions and closures, `let rec`, exhaustive matching, lists/tuples, refs for guest environments, and explicit AST constructors. Direct and analyzed evaluators consume the same typed AST. Operand-order lessons state OCaml's unspecified order rather than inventing one. |
| 4.2, 4.25–4.34 | Applicative versus normal order, thunks, forcing, memoization, lazy-list behavior | Separate lazy experiment (§10): explicit thunk and memo cells; strict primitives; forcing rules are experiment behavior, not default core evaluation. |
| 4.3, 4.35–4.54 | Choice, failure, constraint search, ambiguous parsing, backtracking, reversible/permanent mutation, fallback | Separate search experiment (§10), driven by typed choice/failure/answer events and restartable choice paths. Its `require`-style failures and permanent updates never change the core `ref` contract. |
| 4.4, 4.55–4.79 | Query terms, rules, unification, occurs checks, frames, fair stream evaluation, delayed filtering, query dispatch | Query, term, frame, and rule ADTs built with host constructors and lists. Frames are immutable. Search over answers is an explicit lazy query engine, not a new core expression grammar. |
| 5.1, 5.1–5.6 | Register-machine design, controller steps, labels, stack reasoning | `source`, `instruction`, `word`, and `controller` variants/lists; labels are typed data. Diagrams and hand simulation describe the same transitions as the simulator. |
| 5.2, 5.7–5.19 | Assembler, machine execution, register derivation, stack monitors, tracing, breakpoints | Typed instruction constructors, label-to-index arrays, hash-table operation dispatch, arrays for registers, refs for program counter and stack state, and explicit `option`/`result` errors. Validate duplicate labels, registers, and operations before execution. |
| 5.3, 5.20–5.22 | Pair memory, vector layout, allocation, roots, copying collection | Two semispaces each use parallel `word array` car/cdr storage. The heap and word are host data constructors; `Array.get`/`Array.set` implement indexed cells. The collector follows forwarding pointers, rewrites every register/stack root, and preserves shared pairs. |
| 5.4, 5.23–5.30 | Explicit-control evaluator, derived syntax, evaluation order, tail calls, stack behavior, error signaling | Reuse chapter 4 `expr`/`value`/environment types and chapter 5 `instruction`/`word` types. Controller is a checked instruction list. Stack save/restore uses lists or arrays and explicit mutation; machine errors are typed values. |
| 5.5, 5.31–5.52 | Compiler instruction sequences, register preservation, linkage, lexical addresses, open coding, mixed compiled/interpreted calls, compile-and-run, compiler output | Compile the shared typed AST to typed instruction constructors and sequences represented by tuples, variants, lists, refs, and arrays. Preserve target/linkage/needed-register contracts. Target C output is represented by host constructors and rendered as text; this is a teaching compiler, not a full OCaml compiler. |

Exercises 5.50 and 5.52 require the translated evaluator itself as valid guest source. The unit must parse and type-check that source, run it with the teaching evaluator on a translated guest program, then compare the observable result with direct native execution. The evaluator's recursion, types, patterns, functions, closures, refs, arrays, and prelude calls must all remain inside this contract. Exercise 5.51 teaches translating the evaluator to a C runtime; it does not add C or a host compiler to the OCaml guest grammar. The self-interpretation comparison, compiled-evaluator runs, and every teaching-engine claim in this table are **UNRUN** until Phase 2; the parent-verified kernel run establishes only that the subset can express an evaluator kernel natively.

Chapter-specific additions use only these same constructors and contracts: thunk counters and backtrack counts are explicit state; query frame metrics inspect ordinary frame lists; simulator traces and heap renderings observe typed machine data. An addition cannot silently admit another syntax form or host module.

## 14. Scope boundary

This grammar specifies a finite, native-typed OCaml subset and the native commands that serve as its parent oracle. It does not claim that every OCaml program is accepted, that the teaching evaluator implements every OCaml module or runtime feature, or that the exercise compiler is a production OCaml compiler. Any expansion of syntax, primitive modules, type forms, or evaluator modes requires an explicit contract update before its consumer is implemented.
