# Rust edition input sections

Toolchain pins (grounded this session): Rust 1.98.1 stable, released 2026-09-03 (https://blog.rust-lang.org/2026/09/03/Rust-1.98.1, https://releases.rs/), edition 2024 (https://doc.rust-lang.org/edition-guide/); thiserror 2.0.20 (https://docs.rs/thiserror/2.0.20/thiserror/); anyhow 1.0.104 (https://docs.rs/anyhow/1.0.104/anyhow/); proptest 1.11.0 (https://docs.rs/proptest/1.11.0/proptest/); insta 1.48.0 (https://docs.rs/insta/1.48.0/insta/); cargo-nextest (https://nexte.st/). Std APIs are cited inline where first used. Every API name below appears in a page read this session; anything else is marked `unverified`.

Grounding note on the source: `sicp-pocket.texi` prints interpreter results as `@i{...}` lines after expressions, for example `(W1 50)` then `@i{50}` at 14269-14272, `@i{sum 0  New-value = 0}` at 17910-17911, `@i{ok}` at 18238-18239. A file-wide grep found no `;Value` and no `@result{}` anywhere. The interaction convention in part 3 is therefore a stated edition choice, not a verbatim fact about the texinfo.

## Part 0. Chapter 0 primer (about 30 pages)

The reader knows how to program but not Rust. Chapter 0 teaches only the subset the book uses. Each later chapter assumes it.

| Section | Teaches | Notes |
|---|---|---|
| 0.1 Values, numbers, bindings | `i128`, `f64`, `bool`, `char`; `let` and `let mut`; shadowing versus assignment; checked arithmetic | Sets up the number policy in part 3; `+`/`*` on `i128` wrap in release, so the runtime uses `checked_mul` |
| 0.2 Functions, recursion, iteration | `fn`, `if` as an expression, `loop`/`while`/`for`; recursion depth and the call stack; rewriting tail recursion as a loop | Rust has no tail-call optimization; this is the recurring difference in 1.2 |
| 0.3 Ownership, moves, borrowing | move semantics, `&T`, `&mut T`, borrow rules; why Rust needs this where Scheme has a garbage collector | The re-cut 3.2 builds directly on this section |
| 0.4 Product and sum data | `struct`, `enum`, `match`, `Box`, `Option`, `Result` | Expression-oriented algebraic data replaces quoted lists |
| 0.5 Collections and iterators | `Vec`, slices `&[T]`, `HashMap`, `VecDeque`; `Iterator` adapters `map`/`filter`/`fold`; laziness of adapters (https://doc.rust-lang.org/std/iter/index.html, https://doc.rust-lang.org/std/collections/index.html) | The book's 2.2.3 sequence operations land here |
| 0.6 Closures and trait objects | `Fn`/`FnMut`/`FnOnce`, capture by reference and by `move`, returning `impl Fn`, `dyn Trait` for heterogeneous collections | The book's `lambda`; style contract puts `dyn Trait` where the book uses first-class procedures |
| 0.7 Shared ownership and interior mutability | `Rc`, `Weak` and cycles (https://doc.rust-lang.org/std/rc/index.html), `Cell`, `RefCell` with `borrow`/`borrow_mut` and dynamic borrow panics (https://doc.rust-lang.org/std/cell/struct.RefCell.html) | The `set!` analog for host chapters 2-3; guest chapters 4-5 use arena indexes and explicit experiment data instead (grammar §§5-8) |
| 0.8 Errors and tests | `Result<T, E>`, `?`, deriving errors with thiserror enums, `#[test]`, `assert_eq!`, one-page proptest and insta tour, `cargo nextest run` | Conventions of part 3 introduced once |
| 0.9 Modules, crates, workspace | the book's `rust/` workspace, how to run an example, where exercises live, doc comments, reading borrow-checker errors | Maps part 5's layout |

Chapter 0 exercises (numbered 0.1 onward):

- 0.1 Compute `factorial` recursively with `checked_mul` returning `Result`; report the largest `n` whose factorial fits `i128`. Introduces the overflow policy used throughout.
- 0.2 Write `count_change` (1.2.2 style) recursively, then as a loop with an accumulator; compare maximum recursion depth against iteration on amounts up to 400.
- 0.3 Predict-then-compile: five short snippets mixing moves, borrows, and mutation; state for each whether it compiles and why before checking. The book's pencil-exercise style applied to borrow rules.
- 0.4 Write `make_accumulator`: a function returning two closures that must share one counter; force the sharing with `Rc<Cell<i128>>` and explain why a plain captured `i128` fails.
- 0.5 Implement `sum_cubes(a, b)` twice: a recursive function and an iterator chain over a range; then make `div(a, b)` return `Result` and propagate with `?` from both versions.

## Part 1. Concept map

| SICP concept | Construct in Rust | Notes |
|---|---|---|
| `define` | `let`, `fn`, nested `fn` | internal names are nested items |
| `lambda` | closure, `impl Fn` return | first-class from 1.3 on |
| `let` | `let` binding | same eager semantics |
| `cond`/`if`, special forms | `if`/`match` expressions | syntax, not procedures; 1.6 lesson survives |
| quotation, symbols | `Symbol` (`Rc<str>`), constructors | no reader macros; display via `Display` |
| `cons`/`car`/`cdr` pairs | `List<T>` enum, then `ConsCell` under `Rc` | layered, see sketches |
| lists | `Vec<T>`, `List<T>` | `Vec` for typed, `List` where sharing diagrams matter |
| trees | recursive `enum` + `Box` | 2.2.2, 2.3.4 |
| tagged data | host `Key`-tagged dispatch rows | host-ordinary 2.4.2; guest uses `Term` constructors (grammar §7) |
| `put`/`get` dispatch | host `OpTable` of `Rc<dyn Fn>` | host-ordinary 2.4; guest uses `Term`/`Instruction` constructors (grammar §7) |
| message passing | closure returning `Rc<dyn Fn(Msg)>` | 2.1.3, 2.4.3 |
| `set!`, local state | captured `Rc<Cell<T>>`/`Rc<RefCell<T>>` | 3.1 |
| `set-car!`/`set-cdr!` | `RefCell` fields of `ConsCell` | 3.3.1 |
| queues, tables | `Queue` header over cons cells; `HashMap<Key, Value>` | 3.3.2, 3.3.3 |
| wires, agenda | `Wire` with `Cell` signal + action list; `BinaryHeap` agenda | 3.3.4 |
| constraints | `Weak<dyn Constraint>` links | 3.3.5 |
| serializer, mutex | `Arc<Mutex<()>>`-based serializer | 3.4 |
| `delay`, memo-proc | `Lazy<T>` memoized thunk | 3.5 |
| streams | `Stream<T> = Rc<SNode<T>>` with `Lazy` tail | cycles leak by design, see notes |
| environments | `Store` arena (`Vec<Frame>`) with `usize` frame indexes | guest subset; 3.2 re-cut, chapter 4 (grammar §§8) |
| `evaluate`/`apply` | functions over typed `Expr`/`Program` and the arena `Store` | guest subset; Part 4 sketch (grammar §§8) |
| thunks (lazy) | explicit `Thunk`/`Force` data in `lazy-recompute/1` and `lazy-memo/1` | guest experiments; core stays strict (grammar §7) |
| search | `search-depth-first/1` over `Choose`/`Fail`/`Success` with `Vec` order and explicit trail | guest experiment; 4.3 (grammar §7) |
| query unification | `Term`/`Substitution` (`HashMap<String, Term>`), ordered `Vec` answers | guest subset; 4.4 (grammar §§7, 9) |
| registers, stack, controller | `Register`/`Label`/`Operand`/`Instruction`/`MachineProgram` values, explicit stack | guest subset; 5.1-5.5 (grammar §§7, 9) |
| memory vectors, GC | `Vec`-backed heap, `Word` enum, checked `usize` indexes | guest subset; 5.3 stays a simulation (grammar §9) |
| instruction sequences, `preserving` | typed sequence/label data with exact needed/modified sets | guest subset; 5.5 (grammar §9) |

Why the `put`/`get` table and not traits (2.4): the section's subject is that dispatch is data extended at runtime, by "installing packages" that may not exist when the caller compiles (exercise 2.73 installs derivative rules; 2.74 looks up per-division record handlers by runtime key; 2.5.3 adds polynomial-over-polynomial arithmetic). Rust traits close the type set at compile time and cannot be extended by later installs or dispatch on dynamically discovered tags. The handlers themselves are `Rc<dyn Fn>` trait objects, which satisfies the style contract's `dyn Trait` rule for heterogeneous collections. Where the book's datum is statically typed (chapters 1-3 examples), plain enums and `match` are used instead.

### Sketches for hard mappings (all comment-free; rationale in prose)

Cons as procedure (2.1.3 host-ordinary conceptual sketch; the mutable variant at texi 16583-16591 is built the same way around `make-account`). Guest evaluators do not use this shape; they use the arena `Store` in Part 4 and grammar §8.

```rust
// Host-ordinary conceptual sketch for chapters 2-3 only.
// Nil/Pair cover the cons-cell lessons; Integer covers arithmetic lessons.
enum HostVal { Integer(i128), Nil, Pair(Pair) }
type Pair = Rc<ConsCell>;
struct ConsCell { car: RefCell<HostVal>, cdr: RefCell<HostVal> }
enum ProcError { UnknownMessage(u8) }
type PairProc = Rc<dyn Fn(u8) -> Result<HostVal, ProcError>>;
fn cons_proc(x: HostVal, y: HostVal) -> PairProc {
    Rc::new(move |m| match m {
        0 => Ok(x.clone()),
        1 => Ok(y.clone()),
        _ => Err(ProcError::UnknownMessage(m)),
    })
}
fn car_proc(p: &PairProc) -> Result<HostVal, ProcError> { p(0) }
```

Church numerals (exercise 2.6) encode a count by repeated application. Zero returns an identity step. A wrapper struct breaks the recursive type cycle that a bare `type` alias cannot express:

```rust
struct Step(Rc<dyn Fn(&mut i128)>);
struct Church(Rc<dyn Fn(Step) -> Step>);
fn zero() -> Church { Church(Rc::new(|_| Step(Rc::new(|_| ())))) }
fn church_succ(n: &Church) -> Church {
    let n = Church(Rc::clone(&n.0));
    Church(Rc::new(move |f: Step| {
        let g = Rc::clone(&f.0);
        let previous = Rc::clone(&n.0);
        Step(Rc::new(move |x: &mut i128| { g(x); (previous(Step(Rc::clone(&g))).0)(x) }))
    }))
}
```

Persistent list with structural sharing (2.2):

```rust
#[derive(Clone)]
pub enum List<T> { Nil, Cons(T, Rc<List<T>>) }
impl<T> List<T> {
    pub fn cons(x: T, rest: Rc<List<T>>) -> Self { List::Cons(x, rest) }
    pub fn car(&self) -> Option<&T> { match self { List::Cons(x, _) => Some(x), _ => None } }
    pub fn cdr(&self) -> Option<&Self> { match self { List::Cons(_, r) => Some(r), _ => None } }
}
```

Rationals with constructor-enforced invariants (2.1.1 host-ordinary checked source):

```rust
pub struct Rational { num: i128, den: i128 }
pub enum RationalError { DivisionByZero }
impl Rational {
    pub fn new(num: i128, den: i128) -> Result<Self, RationalError> {
        let g = gcd(num.abs(), den.abs());
        let (num, den) = (num / g, den / g);
        (den > 0).then(|| Rational { num: den.signum() * num, den: den.abs() })
            .ok_or(RationalError::DivisionByZero)
    }
}
```

Tagged data and the operation table (2.4.2-2.4.3 host-ordinary conceptual sketch). Host `HostVal` carries closed host data, so dynamic tables key on a separate `Key` wrapper. Guest query and machine engines do not use this table; they use `Term`/`Query` and `Instruction`/`MachineProgram` constructor values per grammar §7.

```rust
// Host-ordinary conceptual sketch for chapter 2 only.
pub enum Key { Sym(Rc<str>), Int(i128), Pair(Box<Key>, Box<Key>), Str(Rc<str>) }
pub enum HostError { Dispatch(String) }
pub struct OpTable(RefCell<HashMap<(Key, Key), Handler>>);
impl OpTable {
    pub fn put(&self, op: Key, tag: Key, h: Handler) { self.0.borrow_mut().insert((op, tag), h); }
    pub fn get(&self, op: &Key, tag: &Key) -> Option<Handler> { self.0.borrow().get(&(op.clone(), tag.clone())).cloned() }
}
```

Painter as closure with an SVG sink (2.2.4; `segments->painter` and `draw-line` are grounded in the section):

```rust
pub struct Frame { pub origin: Vec2, pub e1: Vec2, pub e2: Vec2 }
pub trait Sink { fn line(&mut self, a: Vec2, b: Vec2); }
pub type Painter = Rc<dyn Fn(&Frame, &mut dyn Sink)>;
pub fn segments_painter(segs: Vec<(Vec2, Vec2)>) -> Painter {
    Rc::new(move |f: &Frame, s: &mut dyn Sink| {
        for (a, b) in &segs { s.line(map_frame(f, *a), map_frame(f, *b)) }
    })
}
```

Symbolic differentiation by `match` (2.3.2):

```rust
pub enum Expr { Num(i128), Var(Rc<str>), Sum(Box<Expr>, Box<Expr>),
                Prod(Box<Expr>, Box<Expr>), Pow(Box<Expr>, Box<Expr>) }
pub fn deriv(e: &Expr, v: &str) -> Expr {
    match e {
        Expr::Num(_) => Expr::Num(0),
        Expr::Var(x) => Expr::Num((x.as_ref() == v) as i128),
        Expr::Sum(a, b) => sum(deriv(a, v), deriv(b, v)),
        Expr::Prod(a, b) => sum(prod(deriv(a, v), b.clone()), prod(a.clone(), deriv(b, v))),
        Expr::Pow(u, n) => prod(prod(n.clone(), pow(u.clone(), sub(n.clone(), Expr::Num(1)))), deriv(u, v)),
    }
}
```

`make-account` with shared state captured by two closures (3.1.1; grounded at texi 14292-14317):

```rust
pub struct Account { call: Rc<dyn Fn(Msg, i128) -> Result<i128, BankError>> }
pub fn make_account(balance: i128) -> Account {
    let bal = Rc::new(Cell::new(balance));
    let call = Rc::new(move |m: Msg, amt: i128| {
        let b = bal.get();
        match m {
            Msg::Withdraw if b >= amt => { bal.set(b - amt); Ok(b - amt) }
            Msg::Deposit => { bal.set(b + amt); Ok(b + amt) }
            _ => Err(BankError::InsufficientFunds),
        }
    });
    Account { call }
}
```

Mutable pair and aliasing questions (3.3.1 host-ordinary; `Rc::ptr_eq` is documented on the `Rc` page, https://doc.rust-lang.org/std/rc/struct.Rc.html). Guest engines do not use reference-counted cells; they use arena indexes per grammar §§5, 8.

```rust
pub fn set_car(p: &Pair, v: HostVal) { *p.car.borrow_mut() = v; }
pub fn eq_pair(a: &Pair, b: &Pair) -> bool { Rc::ptr_eq(a, b) }
```

Queue over mutable pairs (3.3.2 host-ordinary), exactly the book's front and rear pointers:

```rust
pub struct Queue { front: RefCell<Option<Pair>>, rear: RefCell<Option<Pair>> }
impl Queue {
    pub fn insert(&self, v: HostVal) { let p = cons_cell(v, HostVal::Nil);
        match &*self.rear.borrow() { Some(r) => { *r.cdr.borrow_mut() = HostVal::Pair(Rc::clone(p)); },
            None => *self.front.borrow_mut() = Some(Rc::clone(&p)) }
        *self.rear.borrow_mut() = Some(p); }
}
```

Memoized table (3.3.3 host-ordinary; `memo-fib` keys are host values wrapped in `Key`):

```rust
pub struct MemoTable(RefCell<HashMap<Key, HostVal>>);
impl MemoTable {
    pub fn lookup_insert(&self, k: Key, f: impl FnOnce() -> HostVal) -> HostVal {
        if let Some(v) = self.0.borrow().get(&k) { return v.clone(); }
        let v = f(); self.0.borrow_mut().insert(k, v.clone()); v
    }
}
```

Wire and agenda (3.3.4; grounded `make-wire` at texi 17724, half-adder 17505, probe output 17911). Gates hold `Weak<Wire>` handles because gate actions capture wires while wires own the action list; strong `Rc` on both sides would form a permanent cycle (`Weak` for cycles is the documented pattern, https://doc.rust-lang.org/std/rc/index.html):

```rust
pub struct Wire { pub sig: Cell<u8>, actions: RefCell<Vec<Rc<dyn Fn()>>> }
pub struct Agenda { now: Cell<u64>, seq: Cell<u64>,
    heap: RefCell<BinaryHeap<Reverse<(u64, u64, Rc<dyn Fn()>)>>> }
impl Agenda {
    pub fn after_delay(&self, d: u64, a: Rc<dyn Fn()>) {
        let s = self.seq.replace(self.seq.get() + 1);
        self.heap.borrow_mut().push(Reverse((self.now.get() + d, s, a)));
    }
}
```

Constraint connector (3.3.5; grounded `make-connector` at texi 18550, converter 18246-18256):

```rust
pub trait Constraint { fn new_value(&self); fn forget_value(&self); }
pub struct Connector { val: RefCell<Option<Value>>,
    informant: RefCell<Option<Weak<dyn Constraint>>>,
    links: RefCell<Vec<Weak<dyn Constraint>>> }
```

Serializer over `Arc<Mutex<()>>` (3.4; grounded at texi 19619-19624; `Arc` and `Mutex` per https://doc.rust-lang.org/std/sync/index.html; scoped threads per https://doc.rust-lang.org/std/thread/index.html):

```rust
pub type Proc = Box<dyn FnOnce() + Send>;
pub struct Serializer(Arc<Mutex<()>>);
impl Serializer {
    pub fn call(&self, p: Proc) -> Proc {
        let m = Arc::clone(&self.0);
        Box::new(move || { let _guard = m.lock(); p(); })
    }
}
```

`test-and-set!` and the exercise 3.47 semaphore. `AtomicBool` and `Ordering` are grounded on the atomic module page, which also discusses `compare_exchange` in its memory-model text (https://doc.rust-lang.org/std/sync/atomic/index.html); the struct page 404'd and is not cited. `Condvar` per https://doc.rust-lang.org/std/sync/index.html:

```rust
pub struct TestAndSetCell(AtomicBool);
impl TestAndSetCell {
    pub fn test_and_set(&self) -> bool {
        self.0.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err()
    }
}
pub struct Semaphore { n: Mutex<usize>, cv: Condvar }
```

Memoized `Lazy` and `Stream` (3.5; this is the book's `memo-proc`):

```rust
pub struct Lazy<T> { f: RefCell<Option<Box<dyn FnOnce() -> T>>>, memo: RefCell<Option<Rc<T>>> }
impl<T: 'static> Lazy<T> {
    pub fn force(&self) -> Rc<T> {
        if let Some(v) = &*self.memo.borrow() { return Rc::clone(v); }
        let f = self.f.borrow_mut().take();
        let v = Rc::new(f.expect("reentrant force").call_mut());
        *self.memo.borrow_mut() = Some(Rc::clone(&v)); v
    }
}
pub struct SNode<T> { pub head: T, pub tail: Lazy<SNode<T>> }
pub type Stream<T> = Rc<SNode<T>>;
```

Stream sharing rule, stated as a property (3.5 host-ordinary): the book's self-referential streams (`fibs` defined in terms of itself, the `solve` integral at 3.5.4) share memoized tails between cursors; two traversals of one stream do half the work of two fresh iterator traversals. The `Rc` cycle this sharing forms in the host port is bounded to streams the program still holds and is reclaimed at process exit. Guest lazy experiments do not inherit this host cycle; `lazy-recompute/1` reevaluates each force and `lazy-memo/1` stores the first successful result with effects once while a failed force stays delayed (grammar §7).

`search-depth-first/1` (4.3 guest subset; semantic contract in Part 4 and grammar §7). The engine runs over explicit `Choose(Vec<T>)`, `Fail`, and `Success(T)` data depth-first in vector order with an explicit trail that rolls back only trailed assignments. It is deterministic for a fixed program and seed. Ordinary `?`, `Result`, panics, and `return` never trigger backtracking. Conceptual shape only; the normative data shapes are grammar §7:

```rust
// Guest-subset conceptual shape; normative shapes are grammar §7.
struct SearchFrame<T> { alternatives: Vec<T>, trail: Vec<TrailEntry> }
struct SearchEngine<T> { stack: Vec<SearchFrame<T>>, answers: Vec<T> }
```

Unification (4.4 guest subset; semantic contract in Part 4 and grammar §§7, 9). Queries unify `Term` values and extend `Substitution` (`HashMap<String, Term>`) with the occurs-check policy stated by each query-engine exercise, collecting answer frames in `Vec` order. A `HashMap` is only an index and never determines answer order. Conceptual shape only; the normative shapes are grammar §7:

```rust
// Guest-subset conceptual shape; normative shapes are grammar §7.
fn unify(term: &Term, datum: &Term, subst: &Substitution) -> Option<Substitution>;
```

`preserving` (5.5.4 guest subset; the book's algorithm, grounded in 5.5.1's examples). Compile output is typed instruction data compared by observable result and effect order; needed/modified register sets must stay exact because a wrong set silently changes semantics (grammar §9). Conceptual shape only; the normative instruction shapes are grammar §7:

```rust
// Guest-subset conceptual shape; normative shapes are grammar §7.
fn preserving(registers: &[Register], first: Sequence, second: Sequence) -> Sequence;
```

Stop-and-copy core (5.3.2 guest subset; broken hearts and forwarding addresses grounded at texi 33541-33570, scan overtaking free at 33745). Heap addresses are checked `usize` indexes into `Vec`-backed storage with a `Word` enum and work lists; no raw pointers, `unsafe`, or hidden collector is claimed (grammar §9):

```rust
// Guest-subset conceptual shape; normative heap boundary is grammar §9.
enum Word { Ptr(usize), Int(i64), Sym(usize), Bool(bool), BrokenHeart(usize), Free }
fn gc(memory: &mut Memory);
```

## Part 2. Per-section notes

Section numbers follow the checked-in section/exercise inventories with Git provenance (see `docs/plan/host-subsets-specification.md`); the old `text/original/sicp-pocket.texi` archive is not source authority.

**1.1 The elements of programming (1107).** What changes: special forms are Rust syntax; `define` becomes `let`/`fn`; the interpreter transcript becomes compiled example output. Hard spot: `new-if` (exercise 1.6, grounded: Eva's `new-if` and Alyssa's `sqrt-iter` rewrite appear in this section) teaches eager argument evaluation; a Rust `fn new_if(...)` evaluates its arguments exactly the same way, so the infinite loop reproduces faithfully. Representative program: `sqrt` by Newton's method (1.1.7), with `improve`, `good_enough`, and block structure as nested functions. Tailored: 1.1a express `sqrt_iter` as an iterator chain (`iter::successors` plus a take-while on `good_enough`, both grounded on https://doc.rust-lang.org/std/iter/index.html) and compare readability; 1.1b property-test `sqrt` monotonicity and relative error against `f64::sqrt` with proptest.

**1.2 Procedures and the processes they generate (2630).** What changes: Scheme's implicit tail-call optimization becomes explicit loops in the iterative variants; recursion depth is Rust stack. Hard spot: exact-integer growth; `i128` covers factorial to 33 and the 1.22-1.23 prime searches near 1e12, but exercise 1.25's Alyssa-style `expmod` computes `a^(n-1)` for 13-digit `n`, unrepresentable at any fixed width; with checked arithmetic the overflow error is the lesson (see part 3). Representative program: `timed_prime_test` using `Instant::now`/`elapsed` (https://doc.rust-lang.org/std/time/struct.Instant.html). Tailored: 1.2a a noise-aware timing harness: median of 30 runs per prime, table snapshot with insta; 1.2b memoized `count_change` via `HashMap` with a proptest case equating it to the naive version for amounts up to 30.

**1.3 Formulating abstractions with higher-order procedures (4192).** What changes: procedures as arguments and return values become closures with `impl Fn` bounds; `sum`, `pi_sum`, `integral` (1.3.1) translate directly. Hard spot: returning closures that capture parameters, and exercise 1.41/1.43 composition (`double`, `repeated`) where the closure must clone captured data rather than borrow it. Representative program: `sum` as a generic higher-order function plus `fixed_point` and Newton's method (1.3.4). Tailored: 1.3a implement `repeated(f, n)` twice, recursive closure tree versus loop-built `Rc<dyn Fn>`, and compare call depth; 1.3b `smooth` (1.44) as a combinator, property-tested for symmetry of the double-smoothed sine approximations.

**2.1 Introduction to data abstraction (5905).** What changes: constructors and selectors become a `struct` with private fields and methods, which makes the abstraction barrier compile-time rather than conventional. Hard spot: 2.1.3 "what is meant by data" (procedural cons, exercise 2.6 church numerals; `zero` grounded in the section) needs the wrapper-struct trick above. Representative program: `Rational` arithmetic with `print_rat`-style `Display` (2.1.1), then `par1`/`par2` interval divergence (2.1.4). Tailored: 2.1a `Rational` checked operations returning `Result`, with a test pinning the first overflowing operation in a long product; 2.1b measure `par1`/`par2` relative error growth across 10 equivalent formulas for two-resistor networks, snapshot the table.

**2.2 Hierarchical data and the closure property (6781).** What changes: three representations in sequence: `Vec<T>` for flat sequences (2.2.3's map/filter/accumulate become iterator adapters), `List<T>` for cons-cell structure where sharing diagrams matter (2.2.1-2.2.2), and closures plus structs for the picture language (2.2.4). Hard spot: exercise 2.18 reverse and 2.20 variadic procedures on `List<T>`; and painter combinators returning closures that capture frames (`beside`, `below`, `flip_vert`), all `'static` via owned data. Representative program: `count_leaves`/`fringe` on `List`, `queens` backtracking over `Vec`, and `segments_painter` writing SVG (exercise 2.49's anchor format is grounded). Tailored: 2.2a `fringe` as a lazy `Iterator` over `List`, proptest-equated with the eager version; 2.2b `beside(flip_vert(p), p)` SVG snapshot pair asserted symmetric via insta.

**2.3 Symbolic data (9608).** What changes: quotation disappears into constructors; `eq?` on symbols becomes `Rc<str>` comparison; `memq`/`equal?` become `Key` recursion. Hard spot: 2.3.2's list-pattern matching becomes exhaustive `match` on `Expr`, which is a teaching upgrade: the compiler proves the cases total. Sets (2.3.3) as sorted `Vec`, binary-search tree enum; Huffman trees (2.3.4) as an enum with weights. Representative program: `deriv` plus `simplify`, and Huffman `encode`/`decode`. Tailored: 2.3a a tiny `Expr` reader/parser and a proptest round-trip `parse(display(e)) == e`; 2.3b property-test the prefix-freeness of generated Huffman codes.

**2.4 Multiple representations for abstract data (11234).** What changes: this is where the host-ordinary dispatch table enters for chapter 2 (sketch above), not traits, because the section's point is runtime extension. Tags are host `Key` values; the guest query and machine engines in chapters 4-5 use `Term`/`Query` and `Instruction` constructor values per grammar §7, not this table. Hard spot: exercise 2.73's "why can `number?` and `same-variable?` not be installed in the table" becomes concrete: those predicates dispatch on the datum itself rather than a tag, and the Rust port shows the residual `match` arms beside the table. Representative program: rectangular and polar packages installed into the table with `apply_generic` on top (2.4.2-2.4.3). Tailored: 2.4a micro-benchmark table dispatch versus an exhaustive enum `match` for the complex operations, snapshot medians; 2.4b `apply_generic`'s missing-handler error path: a typed error listing the operation and tag, tested.

**2.5 Systems with generic operations (12310).** What changes: the tower uses `From`/`TryFrom` conversions for `raise`/`project` (https://doc.rust-lang.org/std/convert/index.html) on top of the same table; ordinary numbers, rationals, reals, complex. Hard spot: coercion traps (exercises 2.81-2.83: raising into sameness, infinite raise loops) map onto the `apply_generic` retry loop; 2.5.3 term lists are sparse `Vec<(u32, Coeff)>` with coefficient arithmetic pulled from the table so polynomials over polynomials work. Representative program: `add_poly` on sparse term lists, then `div_poly` toward the gcd/reduce exercises (2.94-2.97). Tailored: 2.5a dense/sparse term-list equivalence by proptest over random polynomials; 2.5b a `raise(project(z)) == z` round-trip suite that classifies exactly which tower levels are lossless.

**3.1 Assignment and local state (14039).** What changes: `set!` becomes interior mutability captured by returned closures; `make_withdraw` (grounded at 14252) returns one closure, `make_account` (14292) returns a dispatch over a shared `Rc<Cell<i128>>`. Hard spot: exercise 3.7 `make_joint` (grounded at 14981) requires two password front-ends sharing one balance, which forces the state into the shared `Rc` rather than each closure. Representative program: `make_account` with password (exercise 3.3 at 14399) and the Monte Carlo estimate of pi from 3.1.2-3.1.3 using a hand-rolled `rand_update` (`u64` xorshift, no external crate). Tailored: 3.1a `rand_update` as a pure function; snapshot the deterministic stream for a fixed seed; 3.1b compare Cesaro estimates at 100, 1_000, 10_000 trials in a table test with a loose tolerance.

**3.2 The environment model of evaluation (15026). Re-cut:** this section becomes the ownership and closure-capture model. The book's figures (grounded: environment diagrams for `make-withdraw` with `@ifinfo` ASCII art at 15533-15617, exercise 3.10 at 15783, exercise 3.11 at 15955-15996) are retained, and the prose walks the same three questions: where the binding lives, what the procedure object captures, and why `W1` and `W2` share code but not state (grounded at 15727-15746). The Rust answer: frames are owned bindings, a closure captures its environment by borrow or `move`, and a `move` closure moves (never implicitly clones) each non-`Copy` capture. Hard spot: making readers see that an explicit clone is a separate operation and that the guest evaluator threads an arena-index environment instead of a reference-counted cell (grammar §§5, 8). Representative program: a three-listing walkthrough of `make_withdraw` showing the captured environment, two independent accounts, and a deliberately non-compiling fourth listing where a closure outlives a borrowed capture. Exercises 3.10 and 3.11 keep their numbers and become predict-then-verify: state the sharing before running, then assert with a debug printer that shows `Rc` pointer identity per frame (this is the re-cut's one forced exercise divergence: diagram drawing becomes pointer-identity verification).

**3.3 Modeling with mutable data (15999).** What changes: everything is host-ordinary shared-mutable graphs (`Rc<RefCell<...>>`); guest engines use arena indexes instead (grammar §§5-6); aliasing questions (3.12-3.18) become `Rc::ptr_eq` questions, sharper than in Scheme because sharing is visible in the types. Queues (3.3.2) mirror the book's front/rear pair pointers exactly; tables (3.3.3) use `HashMap<Key, HostVal>` with the pairs-based table as a tailored addition; the circuit simulator (3.3.4, `make-wire` grounded at 17724) uses the agenda sketch above with a `(time, seq)` heap key so propagation order is deterministic; exercise 3.31's immediate action run (grounded at 17953) is a test. Constraints (3.3.5) use `Weak<dyn Constraint>` links to break the connector-constraint cycle (grounded `make-connector` at 18550). Hard spot: agenda determinism and the deliberate `Weak` in the two cyclic graphs. Representative program: the half-adder with probe output snapshots (`sum 0  New-value = 0` grounded at 17911), and the Celsius-Fahrenheit converter (18246). Tailored: 3.3a tortoise-hare cycle detection with a proptest over randomly linked and randomly mutated cons cells; 3.3b the pairs-based `make-table` with `equal?` keys and per-table `Key` equality, tested against the `HashMap` version.

**3.4 Concurrency: time is of the essence (18791). Re-cut:** the section keeps its number and its lesson set (interleavings, serializers, deadlock) but is taught with Rust's own concurrency mechanisms. The ground truth is that ownership is the first serializer: the borrow checker rejects at compile time the data race the book's first example races into, so 3.4.1 begins with "why the compiler stopped you" and then deliberately moves the state into `Arc<Mutex<..>>` to allow the race-shaped program. `parallel-execute` (grounded at 19179) becomes `thread::scope` with spawned closures; the book's footnote (19190-19194) says the Scheme primitive returns a control object that can halt the processes, which `thread::scope` cannot do, so the port maps that handle to a shared `AtomicBool` stop flag that each procedure polls; that mapping is stated in the text. `make-serializer` (19619) is the `Arc<Mutex<()>>` sketch; `test-and-set!` is `compare_exchange` on `AtomicBool` per the atomic module page; exercise 3.47 semaphores use `Condvar`; the serialized `make-account` (19251), `make-account-and-serializer` (19456), `transfer` (19530), and the numbered-accounts deadlock avoidance of exercise 3.48 (grounded at 19780) run as written. Hard spot: `Rc<RefCell>` objects from 3.3 are not `Send` (grounded: the `rc` module page states `Rc` does not implement `Send`; use `Arc`), so 3.4 restates the 3.3 account on `Arc<Mutex<>>`: a deliberate, stated type migration. Representative program: serialized `exchange` demonstrating both the lost-update fix and the two-account deadlock. Tailored: 3.4a exercise 3.39's possible outcomes (101, 121, 100) enumerated deterministically with a seeded interleaving harness over `AtomicBool` yield points; 3.4b a `Condvar::wait_timeout` variant of `acquire` with timeout semantics tested by a holding thread.

**3.5 Streams (19855).** What changes: `cons-stream`/`delay` become `Stream<T>` nodes with `Lazy` tails (sketch above); `map`/`filter`/`accumulate` exist twice, once on streams and once as iterator adapters, and the text says exactly when each applies. `Iterator` suffices where a single cursor consumes a stream linearly; it cannot replace `Stream` where (a) two independent cursors must share one memoized tail, (b) 3.5.4 feedback (`solve`, the integral loop) demands a value before it exists, or (c) explicit guest search and query answer collection use `Vec` order with an explicit trail, not host stream sharing (grammar §§7, 9). Host stream sharing is bounded as stated in part 1; guest lazy experiments follow the `lazy-recompute/1` / `lazy-memo/1` contracts. Hard spot: making the leak rule and the memoization-sharing rule explicit before readers build `fibs`. Representative program: the primes sieve and `sqrt_stream`; 3.5.4's `solve` for `dy/dt = y`, initial 1, step 0.001. Tailored: 3.5a implement primes twice (iterator versus stream) and test the memoization difference: two traversals of one stream do half the work of two `nth` calls on a fresh iterator each, measured with a counter cell; 3.5b the Cesaro stream from 3.5.5 driving a convergence table snapshot.

**4.1 The metacircular evaluator (22672).** What changes: the evaluator parses and type-checks checked host source before any effect and executes the guest `evaluate`/`apply` algorithm over the arena `Store` (Part 4 sketch; grammar §§8); syntax dispatch is an exhaustive `match` over the typed `Expr` constructors. 4.1.6 internal definitions follow the book's sequential-frame treatment over owned frames; 4.1.7 `analyze` emits a typed plan/enum over the same representation, exactly the book's closures-produced-once design. Hard spot: deep recursion. The stack bound is stated explicitly: deep recursion is bounded by the Rust stack; tail recursion is not eliminated in chapter 4, and the honest escape is the chapter 5 machine, which is tail-recursive by construction (grounded at 34070-34076). Representative program: the driver loop running factorial of 6 and a leaf-counting definition, `println!` transcripts snapshotted and compared byte-for-byte with native runs (grammar §§8). Tailored: 4.1a an instrumented eval counting dispatches, comparing 4.1.1 versus 4.1.7 `analyze` on `fib 15`, table snapshot; 4.1b `and`/`or` as native special forms with short-circuit, tested equivalent to nested `if` desugaring.

**4.2 Lazy evaluation (24872).** What changes: `delay-it`/`force-it` (grounded at texi 25222 and the memoized 25267) become the `lazy-recompute/1` and `lazy-memo/1` experiments over explicit `Thunk`/`Force` AST data (grammar §7); the core stays strict, so exercise 4.27's count-of-forcing question is answered by a probe cell, not by prose. `actual-value` forces at primitive-application boundaries; 4.2.3 streams-as-lazy-lists falls out of the same thunk. Hard spot: interaction of thunks with `set!` (4.27-4.29): side effects inside a thunk must run at most once, which the `lazy-memo/1` stored-result rule guarantees structurally (a failed force stays delayed). Representative program: the identity-function forcing-count demonstration and a lazy list consumed through explicit forcing. Tailored: 4.2a exercise 4.31's per-parameter strictness annotations with a demand probe counting forces per parameter; 4.2b lazy `fib` versus strict `fib` in the same evaluator, thunk-allocation counts tabled.

**4.3 Nondeterministic computing (25622).** What changes: chosen mechanism is explicit backtracking, not continuation-passing closures; the mechanism is `search-depth-first/1` over explicit `Choose(Vec<T>)`, `Fail`, and `Success(T)` data with vector order and an explicit rollback trail (Part 4 sketch; grammar §7): plain code, testable, and deterministic for a fixed program and seed. Answer iteration replays the explicit trail instead of re-running from scratch. Hard spot: `require` and `an-element-of` must restore state on backtracking; the undo trail is the piece CPS gets for free. Representative program: `multiple_dwelling` with all solutions iterated, and the parse example from 4.3.2. Tailored: 4.3a instrument failures per `amb` site, snapshot the counts for `multiple_dwelling`; 4.3b exercise 4.40-style reordering: measure step counts across orderings of the dwelling constraints, table snapshot.

**4.4 Logic programming (27116).** What changes: assertions and rules are `Term` values; the database is a `Vec<Term>` plus a `HashMap<String, Term>` index used only as an index (the indexing of 4.4.4); query evaluation collects answer substitutions in `Vec` order (grammar §§7, 9). Unification extends substitutions with the per-exercise occurs-check policy (Part 4 sketch). Hard spot: `negation` and `lisp_value` as singleton-stream filters, and rule-application depth for `lives_near`-style recursions. Representative program: `simple_query` over the micro-employee database, the `append` rule answering both directions, and `unique` (exercise 4.75 territory). Tailored: 4.4a add the symbol index and measure queries before and after, snapshot timings as counts of frames examined; 4.4b an `max_of` aggregation built as a `lisp_value` primitive over collected frames.

**5.1 Designing register machines (30358).** What changes: prose plus the controller language; machines are described with `Register`/`Label`/`Operand`/`Instruction` constructor values (grammar §7) and become executable from 5.2 on. Hard spot: recursion as stack discipline (5.1.4) with the `continue` register; the fib machine is the canonical listing. Representative program: the GCD controller as a `MachineProgram` value. Tailored: 5.1a hand-simulate GCD as a Rust `enum`-state machine mirroring the controller line for line, asserting step counts; 5.1b the factorial machine with push counts annotated per controller line (book 5.5.5-style table).

**5.2 A register-machine simulator (31562).** What changes: the book's `make-machine` is itself message-passing (grounded at 31678); the Rust port is a typed assembler function plus a machine runner over `MachineProgram` values: assembly validation returns typed faults before execution, and only validated programs run (grammar §§7, 9). The assembler (grounded at 31939: "much like the evaluators of Chapter 4") is a single pass building instruction/label vectors and a label table; the book's `extract-labels` footnote about returning two values with `receive` (grounded at 31962-32029) becomes an ordinary tuple, noted in passing. Monitoring (5.2.4, grounded at 32615) counts instructions, pushes, and maximum depth in `Cell<u64>`. Hard spot: the ops table must not let an operation read the `pc` register (exercise 5.9): ops receive only admitted operand values and never the program counter directly (exercise 5.9 boundary; grammar §§7). Representative program: GCD machine with statistics; the recursive fib machine with stack depth report. Tailored: 5.2a duplicate-label detection (exercise 5.8's territory) as a typed assembler error with a test; 5.2b a trace mode printing one line per executed instruction, insta-snapshot of the GCD trace.

**5.3 Storage allocation and garbage collection (32786). Confirmed unchanged:** this stays a memory simulation. Pairs live in two `Vec<Word>` semispaces at base pointers; roots are the machine registers plus the stack; the collector is the book's stop-and-copy with broken hearts and forwarding addresses (grounded at 33541-33570 and 33745). Hard spot: sizing exercises and root scanning; the exercise set is design-heavy and stays prose-plus-simulation. Representative program: allocate three pairs, drop one by mutation, run `gc`, print both semispaces before and after. Tailored: 5.3a proptest: for random allocation/mutation scripts, the multiset of reachable pairs is identical before and after collection; 5.3b a mark-sweep alternative collector reporting fragmentation statistics alongside the copying one.

**5.4 The explicit-control evaluator (33544).** What changes: nothing conceptual; the book's controller (grounded: `eval-dispatch` at 33642, `ev-application` at 33791, `ev-if-decide` at 34161, tail-recursion prose at 34070) is a checked `MachineProgram` value run on the 5.2 machine data, with the chapter 4 arena environment and primitive operations supplied as typed values. The driver loop prints through admitted `println!` and compares byte-for-byte with native runs. Hard spot: controller size; it lives as its own constructor-value module in the crate. The payoff listing: Fibonacci of 10 on EC-Eval grows the simulated stack, not the Rust stack, and deep tail recursion runs where the chapter 4 evaluator could not. Representative program: `factorial` and `fib` on EC-Eval with `print_statistics`, compared against chapter 4 step counts. Tailored: 5.4a generate a stack-depth histogram per program (5.27-style) automatically, snapshot; 5.4b install `cond` directly in the controller and compare with desugaring to nested `if`, asserting equal value and fewer dispatch steps.

**5.5 Compilation (34641).** What changes: `compile` consumes the same typed AST as 4.1; output is typed instruction/sequence/label data combined by `preserving` (Part 4 sketch; grounded examples in 5.5.1). Lexical addressing (5.5.6) uses a compile-time environment of name vectors; open-coded arithmetic emits primitive assignments. 5.5.7 keeps compiled procedures as typed values holding entry, parameters, and environment; both standard entry points route through the machine runner and compare observable results (grammar §9); exercise 5.50 runs the guest evaluator through the compiler and 5.51/5.52 keep their C targets outside the Rust guest grammar; both appear as example programs, not only as a standalone machine run. Hard spot: `preserving` bookkeeping and keeping `needs`/`modifies` exact, since a wrong set silently changes semantics. Representative program: compile recursive `factorial`, run it through the machine runner, and table its instruction and push counts against interpreted EC-Eval (the book's 5.5.5 comparison). Tailored: 5.5a per-expression compiled-listing snapshots with insta (each expression's rendered instruction sequence is a golden file); 5.5b a constant-folding pass over the sequence data with a proptest that folded-then-run equals run-then-value on arithmetic constants.

## Part 3. Edition conventions

- Scope. Chapters 0-3 are ordinary host Rust (1.98.1, edition 2024) and may use the full host teaching surface named in Part 0-1 (`i128`, `f64`, `Rc`, `RefCell`, `Arc`, threads) where the lesson needs it. Chapters 4-5 are the restricted guest subset in `spec/host-subsets/rust/grammar.md` §§1-7: `i64` plus `usize` only, `Box`/`Vec`/`HashMap<String, T>` indirection, no `Rc`/`Arc`/`RefCell`, no added crates in guest core. Ownership, move, borrow, and lifetime-elision obligations apply in both scopes; the guest adds arena-index sharing instead of reference-counted cells (grammar §§3, 5, 6, 8).
- Numbers. Host-ordinary (chapters 1-3): exact integers are `i128` with checked arithmetic returning chapter-local typed errors; inexact numbers are `f64`; `Rational` appears in 2.1.1; the `Number` tower enum appears in 2.5. Where `i128` is not enough, stated exactly: exercise 1.25/1.26 naive `expmod` intermediates (`a^(n-1)` for 13-digit `n`, beyond any fixed width; the checked overflow is the lesson), factorials past 33!, and fibonacci stream values past `n` about 184. The book never prints 100! (a file-wide grep for `100!` and `factorial 100` found no hits). No numeric crates in host chapters; if that default changes, only the `Number` internals change. Guest-subset (chapters 4-5): integer computation uses signed `i64`; `usize` is for lengths, indexes, and machine/arena identifiers only. `i128`, unsigned arithmetic other than `usize`, floating point, fractions, complex values, and implicit conversion are excluded (grammar §3). Literals infer as `i64`/`usize` from context; a literal left at `i32` inference is host-valid but subset-unsupported. `i64` overflow, division or remainder by zero trap under checked-build overflow checks; `usize` admits `+`/`-` only. Oracle binaries build with overflow checks on. Comparisons are same-type; conditions never use truthiness. Guest overflow is a trap that stops execution (grammar §6 class 4), never a returned `Err` value; host overflow stays a checked error value. The required change is removal of the old source plus correct guest contracts, not a rewrite of valid earlier host numeric objectives.
- Errors. No `SchemeError` type governs any scope. Host chapters use chapter-local typed errors (thiserror enums per crate family, `#[error("...")]` messages, `#[from]` for io and parse sources, per https://docs.rs/thiserror/2.0.20/thiserror/; `anyhow::Result` only in `examples/` mains per https://docs.rs/anyhow/1.0.104/anyhow/). Guest engines use `Result<Value, Fault>` and typed machine errors with the five grammar §6 dispositions: invalid Rust syntax, invalid Rust typing/ownership, host-valid but excluded (`Unsupported` with location), subset semantic/runtime trap (out-of-range index, checked-build overflow, division/remainder by zero), and observable stdout. An explicit returned `Err`/`None` is an ordinary result mapped to output or propagated with `?`, not a trap. The source checker (rustc authoritative for type/move/borrow plus the subset boundary) rejects the complete program before `main` or any guest effect runs (grammar §§1, 3, 6).
- Interactions. The texinfo source has none of the edition's output form; it prints results as `@i{...}` lines after expressions (14269-14272). Edition choice, stated as such: example output is the exact byte sequence from admitted `print!`/`println!` in evaluation order, compared byte-for-byte between checked guest runs and native runs (grammar §§6, 8, 10). `println!` appends one line feed. No Scheme-surface syntax, no custom `Display`/`Debug` in guest source (excluded by grammar §6), no `;Value:` transcript. In doc comments above exercise tests, the interaction appears as three lines: the definition, the call, and the printed result, mirroring the `@i{}` layout, for example a `withdraw` definition, a call with `50`, then `50`. No inline `//` transcript comments. Panic text and diagnostic wording are not stable contracts.
- Tests. `cargo nextest run` is the runner (https://nexte.st/); doctests run separately with `cargo test --doc` because nextest does not support them (stated on its site); insta's config selects nextest as its runner (https://docs.rs/insta/1.48.0/insta/). Assertions are `assert_eq!` on values; proptest covers recurrences and equivalences (https://docs.rs/proptest/1.11.0/proptest/); insta snapshots cover transcripts, SVG output, machine traces, and compiled listings. No `unwrap` outside tests.
- Naming. Book listings are runnable examples, one file per listing, named for the first procedure defined: `cargo run -p ch03 --example make_account`. Exercise solutions live in `chNN/tests/sec_X_Y.rs`, one test function per exercise (`fn e_3_47()`), with the exercise statement quoted in the `///` doc comment. Shared solution code lives in `chNN/src/sec_X_Y.rs` public modules. Tailored additions take the parent exercise number plus a letter (1.25a).
- Comments. `#![deny(missing_docs)]` via workspace lints; doc comments on public items only; no inline `//` prose in implementation code; `#[expect(clippy::...)]` always carries a reason string in the same doc comment.
- Style. `&T`, `&str`, `&[T]` parameters; generics on hot paths; `dyn Trait` for heterogeneous host collections; guest closures escape only through `Box<dyn Fn... + 'static>` with checked captures (grammar §5). Assembly validation returns typed faults before execution; no host `impl`-block type-state is claimed as guest syntax (grammar §§2, 9).

## Part 4. Architecture sketches

Chapter 4 evaluator (guest subset; semantic contract, not a published API). The teaching evaluator parses and type-checks ordinary Rust source per `spec/host-subsets/rust/grammar.md` §§1-5, then executes the guest `main` including its `evaluate`/`apply` algorithm without routing to the host production evaluator, reflection, or any host `eval` entry point. The checked guest run and the native run must produce identical stdout bytes. See the minimum witness in grammar §8 for the exact admitted forms.

```rust
// Conceptual shape only; the normative witness is grammar §8.
// Closed recursive data, Box indirection, arena environments, typed Result.
enum Expr { Integer(i64), Boolean(bool), Variable(String), Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>), Multiply(Box<Expr>, Box<Expr>), Equal(Box<Expr>, Box<Expr>),
    If { test: Box<Expr>, yes: Box<Expr>, no: Box<Expr> },
    Lambda { parameters: Vec<String>, body: Box<Expr> },
    Call { function: Box<Expr>, arguments: Vec<Expr> },
    Let { name: String, value: Box<Expr>, body: Box<Expr> } }
enum Value { Integer(i64), Boolean(bool), Function(usize),
    Closure { parameters: Vec<String>, body: Box<Expr>, environment: usize } }
struct Frame { bindings: HashMap<String, Value>, parent: Option<usize> }
struct Store { frames: Vec<Frame> }
enum Fault { Unbound, Type, Arity, NotCallable }
fn evaluate(expression: &Expr, environment: usize, global: usize, program: &Program, store: &mut Store) -> Result<Value, Fault>;
fn apply(callable: Value, arguments: Vec<Value>, global: usize, program: &Program, store: &mut Store) -> Result<Value, Fault>;
```

Calls and nested operands evaluate left to right, as in Rust (grammar §4). The `Value::Closure` environment is an index into the owned `Store` arena; closure capture and recursive calls use no `Rc`, no host reflection, and no hidden evaluator (grammar §§5, 8). An analyzer variant emits a typed plan/enum over the same representation; there is no source-string quotation and no automatic truthiness (grammar §9). Deep recursion is bounded by the Rust stack; the chapter 5 machine is the tail-recursive answer, not a larger worker thread.

Lazy and search are separately named experiments, not core modes. `lazy-recompute/1` and `lazy-memo/1` run over explicit `Thunk`/`Force` AST data; the core stays strict, a delayed operand runs only when forced, memo stores the first successful result with effects once, and a failed force stays delayed (grammar §7). `search-depth-first/1` runs over explicit `Choose(Vec<T>)`, `Fail`, and `Success(T)` data depth-first in vector order with an explicit trail that rolls back only trailed assignments (grammar §7). Modes are selected by an explicit runner or typed mode value; `?`, `Result`, panics, and ordinary `return` never trigger search backtracking.

Chapter 5 register machine and query engine (guest subset; semantic contract). Queries and machine programs are typed constructor values, not parsed controller text and not additional Rust syntax (grammar §7). The contract shapes are:

```rust
// Conceptual shape only; the normative shapes are grammar §7.
enum Term { Variable(String), Integer(i64), Text(String), Atom(String), Pair(Box<Term>, Box<Term>), Empty }
enum Query { Unify(Term, Term), Relation { name: String, arguments: Vec<Term> },
    And(Vec<Query>), Or(Vec<Query>), Not(Box<Query>), Unique(Box<Query>) }
type Substitution = HashMap<String, Term>;
struct Register(String); struct Label(String);
enum Operand { Constant(i64), Register(Register), Label(Label) }
enum Instruction { Assign { target: Register, value: Operand },
    Test { predicate: String, arguments: Vec<Operand> }, Branch(Label), Goto(Operand),
    Save(Register), Restore(Register), Perform { operation: String, arguments: Vec<Operand> } }
struct MachineProgram { registers: Vec<Register>, instructions: Vec<(Option<Label>, Instruction)> }
```

`Query` unification extends substitutions with the occurs-check policy stated by each query-engine exercise and collects answer frames in `Vec` order; a `HashMap` is only an index and never determines answer order. `Instruction` steps update a typed register file, program counter, and explicit stack; an unbound label/register, invalid restore, or undefined operation is a typed machine error (grammar §7). Heap addresses are checked `usize` indexes into `Vec`-backed storage with `Word` enums and work lists; no raw pointers, `unsafe`, or hidden collector is claimed (grammar §9). `HashMap` iteration order never determines answer or output order; instructional order travels in an explicit `Vec`. The explicit-control evaluator is a guest data structure and transition function, not a built-in facility. Compile output is data compared by observable result and effect order; exercise 5.50 runs the guest evaluator through the compiler, 5.51 keeps its C translation as a separate C target built by the external harness, and 5.52 emits C from the typed compiler representation without claiming C syntax or process execution as Rust features (grammar §9).

Self-interpretation requirement. A later full self-evaluator test must execute a translated evaluator source containing every additional construct the edition claims to support. The future teaching evaluator must parse and type-check that evaluator source as Rust, execute it as guest code on a translated guest program, and compare the observable result byte-for-byte with direct native execution. Calling a native helper that already evaluates the guest program does not satisfy this requirement (grammar §§1, 8).

## Part 5. Crate layout in rust/

```text
rust/
  Cargo.toml            virtual workspace, members = ["crates/*"], resolver = "3"
  crates/
    sicp-runtime/       host-ordinary shared types (List, Number tower, chapter-local errors)
    ch01/ ... ch05/     one library crate per chapter; sections are modules
  crates/chNN/
    src/lib.rs          pub mod sec_X_Y
    src/sec_X_Y.rs      solution code (documented, public)
    examples/*.rs       book listings, one file per listing
    tests/sec_X_Y.rs    exercise tests, statements in doc comments
    tests/snapshots/    insta golden files
```

Guest-subset types (`Expr`, `Program`, `Value`, `Fault`, `Store`/`Frame` arena, `Term`/`Query`/`Substitution`, `Register`/`Label`/`Operand`/`Instruction`/`MachineProgram`, `Thunk`/`Force` and `Choose`/`Fail`/`Success` experiment data) follow the semantic contracts in Part 4 and the normative shapes in `spec/host-subsets/rust/grammar.md` §§7-8. The layout above names crate homes only; it does not publish guest APIs ahead of the Phase 2 migration.
Reasons: one crate per chapter, not per section, because the sections of a chapter share host runtime types heavily (host `List` and `Number` tower across chapter 2; the 5.4 and 5.5 engines both drive the 5.2 machine data), so 22 crates would multiply build orchestration for no isolation; the per-section module gives the same file-level organization. Guest engines share only the checked guest representation defined by the grammar (same typed source for direct, analyzed, explicit-control, and generated code per grammar §1); no dynamic host cell flows from chapter 2 into the guest evaluator. Shared host runtime types live in `sicp-runtime`, containing only what at least two host chapters use; chapter-local machinery (agenda, constraints, machine) stays in its chapter crate. The plan's three source directories map as: `examples/` (book listings) to cargo `examples/`; `exercises/` (statements plus solutions) to `tests/sec_X_Y.rs` with the statement in the doc comment; `solutions/` (shared non-test code) to `src/sec_X_Y.rs`. Workspace-level pins and lints use `[workspace.dependencies]` and `[workspace.lints]` with `[lints] workspace = true` in members (all grounded on https://doc.rust-lang.org/cargo/reference/workspaces.html): thiserror 2.0.20, anyhow 1.0.104, proptest 1.11.0, insta 1.48.0 as dev-dependencies; `missing_docs = "deny"` in `[workspace.lints.rust]`.
## Divergences (re-cuts) introduced

Exercise numbers stay 1:1 except where noted. Every divergence:

| Where | Divergence | Exercise impact |
|---|---|---|
| Chapter 0 | new primer, exercises 0.1-0.5 | additive only |
| 3.2 | environment model re-cut to the ownership and closure-capture model; figures retained, prose rewritten around owned frames and `move` capture (guest uses arena-index frames per grammar §8, not reference-counted cells) | 3.10, 3.11 keep numbers; diagram drawing becomes pointer-identity verification |
| 3.4 | taught with Rust's own mechanisms: borrow checker as first serializer, then `Arc<Mutex>`, `AtomicBool::compare_exchange`, `Condvar`; `parallel-execute` mapped to `thread::scope` plus an `AtomicBool` halt flag for the book's control object | numbers kept; 3.38-3.48 solutions change shape, statements unchanged |
| 5.3 | none; stays a memory simulation | none |
| 1.1 | `new-if` mechanism is Rust's eager argument evaluation | none |
| 1.2 | exact integers are checked `i128`; 1.25/1.26 overflow becomes the demonstration | none |
| 2.2.4 | painters draw to SVG files instead of a graphics terminal | none |
| 2.4 | dispatch is the `OpTable`, not traits; the residual `match` arms beside the table are named | none |
| 3.5 | streams presented beside `Iterator`, with an explicit rule for when each applies | none |
| 4.1 | evaluator parses and type-checks checked host source before any effect and executes the guest `evaluate`/`apply` algorithm over the arena `Store`; calls evaluate left to right; chapter 5 machine is the tail-recursive answer (grammar §§1, 4, 5, 8) | none |
| 4.3 | `search-depth-first/1` over explicit `Choose`/`Fail`/`Success` data with `Vec` order and an explicit rollback trail, selected by an explicit runner; core stays strict with no implicit backtracking (grammar §7) | numbers kept |
| 5.2 | machine is a typed assembler function plus runner over `MachineProgram` values; only validated programs run (grammar §§7, 9) | none |

No other exercise numbering diverges.

### Anchor files

- Section, listing, and exercise mappings: the checked-in inventories (`docs/exercise-map.md`) with Git provenance; policy in `modern-sicp/docs/plan/host-subsets-specification.md` and `modern-sicp/docs/plan/host-subsets-migration.md`. The old `text/original/sicp-pocket.texi` archive is not source authority.
- `rust/Cargo.toml`: to be created, virtual workspace, member glob, dependency and lint tables
- `rust/crates/sicp-runtime/src/lib.rs`: to be created, host-ordinary shared types only; guest `Expr`/`Value`/`Fault`/`Store`, `Term`/`Query`, and `Instruction` shapes follow `spec/host-subsets/rust/grammar.md` §§7-8 and are not published here ahead of Phase 2
- `rust/crates/ch04/src/eval.rs`: to be created, guest `evaluate`/`apply` over the arena `Store` per Part 4 and grammar §8
- `rust/crates/ch05/src/machine.rs`: to be created, typed `Instruction`/`MachineProgram` runner per Part 4 and grammar §§7, 9; no controller-text parser
