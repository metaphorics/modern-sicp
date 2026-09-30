# TypeScript guest subset

Anchor: Tony Hoare, Otl Aicher, David Parnas

Status: migration contract. Native evidence recorded (parent-run): the §9 kernel compiles under the pinned checker and runs under pinned Node printing `120`, and the negative type fixtures are rejected with `TS2322`/`TS7006` (`local://modern-sicp-typescript-contract-native-results.json`, `local://modern-sicp-typescript-admission-native-results.json`). All subset-checker, teaching-evaluator/compiler, and experimental-mode claims remain UNRUN. This file defines the language accepted by the TypeScript edition; it does not claim that the edition supports all TypeScript.

## 1. Boundary and acceptance

The guest subset is a finite, typed subset of TypeScript source. A source unit is admitted only when all three gates pass, in order:

1. The edition subset parser accepts the grammar and feature restrictions in this contract. It produces the shared tagged syntax representation described in §3. It rejects unsupported syntax before evaluating any source effect.
2. The pinned TypeScript checker accepts the source under the edition's project configuration. JavaScript execution alone is not evidence of TypeScript validity.
3. The native Node run, teaching evaluator, and (where applicable) compiled machine agree on the observable behavior specified in §10.

`typescript/tsconfig.base.json` is the type-checking authority: `strict`, `noUncheckedIndexedAccess`, `exactOptionalPropertyTypes`, `noImplicitOverride`, `noFallthroughCasesInSwitch`, `noPropertyAccessFromIndexSignature`, `noImplicitReturns`, `allowUnreachableCode: false`, `verbatimModuleSyntax`, `erasableSyntaxOnly`, `allowImportingTsExtensions`, `noEmit`, and `skipLibCheck`; target and library are `es2024`. The repository pins TypeScript `7.0.2`, Node `24.21.0`, and pnpm `12.5.1` (`typescript/package.json`, `typescript/.node-version`). These pins are requirements, not claims about unpinned installations. The host checker is TypeScript itself; the edition implements only this restricted grammar and does not implement, emulate, or claim the complete TypeScript type system.

The configured `lib` is exactly `["es2024"]` — there is no DOM library — and `types` is `["node"]`, which supplies the Node global and `node:*` module typings used at the boundary. Static named imports may target edition-local typed modules and dependencies already pinned by the edition. No dynamic import, `require`, untyped import, ambient declaration, or dependency addition is part of the guest core. Existing Node I/O used by exercise harnesses is a boundary operation, not an evaluator primitive. The self-interpreter witness below has no imports.

### 1.1 Concrete source grammar

This is the admitted grammar, not a grammar for all TypeScript. The `Type` and `TypeParams` productions above are closed; §2 adds the static rules. Identifiers use ECMAScript identifier spelling; string escapes and comments use the Node 24.21.0 ECMAScript rules. Numeric literals are decimal integer/fraction/exponent forms only. Every list below is closed: an unlisted production is unsupported, even if `tsc` accepts it.

