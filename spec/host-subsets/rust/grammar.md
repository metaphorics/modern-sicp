# Rust host subset

Anchor: Tony Hoare, David Parnas

Status: implementation contract for the Rust edition. Native oracle witnesses are **VERIFIED** by the parent runs recorded in `local://modern-sicp-rust-contract-native-results.json` (see §10 for per-witness outcomes). All subset-checker, teaching-evaluator, and compiler witnesses remain **UNRUN**. This document defines the Rust 2024 guest language; it does not implement a parser, checker, interpreter, compiler, or book change.

## 1. Acceptance boundary

The edition targets `rustc +1.98.1`, edition 2024, from `rust/rust-toolchain.toml`. There are no added crates. A program is accepted for a teaching engine only when both conditions hold:

1. `rustc` type-checks the complete source in the pinned edition and target context; and
2. the edition's subset checker accepts every item, type, expression, statement, pattern, library path, and effect under this contract.

Native Rust validity alone is not subset acceptance. Conversely, an internal dynamic value representation MUST NOT let a source item bypass Rust's static checks. A source checker MUST reject the complete program before executing its `main` or any other effect when it finds unsupported syntax, an unsupported type, a Rust type/borrow error, or a violated subset rule. The same typed source representation and rules apply to direct evaluation, analyzed evaluation, explicit-control execution, and generated code.

This profile is deliberately smaller than Rust. Its positive witness is ordinary Rust source accepted by the pinned native compiler and is written without an evaluator primitive. The witness also defines a guest evaluator algorithm over typed expression data; an edition evaluator must parse and type-check that source and execute it as guest code, then compare its observable result with direct native execution. It is not enough to call the host evaluator, reflection, or a host `eval` function.

## 2. Lexical and item grammar

The grammar below is EBNF over Rust tokens. Rust lexical rules apply except for the explicit literal limits below. Braces, parentheses, brackets, commas, colons, semicolons, and operators are literal tokens. Ordinary line and nested block comments are ignored. Identifier spelling is `[A-Za-z_][A-Za-z0-9_]*`; raw identifiers are excluded.

```text
program       ::= use_item* item*
item          ::= derive_attr? struct_item
                | derive_attr? enum_item
                | type_alias
                | function_item
use_item      ::= "use" allowed_std_path ("as" IDENT)? ";"
allowed_std_path ::= "std::collections::HashMap"
derive_attr   ::= "#[derive(" derive_name ("," derive_name)* ","? ")]"
derive_name   ::= "Clone" | "Debug" | "PartialEq" | "Eq" | "Hash"
struct_item   ::= "struct" IDENT "{" field_decl ("," field_decl)* ","? "}"
                | "struct" IDENT "(" type_list? ")" ";"
enum_item     ::= "enum" IDENT "{" variant ("," variant)* ","? "}"
variant       ::= IDENT
                | IDENT "(" type_list? ")"
                | IDENT "{" field_decl ("," field_decl)* ","? "}"
field_decl    ::= IDENT ":" type
type_list     ::= type ("," type)*
type_alias    ::= "type" IDENT "=" type ";"
function_item ::= "fn" IDENT "(" parameter_list? ")" ("->" type)? block
parameter_list ::= parameter ("," parameter)* ","?
parameter     ::= "mut"? IDENT ":" type
integer_literal ::= DIGIT ("_"? DIGIT)* ("_"? ("i64" | "usize"))?
DIGIT         ::= [0-9]
```

Only imports from `std::collections::HashMap` are admitted. Unqualified prelude names listed below are also admitted. `use` aliases may rename only an admitted standard-library item. Items are private and live in one source module; top-level functions may call functions declared before or after them. `main` is the sole entry point and returns `()`.

Structs and enums are closed, nongeneric, and may be recursive only through `Box<T>`, `Vec<T>`, or another admitted indirection. Struct fields and enum payloads have the declared type. Tuple structs and unit variants are admitted. User-defined `impl`, trait, generic, module, union, extern, `const`, `static`, and `macro_rules!` items are not.

## 3. Types and values

The complete admitted type grammar is:

```text
type          ::= "()" | "bool" | "i64" | "usize" | "String" | "&str"
                | "&" type | "&mut" type
                | "Box<" type ">" | "Vec<" type ">"
                | "Option<" type ">" | "Result<" type "," type ">"
                | "HashMap<String, " type ">"
                | "[" type ";" integer_literal "]"
                | "(" type "," type ")"
                | IDENT
                | "fn(" type_list? ")" "->" type
                | "Box<dyn " closure_trait "(" type_list? ")" "->" type
                    "+ 'static>"
closure_trait ::= "Fn" | "FnMut" | "FnOnce"
```

