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
| 0.7 Shared ownership and interior mutability | `Rc`, `Weak` and cycles (https://doc.rust-lang.org/std/rc/index.html), `Cell`, `RefCell` with `borrow`/`borrow_mut` and dynamic borrow panics (https://doc.rust-lang.org/std/cell/struct.RefCell.html) | The `set!` analog; the type basis of chapters 2 through 4 |
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
| tagged data | `Value::Tagged { tag, data }` | 2.4.2 |
| `put`/`get` dispatch | `OpTable` of `Rc<dyn Fn>` | chosen over traits, reason below |
| message passing | closure returning `Rc<dyn Fn(Msg)>` | 2.1.3, 2.4.3 |
| `set!`, local state | captured `Rc<Cell<T>>`/`Rc<RefCell<T>>` | 3.1 |
| `set-car!`/`set-cdr!` | `RefCell` fields of `ConsCell` | 3.3.1 |
| queues, tables | `Queue` header over cons cells; `HashMap<Key, Value>` | 3.3.2, 3.3.3 |
| wires, agenda | `Wire` with `Cell` signal + action list; `BinaryHeap` agenda | 3.3.4 |
| constraints | `Weak<dyn Constraint>` links | 3.3.5 |
| serializer, mutex | `Arc<Mutex<()>>`-based serializer | 3.4 |
| `delay`, memo-proc | `Lazy<T>` memoized thunk | 3.5 |
| streams | `Stream<T> = Rc<SNode<T>>` with `Lazy` tail | cycles leak by design, see notes |
| environments | `Rc<Env>` chain with `RefCell<HashMap>` | 3.2 re-cut, chapter 4 |
| `eval`/`apply` | functions over `Value`, `Rc<Env>` | part 4 sketch |
| thunks (lazy) | `Value::Thunk(Rc<RefCell<ThunkState>>)` | 4.2 |
| `amb` | explicit backtracking engine with resumable frames | 4.3, chosen over CPS |
| frames, unification | `Frame = HashMap<Key, Value>`, `unify_match` | 4.4 |
| registers, stack, controller | `Machine` type-state, `Inst` enum, `Vec<Value>` stack | 5.1-5.5 |
| memory vectors, GC | `Vec<Word>` semispaces, broken-heart forwarding | 5.3 stays a simulation |
| instruction sequences, `preserving` | `ISeq { needs, modifies, code }` | 5.5 |

Why the `put`/`get` table and not traits (2.4): the section's subject is that dispatch is data extended at runtime, by "installing packages" that may not exist when the caller compiles (exercise 2.73 installs derivative rules; 2.74 looks up per-division record handlers by runtime key; 2.5.3 adds polynomial-over-polynomial arithmetic). Rust traits close the type set at compile time and cannot be extended by later installs or dispatch on dynamically discovered tags. The handlers themselves are `Rc<dyn Fn>` trait objects, which satisfies the style contract's `dyn Trait` rule for heterogeneous collections. Where the book's datum is statically typed (chapters 1-3 examples), plain enums and `match` are used instead.

### Sketches for hard mappings (all comment-free; rationale in prose)

Cons as procedure (2.1.3; the mutable variant at texi 16583-16591 is built the same way around `make-account`):

```rust
type PairProc = Rc<dyn Fn(u8) -> Result<Value, SchemeError>>;
fn cons_proc(x: Value, y: Value) -> PairProc {
    Rc::new(move |m| match m {
        0 => Ok(x.clone()),
        1 => Ok(y.clone()),
        _ => Err(SchemeError::UnknownMessage(m)),
    })
}
fn car_proc(p: &PairProc) -> Result<Value, SchemeError> { p(0) }
```

Church numerals (exercise 2.6; `zero` is grounded as `(define zero (lambda (f) (lambda (x) x)))` in 2.1.3). The hard part is the recursive type: a wrapper struct breaks the cycle a bare `type` alias cannot:

```rust
struct Step(Rc<dyn Fn(&mut i128)>);
struct Church(Rc<dyn Fn(Step) -> Step>);
fn zero() -> Church { Church(Rc::new(|_| Step(Rc::new(|_| ())))) }
fn church_succ(n: &Church) -> Church {
    let n = Church(Rc::clone(&n.0));
    Church(Rc::new(move |f: Step| {
        let g = Rc::clone(&f.0);
        Step(Rc::new(move |x: &mut i128| { g(x); (n.0)(Step(Rc::clone(&g)))(x) }))
    }))
}
```

Persistent list with structural sharing (2.2):

```rust
pub enum List<T> { Nil, Cons(T, Rc<List<T>>) }
impl<T> List<T> {
    pub fn cons(x: T, rest: &Self) -> Self { List::Cons(x, Rc::new(rest.clone())) }
    pub fn car(&self) -> Option<&T> { match self { List::Cons(x, _) => Some(x), _ => None } }
    pub fn cdr(&self) -> Option<&Self> { match self { List::Cons(_, r) => Some(r), _ => None } }
}
```

Rationals with constructor-enforced invariants (2.1.1):

```rust
pub struct Rational { num: i128, den: i128 }
impl Rational {
    pub fn new(num: i128, den: i128) -> Result<Self, SchemeError> {
        let g = gcd(num.abs(), den.abs());
        let (num, den) = (num / g, den / g);
        (den > 0).then(|| Rational { num: den.signum() * num, den: den.abs() })
            .ok_or(SchemeError::DivisionByZero)
    }
}
```

Tagged data and the operation table (2.4.2-2.4.3). `Value` contains `f64` and closures, so it cannot derive `Eq`/`Hash`; dynamic tables key on a separate `Key` wrapper:

```rust
pub enum Key { Sym(Rc<str>), Int(i128), Pair(Box<Key>, Box<Key>), Str(Rc<str>) }
pub type Handler = Rc<dyn Fn(&[Value]) -> Result<Value, SchemeError>>;
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

Mutable pair and aliasing questions (3.3.1; `Rc::ptr_eq` is documented on the `Rc` page, https://doc.rust-lang.org/std/rc/struct.Rc.html):

```rust
pub struct ConsCell { pub car: RefCell<Value>, pub cdr: RefCell<Value> }
pub type Pair = Rc<ConsCell>;
pub fn set_car(p: &Pair, v: Value) { *p.car.borrow_mut() = v; }
pub fn eq_pair(a: &Pair, b: &Pair) -> bool { Rc::ptr_eq(a, b) }
```

Queue over mutable pairs (3.3.2), exactly the book's front and rear pointers:

```rust
pub struct Queue { front: RefCell<Option<Pair>>, rear: RefCell<Option<Pair>> }
impl Queue {
    pub fn insert(&self, v: Value) { let p = cons_cell(v, Value::Nil);
        match &*self.rear.borrow() { Some(r) => { *r.cdr.borrow_mut() = Value::Pair(Rc::clone(p)); },
            None => *self.front.borrow_mut() = Some(Rc::clone(&p)) }
        *self.rear.borrow_mut() = Some(p); }
}
```

Memoized table (3.3.3; `memo-fib` keys are `Value`s, wrapped in `Key`):

```rust
pub struct MemoTable(RefCell<HashMap<Key, Value>>);
impl MemoTable {
    pub fn lookup_insert(&self, k: Key, f: impl FnOnce() -> Value) -> Value {
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

Stream cycle rule, stated as a property: the book's self-referential streams (`fibs` defined in terms of itself, the `solve` integral at 3.5.4) make the tail thunk capture the head node, so the Rust port necessarily forms an `Rc` cycle that leaks by design. The leak is bounded to streams the program still holds and is reclaimed at process exit; it is not a defect to remove, and chapter-4 memoized thunks have the same property.

`amb` as an explicit engine with resumable frames (4.3). A bare vector of alternative values cannot resume `try-again` after a solution is returned, nor restore mutations made on the path; the stack therefore stores resumable continuations:

```rust
pub struct AmbFrame { alts: VecDeque<(Value, Rc<Env>)>, k: Rc<Cont>, trail: Vec<Undo> }
pub struct Amb { stack: RefCell<Vec<AmbFrame>>, solutions: RefCell<Vec<Value>> }
pub type Cont = Rc<dyn Fn(&mut Amb, Value) -> Result<Value, SchemeError>>;
pub fn fail(a: &mut Amb) -> Result<Value, SchemeError> { a.unwind_to_choice() }
```

`fail` unwinds by returning `Err(SchemeError::Backtrack)` up the Rust stack to the nearest `amb` site, which pops the next alternative and replays the saved continuation and undo trail. The driver's `try-again` becomes an iterator over stored solutions that resumes the top frame, matching the book's driver loop instead of re-running from scratch.

Frame and unification (4.4):

```rust
pub type Frame = HashMap<Key, Value>;
pub fn unify(p: &Value, d: &Value, f: &Frame) -> Option<Frame> {
    match (p, d) {
        (Value::Var(v), _) => extend_if_fresh(v, d, f),
        (_, Value::Var(v)) => extend_if_fresh(v, p, f),
        (Value::Pair(a1, d1), Value::Pair(a2, d2)) =>
            unify(d1, d2, &unify(a1, a2, f)?),
        _ => (p == d).then(|| f.clone()),
    }
}
```

`preserving` (5.5.4; the book's algorithm, grounded in 5.5.1's examples):

```rust
pub fn preserving(regs: &[Reg], s1: ISeq, s2: ISeq) -> ISeq {
    match regs {
        [] => ISeq::append(s1, s2),
        [r, rest @ ..] =>
            if s2.needs(r) && s1.modifies(r) { preserving(rest, s1.save(r), s2.restore(r)) }
            else { preserving(rest, s1, s2) }
    }
}
```

Stop-and-copy core (5.3.2; broken hearts and forwarding addresses grounded at texi 33541-33570, scan overtaking free at 33745):

```rust
pub enum Word { Ptr(usize), Int(i128), Sym(usize), Bool(bool), BrokenHeart(usize), Free }
pub fn gc(mem: &mut Memory) {
    let mut scan = 0;
    while scan < mem.free {
        let (car, cdr) = mem.relocate_pair_at(scan);
        mem.to[scan] = car; mem.to[scan + 1] = cdr; scan += 2;
    }
    mem.flip();
}
```

## Part 2. Per-section notes

Section numbers and line anchors use the given section starts (1.1 at 1107 through 5.5 at 34641 of `sicp-pocket.texi`).

**1.1 The elements of programming (1107).** What changes: special forms are Rust syntax; `define` becomes `let`/`fn`; the interpreter transcript becomes compiled example output. Hard spot: `new-if` (exercise 1.6, grounded: Eva's `new-if` and Alyssa's `sqrt-iter` rewrite appear in this section) teaches eager argument evaluation; a Rust `fn new_if(...)` evaluates its arguments exactly the same way, so the infinite loop reproduces faithfully. Representative program: `sqrt` by Newton's method (1.1.7), with `improve`, `good_enough`, and block structure as nested functions. Tailored: 1.1a express `sqrt_iter` as an iterator chain (`iter::successors` plus a take-while on `good_enough`, both grounded on https://doc.rust-lang.org/std/iter/index.html) and compare readability; 1.1b property-test `sqrt` monotonicity and relative error against `f64::sqrt` with proptest.

**1.2 Procedures and the processes they generate (2630).** What changes: Scheme's implicit tail-call optimization becomes explicit loops in the iterative variants; recursion depth is Rust stack. Hard spot: exact-integer growth; `i128` covers factorial to 33 and the 1.22-1.23 prime searches near 1e12, but exercise 1.25's Alyssa-style `expmod` computes `a^(n-1)` for 13-digit `n`, unrepresentable at any fixed width; with checked arithmetic the overflow error is the lesson (see part 3). Representative program: `timed_prime_test` using `Instant::now`/`elapsed` (https://doc.rust-lang.org/std/time/struct.Instant.html). Tailored: 1.2a a noise-aware timing harness: median of 30 runs per prime, table snapshot with insta; 1.2b memoized `count_change` via `HashMap` with a proptest case equating it to the naive version for amounts up to 30.

**1.3 Formulating abstractions with higher-order procedures (4192).** What changes: procedures as arguments and return values become closures with `impl Fn` bounds; `sum`, `pi_sum`, `integral` (1.3.1) translate directly. Hard spot: returning closures that capture parameters, and exercise 1.41/1.43 composition (`double`, `repeated`) where the closure must clone captured data rather than borrow it. Representative program: `sum` as a generic higher-order function plus `fixed_point` and Newton's method (1.3.4). Tailored: 1.3a implement `repeated(f, n)` twice, recursive closure tree versus loop-built `Rc<dyn Fn>`, and compare call depth; 1.3b `smooth` (1.44) as a combinator, property-tested for symmetry of the double-smoothed sine approximations.

**2.1 Introduction to data abstraction (5905).** What changes: constructors and selectors become a `struct` with private fields and methods, which makes the abstraction barrier compile-time rather than conventional. Hard spot: 2.1.3 "what is meant by data" (procedural cons, exercise 2.6 church numerals; `zero` grounded in the section) needs the wrapper-struct trick above. Representative program: `Rational` arithmetic with `print_rat`-style `Display` (2.1.1), then `par1`/`par2` interval divergence (2.1.4). Tailored: 2.1a `Rational` checked operations returning `Result`, with a test pinning the first overflowing operation in a long product; 2.1b measure `par1`/`par2` relative error growth across 10 equivalent formulas for two-resistor networks, snapshot the table.

**2.2 Hierarchical data and the closure property (6781).** What changes: three representations in sequence: `Vec<T>` for flat sequences (2.2.3's map/filter/accumulate become iterator adapters), `List<T>` for cons-cell structure where sharing diagrams matter (2.2.1-2.2.2), and closures plus structs for the picture language (2.2.4). Hard spot: exercise 2.18 reverse and 2.20 variadic procedures on `List<T>`; and painter combinators returning closures that capture frames (`beside`, `below`, `flip_vert`), all `'static` via owned data. Representative program: `count_leaves`/`fringe` on `List`, `queens` backtracking over `Vec`, and `segments_painter` writing SVG (exercise 2.49's anchor format is grounded). Tailored: 2.2a `fringe` as a lazy `Iterator` over `List`, proptest-equated with the eager version; 2.2b `beside(flip_vert(p), p)` SVG snapshot pair asserted symmetric via insta.

**2.3 Symbolic data (9608).** What changes: quotation disappears into constructors; `eq?` on symbols becomes `Rc<str>` comparison; `memq`/`equal?` become `Key` recursion. Hard spot: 2.3.2's list-pattern matching becomes exhaustive `match` on `Expr`, which is a teaching upgrade: the compiler proves the cases total. Sets (2.3.3) as sorted `Vec`, binary-search tree enum; Huffman trees (2.3.4) as an enum with weights. Representative program: `deriv` plus `simplify`, and Huffman `encode`/`decode`. Tailored: 2.3a a tiny `Expr` reader/parser and a proptest round-trip `parse(display(e)) == e`; 2.3b property-test the prefix-freeness of generated Huffman codes.

**2.4 Multiple representations for abstract data (11234).** What changes: this is where the dynamic `Value` runtime enters and stays through chapter 4. Tags are `Value::Tagged`; the dispatch is the `OpTable` (sketch above), not traits, because the section's point is runtime extension. Hard spot: exercise 2.73's "why can `number?` and `same-variable?` not be installed in the table" becomes concrete: those predicates dispatch on the datum itself rather than a tag, and the Rust port shows the residual `match` arms beside the table. Representative program: rectangular and polar packages installed into the table with `apply_generic` on top (2.4.2-2.4.3). Tailored: 2.4a micro-benchmark table dispatch versus an exhaustive enum `match` for the complex operations, snapshot medians; 2.4b `apply_generic`'s missing-handler error path: a typed error listing the operation and tag, tested.

**2.5 Systems with generic operations (12310).** What changes: the tower uses `From`/`TryFrom` conversions for `raise`/`project` (https://doc.rust-lang.org/std/convert/index.html) on top of the same table; ordinary numbers, rationals, reals, complex. Hard spot: coercion traps (exercises 2.81-2.83: raising into sameness, infinite raise loops) map onto the `apply_generic` retry loop; 2.5.3 term lists are sparse `Vec<(u32, Coeff)>` with coefficient arithmetic pulled from the table so polynomials over polynomials work. Representative program: `add_poly` on sparse term lists, then `div_poly` toward the gcd/reduce exercises (2.94-2.97). Tailored: 2.5a dense/sparse term-list equivalence by proptest over random polynomials; 2.5b a `raise(project(z)) == z` round-trip suite that classifies exactly which tower levels are lossless.

**3.1 Assignment and local state (14039).** What changes: `set!` becomes interior mutability captured by returned closures; `make_withdraw` (grounded at 14252) returns one closure, `make_account` (14292) returns a dispatch over a shared `Rc<Cell<i128>>`. Hard spot: exercise 3.7 `make_joint` (grounded at 14981) requires two password front-ends sharing one balance, which forces the state into the shared `Rc` rather than each closure. Representative program: `make_account` with password (exercise 3.3 at 14399) and the Monte Carlo estimate of pi from 3.1.2-3.1.3 using a hand-rolled `rand_update` (`u64` xorshift, no external crate). Tailored: 3.1a `rand_update` as a pure function; snapshot the deterministic stream for a fixed seed; 3.1b compare Cesaro estimates at 100, 1_000, 10_000 trials in a table test with a loose tolerance.

**3.2 The environment model of evaluation (15026). Re-cut:** this section becomes the ownership and closure-capture model. The book's figures (grounded: environment diagrams for `make-withdraw` with `@ifinfo` ASCII art at 15533-15617, exercise 3.10 at 15783, exercise 3.11 at 15955-15996) are retained, and the prose walks the same three questions: where the binding lives, what the procedure object captures, and why `W1` and `W2` share code but not state (grounded at 15727-15746). The Rust answer: frames are `Rc<Env>` nodes, a closure captures a clone of the `Rc`, `move` closures copy the pointer and not the binding. Hard spot: making readers see that `Rc::clone` is the book's "pointer to the environment" made explicit in syntax. Representative program: a three-listing walkthrough of `make_withdraw` showing the captured environment, two independent accounts, and a deliberately non-compiling fourth listing where a closure outlives a borrowed capture. Exercises 3.10 and 3.11 keep their numbers and become predict-then-verify: state the sharing before running, then assert with a debug printer that shows `Rc` pointer identity per frame (this is the re-cut's one forced exercise divergence: diagram drawing becomes pointer-identity verification).

**3.3 Modeling with mutable data (15999).** What changes: everything is `Rc<RefCell<...>>` graphs; aliasing questions (3.12-3.18) become `Rc::ptr_eq` questions, sharper than in Scheme because sharing is visible in the types. Queues (3.3.2) mirror the book's front/rear pair pointers exactly; tables (3.3.3) use `HashMap<Key, Value>` with the pairs-based table as a tailored addition; the circuit simulator (3.3.4, `make-wire` grounded at 17724) uses the agenda sketch above with a `(time, seq)` heap key so propagation order is deterministic; exercise 3.31's immediate action run (grounded at 17953) is a test. Constraints (3.3.5) use `Weak<dyn Constraint>` links to break the connector-constraint cycle (grounded `make-connector` at 18550). Hard spot: agenda determinism and the deliberate `Weak` in the two cyclic graphs. Representative program: the half-adder with probe output snapshots (`sum 0  New-value = 0` grounded at 17911), and the Celsius-Fahrenheit converter (18246). Tailored: 3.3a tortoise-hare cycle detection with a proptest over randomly linked and randomly mutated cons cells; 3.3b the pairs-based `make-table` with `equal?` keys and per-table `Key` equality, tested against the `HashMap` version.

**3.4 Concurrency: time is of the essence (18791). Re-cut:** the section keeps its number and its lesson set (interleavings, serializers, deadlock) but is taught with Rust's own concurrency mechanisms. The ground truth is that ownership is the first serializer: the borrow checker rejects at compile time the data race the book's first example races into, so 3.4.1 begins with "why the compiler stopped you" and then deliberately moves the state into `Arc<Mutex<..>>` to allow the race-shaped program. `parallel-execute` (grounded at 19179) becomes `thread::scope` with spawned closures; the book's footnote (19190-19194) says the Scheme primitive returns a control object that can halt the processes, which `thread::scope` cannot do, so the port maps that handle to a shared `AtomicBool` stop flag that each procedure polls; that mapping is stated in the text. `make-serializer` (19619) is the `Arc<Mutex<()>>` sketch; `test-and-set!` is `compare_exchange` on `AtomicBool` per the atomic module page; exercise 3.47 semaphores use `Condvar`; the serialized `make-account` (19251), `make-account-and-serializer` (19456), `transfer` (19530), and the numbered-accounts deadlock avoidance of exercise 3.48 (grounded at 19780) run as written. Hard spot: `Rc<RefCell>` objects from 3.3 are not `Send` (grounded: the `rc` module page states `Rc` does not implement `Send`; use `Arc`), so 3.4 restates the 3.3 account on `Arc<Mutex<>>`: a deliberate, stated type migration. Representative program: serialized `exchange` demonstrating both the lost-update fix and the two-account deadlock. Tailored: 3.4a exercise 3.39's possible outcomes (101, 121, 100) enumerated deterministically with a seeded interleaving harness over `AtomicBool` yield points; 3.4b a `Condvar::wait_timeout` variant of `acquire` with timeout semantics tested by a holding thread.

**3.5 Streams (19855).** What changes: `cons-stream`/`delay` become `Stream<T>` nodes with `Lazy` tails (sketch above); `map`/`filter`/`accumulate` exist twice, once on streams and once as iterator adapters, and the text says exactly when each applies. `Iterator` suffices where a single cursor consumes a stream linearly; it cannot replace `Stream` where (a) two independent cursors must share one memoized tail, (b) 3.5.4 feedback (`solve`, the integral loop) demands a value before it exists, or (c) 4.4's frame streams interleave delayed appends with backtracking. Stream cycles leak by design, as stated in part 1. Hard spot: making the leak rule and the memoization-sharing rule explicit before readers build `fibs`. Representative program: the primes sieve and `sqrt_stream`; 3.5.4's `solve` for `dy/dt = y`, initial 1, step 0.001. Tailored: 3.5a implement primes twice (iterator versus stream) and test the memoization difference: two traversals of one stream do half the work of two `nth` calls on a fresh iterator each, measured with a counter cell; 3.5b the Cesaro stream from 3.5.5 driving a convergence table snapshot.

**4.1 The metacircular evaluator (22672).** What changes: the evaluator evaluates the book's Scheme subset represented as `Value`; syntax dispatch is an exhaustive `match`; primitive procedures are `Rc<dyn Fn(&[Value]) -> Result<Value, SchemeError>>`. `define!`/`set!` mutate `RefCell<HashMap>` frames; 4.1.6 internal definitions follow the book's sequential-frame treatment; 4.1.7 `analyze` pre-compiles expressions into `Rc<dyn Fn(&Rc<Env>) -> Result<Value, SchemeError>>` execution procedures, exactly the book's closures-produced-once design. Hard spot: deep recursion. The stack bound is stated explicitly: eval runs on a worker thread with `Builder::stack_size` of 256 MiB (default Tier-1 stacks are 2 MiB, per https://doc.rust-lang.org/std/thread/index.html), which only postpones overflow; tail recursion is not eliminated in chapter 4, and the honest escape is deferred to the chapter 5 machine, which is tail-recursive by construction (grounded at 34070-34076). Where a chapter 4 exercise needs unbounded tail calls, a trampoline in `eval_sequence`'s last-expression position is the stated remedy, with its overhead measured in a tailored exercise. Representative program: the driver loop running `(factorial 6)` and a `count-leaves` definition, transcripts snapshotted. Tailored: 4.1a an instrumented eval counting dispatches, comparing 4.1.1 versus 4.1.7 `analyze` on `fib 15`, table snapshot; 4.1b `and`/`or` as native special forms with short-circuit, tested equivalent to nested `if` desugaring.

**4.2 Lazy evaluation (24872).** What changes: `delay-it`/`force-it` (grounded at texi 25222 and the memoized 25267) become `Value::Thunk(Rc<RefCell<ThunkState>>)` with memoized forcing, so exercise 4.27's count-of-forcing question is answered by a probe cell, not by prose. `actual-value` forces at primitive-application boundaries; 4.2.3 streams-as-lazy-lists falls out of the same thunk. Hard spot: interaction of thunks with `set!` (4.27-4.29): side effects inside a thunk must run at most once, which the memoized `ThunkState` guarantees structurally. Representative program: the `(id x)` forcing-count demonstration and a lazy `list` consumed by `ref`. Tailored: 4.2a exercise 4.31's per-parameter strictness annotations with a demand probe counting forces per parameter; 4.2b lazy `fib` versus strict `fib` in the same evaluator, thunk-allocation counts tabled.

**4.3 Nondeterministic computing (25622).** What changes: chosen mechanism is explicit backtracking, not continuation-passing closures; the reason is that self-referential `Rc<dyn Fn>` continuations are painful in Rust while an `Err(Backtrack)` unwind to the nearest `amb` frame is plain code, testable, and semantically equivalent (the book's fail continuation is the engine's saved frame). Because a returned solution does not preserve the Rust stack, the engine stores resumable frames (sketch above): pending alternatives, the continuation, and an undo trail of environment mutations, so `try-again` genuinely resumes. Hard spot: `require` and `an-element-of` must restore state on backtracking; the undo trail is the piece CPS gets for free. Representative program: `multiple_dwelling` with all solutions iterated, and the parse example from 4.3.2. Tailored: 4.3a instrument failures per `amb` site, snapshot the counts for `multiple_dwelling`; 4.3b exercise 4.40-style reordering: measure step counts across orderings of the dwelling constraints, table snapshot.

**4.4 Logic programming (27116).** What changes: assertions and rules are `Value`s; the database is a `Vec<Value>` plus a `HashMap<Key, Vec<usize>>` index (the indexing of 4.4.4); query evaluation produces a stream of `Frame`s over the 3.5 `Stream` type, because `stream-append-delayed`/`interleave-delayed` and the infinite answers of recursive rules require laziness. Unification is the sketch above with the `depends_on` cycle check the book specifies. Hard spot: `negation` and `lisp_value` as singleton-stream filters, and rule-application depth for `lives_near`-style recursions. Representative program: `simple_query` over the micro-employee database, the `append` rule answering both directions, and `unique` (exercise 4.75 territory). Tailored: 4.4a add the symbol index and measure queries before and after, snapshot timings as counts of frames examined; 4.4b an `max_of` aggregation built as a `lisp_value` primitive over collected frames.

**5.1 Designing register machines (30358).** What changes: prose plus the controller language; machines are described in the book's textual language and become executable from 5.2 on. Hard spot: recursion as stack discipline (5.1.4) with the `continue` register; the fib machine is the canonical listing. Representative program: the GCD controller text as a constant. Tailored: 5.1a hand-simulate GCD as a Rust `enum`-state machine mirroring the controller line for line, asserting step counts; 5.1b the factorial machine with push counts annotated per controller line (book 5.5.5-style table).

**5.2 A register-machine simulator (31562).** What changes: the book's `make-machine` is itself message-passing (grounded at 31678); the Rust port is a plain struct with a type-state boundary: assembly is a `MachineBuilder` and `assemble` produces the only type that exposes `run`, so execution before assembly does not compile (the style contract's type-state rule; a third `Running` state is rejected because `run` takes `&mut self` and re-running is legal). The assembler (grounded at 31939: "much like the evaluators of Chapter 4") is a single pass building a `Vec<Inst>` and a label table; the book's `extract-labels` footnote about returning two values with `receive` (grounded at 31962-32029) becomes an ordinary tuple, noted in passing. Monitoring (5.2.4, grounded at 32615) counts instructions, pushes, and maximum depth in `Cell<u64>`. Hard spot: the ops table must not let an operation read the `pc` register (exercise 5.9): ops receive a `RegisterView` that hides `pc`, enforced by the type of the ops table. Representative program: GCD machine with statistics; the recursive fib machine with stack depth report. Tailored: 5.2a duplicate-label detection (exercise 5.8's territory) as a typed assembler error with a test; 5.2b a trace mode printing one line per executed instruction, insta-snapshot of the GCD trace.

**5.3 Storage allocation and garbage collection (32786). Confirmed unchanged:** this stays a memory simulation. Pairs live in two `Vec<Word>` semispaces at base pointers; roots are the machine registers plus the stack; the collector is the book's stop-and-copy with broken hearts and forwarding addresses (grounded at 33541-33570 and 33745). Hard spot: sizing exercises and root scanning; the exercise set is design-heavy and stays prose-plus-simulation. Representative program: allocate three pairs, drop one by mutation, run `gc`, print both semispaces before and after. Tailored: 5.3a proptest: for random allocation/mutation scripts, the multiset of reachable pairs is identical before and after collection; 5.3b a mark-sweep alternative collector reporting fragmentation statistics alongside the copying one.

**5.4 The explicit-control evaluator (33544).** What changes: nothing conceptual; the book's controller (grounded: `eval-dispatch` at 33642, `ev-application` at 33791, `ev-if-decide` at 34161, tail-recursion prose at 34070) is a large constant assembled onto the 5.2 machine, with Chapter 4's runtime supplying the global environment and primitive ops. The driver loop prints `;;; EC-Eval value:` exactly as the book's controller specifies (grounded near 34330). Hard spot: controller size; it lives as its own constant file in the crate. The payoff listing: `(fib 10)` on EC-Eval grows the simulated stack, not the Rust stack, and deep tail recursion runs forever where chapter 4's evaluator could not. Representative program: `factorial` and `fib` on EC-Eval with `print_statistics`, compared against chapter 4 step counts. Tailored: 5.4a generate a stack-depth histogram per program (5.27-style) automatically, snapshot; 5.4b install `cond` directly in the controller and compare with desugaring to nested `if`, asserting equal value and fewer dispatch steps.

**5.5 Compilation (34641).** What changes: `compile` dispatches on the same syntax predicates as 4.1; output is `ISeq` instruction sequences combined by `preserving` (sketch; grounded examples in 5.5.1). Lexical addressing (5.5.6) uses a compile-time environment `Vec<Vec<Symbol>>`; open-coded arithmetic emits primitive `Assign`s. 5.5.7 is modeled explicitly: compiled procedures are values holding an entry address, parameters, and environment; the machine gains a `compiled-apply` entry point that saves `continue`, installs the frame, and jumps to the entry; `compile_and_go` loads compiled code into the EC-Eval machine before the driver loop, and `compile_and_run` (the book's footnote-323 operations) runs a compiled expression directly; both appear as example programs, not only as a standalone machine run. Hard spot: `preserving` bookkeeping and keeping `needs`/`modifies` exact, since a wrong set silently changes semantics. Representative program: compile recursive `factorial`, run via `compile_and_go`, and table its instruction and push counts against interpreted EC-Eval (the book's 5.5.5 comparison). Tailored: 5.5a per-expression compiled-listing snapshots with insta (each expression's `ISeq` display is a golden file); 5.5b a constant-folding pass over `ISeq` with a proptest that folded-then-run equals run-then-value on arithmetic constants.

## Part 3. Edition conventions

- Numbers. Exact integers are `i128` with checked arithmetic returning `SchemeError::Overflow`; inexact numbers are `f64`; `Rational` appears in 2.1.1; the `Number` tower enum appears in 2.5. Where `i128` is not enough, stated exactly: exercise 1.25/1.26 naive `expmod` intermediates (`a^(n-1)` for 13-digit `n`, beyond any fixed width; the checked overflow is the lesson), factorials past 33!, and fibonacci stream values past `n` about 184. The book never prints 100! (a file-wide grep for `100!` and `factorial 100` found no hits). No numeric crates; if that default changes, only the `Number` internals change.
- Error type. One `SchemeError` enum per crate family in `sicp-runtime`, derived with thiserror (`#[error("...")]` messages, `#[from]` for io and parse sources, per https://docs.rs/thiserror/2.0.20/thiserror/); variants include `UnboundVariable`, `NotProcedure`, `WrongArity`, `TypeMismatch`, `DivisionByZero`, `Overflow`, `Parse`, `Backtrack` (4.3 control flow), `UserRaised`. `anyhow::Result` appears only in `examples/` mains (https://docs.rs/anyhow/1.0.104/anyhow/).
- `;Value:` interactions. The texinfo source has none; it prints results as `@i{...}` lines after expressions (14269-14272). Edition choice, stated as such: example programs print results via `Display` for `Value` in Scheme surface syntax, and the expected transcript is pinned with an insta snapshot. In doc comments above exercise tests, the interaction appears as three lines: the definition, the call, and the result, mirroring the `@i{}` layout, for example `(w1 50)` then `50`. No inline `//` transcript comments.
- Tests. `cargo nextest run` is the runner (https://nexte.st/); doctests run separately with `cargo test --doc` because nextest does not support them (stated on its site); insta's config selects nextest as its runner (https://docs.rs/insta/1.48.0/insta/). Assertions are `assert_eq!` on values; proptest covers recurrences and equivalences (https://docs.rs/proptest/1.11.0/proptest/); insta snapshots cover transcripts, SVG output, machine traces, and compiled listings. No `unwrap` outside tests.
- Naming. Book listings are runnable examples, one file per listing, named for the first procedure defined: `cargo run -p ch03 --example make_account`. Exercise solutions live in `chNN/tests/sec_X_Y.rs`, one test function per exercise (`fn e_3_47()`), with the exercise statement quoted in the `///` doc comment. Shared solution code lives in `chNN/src/sec_X_Y.rs` public modules. Tailored additions take the parent exercise number plus a letter (1.25a).
- Comments. `#![deny(missing_docs)]` via workspace lints; doc comments on public items only; no inline `//` prose in implementation code; `#[expect(clippy::...)]` always carries a reason string in the same doc comment.
- Style. `&T`, `&str`, `&[T]` parameters; generics on hot paths; `dyn Trait` for heterogeneous collections; type-state where an invalid transition must not compile (the one use is the 5.2 machine assembly boundary).

## Part 4. Architecture sketches

Chapter 4 evaluator (module `sicp-runtime::value`, `ch04::eval`):

```rust
pub enum Value {
    Int(i128), Real(f64), Bool(bool), Sym(Rc<str>), Str(Rc<str>),
    Nil, Pair(Rc<ConsCell>), Tagged { tag: Rc<str>, data: Box<Value> },
    Primitive { name: Rc<str>, f: Rc<dyn Fn(&[Value]) -> Result<Value, SchemeError>> },
    Closure(Rc<Closure>),
    Thunk(Rc<RefCell<ThunkState>>),
    CompiledProc(Rc<CompiledProc>),
}
pub struct Closure { pub params: Vec<Rc<str>>, pub rest: Option<Rc<str>>,
    pub body: Vec<Value>, pub env: Rc<Env> }
pub struct Env { pub frame: RefCell<HashMap<Rc<str>, Value>>, pub outer: Option<Rc<Env>> }
impl Env { pub fn define(&self, k: Rc<str>, v: Value); pub fn lookup(&self, k: &str) -> Result<Value, SchemeError>; }
pub type Exec = Rc<dyn Fn(&Rc<Env>) -> Result<Value, SchemeError>>;
pub fn eval(expr: &Value, env: &Rc<Env>) -> Result<Value, SchemeError>;
pub fn apply(proc_: &Value, args: Vec<Value>, env: &Rc<Env>) -> Result<Value, SchemeError>;
pub fn analyze(expr: &Value) -> Result<Exec, SchemeError>;
pub fn with_eval_stack<T>(f: impl FnOnce() -> T) -> T;
```

`with_eval_stack` runs the closure on a 256 MiB worker thread via `Builder::stack_size`; the bound is stated as a postponement, with the trampoline and the chapter 5 machine as the real answers to unbounded recursion.

Chapter 5 register-machine simulator (module `ch05::machine`):

```rust
pub struct MachineBuilder { ops: OpRegistry, regs: Vec<Rc<str>> }
pub enum Inst {
    Assign { target: Rc<str>, src: Src },
    Test { cond: Rc<str>, ins: Vec<Src> },
    Branch(Label), Goto(GotoTarget), Save(Rc<str>), Restore(Rc<str>),
    Perform { op: Rc<str>, ins: Vec<Src> },
}
pub enum Src { Const(Value), Reg(Rc<str>), Op { name: Rc<str>, ins: Vec<Src> } }
pub struct Machine { pc: usize, regs: HashMap<Rc<str>, Value>,
    stack: Stack, insts: Vec<Inst>, labels: HashMap<Rc<str>, usize>, ops: OpRegistry }
impl MachineBuilder { pub fn register(&mut self, name: &str) -> &mut Self;
    pub fn assemble(self, text: &str) -> Result<Machine, AsmError> }
impl Machine { pub fn run(&mut self) -> Result<(), SchemeError>;
    pub fn set_register(&mut self, name: &str, v: Value) -> Result<(), SchemeError>;
    pub fn statistics(&self) -> Stats }
pub struct Stack { v: Vec<Value>, pushes: Cell<u64>, max_depth: Cell<usize> }
pub type OpFn = Rc<dyn Fn(&RegisterView, &[Value]) -> Result<Value, SchemeError>>;
```

`RegisterView` hides the `pc` index from operations, which is the compile-time form of exercise 5.9. Only the assembled `Machine` exposes `run`; the builder cannot execute.

Chapter 5 compiler (module `ch05::compile`):

```rust
pub enum Linkage { Return, Next, Goto(Label) }
pub struct ISeq { pub needs: BTreeSet<Reg>, pub modifies: BTreeSet<Reg>, pub code: Vec<Inst> }
impl ISeq { pub fn append(a: ISeq, b: ISeq) -> ISeq; pub fn save(self, r: Reg) -> ISeq; pub fn restore(self, r: Reg) -> ISeq }
pub struct CompileEnv(Vec<Vec<Rc<str>>>);
pub fn compile(e: &Value, target: Reg, link: Linkage, ce: &CompileEnv) -> Result<ISeq, SchemeError>;
pub struct CompiledProc { pub entry: Label, pub params: Vec<Rc<str>>, pub body: ISeq }
pub fn compile_and_go(m: &mut Machine, e: &Value) -> Result<(), SchemeError>;
pub fn compile_and_run(m: &mut Machine, e: &Value) -> Result<Value, SchemeError>;
```

`compile_and_go` saves `continue`, installs the compiled definition, and re-enters the EC-Eval driver loop; `compile_and_run` evaluates a compiled expression directly. Both route through the machine's `compiled-apply` entry, which is the 5.5.7 contract.

## Part 5. Crate layout in rust/

```text
rust/
  Cargo.toml            virtual workspace, members = ["crates/*"], resolver = "3"
  crates/
    sicp-runtime/       Value, Symbol, Key, ConsCell, List, Lazy, Stream, Env, Number, SchemeError
    ch01/ ... ch05/     one library crate per chapter; sections are modules
  crates/chNN/
    src/lib.rs          pub mod sec_X_Y
    src/sec_X_Y.rs      solution code (documented, public)
    examples/*.rs       book listings, one file per listing
    tests/sec_X_Y.rs    exercise tests, statements in doc comments
    tests/snapshots/    insta golden files
```

Reasons: one crate per chapter, not per section, because the sections of a chapter share runtime types heavily (`Value`, `Stream`, and `Env` flow from 2.4 through 4.4; 5.4 and 5.5 both drive the 5.2 machine), so 22 crates would multiply build orchestration for no isolation; the per-section module gives the same file-level organization. Shared runtime types live in `sicp-runtime`, containing only what at least two chapters use; chapter-local machinery (agenda, constraints, machine) stays in its chapter crate. The plan's three source directories map as: `examples/` (book listings) to cargo `examples/`; `exercises/` (statements plus solutions) to `tests/sec_X_Y.rs` with the statement in the doc comment; `solutions/` (shared non-test code) to `src/sec_X_Y.rs`. Workspace-level pins and lints use `[workspace.dependencies]` and `[workspace.lints]` with `[lints] workspace = true` in members (all grounded on https://doc.rust-lang.org/cargo/reference/workspaces.html): thiserror 2.0.20, anyhow 1.0.104, proptest 1.11.0, insta 1.48.0 as dev-dependencies; `missing_docs = "deny"` in `[workspace.lints.rust]`.

## Divergences (re-cuts) introduced

Exercise numbers stay 1:1 except where noted. Every divergence:

| Where | Divergence | Exercise impact |
|---|---|---|
| Chapter 0 | new primer, exercises 0.1-0.5 | additive only |
| 3.2 | environment model re-cut to the ownership and closure-capture model; figures retained, prose rewritten around `Rc<Env>` capture | 3.10, 3.11 keep numbers; diagram drawing becomes pointer-identity verification |
| 3.4 | taught with Rust's own mechanisms: borrow checker as first serializer, then `Arc<Mutex>`, `AtomicBool::compare_exchange`, `Condvar`; `parallel-execute` mapped to `thread::scope` plus an `AtomicBool` halt flag for the book's control object | numbers kept; 3.38-3.48 solutions change shape, statements unchanged |
| 5.3 | none; stays a memory simulation | none |
| 1.1 | `new-if` mechanism is Rust's eager argument evaluation | none |
| 1.2 | exact integers are checked `i128`; 1.25/1.26 overflow becomes the demonstration | none |
| 2.2.4 | painters draw to SVG files instead of a graphics terminal | none |
| 2.4 | dispatch is the `OpTable`, not traits; the residual `match` arms beside the table are named | none |
| 3.5 | streams presented beside `Iterator`, with an explicit rule for when each applies | none |
| 4.1 | recursion depth bounded by a stated 256 MiB worker stack; trampoline and chapter 5 machine named as the real answers | none |
| 4.3 | `amb` via explicit backtracking engine with resumable frames and undo trail, not CPS fail continuations; `try-again` is a solution iterator | numbers kept |
| 5.2 | machine model is a type-state struct (`MachineBuilder` to assembled `Machine`), not message passing | none |

No other exercise numbering diverges.

### Anchor files

- `modern-sicp/sicp-pocket.texi`: source of truth; section anchors 1107 through 34641, `@i{}` interaction format, exercise `@quotation` blocks
- `rust/Cargo.toml`: to be created, virtual workspace, member glob, dependency and lint tables
- `rust/crates/sicp-runtime/src/lib.rs`: to be created, `Value`, `Key`, `ConsCell`, `List`, `Lazy`, `Stream`, `Env`, `SchemeError`
- `rust/crates/ch04/src/eval.rs`: to be created, `eval`/`apply`/`analyze`, part 4 sketch
- `rust/crates/ch05/src/machine.rs`: to be created, type-state simulator, 5.4 controller constant, compiler entry points