```ebnf
Unit        ::= Import* DeclarationOrStatement*
Import      ::= "import" ["type"] "{" ImportName ("," ImportName)* [","] "}" "from" StringLiteral ";"
ImportName  ::= ["type"] Identifier ["as" Identifier]
DeclarationOrStatement ::= TypeDecl | InterfaceDecl | ExportDecl | VarDecl
                         | FunctionDecl | Statement
ExportDecl  ::= "export" (TypeDecl | InterfaceDecl | VarDecl | FunctionDecl)
Statement   ::= Block | If | While | ForOf | Switch
              | Return | Break | Continue | Throw | Try | Expr ";"
Block       ::= "{" DeclarationOrStatement* "}"
If          ::= "if" "(" Expr ")" Statement ["else" (Statement | If)]
While       ::= "while" "(" Expr ")" Statement
ForOf       ::= "for" "(" "const" Identifier "of" Expr ")" Statement
Switch      ::= "switch" "(" Expr ")" "{" CaseClause* [DefaultClause] "}"
CaseClause  ::= "case" Expr ":" DeclarationOrStatement*
DefaultClause ::= "default" ":" DeclarationOrStatement*
Return      ::= "return" [Expr] ";"
Break       ::= "break" ";"
Continue    ::= "continue" ";"
Throw       ::= "throw" Expr ";"
Try         ::= "try" Block CatchClause [FinallyClause] | "try" Block FinallyClause
CatchClause ::= "catch" ["(" Identifier ")"] Block
FinallyClause ::= "finally" Block

TypeDecl    ::= "type" Identifier [TypeParams] "=" Type ";"
InterfaceDecl ::= "interface" Identifier [TypeParams] ["extends" Type ("," Type)*]
                 "{" [PropertyDecl (Sep PropertyDecl)* [Sep]] "}"
PropertyDecl ::= ["readonly"] Identifier ["?"] ":" Type
VarDecl     ::= ("const" | "let") Identifier [":" Type] "=" Expr ";"
FunctionDecl ::= "function" Identifier [TypeParams] "(" [Parameters] ")" [":" Type] Block
Parameters  ::= Parameter ("," Parameter)* ["," [RestParameter]] | RestParameter
Parameter   ::= Identifier ["?"] ":" Type
RestParameter ::= "..." Identifier ":" PostfixType

Type        ::= ["|"] PostfixType ("|" PostfixType)*
PostfixType ::= PrimaryType {"[]"}
PrimaryType ::= "number" | "string" | "boolean" | "null" | "undefined" | "void" | "unknown"
             | LiteralType | TypeName ["<" Type ("," Type)* ">"]
             | TupleType | ObjectType | FunctionType | "(" Type ")"
LiteralType ::= NumberLiteral | StringLiteral | "true" | "false"
TypeName    ::= Identifier ["." Identifier]
TupleType   ::= "[" [Type ("," Type)* [","]] "]"
ObjectType  ::= "{" [PropertyDecl (Sep PropertyDecl)* [Sep]] "}"
Sep         ::= ";" | ","
FunctionType ::= "(" [Parameters] ")" "=>" Type
TypeParams  ::= "<" TypeParam ("," TypeParam)* ">"
TypeParam   ::= Identifier ["extends" Type]

Expr        ::= Assignment
Assignment  ::= Conditional ["=" Assignment]
Conditional ::= Or ["?" Expr ":" Expr]
Or          ::= And {"||" And}
And         ::= Equality {"&&" Equality}
Equality    ::= Compare {("===" | "!==") Compare}
Compare     ::= Add {("<" | "<=" | ">" | ">=") Add}
Add         ::= Multiply {("+" | "-") Multiply}
Multiply    ::= Unary {("*" | "/" | "%") Unary}
Unary       ::= ("!" | "+" | "-" | "typeof") Unary | Postfix
Postfix     ::= Primary {"." Identifier | "[" Expr "]" | "(" [Arguments] ")"}
Primary     ::= NumberLiteral | StringLiteral | "true" | "false" | "null" | "undefined"
             | Identifier | "(" Expr ")" | ArrayLiteral | ObjectLiteral | TemplateLiteral
             | Arrow | NewExpr
NewExpr     ::= NewError | NewMap | NewSet
NewError    ::= "new" "Error" "(" [Expr] ")"
NewMap      ::= "new" "Map" ["<" Type ("," Type)* ">"] "(" [Expr] ")"
NewSet      ::= "new" "Set" ["<" Type ">"] "(" [Expr] ")"
ArrayLiteral ::= "[" [ArrayElement ("," ArrayElement)* [","]] "]"
ArrayElement ::= "..." Expr | Expr
ObjectLiteral ::= "{" [Property ("," Property)* [","]] "}"
Property    ::= Identifier | Identifier ":" Expr | StringLiteral ":" Expr
TemplateLiteral ::= "`" TemplatePart* "`"
TemplatePart ::= TemplateChar | Escape | "${" Expr "}"
TemplateChar ::= any character other than "`", "\", or the start of "${"
Escape       ::= "\" any character
Arrow       ::= Identifier "=>" (Expr | Block)
             | "(" [Parameters] ")" [":" Type] "=>" (Expr | Block)
Arguments   ::= Argument ("," Argument)* [","]
Argument    ::= "..." Expr | Expr
```

A parser may preserve source spans for every node, but must not lower unsupported host syntax into a supported node. This core excludes class declarations/expressions, `new` except `NewExpr`, enums, namespaces, decorators, `this`/`super`, generators, `async`/`await`, `yield`, `for-in`, `for`-counter loops, labels, tagged templates, regular-expression literals, type assertions (`as`, angle-bracket form), non-null assertions, `satisfies`, overloads, declaration merging, conditional/mapped/indexed-access/template-literal types, and JSX. `erasableSyntaxOnly` independently rejects TypeScript constructs requiring runtime erasure or transformation beyond the pinned setup.

Admission rules for the productions above: generic type parameters are admitted on type aliases, interfaces, and functions only — at most two parameters per declaration, each optionally bounded by `extends Type`; conditional, mapped, variadic, and higher-kinded types stay excluded. `readonly` is admitted on `PropertyDecl` fields only; `readonly` parameters are excluded because the TypeScript form is a constructor parameter property and classes are excluded. A non-block `if`/`else`, `while`, or `for-of` body is exactly one `Statement`, and `else` binds to the nearest `if` (the ECMAScript rule). Every non-empty `switch` case body must guarantee abrupt completion: control cannot reach the end of the body on any path (its last statement is `return`, `throw`, `break`, or a construct that completes abruptly on every path); a body that can complete normally is rejected, empty fallthrough between labels is fine, matching the pinned `noFallthroughCasesInSwitch`. `typeof` is a unary operator returning `string` and is the required narrowing probe for union values. `TupleType` arity is fixed by its element list; `PostfixType` `[]` nesting is the only array-type form. A `RestParameter` must be the last parameter and admits no trailing separator; its type must be an array type, and at a call it collects the remaining arguments left-to-right into one fresh array of that element type.

`try`/`catch`/`finally` and `throw` are admitted only at host I/O / process boundary code (not inside the guest evaluator kernel); host abrupt-completion behavior is exactly JavaScript behavior. Recoverable evaluator, query, parser, and machine failures use the explicit result unions in §8. Do not convert a caught host failure into a plausible guest value.

### 1.2 Shared syntax contract

The contract applies identically to the three existing entry points: `typescript/packages/ch4/src/read.ts` `read(text)` reads exactly one expression and rejects trailing input; `readAll(text)` reads a program (zero or more forms in source order); `typescript/packages/ch5/src/04-eceval.ts` `readProgram(text)` reads a program and is re-exported as `parse`. All three lower accepted source to the same typed `Expr`/declaration AST of §3 — that AST is the common boundary every evaluator, compiler, and machine consumer reads, and no chapter keeps a second representation. After migration both chapter paths must accept and reject the same source productions, preserve the same source locations and diagnostic categories, and lower accepted programs to that shared representation. Chapter-specific machine and query data are constructed as typed host values (§7), not by adding private parser grammars. Tests must feed the same positive and negative source corpus through both chapter paths at each entry-point shape (single-expression `read`, program `readAll`/`readProgram`); no old-source compatibility path or syntax alias remains.

## 2. Types and static rules

Admit primitive types `number`, `string`, `boolean`, `null`, `undefined`, `void`, `unknown`, declared names, bounded generic aliases/interfaces/functions, unions, fixed tuples, `T[]`, `ReadonlyArray<T>`, and object types with explicitly declared fields. The existing built-ins `Readonly<T>` and `Record<string, T>` may be used where their indexed reads remain checked under `noUncheckedIndexedAccess`. Recursive types use interfaces or aliases with object/array/union members. Discriminated unions use a literal string field such as `tag`; exhaustive `switch` statements must cover each variant and return on every reachable path.

`unknown` is permitted only at a host boundary and must be narrowed before use. Explicit and inferred `any`, user-defined type predicates, unsafe type assertions, suppression comments (`@ts-ignore`, `@ts-expect-error`), and untyped module boundaries are rejected by the edition subset checker even where TypeScript accepts them. Public function parameters and exported values have explicit types; local types may be inferred when `tsc` determines them without widening to `any`. `noUncheckedIndexedAccess` means an array or record index can be absent; code must narrow `undefined` before treating the read as an element. Optional fields and `undefined` obey `exactOptionalPropertyTypes`.

The subset checker imposes two stricter rules than `tsc` alone: `if`, `while`, `&&`, and `||` conditions/operands must be `boolean` (no JavaScript truthiness); and arithmetic operands must be compatible numeric values. For `+`, both operands must be `number` or both must be `string`; mixed concatenation/coercion is unsupported. Equality is strict equality only. The pinned checker must also accept the program; this contract does not replace TypeScript's type checking.

Static rejection is complete before guest-visible effects. The driver sequence is: parse all source units; reject unsupported syntax; run subset checks; run pinned `tsc --noEmit`; only then execute. A failure at any earlier stage returns diagnostics and performs no program I/O or mutation. Host-valid but unsupported source is reported as `UnsupportedSyntax` (with source span and construct kind), not rewritten or sent to the evaluator.

## 3. Shared typed syntax and values

The common syntax model is a tagged discriminated union, not an untyped token list. The minimum core nodes are literals, variable reads, arrays and object construction/property access, unary/binary operations, conditional expressions, assignment, blocks/sequences, `const`/`let` binding, functions, calls, `if`, `while`, `for-of`, `return`, and the boundary-only try/throw forms. Each node records its source span. Node construction and every evaluator/compiler `switch` are exhaustively typed. The representation for a guest evaluator's own syntax is itself an ordinary recursive data type: a union of tagged records and arrays; it is data, never host executable code.

Runtime values are JavaScript primitives plus dense arrays, plain data records, `Map`, `Set`, and closures. A closure is the function body plus the lexical environment in which it was created; calls allocate a child environment. Pair/list structures are recursive tagged objects (for example, `{ tag: "cons", head: value, tail: list }` and `{ tag: "nil" }`), so recursive data is finite, inspectable, and type-checked. Query ASTs, machine words, evaluator syntax, errors, and outcomes are discriminated unions. Do not use `unknown`, unchecked casts, or a catch-all object payload as a substitute for these types in the guest evaluator kernel.

## 4. Numeric, text, collection, and record rules

### 4.1 Numbers: `number` only in the core

The core numeric type is JavaScript `number` (IEEE-754 binary64); `bigint` is not admitted and there are no implicit or explicit number/bigint conversions. This follows the current teaching model: chapter 4 `NumberValue.n` and chapter 5 machine `Value` are `number`; the machine exercises use ordinary arithmetic/counters, and the square-root exercise uses `Math.sqrt` with a stated approximation tolerance. Adding a second numeric tower would require teaching mixed arithmetic and conversion rules that those examples do not use.

Admitted operators are numeric `+`, `-`, `*`, `/`, `%`, and comparisons `<`, `<=`, `>`, `>=`; exact equality is `===`/`!==`. Arithmetic follows native `number` rules without rational repair: rounding is binary64, `1 / 0` is positive infinity, `0 / 0` is `NaN`, and overflow can produce infinity. Only integers with magnitude at most `Number.MAX_SAFE_INTEGER` are guaranteed exact. Programs must not claim unbounded or exact integer arithmetic. `Math` calls are limited to `abs`, `floor`, `max`, `min`, `sqrt`, and `trunc`; `Number.isInteger` is admitted for integer-domain checks. `Math.random` and wall-clock/time sources are excluded from core behavior; randomized search belongs to the named experiment in §6.

### 4.2 Strings and arrays

Strings are immutable ECMAScript strings. Admit `.length`, numeric character reads, `slice`, `split`, `startsWith`, `includes`, `replaceAll`, `padEnd`, `trimEnd`, and `at`, plus same-type string concatenation and template literals without tagged templates. These cover token/source diagnostics, query-variable normalization, and text rendering; current uses include `read.ts` slicing, `04-eceval.ts` tokenization, and `04-logic.ts` query-variable handling.

Arrays are ordered and mutable. Admit array literals, dense element construction, numeric read/write, `.length`, spread of arrays/tuples, and `push`, `pop`, `slice`, `map`, `flat`, `flatMap`, `filter`, `find`, `some`, `every`, `includes`, `reduce`, `reduceRight`, `join`, `forEach`, `reverse`, and `at`. `Array.from` is admitted only for finite inputs. Native mutation/iteration order applies; indexed reads remain possibly absent statically. Do not treat arrays as persistent lists: a guest interpreter that needs persistent recursive values uses the tagged object constructors above. Existing implementations use these collection idioms in `list.ts`, `03-nondeterministic.ts`, `04-logic.ts`, and `02-simulator.ts`.

### 4.3 Recursive objects and unions

Object literals contain only statically named own data fields. Property access and assignment use declared fields; `readonly` forbids writes. Computed keys, getters/setters, prototype mutation, property descriptors, `Object.create`, `Object.assign`, and reflection are excluded. A recursive object may point to itself or to other typed objects; identity comparison is object identity, not deep structural equality. `Map<K,V>` and `Set<T>` are constructed only by `NewMap`/`NewSet`: `new Map<K,V>()` or `new Map<K,V>(entries)` with exactly one `ReadonlyArray<[K, V]>` argument, and `new Set<T>()` or `new Set<T>(items)` with exactly one `T[]`/`ReadonlyArray<T>` argument; the explicit type arguments are optional where TypeScript infers them. Their methods are admitted only through `Map` (`get`, `set`, `has`, `delete`, `entries`, `keys`, `values`, `size`) and `Set` (`add`, `has`, `delete`, `values`, `size`). Their native insertion order and key equality apply. `undefined` from absent map/array lookups must be tested before dereference.

### 4.4 Admitted host library and I/O boundary

The core evaluator kernel may use only the language forms and library members named in this contract; no reflection, dynamic code generation, `eval`, `Function` constructor, or module loading is a guest primitive. Static named imports link the edition's typed modules and already-pinned dependencies; they do not enlarge guest expression syntax. Existing `effect` operations may remain in a driver/error-boundary module only, with their typed error channel preserved; they are not added to the guest evaluator's builtin table. Node `node:fs`, `node:path`, `node:os`, and `node:child_process` are boundary-only for the existing file/compiler exercise harnesses, not evaluator primitives. `console.log` with one argument is the admitted ordered output operation at the boundary driver; the guest evaluator kernel returns transcript data instead of calling it. `new` is excluded everywhere except `NewExpr`: `new Error(message)` is admitted only for boundary driver assertions and failures (as in the §9 witness tail), while `new Map(...)` and `new Set(...)` are the §4.3 core constructors; guest evaluator, query, and machine errors remain the §8 data unions. No dependency is added. `JSON.stringify` and `String(value)` are allowed only for diagnostics/transcript rendering, not to serialize closures or replace typed ASTs.

## 5. Bindings, closures, and state

Only initialized `const` and `let` bindings are admitted; `var` is excluded. `const` prevents rebinding, not mutation of an object reachable through the binding. `let` may be reassigned only with a value assignable to its declared/inferred TypeScript type. A binding is block-scoped; duplicate declarations in the same lexical scope are rejected. Function declarations and arrow functions capture lexical bindings. A captured `const` remains fixed; a captured `let` is a shared binding/cell, so a closure called after a write observes the new value. Object and array captures share object identity; mutations are visible through every alias. Function arguments are eagerly evaluated once, left-to-right, before entry. Recursion is admitted through a function declaration and through a `const`/`let`-bound function expression that references its own binding lexically (for example `const f = (n: number): number => n <= 1 ? 1 : n * f(n - 1);`); the recursive call is legal once initialization has completed, and a call reached before initialization is a TDZ `ReferenceError`. Recursive environment frames must not copy captured mutable cells.

The subset follows ECMAScript temporal-dead-zone behavior for a read before a `let`/`const` initialization (runtime `ReferenceError` if a program can reach it). The subset checker rejects statically evident use-before-initialization and reassignment of a `const`; it does not pretend that TypeScript's definite-assignment analysis proves every closure ordering safe. All variable/member/index writes are simple `=` assignments; `++`, `--`, compound assignment, `delete`, and rebinding a read-only field are excluded. The evaluator's binding model must preserve shared-cell writes and closure escape; it must not copy a captured value when creating a procedure.

## 6. Evaluation order, effects, and named experiments

Core execution is strict ECMAScript evaluation, with module strict mode and the pinned Node 24.21.0 runtime. Evaluate a call's callee before arguments, arguments left-to-right, array elements and object property initializers left-to-right, binary operands left-to-right, and a conditional's test before exactly one selected arm. `&&` and `||` short-circuit left-to-right and are admitted only for boolean operands. A simple assignment evaluates its target reference (base and computed key, when present) before the right-hand side, then performs the write. Blocks, declarations, loops, function calls, and explicit output preserve source order. `for-of` is restricted to arrays and uses Node's native array-iterator behavior. Core source has no implicit nondeterministic, lazy, or backtracking behavior.

Evaluator effects use explicit mutable cells/records and ordered transcript/output arrays. The evaluator itself returns transcript data; console/process/file I/O is confined to the boundary drivers. No oracle comparison may reorder output or erase a mutation. For any source rejected statically, no guest effect occurs. JavaScript exceptions from the boundary are not guest evaluator results.

Two experiments are separate named execution modes, never core TypeScript features and never submitted to the native oracle as if JavaScript were lazy or nondeterministic:

- `lazy-memoized-experiment`: admits only the explicitly marked delay/force AST extension. A delay captures its lexical environment; first force computes and stores the value; later forces return the stored value. The sibling `lazy-recompute-experiment` is a distinct mode whose force recomputes. They are not parser aliases for ordinary function calls and do not alter core strictness.
- `amb-depth-first-experiment`: admits the `choose`, `require`, `permanentAssign(target, value)`, and `ifFail(expression, fallback)` AST extensions. Alternatives run left-to-right depth-first. Ordinary `=` assignment is undone newest-first on backtracking; `permanentAssign` keeps its write. `ifFail` yields every primary-expression result and evaluates the fallback once only after the primary exhausts. `amb-ramb-experiment` admits the same forms plus seeded, deterministic `ramb`; it is a named experiment, not core randomness.

`SearchRun.failures` counts failed candidate computations that schedule backtracking, not successful-result resumptions or exhausted-choice propagation. `steps` counts deferred continuations actually executed; `maxSteps` excludes the initial evaluation. `maxAnswers` and `maxSteps` are checked before a pending continuation runs, so `cut-off` means search work remains; `maxAnswers: 0` performs no guest evaluation. These cooperative limits do not interrupt one guest evaluation or recursive continuation that fails to yield.

Every extension is rejected by the core parser/checker and requires its own finite independent reference model, named mode, source-admission rule, and behavioral oracle. The default evaluator and native oracle continue to use eager JavaScript evaluation and effect order.

## 7. Queries and register machines are host data

The 4.4 query language and 5.1–5.3 register-machine controller are domain models, not extra host syntax or claims about built-in TypeScript features. Build them from typed constructors and discriminated data values. For example:

```ts
type Query =
  | { readonly tag: "atom"; readonly relation: string; readonly fields: ReadonlyArray<Term> }
  | { readonly tag: "and"; readonly clauses: ReadonlyArray<Query> }
  | { readonly tag: "or"; readonly clauses: ReadonlyArray<Query> }
  | { readonly tag: "not"; readonly clause: Query };
type Term = { readonly tag: "var"; readonly name: string }
  | { readonly tag: "text"; readonly value: string };
const queryAtom = (relation: string, ...fields: Term[]): Query =>
  ({ tag: "atom", relation, fields });

type Input = { readonly tag: "reg"; readonly name: string }
  | { readonly tag: "const"; readonly value: MachineValue }
  | { readonly tag: "label"; readonly name: string };
type Instruction =
  | { readonly tag: "assign"; readonly register: string; readonly source: Source }
  | { readonly tag: "test"; readonly operation: string; readonly args: ReadonlyArray<Input> }
  | { readonly tag: "branch"; readonly label: string }
  | { readonly tag: "goto-label"; readonly label: string }
  | { readonly tag: "goto-register"; readonly register: string }
  | { readonly tag: "save"; readonly register: string }
  | { readonly tag: "restore"; readonly register: string };
const register = (name: string): Input => ({ tag: "reg", name });
const constant = (value: MachineValue): Input => ({ tag: "const", value });
```

`Source` and `MachineValue` are ordinary recursively typed unions. Constructor functions and record literals are checked by `tsc`; unknown operation names, registers, labels, invalid operand types, stack underflow, and restore mismatch are runtime `MachineError` variants, not parser fallbacks. This follows current data models in `04-logic.ts`, `01-register-machines.ts`, and `02-simulator.ts`. Query/controller text parsers, if retained temporarily during cutover, must consume the same typed model and be removed when their callers migrate.

Chapter 2.5 data-directed arithmetic uses each operation name as its exact registry key. Polynomial operations are installed as `put("add", ["polynomial", "polynomial"], ...)`, `put("mul", ["polynomial", "polynomial"], ...)`, and `put("div", ["polynomial", "polynomial"], ...)`; helper names and diagnostic labels are not dispatch keys. The complex constructor is installed as `put("make-from-real-imag", ["complex"], ...)` and looked up with the same operation string and tag tuple. Book examples must use these registered key forms rather than invented capitalization or helper labels.

## 8. Errors and diagnostics

Keep failure layers distinct and stable by category, not by compiler prose:

| Layer | Required result |
|---|---|
| Host syntax not in §1.1 | `UnsupportedSyntax { span, construct }`; reject before evaluation. |
| Host TypeScript syntax/type failure | Pinned TypeScript diagnostic; reject before evaluation. Preserve diagnostic file/span/code, not exact wording. |
| Core parser/subset rule failure | Typed diagnostic such as `ExpectedBoolean`, `UnsupportedType`, or `ForbiddenHostPrimitive`, with source span. |
| Guest evaluator failure | `Outcome = { tag: "ok", value } \| { tag: "error", error }` (the §9 name); variants include unbound name, non-callable value, wrong arity, bad operand, and unknown syntax node. |
| Query/machine failure | Typed `QueryError`/`MachineError`; machine categories include duplicate/unknown label, unknown register/operation, bad target, stack underflow/mismatch, and out-of-steps. |
| Host runtime or I/O failure | Preserve the JavaScript/Node abrupt completion at the host boundary; do not report it as successful guest evaluation or compare unstable stack text. |

A source checker must not equate a late runtime failure with a static rejection. Runtime checks that remain necessary (unbound names, malformed externally constructed domain data, machine bounds, numeric domain preconditions) return the declared typed error and do not silently substitute `false`, zero, or an empty result.

## 9. Guest evaluator kernel witness (native-checked; subset/teaching UNRUN)

This standalone host source is the minimum positive self-interpreter kernel: typed recursive syntax data, pattern matching, lexical closures, shared recursive binding, conditionals, arithmetic, and evaluation of a recursively defined guest factorial. The kernel proper is the `Expr`/`Value`/`Env` data, `findCell`, and `evalExpr`; the file tail (`guestFactorial`, the native comparison, `throw`, `console.log`) is boundary driver code under §1.1 and §4.4, not an evaluator primitive. It uses only productions and values admitted above; it has no host evaluator, reflection, generated code, or production interpreter primitive. In the migrated edition it must pass the subset parser/checker and pinned `tsc`, then run directly under the pinned Node type stripper. The guest program is assembled from typed syntax constructors, as permitted for source trees and domain data. Final output is `120`: the parent native run recorded pinned `tsc` exit 0 with empty diagnostics and pinned Node exit 0 with stdout `120\n` (`local://modern-sicp-typescript-contract-native-results.json`); the kernel's own interpreted-vs-native factorial assertion passed inside that run. Subset-checker acceptance and teaching-evaluator/compiler conformance for this source remain UNRUN.

```ts
type Expr =
  | { readonly tag: "number"; readonly value: number }
  | { readonly tag: "variable"; readonly name: string }
  | { readonly tag: "add"; readonly left: Expr; readonly right: Expr }
  | { readonly tag: "subtract"; readonly left: Expr; readonly right: Expr }
  | { readonly tag: "multiply"; readonly left: Expr; readonly right: Expr }
  | { readonly tag: "lessOrEqual"; readonly left: Expr; readonly right: Expr }
  | { readonly tag: "if"; readonly condition: Expr; readonly consequent: Expr; readonly alternative: Expr }
  | { readonly tag: "lambda"; readonly parameter: string; readonly body: Expr }
  | { readonly tag: "call"; readonly operator: Expr; readonly argument: Expr }
  | { readonly tag: "letrec"; readonly name: string; readonly parameter: string; readonly definition: Expr; readonly body: Expr };
type Closure = { readonly tag: "closure"; readonly parameter: string; readonly body: Expr; readonly environment: Env | null };
type Value = number | boolean | Closure;
type EvalError =
  | { readonly tag: "unbound-name"; readonly name: string }
  | { readonly tag: "expected-number"; readonly operator: string }
  | { readonly tag: "expected-boolean" }
  | { readonly tag: "not-callable" };
type Outcome = { readonly tag: "ok"; readonly value: Value }
  | { readonly tag: "error"; readonly error: EvalError };
type Cell = { value: Value | undefined };
type Binding = { readonly name: string; readonly cell: Cell };
type Env = { readonly bindings: Binding[]; readonly parent: Env | null };

const ok = (value: Value): Outcome => ({ tag: "ok", value });
const fail = (error: EvalError): Outcome => ({ tag: "error", error });
const child = (parent: Env | null): Env => ({ bindings: [], parent });
const findCell = (environment: Env | null, name: string): Cell | undefined => {
  let current = environment;
  while (current !== null) {
    for (const binding of current.bindings) {
      if (binding.name === name) return binding.cell;
    }
    current = current.parent;
  }
  return undefined;
};
const evalExpr = (expression: Expr, environment: Env | null): Outcome => {
  switch (expression.tag) {
    case "number":
      return ok(expression.value);
    case "variable": {
      const cell = findCell(environment, expression.name);
      if (cell === undefined || cell.value === undefined)
        return fail({ tag: "unbound-name", name: expression.name });
      return ok(cell.value);
    }
    case "add":
    case "subtract":
    case "multiply": {
      const left = evalExpr(expression.left, environment);
      if (left.tag !== "ok") return left;
      const right = evalExpr(expression.right, environment);
      if (right.tag !== "ok") return right;
      if (typeof left.value !== "number" || typeof right.value !== "number")
        return fail({ tag: "expected-number", operator: expression.tag });
      if (expression.tag === "add") return ok(left.value + right.value);
      if (expression.tag === "subtract") return ok(left.value - right.value);
      return ok(left.value * right.value);
    }
    case "lessOrEqual": {
      const left = evalExpr(expression.left, environment);
      if (left.tag !== "ok") return left;
      const right = evalExpr(expression.right, environment);
      if (right.tag !== "ok") return right;
      if (typeof left.value !== "number" || typeof right.value !== "number")
        return fail({ tag: "expected-number", operator: "lessOrEqual" });
      return ok(left.value <= right.value);
    }
    case "if": {
      const condition = evalExpr(expression.condition, environment);
      if (condition.tag !== "ok") return condition;
      if (typeof condition.value !== "boolean") return fail({ tag: "expected-boolean" });
      return evalExpr(condition.value ? expression.consequent : expression.alternative, environment);
    }
    case "lambda":
      return ok({ tag: "closure", parameter: expression.parameter, body: expression.body, environment: environment });
    case "call": {
      const target = evalExpr(expression.operator, environment);
      if (target.tag !== "ok") return target;
      const argument = evalExpr(expression.argument, environment);
      if (argument.tag !== "ok") return argument;
      if (typeof target.value !== "object" || target.value.tag !== "closure")
        return fail({ tag: "not-callable" });
      const callEnvironment = child(target.value.environment);
      callEnvironment.bindings.push({ name: target.value.parameter, cell: { value: argument.value } });
      return evalExpr(target.value.body, callEnvironment);
    }
    case "letrec": {
      const recursiveEnvironment = child(environment);
      const cell: Cell = { value: undefined };
      recursiveEnvironment.bindings.push({ name: expression.name, cell });
      cell.value = {
        tag: "closure",
        parameter: expression.parameter,
        body: expression.definition,
        environment: recursiveEnvironment,
      };
      return evalExpr(expression.body, recursiveEnvironment);
    }
  }
};
const guestFactorial: Expr = {
  tag: "letrec",
  name: "fact",
  parameter: "n",
  definition: {
    tag: "if",
    condition: {
      tag: "lessOrEqual",
      left: { tag: "variable", name: "n" },
      right: { tag: "number", value: 1 },
    },
    consequent: { tag: "number", value: 1 },
    alternative: {
      tag: "multiply",
      left: { tag: "variable", name: "n" },
      right: {
        tag: "call",
        operator: { tag: "variable", name: "fact" },
        argument: {
          tag: "subtract",
          left: { tag: "variable", name: "n" },
          right: { tag: "number", value: 1 },
        },
      },
    },
  },
  body: {
    tag: "call",
    operator: { tag: "variable", name: "fact" },
    argument: { tag: "number", value: 5 },
  },
};
const nativeFactorial = (n: number): number => n <= 1 ? 1 : n * nativeFactorial(n - 1);
const answer = evalExpr(guestFactorial, null);
if (answer.tag !== "ok" || typeof answer.value !== "number" || answer.value !== nativeFactorial(5))
  throw new Error("native and interpreted factorial disagree");
console.log(answer.value);
```

## 10. Witnesses and independent oracle protocol

All examples in this section are witness source. Native evidence recorded so far (parent-run, `local://modern-sicp-typescript-contract-native-results.json` and `local://modern-sicp-typescript-admission-native-results.json`): the §9 kernel compiles and runs under the pinned checker/runtime, and the negative type fixtures are rejected with the codes shown. No subset checker, teaching evaluator/compiler, or experimental mode has run; `tsc` acceptance is not subset acceptance. The native oracle is the pinned checker plus direct Node 24.21.0 execution; it is not a replacement for the two teaching evaluators or the independent reference expectations.

| Witness | Expected contract result | Status |
|---|---|---|
| Positive: §9 standalone evaluator kernel and recursive guest factorial | Subset parse/check succeeds; pinned `tsc` accepts; Node prints `120`; interpreted result equals native factorial. | Native-verified: pinned `tsc` exit 0, empty diagnostics; Node exit 0, stdout `120\n`; interpreted-vs-native assertion passed (`contract-native-results.json`). Subset parse/check and teaching-evaluator comparison UNRUN. |
| Positive: `const pair: { readonly tag: "pair"; readonly head: number; readonly tail: List } = ...` with a recursive `List = { readonly tag: "nil" } \| { readonly tag: "cons"; readonly head: number; readonly tail: List }` | Recursive tagged object union is accepted; exhaustive switch handles both cases. | UNRUN — not probed natively; subset acceptance is a Task-9 checker claim. |
| Negative host type: `const count: number = "five";` | Pinned TypeScript reports `TS2322`; no guest code runs. | Native-verified: `TS2322` reported, exit 1, not executed (`admission-native-results.json`). Subset-driver no-effect ordering UNRUN. |
| Negative host type: `const step = (x) => x + 1;` | Pinned TypeScript reports implicit-`any` diagnostic `TS7006`; no guest code runs. | Native-verified: `TS7006` reported, exit 1, not executed. Subset-driver no-effect ordering UNRUN. |
| Negative host-valid unsupported: `class OutsideCore { readonly n = 1; }` | Pinned TypeScript accepts it (host-valid); subset parser reports `UnsupportedSyntax(ClassDeclaration)` before effects. | Native-verified: pinned `tsc` exit 0, not executed. Subset `UnsupportedSyntax` rejection UNRUN. |
| Negative subset typing: `const condition: number = 1; if (condition) {}` | TypeScript's permissive truthiness is not the subset rule; subset checker reports `ExpectedBoolean`. | Native-verified: pinned `tsc` exit 0 (host-valid), not executed. Subset `ExpectedBoolean` rejection UNRUN. |
| Negative host primitive: `const value = eval("1 + 2");` | `ForbiddenHostPrimitive`; never passed to either teaching evaluator. | Native-verified: pinned `tsc` exit 0 (host-valid), not executed. Subset `ForbiddenHostPrimitive` rejection UNRUN. |

### Pinned commands

Run from `modern-sicp/typescript/` with the checked-in package manager and Node pin; save the §9 witness as `packages/ch4/src/host-evaluator-witness.ts`, which `packages/ch4/tsconfig.json` includes through `src/**/*.ts`. `noEmit: true` means the acceptance check emits nothing; acceptance always runs `tsc` with the pinned options unchanged, and running the checked `.ts` directly under the pinned Node 24.21.0 type stripping is the simplest run path. Emitting checked JavaScript to a throwaway output directory for probes or oracle comparisons is permitted and is neither a new toolchain nor new scope. Type stripping removes erasable annotations only; it does not validate types or expand the admitted grammar, which is why execution never replaces the subset gate and `tsc`.

```sh
# Show the exact checker selected by the pinned workspace dependency.
env PATH="$HOME/.local/share/mise/installs/node/24.21.0/bin:$PATH" pnpm exec tsc --version
# Expected: Version 7.0.2 (observed in local://modern-sicp-native-oracles.md and the parent probes).

# Check every chapter package under the checked-in base options.
env PATH="$HOME/.local/share/mise/installs/node/24.21.0/bin:$PATH" pnpm run typecheck
# package.json expands this to: pnpm -r --filter './packages/*' exec tsc --noEmit

# Focused check when the witness is saved as the named included source file.
env PATH="$HOME/.local/share/mise/installs/node/24.21.0/bin:$PATH" pnpm exec tsc --noEmit --project packages/ch4/tsconfig.json

# Native execution is separate and only follows both subset acceptance and tsc success.
env PATH="$HOME/.local/share/mise/installs/node/24.21.0/bin:$PATH" node packages/ch4/src/host-evaluator-witness.ts
```

For a one-file negative fixture under `packages/ch4/src/`, use the same pinned checker and project configuration, expect the stated diagnostic category/code, and do not run that rejected file with Node:

```sh
env PATH="$HOME/.local/share/mise/installs/node/24.21.0/bin:$PATH" pnpm exec tsc --noEmit --project packages/ch4/tsconfig.json --pretty false
```

The independent conformance protocol is:

1. Freeze each case's TypeScript source, guest input, expected value/output/effect trace, and provenance before running an engine. Native core expectations must be regenerated from checked host programs under the pinned native compiler/runtime and carry that native-execution provenance; the migrated evaluator's own output is never provenance. Experimental lazy/search expectations require an independent finite reference model or a reviewed source derivation.
2. Parse the same host source through both chapter parser entry paths and compare normalized typed ASTs or diagnostic categories/spans. Type-check accepted source with the exact pinned workspace command before any execution.
3. Execute accepted core input directly with pinned Node and with the teaching evaluator; also run the compiler/machine where the lesson uses it. Compare value, error category, and ordered externally visible effects. Compare stack counts/step traces only where the teaching objective specifies the same controller and instrumentation; do not require internal counts to match distinct implementations.
4. Rejected host-invalid or subset-invalid input must produce no guest output or mutation. Host-valid unsupported constructs must be rejected by the subset gate, not treated as native host features.
5. Run lazy/search cases only through their separately named experimental modes. Derive expected outcomes from an independent finite reference model and invariants; never treat direct JavaScript execution as an oracle for experimental semantics.

## 11. Lesson coverage map

| Lesson family | Admitted representation and execution path |
|---|---|
| 4.1 evaluator, syntax abstraction, environments, procedure application, analyzed evaluator | Tagged recursive expression/data unions; `switch` dispatch; lexical closure records; mutable environment cells; ordinary functions and typed result unions. Syntax is explicit typed data, not hidden in string conventions. |
| 4.2 lazy evaluator and exercises | `lazy-memoized-experiment` and `lazy-recompute-experiment` only; explicit delay/force extension and dedicated reference model (§6). |
| 4.3 nondeterministic evaluation and exercises | `amb-depth-first-experiment`, explicit choice/failure continuations and backtrackable-state policy; `ramb` is seeded and separately named (§6). |
| 4.4 query system | Typed query/rule/term constructors, record patterns, arrays/maps, lexical closures for processors, and typed query errors (§7–8). Query forms remain domain values, not host-language parser productions. |
| 5.1 controller descriptions | `Machine`, `Instruction`, `Source`, and operand tagged unions plus constructor functions; labels/instructions remain ordered arrays (§7). |
| 5.2 assembler, simulator, monitoring | Typed assembly passes, `Map`/`Set` tables, mutable register/stack state, ordered event arrays, exhaustive `MachineError` variants. |
| 5.3 memory and pair operations | Explicit numeric memory indices, mutable `number[]` vectors, tagged pair/empty-list words, checked allocation and selectors; numeric operations retain binary64 rules. |
| 5.4 explicit-control evaluator | Same typed evaluator syntax and value unions as 4.1; controller is machine data; frames, continuation state, stack, input, and transcript are explicit records/arrays. |
| 5.5 compiler and compiled/interpreted integration | Exhaustive source-union compiler emits typed instruction-sequence/controller data and runs through the 5.4 machine; no JavaScript source generation, reflection, or `eval` in the core compiler. |
| 5.50 self-evaluator | The metacircular evaluator is ordinary checked TypeScript subset source and runs as a guest program on the compiled 5.5 machine; compare its result and transcript with the native evaluator. The §9 kernel is a minimum typeable execution witness, not a claim to cover all 4.1 forms. |
| 5.51 C evaluator exercise | Translate the explicit-control machine/controller to the exercise's C representation; Node filesystem/process calls and `cc` belong only to the boundary harness. Compare the C process transcript with the same pinned input/reference cases. |
| 5.52 C compiler-backend exercise | The compiler emits C from typed controller data; exercise-specific C runtime/`cc` remain an external boundary. No generated C or compiler call is a guest evaluator primitive. |

The central admission invariant is one TypeScript grammar and one typed source contract across ch4 and ch5; chapter differences live in typed evaluators, query/machine domain data, and named experiment modes, not in parser-specific alternate languages.