`IDENT` in a type position MUST name a local type alias, struct, or enum. Tuple types have arity two; larger records use named structs. Array types `[T; N]` admit only the `integer_literal` length form. The only generic instantiations are the standard containers shown above, including nested combinations of those forms. User-defined generic types, type parameters, trait bounds, `impl Trait`, and trait objects other than the listed boxed closure forms are excluded. Named lifetime parameters are excluded; Rust's ordinary elision rules apply to admitted references, and `'static` is allowed only on the listed boxed closure object types.

Values are owned unless their type is a reference. `i64`, `usize`, `bool`, unit, shared references, and function pointers have Rust's built-in `Copy` behavior. `String`, `Vec`, `HashMap`, `Box`, user data, and mutable references are not implicitly copied. Explicit `.clone()` is admitted only when Rust's `Clone` implementation exists for the receiver. No runtime reference counter, handle, or evaluator value grants implicit copy, alias, or lifetime extension. `Rc`, `Arc`, `RefCell`, raw pointers, and unsafe code are excluded.

### Numeric and string rules

- Integer computation uses signed `i64`; `usize` is for collection lengths, indexes, and machine/arena identifiers. `i128`, unsigned arithmetic other than `usize`, floating point, exact fractions, complex values, and implicit numeric conversion are excluded.
- A decimal integer literal (optionally with `_` separators or an `i64`/`usize` suffix) MUST be inferred as `i64` or `usize` from an admitted context. A literal left to Rust's default `i32` inference is host-valid but outside the subset. The negative sign is unary negation, not part of the literal token.
- `i64` `+`, `-`, `*`, `/`, and `%` use Rust signed-integer behavior. `usize` admits `+` and `-` only, with checked-build overflow behavior; `usize` `*`, `/`, `%`, and mixed-type arithmetic are excluded. Oracle binaries MUST be built with overflow checks enabled. Addition, subtraction, multiplication, negation, or division overflow, and division or remainder by zero, trap; division truncates toward zero and remainder follows Rust's sign rule. No wrapping or saturating arithmetic is implicit. The accepted behavioral witnesses stay within range.
- Numeric comparisons are between values of the same admitted integer type and produce `bool`. `==` and `!=` are admitted only where the type implements Rust `PartialEq`; ordering operators are admitted only for `i64`, `usize`, and `String`. `&&` and `||` short-circuit and require `bool`. Conditions never use truthiness.
- String literals are UTF-8 `&str` values with ordinary Rust escapes. Owned strings are constructed with `String::from`, and updated with the admitted `push`/`push_str` methods. String concatenation by `+`, byte-string/C-string/raw-string literals, and implicit conversion between `String` and `&str` are excluded.

### Static typing and control flow

Rust's pinned native type checker is authoritative for all admitted Rust type, move, borrow, lifetime-elision, closure-trait, method-resolution, and format-string rules. The subset checker additionally enforces the boundary above and MUST produce a typed representation before any teaching engine executes a program.

The subset's explicit typing obligations are:

- `let name [: T] = e;` binds an immutable local of `e`'s inferred type (or `T` after Rust checks equality). `let mut` is required for later assignment or mutable borrowing. Shadowing creates a new binding; assignment does not.
- Function parameters have explicit types. Function return types are explicit when non-unit; an omitted return type means `()`. A `return e` type-checks against the enclosing return type. Recursive and mutually recursive top-level functions are permitted.
- `if c { a } else { b }` requires `c: bool` and compatible branch types; an `if` used as a value requires `else`. `if let` and `while let` match only admitted patterns; `if let` used as a value requires `else`. `match` must be exhaustive and all arm values must have one compatible type. Every `while` condition is `bool`; `while let` tests a pattern against its scrutinee expression. Conditions never use truthiness.
- `+`, `-`, `*`, `/`, `%` require two operands of one admitted integer type and produce that type: `i64` admits all five; `usize` admits `+` and `-` only, for program-counter, heap-index, and length progression under checked-build overflow behavior. Integer comparisons produce `bool`. Assignment requires a mutable place of the assigned type. `+=` and `-=` are admitted for mutable `i64` and `usize` places and use the same operator sets.
- Function and closure arguments are checked against their parameter types and arity. Closure results obey the same block/result rules as functions. `?` is admitted only in a function returning `Result<T, E>` or `Option<T>` and only where Rust's ordinary `From`/residual rules type-check without an unlisted trait implementation.
- `Vec<T>`, `HashMap<String, T>`, `Option<T>`, `Result<T, E>`, `Box<T>`, references, and local structs/enums retain their Rust types. Indexing requires `usize`; indexing a vector/array produces the element place and follows Rust bounds-panic behavior.

## 4. Expressions, statements, patterns, and standard operations

The admitted expression forms are: literals; local and function names; admitted enum-variant paths; parenthesized expressions; tuple and array literals; vector literals; named/tuple struct literals with optional field-init shorthand; field read; index; function and closure calls; admitted method/associated-function calls; unary `-`, `!`, `*`, `&`, and `&mut`; the arithmetic, comparison, and short-circuit Boolean operators above; assignment to a mutable place; `if`; `if let`; `match`; blocks; `loop`; `while`; `while let`; `for`; and the postfix `?` operator. Ranges are admitted only as half-open `start..end` iterables. Calls and nested operands evaluate left to right, as in Rust.

The complete production grammar for statements, expressions, patterns, closures, and loops follows. The expression layers are ordered loosest to tightest, and that layering is the precedence. `type` and `integer_literal` are defined above; `string_literal` is the Rust double-quoted token restricted by the string rules above.

```text
statement       ::= let_stmt | block_like | expression ";"
let_stmt        ::= "let" "mut"? let_pattern (":" type)? "=" expression ";"
let_pattern     ::= IDENT | "(" IDENT "," IDENT ")"
block_like      ::= if_expr | if_let_expr | match_expr
                  | while_expr | while_let_expr | for_expr | loop_expr | block
block           ::= "{" statement* expression? "}"

expression      ::= assignment
assignment      ::= range (("=" | "+=" | "-=") assignment)?
range           ::= logical_or (".." logical_or)?
logical_or      ::= logical_and ("||" logical_and)*
logical_and     ::= equality ("&&" equality)*
equality        ::= comparison (("==" | "!=") comparison)?
comparison      ::= additive (("<" | "<=" | ">" | ">=") additive)*
additive        ::= multiplicative (("+" | "-") multiplicative)*
multiplicative  ::= unary (("*" | "/" | "%") unary)*
unary           ::= ("-" | "!" | "*" | "&" "mut"?) unary | postfix
postfix         ::= primary suffix*
suffix          ::= "." IDENT ("(" arg_list? ")")?
                  | "(" arg_list? ")" | "[" expression "]" | "?"
arg_list        ::= expression ("," expression)* ","?

primary         ::= literal | path | "()" | "(" expression ("," expression)? ")"
                  | "[" (expression ("," expression)* ","?)? "]"
                  | macro_call | struct_literal | closure
                  | if_expr | if_let_expr | match_expr | block
                  | while_expr | while_let_expr | for_expr | loop_expr
                  | "return" expression? | "break" expression? | "continue"
literal         ::= integer_literal | string_literal | "true" | "false"
path            ::= IDENT ("::" IDENT)*
struct_literal  ::= path "{" field_init ("," field_init)* ","? "}"
field_init      ::= IDENT (":" expression)?
macro_call      ::= "vec" "!" "[" (expression ("," expression)* ","?
                                   | expression ";" expression) "]"
                  | ("format" | "print" | "println") "!"
                    "(" string_literal ("," expression)* ","? ")"
closure         ::= "move"? ("||" | "|" (closure_param ("," closure_param)* ","?)? "|")
                    (block | expression)
closure_param   ::= IDENT (":" type)?
if_expr         ::= "if" expression block ("else" (if_expr | block))?
if_let_expr     ::= "if" "let" pattern "=" expression block
                    ("else" (if_expr | if_let_expr | block))?
match_expr      ::= "match" expression "{" match_arm* "}"
match_arm       ::= pattern "=>" expression ","?
while_expr      ::= "while" expression block
while_let_expr  ::= "while" "let" pattern "=" expression block
for_expr        ::= "for" pattern "in" expression block
loop_expr       ::= "loop" block

pattern         ::= "_" | IDENT | integer_literal | "true" | "false"
                  | "(" pattern "," pattern ")"
                  | path ("(" pattern ("," pattern)* ","? ")")?
                  | path "{" field_pattern ("," field_pattern)* ","? "}"
field_pattern   ::= IDENT (":" pattern)?
```

Grammar notes: struct literals are excluded from `if`/`while`/`match`/`for` scrutinee expressions, as in Rust. A bare `IDENT` pattern binds a fresh name unless it resolves to a unit struct or unit enum variant. `break expression?` admits value-typed `loop` exit; `continue` takes no value. A `match_arm` body that is a `block` needs no trailing comma.

A block contains zero or more statements and an optional final expression. Statements are `let`/`let mut` bindings, expression statements, `return`, `break`, `continue`, `while`, `while let`, `for`, and `loop`. `for` accepts a range, an array or `Vec` iterated by value, by `&`, or by `&mut` through Rust's implicit `IntoIterator`, or an iterator produced by admitted iterator operations (`.iter()`, `.iter_mut()`, `.into_iter()`, possibly chained through `next`, `enumerate`, `zip`). `break` and `continue` target the innermost loop. `loop` must exit by `break` or a diverging path for a value-typed use.

Patterns are `_`, a binding identifier, admitted integer/Boolean literals, tuples, named or tuple struct patterns with optional field shorthand bindings, and enum variants with recursively admitted patterns. Match ergonomics for matching references follow the pinned compiler. `ref`, `ref mut`, match guards, range patterns, `@` patterns, and pattern alternatives are excluded. Patterns in `let` are limited to identifiers and two-element tuples.

The only admitted macros are:

- `vec![e, ...]` and `vec![value; count]`, with the latter requiring `Clone`;
- `format!("...", args...)`, `print!("...", args...)`, and `println!("...", args...)`, using Rust's compile-time checked format strings; and
- the listed `#[derive(...)]` names.

No other macro invocation is admitted. The standard-library operations available by path or method are limited to constructors and operations needed by the grammar: `String::from`; `Vec::new`, `with_capacity`, `push`, `pop`, `len`, `is_empty`, `get`, `get_mut`, `iter`, `iter_mut`, `into_iter`; `HashMap::new`, `insert`, `get`, `get_mut`, `contains_key`, `remove`, `len`, `is_empty`, `iter`; `Box::new`, `as_ref`, `as_mut`; `Option`/`Result` constructors and pattern matching; and iterator `next`, `enumerate`, and `zip`. `clone`, `as_str`, `push`, and `push_str` are admitted when the receiver type provides them. No method is admitted merely because it happens to resolve on a host type; the checker uses this closed allowlist.

## 5. Ownership, borrowing, mutation, and closures

The subset uses Rust ownership rather than a parallel dynamic ownership model.

- Passing or binding a non-`Copy` value by value moves it. The moved place cannot be used again unless reinitialized. An explicit clone is a separate operation.
- `&T` is a shared borrow; multiple shared borrows may coexist. `&mut T` is an exclusive borrow; while it is live, no conflicting borrow or use of the borrowed place is admitted. Borrow lifetimes and non-lexical-lifetime regions are exactly those checked by `rustc +1.98.1`; the teaching checker MUST reject the same invalid source before effects.
- Mutable local bindings and mutable struct/collection places require `mut` or `&mut`. A field assignment cannot mutate through an immutable binding or shared reference. `Vec` and `HashMap` mutation uses their ordinary `&mut self` methods; no hidden aliasing is granted.
- Recursive records require `Box` or an admitted collection indirection and are still uniquely owned unless an explicit shared borrow is present. Cycles of owning values are not constructible through this profile. The evaluator-kernel witness uses integer arena indexes in its own guest data, not host pointers or reference-counted environment cells.
- A closure is `|params| expr`, `|params| { block }`, `move |params| expr`, or `move |params| { block }`. Parameter types may be inferred only from a checked context; otherwise they are explicit (`|x: i64| ...`). A non-`move` closure captures by the borrow/copy mode Rust infers from its use. A `move` closure moves each non-`Copy` capture; it does not clone it or make it `'static` by itself. `Fn`, `FnMut`, or `FnOnce` is determined by Rust's capture/use rules and controls whether a call borrows shared, mutably, or consumes the closure.
- Inferred closure values may be bound locally and passed where Rust infers their concrete type. A closure may escape through `Box<dyn Fn(...) -> R + 'static>`, `Box<dyn FnMut(...) -> R + 'static>`, or `Box<dyn FnOnce(...) -> R + 'static>` only when its captures satisfy that object lifetime; the standard unsize coercion from `Box<concrete closure>` to one of these object types at a return, assignment, or argument site is admitted. `FnMut` calls require a mutable place; `FnOnce` consumes its value. A closure cannot name itself recursively; use a named function for recursion or an explicitly represented environment algorithm such as the witness below.

## 6. Exclusions and required rejection classes

The following are outside this profile even when they are valid Rust: macros other than the whitelist; imports other than the admitted `HashMap`; modules and visibility; generic declarations and arbitrary trait use; `impl` blocks; user-defined traits; `unsafe`, raw pointers, pointer casts, unions, FFI, and inline assembly; `Rc`, `Arc`, `RefCell`, interior-mutability and synchronization types; `async`/`await`, generators, `dyn` trait objects other than boxed admitted closure types; `String + &str`; floating point and wider integer types; `loop` labels; `for` desugaring over unlisted iterators; `const`/`static`; `std::fs`, process, network, environment, clock, threads, and nondeterministic host services; and custom `Display`/`Debug` implementations.

The checker and its conformance report MUST distinguish:

1. **Invalid Rust syntax**: rejected by the pinned parser/compiler; no evaluator is entered.
2. **Invalid Rust typing or ownership**: including unresolved/incompatible types, invalid method/type use, immutable assignment, overlapping mutable borrows, dangling references, or invalid closure capture; reject before effects.
3. **Host-valid but excluded syntax/type/effect**: report `Unsupported` with the source location, not a fabricated Rust type error and not a runtime failure.
4. **Subset semantic/runtime trap**: a checked program reaches an out-of-range index, checked-build integer overflow, or division/remainder by zero; execution stops at the trap. An explicit returned `Err`/`None` is an ordinary observable result, not a trap; programs map it to their own output (as the witness does) or propagate it with `?`.
5. **Observable stdout**: successful writes from `print!`/`println!` in evaluation order. `println!` appends one line feed. Exact stdout bytes and exit status are observable; panic text and compiler diagnostic wording are not stable contracts.

`HashMap` iteration order is unspecified and MUST NOT determine answer order or expected output. When order is instructional, the program carries a `Vec` order explicitly. The core subset has no implicit laziness, search/backtracking, random choice, or query syntax.

## 7. Explicit data languages and named experiments

Queries and machine programs are values in Rust, not additional Rust syntax and not text parsed as source. Each engine consumes typed constructors and returns typed values or an explicit `Result`.

```rust
#[derive(Clone, PartialEq, Eq, Hash)]
enum Term {
    Variable(String),
    Integer(i64),
    Text(String),
    Atom(String),
    Pair(Box<Term>, Box<Term>),
    Empty,
}

#[derive(Clone)]
enum Query {
    Unify(Term, Term),
    Relation { name: String, arguments: Vec<Term> },
    And(Vec<Query>),
    Or(Vec<Query>),
    Not(Box<Query>),
    Unique(Box<Query>),
}

type Substitution = HashMap<String, Term>;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Register(String);
#[derive(Clone, PartialEq, Eq, Hash)]
struct Label(String);

#[derive(Clone)]
enum Operand {
    Constant(i64),
    Register(Register),
    Label(Label),
}
#[derive(Clone)]
enum Instruction {
    Assign { target: Register, value: Operand },
    Test { predicate: String, arguments: Vec<Operand> },
    Branch(Label),
    Goto(Operand),
    Save(Register),
    Restore(Register),
    Perform { operation: String, arguments: Vec<Operand> },
}
struct MachineProgram {
    registers: Vec<Register>,
    instructions: Vec<(Option<Label>, Instruction)>,
}
```

`Query` semantics are explicit unification over `Term`, substitution extension with occurs-check policy stated by each query-engine exercise, and ordered collection of answer frames using `Vec`; a `HashMap` is only an index and its iteration order is never the answer order. `Instruction` semantics are defined by the machine runner: `Assign`, `Test`, `Branch`, `Goto`, `Save`, `Restore`, and `Perform` update a typed register file, program counter, and explicit stack. An unbound label/register, invalid restore, or undefined operation is a typed machine error, never an invented source-language feature. Machine programs are built with these constructors; no controller-text parser is claimed by the source grammar.

### Optional, named experiments

- **`lazy-recompute/1`** and **`lazy-memo/1`** are separate evaluators over explicit `Thunk`/`Force` AST data. The core Rust evaluator remains strict. In both experiments a delayed operand is evaluated only when forced; `lazy-recompute/1` reevaluates each force, while `lazy-memo/1` stores and returns the first successful result and its effects occur once. A failed force stays delayed. No Rust closure call, iterator, or ordinary function call becomes lazy implicitly.
- **`search-depth-first/1`** is a separate engine over explicit `Choose(Vec<T>)`, `Fail`, and `Success(T)` search data. It explores alternatives depth-first in the vector's written order, records mutations in an explicit trail, and rolls back only trailed assignments when backtracking. It is deterministic for a fixed program and seed. `?`, `Result`, Rust panics, and ordinary `return` do not trigger search backtracking.

These modes are selected by an explicit runner or typed mode value; they are not accepted as native Rust operators or general-purpose collection behaviors. Each experiment requires a finite reference model and dedicated behavioral oracle separate from the core host oracle.

## 8. Evaluator-kernel source witness

This ordinary Rust program is the minimum guest-evaluator witness. It uses only grammar forms above: closed recursive data, match, functions, lexical environment frames, a typed `Result`, and explicit closure capture. `Expr` and `Program` are typed data; `evaluate` and `apply` are guest-written algorithms. The `Value::Closure` environment is an index into an owned vector arena, so closure capture and recursive function calls do not use `Rc`, host reflection, or a hidden evaluator. `main` prints `120`, `42`, `42`, `1`, `2`, each on its own line. **Native run VERIFIED:** compile exit 0 and stdout exactly `120\n42\n42\n1\n2\n` (`local://modern-sicp-rust-contract-native-results.json`, `positive`); the compiler emits one non-blocking `dead_code` warning for the unconstructed `Expr::Boolean` variant. Guest-evaluator execution of this source remains **UNRUN**.

```rust
use std::collections::HashMap;

#[derive(Clone)]
enum Expr {
    Integer(i64),
    Boolean(bool),
    Variable(String),
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Equal(Box<Expr>, Box<Expr>),
    If { test: Box<Expr>, yes: Box<Expr>, no: Box<Expr> },
    Lambda { parameters: Vec<String>, body: Box<Expr> },
    Call { function: Box<Expr>, arguments: Vec<Expr> },
    Let { name: String, value: Box<Expr>, body: Box<Expr> },
}

#[derive(Clone)]
enum Value {
    Integer(i64),
    Boolean(bool),
    Function(usize),
    Closure { parameters: Vec<String>, body: Box<Expr>, environment: usize },
}

struct FunctionDef { name: String, parameters: Vec<String>, body: Expr }
struct Program { functions: Vec<FunctionDef>, factorial: Expr, captured: Expr }
struct Frame { bindings: HashMap<String, Value>, parent: Option<usize> }
struct Store { frames: Vec<Frame> }
enum Fault { Unbound, Type, Arity, NotCallable }

fn boxed(expression: Expr) -> Box<Expr> { Box::new(expression) }
fn integer(value: i64) -> Expr { Expr::Integer(value) }
fn variable(name: &str) -> Expr { Expr::Variable(String::from(name)) }
fn one(expression: Expr) -> Vec<Expr> { vec![expression] }

fn factorial_body() -> Expr {
    Expr::If {
        test: boxed(Expr::Equal(boxed(variable("n")), boxed(integer(0)))),
        yes: boxed(integer(1)),
        no: boxed(Expr::Multiply(
            boxed(variable("n")),
            boxed(Expr::Call {
                function: boxed(variable("factorial")),
                arguments: one(Expr::Subtract(boxed(variable("n")), boxed(integer(1)))),
            }),
        )),
    }
}

fn make_program() -> Program {
    let factorial = Expr::Call {
        function: boxed(variable("factorial")),
        arguments: one(integer(5)),
    };
    let captured = Expr::Let {
        name: String::from("offset"),
        value: boxed(integer(3)),
        body: boxed(Expr::Call {
            function: boxed(Expr::Lambda {
                parameters: vec![String::from("x")],
                body: boxed(Expr::Add(boxed(variable("offset")), boxed(variable("x")))),
            }),
            arguments: one(integer(39)),
        }),
    };
    Program {
        functions: vec![FunctionDef {
            name: String::from("factorial"),
            parameters: vec![String::from("n")],
            body: factorial_body(),
        }],
        factorial,
        captured,
    }
}

fn new_frame(store: &mut Store, parent: Option<usize>) -> usize {
    let index = store.frames.len();
    store.frames.push(Frame { bindings: HashMap::new(), parent });
    index
}

fn lookup(store: &Store, mut frame: Option<usize>, name: &str) -> Option<Value> {
    while let Some(index) = frame {
        let current = &store.frames[index];
        if let Some(value) = current.bindings.get(name) {
            return Some(value.clone());
        }
        frame = current.parent;
    }
    None
}

fn evaluate(
    expression: &Expr,
    environment: usize,
    global: usize,
    program: &Program,
    store: &mut Store,
) -> Result<Value, Fault> {
    match expression {
        Expr::Integer(value) => Ok(Value::Integer(*value)),
        Expr::Boolean(value) => Ok(Value::Boolean(*value)),
        Expr::Variable(name) => match lookup(store, Some(environment), name) {
            Some(value) => Ok(value),
            None => Err(Fault::Unbound),
        },
        Expr::Add(left, right) => {
            let first = evaluate(left, environment, global, program, store)?;
            let second = evaluate(right, environment, global, program, store)?;
            match (first, second) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
                _ => Err(Fault::Type),
            }
        }
        Expr::Subtract(left, right) => {
            let first = evaluate(left, environment, global, program, store)?;
            let second = evaluate(right, environment, global, program, store)?;
            match (first, second) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a - b)),
                _ => Err(Fault::Type),
            }
        }
        Expr::Multiply(left, right) => {
            let first = evaluate(left, environment, global, program, store)?;
            let second = evaluate(right, environment, global, program, store)?;
            match (first, second) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a * b)),
                _ => Err(Fault::Type),
            }
        }
        Expr::Equal(left, right) => {
            let first = evaluate(left, environment, global, program, store)?;
            let second = evaluate(right, environment, global, program, store)?;
            match (first, second) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a == b)),
                (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a == b)),
                _ => Err(Fault::Type),
            }
        }
        Expr::If { test, yes, no } => match evaluate(test, environment, global, program, store)? {
            Value::Boolean(true) => evaluate(yes, environment, global, program, store),
            Value::Boolean(false) => evaluate(no, environment, global, program, store),
            _ => Err(Fault::Type),
        },
        Expr::Lambda { parameters, body } => Ok(Value::Closure {
            parameters: parameters.clone(),
            body: body.clone(),
            environment,
        }),
        Expr::Call { function, arguments } => {
            let callable = evaluate(function, environment, global, program, store)?;
            let mut values = Vec::new();
            for argument in arguments.iter() {
                values.push(evaluate(argument, environment, global, program, store)?);
            }
            apply(callable, values, global, program, store)
        }
        Expr::Let { name, value, body } => {
            let initial = evaluate(value, environment, global, program, store)?;
            let child = new_frame(store, Some(environment));
            store.frames[child].bindings.insert(name.clone(), initial);
            evaluate(body, child, global, program, store)
        }
    }
}

fn apply(
    callable: Value,
    arguments: Vec<Value>,
    global: usize,
    program: &Program,
    store: &mut Store,
) -> Result<Value, Fault> {
    match callable {
        Value::Function(index) => {
            let definition = match program.functions.get(index) {
                Some(definition) => definition,
                None => return Err(Fault::Unbound),
            };
            if definition.parameters.len() != arguments.len() { return Err(Fault::Arity); }
            let child = new_frame(store, Some(global));
            for (name, value) in definition.parameters.iter().zip(arguments.into_iter()) {
                store.frames[child].bindings.insert(name.clone(), value);
            }
            evaluate(&definition.body, child, global, program, store)
        }
        Value::Closure { parameters, body, environment } => {
            if parameters.len() != arguments.len() { return Err(Fault::Arity); }
            let child = new_frame(store, Some(environment));
            for (name, value) in parameters.into_iter().zip(arguments.into_iter()) {
                store.frames[child].bindings.insert(name, value);
            }
            evaluate(&body, child, global, program, store)
        }
        _ => Err(Fault::NotCallable),
    }
}

fn execute_program(program: &Program) -> Result<(Value, Value), Fault> {
    let mut store = Store { frames: Vec::new() };
    let global = new_frame(&mut store, None);
    for (index, definition) in program.functions.iter().enumerate() {
        store.frames[global].bindings.insert(definition.name.clone(), Value::Function(index));
    }
    let factorial = evaluate(&program.factorial, global, global, program, &mut store)?;
    let captured = evaluate(&program.captured, global, global, program, &mut store)?;
    Ok((factorial, captured))
}

fn make_adder(addend: i64) -> Box<dyn Fn(i64) -> i64 + 'static> {
    Box::new(move |value: i64| value + addend)
}

fn make_counter() -> Box<dyn FnMut() -> i64 + 'static> {
    let mut value = 0;
    Box::new(move || {
        value += 1;
        value
    })
}

fn main() {
    match execute_program(&make_program()) {
        Ok((Value::Integer(factorial), Value::Integer(captured))) => {
            println!("{}", factorial);
            println!("{}", captured);
        }
        _ => println!("evaluator error"),
    }
    let add = make_adder(2);
    println!("{}", add(40));
    let mut counter = make_counter();
    println!("{}", counter());
    println!("{}", counter());
}
```

This is an implementability witness, not permission to special-case this input. The future teaching evaluator MUST parse and type-check the witness as Rust source. It MUST execute the guest `main`, including its `evaluate`/`apply` algorithm and the constructed factorial and captured-closure programs, without routing to the host's production evaluator or any other native evaluation entry point. The checked guest run and native run MUST both produce the five lines above; the output comparison is byte-for-byte. A later full self-evaluator test MUST also execute a translated evaluator source containing every additional construct that the edition claims to support. Unsupported source constructs remain rejected even if the host compiler accepts them.

## 9. Lesson coverage map

| Lesson family | Admitted constructs used | Contract boundary |
|---|---|---|
| 4.1 evaluator, analyzer, lexical environments, procedure application, recursion, quotation/symbolic structure, assignment, internal definitions | closed `enum`/`struct` AST and value types; `match`; functions; `if`; blocks; `Vec`/`HashMap`; explicit `Box`; closures with checked capture; mutable places and `&mut`; `Result` | Syntax trees are explicit typed values. An analyzer emits a typed plan/enum. No source-string quotation or automatic truthiness. |
| 4.2 lazy evaluation | explicit `Thunk`/`Force` AST values, enums, `Box`, mutable owner and `Result` | Only `lazy-recompute/1` and `lazy-memo/1`; core calls remain strict. |
| 4.3 nondeterministic evaluation and search | typed `Choose`/`Fail`/`Success` values; `Vec` alternatives; explicit trail and stack; loops and match | Only `search-depth-first/1`, with deterministic vector order and rollback; no implicit nondeterministic host behavior. |
| 4.4 query system, unification, rule application, ordered answers | `Term`, `Query`, `Substitution`, closed enum matches, `HashMap<String, Term>`, ordered `Vec` | Query operators are data constructors, not host syntax. Answer order is explicit and independent of map iteration. |
| 5.1 register-machine design | `Register`, `Label`, `Operand`, and `Instruction` values; `Vec`-backed stack | Instructions and controller labels are typed values, not parsed source text. |
| 5.2 assembler and simulator | instruction/label vectors, `HashMap`, `match`, loops, `Result`, mutable machine state | Assembly validation and step execution return typed faults; stack save/restore is explicit. |
| 5.3 memory and garbage collection | `Word` enum, `Vec`-backed heap, `usize` indexes, mutable fields, loops, work lists | Heap addresses are checked indexes; no raw pointers, unsafe, or hidden garbage collector is claimed. |
| 5.4 explicit-control evaluator | typed evaluator control states, registers, `Vec` stack, loops, match, typed object AST | The explicit-control machine is a guest data structure and transition function, not a built-in evaluator facility. |
| 5.5 compiler, compiled procedures, compiler exercises 5.45–5.52 | typed instruction/sequence/label values, enums, functions, match, `String`/`format!` for emitted text, typed machine runner | Compile output is data, and correctness is compared by observable result/effect order. Exercise 5.50 runs the guest evaluator through the compiler; 5.51 keeps its C translation as a separate C target built by the external harness; 5.52 emits C from the typed compiler representation. The subset does not claim C syntax or process execution as Rust features. |

## 10. Native oracle commands and source witnesses

Save the positive source block in §8 as `/tmp/rust-host-subset-positive.rs`. Save the host-valid-but-excluded block below as `/tmp/rust-host-subset-unsupported.rs`, and the host-invalid block as `/tmp/rust-host-subset-borrow-error.rs`. Run from `modern-sicp/rust` with the pinned toolchain:

```sh
rustc +1.98.1 --edition=2024 -C overflow-checks=on --emit=metadata \
  /tmp/rust-host-subset-positive.rs -o /tmp/rust-host-subset-positive.rmeta
rustc +1.98.1 --edition=2024 -C overflow-checks=on \
  /tmp/rust-host-subset-positive.rs -o /tmp/rust-host-subset-positive
/tmp/rust-host-subset-positive
```

Expected stdout is exactly (VERIFIED: parent run, `local://modern-sicp-rust-contract-native-results.json`, `positive`: compile exit 0, runtime exit 0, stdout `120\n42\n42\n1\n2\n`, plus the non-blocking `dead_code` warning on `Expr::Boolean`):

```text
120
42
42
1
2
```

The following source is valid Rust but excluded because it uses a raw pointer and an `unsafe` block. Native compilation and execution succeed and print `4` (VERIFIED: parent run, `unsupported`: compile exit 0, runtime exit 0, stdout `4\n`); the subset parser/checker MUST reject it as `Unsupported` before any guest effect (UNRUN: no checker exists yet). This distinction prevents native validity from being confused with subset acceptance.

```rust
fn main() {
    let mut value = 3_i64;
    let raw = &mut value as *mut i64;
    unsafe { *raw += 1; }
    println!("{}", value);
}
```

```sh
rustc +1.98.1 --edition=2024 -C overflow-checks=on \
  /tmp/rust-host-subset-unsupported.rs -o /tmp/rust-host-subset-unsupported
/tmp/rust-host-subset-unsupported
```

The following is Rust syntax in the subset, but it violates Rust's exclusive-borrow rule and MUST fail native type checking before execution. The source checker must classify it as an ownership/type rejection, not let a dynamic evaluator run it.

```rust
fn main() {
    let mut value = 1_i64;
    let first = &mut value;
    let second = &mut value;
    *first += 1;
    *second += 1;
    println!("{}", value);
}
```

```sh
rustc +1.98.1 --edition=2024 -C overflow-checks=on --emit=metadata \
  /tmp/rust-host-subset-borrow-error.rs -o /tmp/rust-host-subset-borrow-error.rmeta
```

Observed: nonzero compiler status (VERIFIED: parent run, `borrow-error`: compile exit 1, `E0499`). Do not compare unstable diagnostic wording. The host-valid unsupported case is not expected to fail `rustc`; its rejection is a teaching-checker assertion. At this phase no subset-checker executable exists, so do not invent or claim a checker command. The edition's post-implementation gate must add the corresponding subset-accept/reject and interpreted-versus-native comparisons (all UNRUN).

The edition gates are run from `modern-sicp/rust` after cutover, not by this contract author:

```sh
cargo +1.98.1 fmt --all -- --check
cargo +1.98.1 clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo +1.98.1 nextest run --workspace --locked
cargo +1.98.1 test --doc --workspace --locked
```

When the local environment requires the documented gate setup, unset `CARGO_BUILD_BUILD_DIR` and set `RUSTC_WRAPPER=`. The three native oracle runs are VERIFIED as recorded above and in `local://modern-sicp-rust-contract-native-results.json`; all subset-checker assertions, guest-evaluator executions, interpreted-versus-native comparisons, and compiler-conformance witnesses remain **UNRUN**.
