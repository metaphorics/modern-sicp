// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 2.2

//! Section 2.2: Hierarchical data and the closure property.
//!
//! Three representations in sequence, per the edition plan: the runtime's
//! persistent [`List`] where the book draws cons-cell diagrams (2.2.1 and
//! 2.2.2, with the recursive [`Nest`] on top of it), flat `Vec` sequences
//! with iterator adapters for the signal-flow programs of 2.2.3, and
//! closures plus structs for the picture language of 2.2.4, whose
//! painters draw line art into an SVG sink.

use std::fmt;
use std::rc::Rc;

use sicp_runtime::SchemeError;

/// The runtime's persistent cons list: the section's list representation,
/// re-exported so the listings can build on it.
pub use sicp_runtime::List;

// ---------------------------------------------------------------------
// 2.2.1 List operations
// ---------------------------------------------------------------------

/// Appends two lists: the book's `append`, built by "consing up" a new
/// spine for `list1` that ends in a value copy of `list2`.
#[must_use]
pub fn append<T: Clone>(list1: &List<T>, list2: &List<T>) -> List<T> {
    match list1 {
        List::Nil => list2.clone(),
        List::Cons(x, rest) => List::cons(x.clone(), &append(rest, list2)),
    }
}

/// Returns the `n`-th (0-indexed) item of a list: the book's `list-ref`,
/// which "cdrs down" the list `n` times and takes the `car`.
///
/// # Errors
/// [`SchemeError::TypeMismatch`] when `n` runs past the end of the list,
/// the analog of the book's attempted `car` of the empty list.
pub fn list_ref<T: Clone>(items: &List<T>, n: usize) -> Result<T, SchemeError> {
    match (items, n) {
        (List::Cons(x, _), 0) => Ok(x.clone()),
        (List::Cons(_, rest), _) => list_ref(rest, n - 1),
        (List::Nil, _) => Err(SchemeError::TypeMismatch(
            "list-ref: index past the end".into(),
        )),
    }
}

/// The number of items in a list: the book's recursive `length`, whose
/// reduction step is "1 plus the length of the `cdr`".
#[must_use]
pub fn length<T>(items: &List<T>) -> usize {
    match items {
        List::Nil => 0,
        List::Cons(_, rest) => 1 + length(rest),
    }
}

/// The number of items in a list, computed by the book's iterative
/// `length-iter`: a loop over the spine with a counter, the form Rust
/// writes directly because the language makes the iteration explicit.
#[must_use]
pub fn length_iter<T>(items: &List<T>) -> usize {
    let mut count = 0;
    let mut cursor = Some(items);
    while let Some(List::Cons(_, rest)) = cursor {
        count += 1;
        cursor = Some(rest);
    }
    count
}

/// Applies a transformation to each element of a list: the book's `map`,
/// the higher-order procedure that cdrs down the list consing up the
/// results. Named `map_list` to keep it apart from `Iterator::map`, the
/// adapter the flat sequences of 2.2.3 use.
#[must_use]
pub fn map_list<T, U: Clone>(f: impl Fn(&T) -> U, items: &List<T>) -> List<U> {
    fn go<T, U: Clone>(f: &impl Fn(&T) -> U, items: &List<T>) -> List<U> {
        match items {
            List::Nil => List::Nil,
            List::Cons(x, rest) => List::cons(f(x), &go(f, rest)),
        }
    }
    go(&f, items)
}

/// Scales each number in a list by a factor: the book's `scale-list`,
/// here in its final form as a `map_list` call, exactly the abstraction
/// the section makes of the direct recursive definition. Multiplication
/// is checked, since a factor can push an element past `i128`.
///
/// # Errors
/// [`SchemeError::Overflow`] when any product leaves the `i128` range.
pub fn scale_list(items: &List<i128>, factor: i128) -> Result<List<i128>, SchemeError> {
    items
        .iter()
        .map(|x| x.checked_mul(factor).ok_or(SchemeError::Overflow))
        .collect::<Result<Vec<_>, _>>()
        .map(|scaled| scaled.into_iter().collect())
}

// ---------------------------------------------------------------------
// 2.2.2 Trees: the recursive structure on top of List
// ---------------------------------------------------------------------

/// A tree in the book's list-structure sense: either a leaf holding a
/// value, or a sublist of subtrees built on the same [`List`] cells the
/// section's sharing diagrams draw. The sublist sits under one `Rc`
/// because `List` stores elements inline; cloning a branch therefore
/// copies the head cell and shares the spine, exactly like cloning a
/// `List`. Exercises 2.27, 2.28, 2.30, 2.31, and 2.35 work directly on
/// this type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Nest<T> {
    /// A single value: a leaf of the tree.
    Leaf(T),
    /// A (possibly empty) list of subtrees: a branch.
    Sub(Rc<List<Nest<T>>>),
}

/// Builds a leaf.
#[must_use]
pub fn leaf<T>(x: T) -> Nest<T> {
    Nest::Leaf(x)
}

/// Builds a branch from a slice of subtrees: the edition's `list` of
/// subtrees.
#[must_use]
pub fn sub<T: Clone>(items: &[Nest<T>]) -> Nest<T> {
    Nest::Sub(Rc::new(items.iter().cloned().collect()))
}

impl<T: fmt::Display> fmt::Display for Nest<T> {
    /// Prints the book's form: a leaf prints as its value, a branch as
    /// its list of subtrees, so `((1 2) 3 4)` displays exactly as the
    /// book's box-and-pointer figures print it.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Nest::Leaf(x) => write!(f, "{x}"),
            Nest::Sub(items) => write!(f, "{items}"),
        }
    }
}

/// Counts the leaves of a tree: the book's `count-leaves`, recursing on
/// both the `car` and the `cdr` where `length` recurses only on the
/// `cdr`. The two base cases are the enum's two variants, and the
/// compiler's exhaustiveness check proves the recursion total, which is
/// the teaching upgrade of the `pair?`/`null?` tests.
#[must_use]
pub fn count_leaves<T>(tree: &Nest<T>) -> u64 {
    match tree {
        Nest::Leaf(_) => 1,
        Nest::Sub(items) => match items.as_ref() {
            List::Nil => 0,
            List::Cons(first, rest) => {
                count_leaves(first) + count_leaves(&Nest::Sub(Rc::clone(rest)))
            }
        },
    }
}

/// Scales every leaf of a tree by a factor, keeping the shape: the
/// book's direct `scale-tree`, the analog of `count-leaves` with a
/// multiplication at the leaves.
///
/// # Errors
/// [`SchemeError::Overflow`] when any product leaves the `i128` range.
pub fn scale_tree(tree: &Nest<i128>, factor: i128) -> Result<Nest<i128>, SchemeError> {
    match tree {
        Nest::Leaf(x) => Ok(Nest::Leaf(
            x.checked_mul(factor).ok_or(SchemeError::Overflow)?,
        )),
        Nest::Sub(items) => {
            let scaled: Result<Vec<_>, _> = items.iter().map(|t| scale_tree(t, factor)).collect();
            Ok(Nest::Sub(Rc::new(scaled?.into_iter().collect())))
        }
    }
}

/// Scales a tree regarded as a sequence of sub-trees: the book's second
/// `scale-tree`, which maps over the branch and recurses only where an
/// element is itself a tree. On [`Nest`] the `match` is the `pair?`
/// test, and the map is a plain iterator map.
///
/// # Errors
/// [`SchemeError::Overflow`] when any product leaves the `i128` range.
pub fn scale_tree_map(tree: &Nest<i128>, factor: i128) -> Result<Nest<i128>, SchemeError> {
    match tree {
        Nest::Leaf(x) => Ok(Nest::Leaf(
            x.checked_mul(factor).ok_or(SchemeError::Overflow)?,
        )),
        Nest::Sub(items) => {
            let mapped: Result<Vec<_>, _> = items
                .iter()
                .map(|sub_tree| match sub_tree {
                    Nest::Sub(_) => scale_tree_map(sub_tree, factor),
                    leaf @ Nest::Leaf(_) => scale_tree(leaf, factor),
                })
                .collect();
            Ok(Nest::Sub(Rc::new(mapped?.into_iter().collect())))
        }
    }
}

// ---------------------------------------------------------------------
// 2.2.3 Sequence operations
// ---------------------------------------------------------------------

/// Filters a sequence: the book's `filter`, keeping the elements that
/// satisfy the predicate, in order.
#[must_use]
pub fn filter<T: Clone>(predicate: impl Fn(&T) -> bool, sequence: &[T]) -> Vec<T> {
    sequence.iter().filter(|x| predicate(x)).cloned().collect()
}

/// The book's `accumulate`, also known as `fold-right`: combines the
/// first element with the result of accumulating the rest. The
/// recursion depth is the sequence length, which is Rust's call stack;
/// the book-scale sequences of this section stay far below the default
/// 2 MiB bound, and the flat `Iterator` adapters used by the pipelines
/// below loop instead of recursing.
#[must_use]
pub fn accumulate<T, A>(op: impl Fn(&T, A) -> A, initial: A, sequence: &[T]) -> A {
    fn go<T, A>(op: &impl Fn(&T, A) -> A, initial: A, sequence: &[T]) -> A {
        match sequence {
            [] => initial,
            [first, rest @ ..] => op(first, go(op, initial, rest)),
        }
    }
    go(&op, initial, sequence)
}

/// The sequence of integers from `low` through `high`: the book's
/// `enumerate-interval`. This edition states the translation outright:
/// an enumeration of consecutive integers is a range expression,
/// `low..=high`, and the pipelines use it directly.
#[must_use]
pub fn enumerate_interval(low: i64, high: i64) -> Vec<i64> {
    (low..=high).collect()
}

/// Maps a procedure over a sequence and appends the resulting sequences:
/// the book's `flatmap`, the `accumulate`-`append`-`map` combination the
/// nested mappings of 2.2.3 are built from.
#[must_use]
pub fn flatmap<T, U>(proc: impl Fn(&T) -> Vec<U>, sequence: &[T]) -> Vec<U> {
    sequence.iter().flat_map(proc).collect()
}

/// Trial-division primality test: the `prime?` of 1.2.6, local to this
/// chapter because chapter crates do not depend on one another.
#[must_use]
pub fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    let mut k = 2_i64;
    while k * k <= n {
        if n % k == 0 {
            return false;
        }
        k += 1;
    }
    true
}

/// The `k`-th Fibonacci number, iteratively: the `fib` of 1.2.2, local
/// to this chapter for the `even-fibs` and `list-fib-squares` pipelines.
///
/// # Errors
/// [`SchemeError::Overflow`] when a Fibonacci number leaves `i128`
/// (first past roughly `k = 87`).
pub fn fib(k: u32) -> Result<i128, SchemeError> {
    let (mut a, mut b) = (0_i128, 1_i128);
    for _ in 0..k {
        let next = a.checked_add(b).ok_or(SchemeError::Overflow)?;
        a = b;
        b = next;
    }
    Ok(a)
}

/// Sums the squares of the odd leaves of a tree, as a signal-flow
/// pipeline: enumerate the leaves, filter the odd ones, map `square`,
/// accumulate with `+` from 0. Each stage is one iterator adapter, so
/// the decomposition the book asks the signal-flow diagram to make
/// visible is visible in the code itself.
///
/// # Errors
/// [`SchemeError::Overflow`] when a square or a sum leaves `i128`.
pub fn sum_odd_squares(tree: &Nest<i128>) -> Result<i128, SchemeError> {
    enumerate_tree(tree)
        .iter()
        .filter(|x| *x % 2 != 0)
        .try_fold(0_i128, |acc, x| {
            let square = x.checked_mul(*x).ok_or(SchemeError::Overflow)?;
            acc.checked_add(square).ok_or(SchemeError::Overflow)
        })
}

/// The even Fibonacci numbers `fib(0)` through `fib(n)`, as a pipeline:
/// the range enumerates, `map` computes, `filter` selects, and
/// collecting into a [`List`] is the `accumulate` with `cons`.
///
/// # Errors
/// [`SchemeError::Overflow`] when a Fibonacci number leaves `i128`.
pub fn even_fibs(n: u32) -> Result<List<i128>, SchemeError> {
    let fibs: Result<Vec<_>, _> = (0..=n).map(fib).collect();
    Ok(fibs?.into_iter().filter(|f| f % 2 == 0).collect())
}

/// The squares of the first `n + 1` Fibonacci numbers: the book's
/// `list-fib-squares`, reusing the stages of `even_fibs` in a new order.
///
/// # Errors
/// [`SchemeError::Overflow`] when a Fibonacci number or its square
/// leaves `i128`.
pub fn list_fib_squares(n: u32) -> Result<List<i128>, SchemeError> {
    let fibs: Result<Vec<_>, _> = (0..=n).map(fib).collect();
    let squares: Result<Vec<_>, _> = fibs?
        .iter()
        .map(|f| f.checked_mul(*f).ok_or(SchemeError::Overflow))
        .collect();
    Ok(squares?.into_iter().collect())
}

/// The product of the squares of the odd elements of a sequence: the
/// book's `product-of-squares-of-odd-elements`.
///
/// # Errors
/// [`SchemeError::Overflow`] when a square or the product leaves `i128`.
pub fn product_of_squares_of_odd_elements(sequence: &[i128]) -> Result<i128, SchemeError> {
    sequence
        .iter()
        .filter(|x| *x % 2 != 0)
        .try_fold(1_i128, |acc, x| {
            let square = x.checked_mul(*x).ok_or(SchemeError::Overflow)?;
            acc.checked_mul(square).ok_or(SchemeError::Overflow)
        })
}

/// A personnel record for the salary example: this edition's record is a
/// plain struct, and the selectors `salary` and `programmer?` are its
/// public fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Record {
    /// Whether the record's person is a programmer.
    pub programmer: bool,
    /// The person's salary.
    pub salary: i128,
}

/// The salary of the highest-paid programmer among the records: the
/// book's `salary-of-highest-paid-programmer`, an accumulation of `max`
/// over the mapped, filtered salaries.
#[must_use]
pub fn salary_of_highest_paid_programmer(records: &[Record]) -> i128 {
    records
        .iter()
        .filter(|record| record.programmer)
        .map(|record| record.salary)
        .fold(0, i128::max)
}

/// The sequence of pairs `(i, j)` with `1 <= j < i <= n` whose sum is
/// prime, each extended with its sum: the book's `prime-sum-pairs`, a
/// nested mapping over two ranges.
#[must_use]
pub fn prime_sum_pairs(n: i64) -> Vec<(i64, i64, i64)> {
    (1..=n)
        .flat_map(|i| (1..i).map(move |j| (i, j)))
        .filter(|(i, j)| is_prime(i + j))
        .map(|(i, j)| (i, j, i + j))
        .collect()
}

/// All the permutations of a sequence: the book's `permutations`, which
/// for each item recursively generates the permutations of the rest and
/// adjoins the item to the front of each one.
#[must_use]
pub fn permutations<T: Clone + PartialEq>(s: &[T]) -> Vec<Vec<T>> {
    if s.is_empty() {
        return vec![Vec::new()];
    }
    let mut out = Vec::new();
    for (i, x) in s.iter().enumerate() {
        let mut rest = s.to_vec();
        rest.remove(i);
        for mut p in permutations(&rest) {
            p.insert(0, x.clone());
            out.push(p);
        }
    }
    out
}

/// Enumerates the leaves of a tree, left to right: the book's
/// `enumerate-tree` (exercise 2.28's `fringe`, renamed to emphasize its
/// place in the sequence-operation family).
#[must_use]
pub fn enumerate_tree<T: Clone>(tree: &Nest<T>) -> List<T> {
    match tree {
        Nest::Leaf(x) => List::cons(x.clone(), &List::Nil),
        Nest::Sub(items) => match items.as_ref() {
            List::Nil => List::Nil,
            List::Cons(first, rest) => append(
                &enumerate_tree(first),
                &enumerate_tree(&Nest::Sub(Rc::clone(rest))),
            ),
        },
    }
}

// ---------------------------------------------------------------------
// 2.2.4 Picture language: vectors, frames, segments
// ---------------------------------------------------------------------

/// A two-dimensional vector: the book's `make-vect` data abstraction of
/// exercise 2.46. The coordinates are `f64` because they live in the
/// unit square, not in the exact-integer world of the number convention.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vect {
    /// The `x`-coordinate: the book's `xcor-vect`.
    pub x: f64,
    /// The `y`-coordinate: the book's `ycor-vect`.
    pub y: f64,
}

impl Vect {
    /// Builds the book's `(make-vect x y)`.
    #[must_use]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl fmt::Display for Vect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::ops::Add for Vect {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub for Vect {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Mul<Vect> for f64 {
    type Output = Vect;

    fn mul(self, v: Vect) -> Vect {
        Vect::new(self * v.x, self * v.y)
    }
}

/// A frame: an origin vector and two edge vectors, the book's
/// `make-frame`, `origin-frame`, `edge1-frame`, and `edge2-frame`
/// abstraction. Exercise 2.47 supplies two alternative constructors
/// over other representations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Frame {
    /// The offset of the frame's origin: `origin-frame`.
    pub origin: Vect,
    /// The first edge vector: `edge1-frame`.
    pub edge1: Vect,
    /// The second edge vector: `edge2-frame`.
    pub edge2: Vect,
}

impl Frame {
    /// Builds the book's `(make-frame origin edge1 edge2)`.
    #[must_use]
    pub fn new(origin: Vect, edge1: Vect, edge2: Vect) -> Self {
        Self {
            origin,
            edge1,
            edge2,
        }
    }

    /// The frame the book's figures draw in: the unit square itself,
    /// used by the SVG renderer and the figure generators.
    #[must_use]
    pub fn unit_square() -> Self {
        Self::new(
            Vect::new(0.0, 0.0),
            Vect::new(1.0, 0.0),
            Vect::new(0.0, 1.0),
        )
    }
}

/// A directed line segment from `start` to `end`: the book's
/// `make-segment` abstraction of exercise 2.48, which the primitive
/// painters are built from.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Segment {
    /// The start point: `start-segment`.
    pub start: Vect,
    /// The end point: `end-segment`.
    pub end: Vect,
}

impl Segment {
    /// Builds the book's `(make-segment start end)`.
    #[must_use]
    pub fn new(start: Vect, end: Vect) -> Self {
        Self { start, end }
    }
}

/// The frame coordinate map: maps a vector in the unit square to the
/// corresponding vector in the frame, `Origin + x*Edge1 + y*Edge2`. The
/// book builds the same function from `add-vect` and `scale-vect`; this
/// edition's arithmetic operators are those procedures.
pub fn frame_coord_map(frame: &Frame) -> impl Fn(Vect) -> Vect + use<'_> {
    let origin = frame.origin;
    let edge1 = frame.edge1;
    let edge2 = frame.edge2;
    move |v| origin + v.x * edge1 + v.y * edge2
}

/// Receives the line segments a painter draws: the edition stands in for
/// the book's graphics terminal with this sink, so a painter is a
/// function from a frame to a sequence of `line` calls.
pub trait Sink {
    /// Draws one line segment, in the frame's coordinates.
    fn line(&mut self, a: Vect, b: Vect);
}

/// A painter: a function from a frame to the segments it draws into a
/// sink. This is the book's procedural representation of painters,
/// `Rc<dyn Fn>` because combinators hold several painters and clone the
/// handle, never the drawing.
pub type Painter = Rc<dyn Fn(&Frame, &mut dyn Sink)>;

/// Builds a painter from a list of segments in unit-square coordinates:
/// the book's `segments->painter`. Each segment's endpoints go through
/// the frame coordinate map, then to the sink.
#[must_use]
pub fn segments_painter(segments: Vec<Segment>) -> Painter {
    Rc::new(move |frame: &Frame, sink: &mut dyn Sink| {
        let m = frame_coord_map(frame);
        for segment in &segments {
            sink.line(m(segment.start), m(segment.end));
        }
    })
}

/// The section's given `wave` painter: the hand-drawn segment list the
/// book takes as given (its construction from `segments->painter` is
/// exercise 2.49d).
#[must_use]
pub fn wave() -> Painter {
    segments_painter(wave_segments())
}

/// The segment list of the [`wave`] painter.
#[must_use]
pub fn wave_segments() -> Vec<Segment> {
    let v = Vect::new;
    let s = |(x1, y1): (f64, f64), (x2, y2): (f64, f64)| Segment::new(v(x1, y1), v(x2, y2));
    vec![
        s((0.00, 0.85), (0.12, 0.62)), // left side of the head
        s((0.12, 0.62), (0.30, 0.68)), // up to the crown
        s((0.30, 0.68), (0.42, 0.72)), // crown peak, left half
        s((0.42, 0.72), (0.52, 0.70)), // crown peak, right half
        s((0.52, 0.70), (0.60, 0.60)), // down to the neck
        s((0.60, 0.60), (0.65, 0.45)), // right shoulder
        s((0.65, 0.45), (1.00, 0.40)), // right arm out
        s((0.30, 0.68), (0.25, 0.55)), // left shoulder
        s((0.25, 0.55), (0.00, 0.65)), // left arm out
        s((0.60, 0.60), (0.72, 0.62)), // right hand back up
        s((0.72, 0.62), (0.85, 0.55)), // right hand wave
        s((0.40, 0.45), (0.42, 0.20)), // left leg
        s((0.42, 0.20), (0.35, 0.00)), // left foot
        s((0.40, 0.45), (0.55, 0.45)), // hips
        s((0.55, 0.45), (0.60, 0.20)), // right leg
        s((0.60, 0.20), (0.70, 0.00)), // right foot
        s((0.32, 0.62), (0.28, 0.55)), // left hand
        s((0.36, 0.52), (0.46, 0.52)), // belt
    ]
}

/// Transforms a painter by giving it a new frame: the book's
/// `transform-painter`. The three vectors are corners of the new frame
/// in the unit square of the frame the transformed painter receives.
#[must_use]
pub fn transform_painter(painter: &Painter, origin: Vect, corner1: Vect, corner2: Vect) -> Painter {
    let painter = Rc::clone(painter);
    Rc::new(move |frame: &Frame, sink: &mut dyn Sink| {
        let m = frame_coord_map(frame);
        let new_origin = m(origin);
        let new_frame = Frame::new(new_origin, m(corner1) - new_origin, m(corner2) - new_origin);
        painter(&new_frame, sink);
    })
}

/// Flips a painter's image upside down: the book's `flip-vert`, the
/// worked example of `transform-painter`.
#[must_use]
pub fn flip_vert(painter: &Painter) -> Painter {
    transform_painter(
        painter,
        Vect::new(0.0, 1.0),
        Vect::new(1.0, 1.0),
        Vect::new(0.0, 0.0),
    )
}

/// Shrinks a painter's image to the upper-right quarter of the frame.
#[must_use]
pub fn shrink_to_upper_right(painter: &Painter) -> Painter {
    transform_painter(
        painter,
        Vect::new(0.5, 0.5),
        Vect::new(1.0, 0.5),
        Vect::new(0.5, 1.0),
    )
}

/// Rotates a painter counterclockwise by 90 degrees: the book's
/// `rotate90`, a pure rotation only for square frames.
#[must_use]
pub fn rotate_90(painter: &Painter) -> Painter {
    transform_painter(
        painter,
        Vect::new(1.0, 0.0),
        Vect::new(1.0, 1.0),
        Vect::new(0.0, 0.0),
    )
}

/// Squashes a painter's image towards the center of the frame: the
/// transform behind the diamond-shaped images of the book's figures.
#[must_use]
pub fn squash_inwards(painter: &Painter) -> Painter {
    transform_painter(
        painter,
        Vect::new(0.0, 0.0),
        Vect::new(0.65, 0.35),
        Vect::new(0.35, 0.65),
    )
}

/// Draws two painters side by side: the book's `beside`, the first in
/// the left half of the frame, the second in the right half.
#[must_use]
pub fn beside(painter1: &Painter, painter2: &Painter) -> Painter {
    let left = transform_painter(
        painter1,
        Vect::new(0.0, 0.0),
        Vect::new(0.5, 0.0),
        Vect::new(0.0, 1.0),
    );
    let right = transform_painter(
        painter2,
        Vect::new(0.5, 0.0),
        Vect::new(1.0, 0.0),
        Vect::new(0.5, 1.0),
    );
    Rc::new(move |frame: &Frame, sink: &mut dyn Sink| {
        left(frame, sink);
        right(frame, sink);
    })
}

/// Draws two painters stacked: the first in the bottom half of the
/// frame, the second in the top. The direct construction, analogous to
/// [`beside`]; exercise 2.51 builds it a second way, out of rotations,
/// and 2.51a checks the two agree.
#[must_use]
pub fn below(painter1: &Painter, painter2: &Painter) -> Painter {
    let bottom = transform_painter(
        painter1,
        Vect::new(0.0, 0.0),
        Vect::new(1.0, 0.0),
        Vect::new(0.0, 0.5),
    );
    let top = transform_painter(
        painter2,
        Vect::new(0.0, 0.5),
        Vect::new(1.0, 0.5),
        Vect::new(0.0, 1.0),
    );
    Rc::new(move |frame: &Frame, sink: &mut dyn Sink| {
        bottom(frame, sink);
        top(frame, sink);
    })
}

/// The edition's `n`-branching `right-split`: paints the image in the
/// upper left, and a `right-split` of order `n - 1` twice, side by side,
/// in the remaining rectangle. The book's recursive plan, with the
/// recursion depth bounded by `n` on the Rust stack.
#[must_use]
pub fn right_split(painter: &Painter, n: u32) -> Painter {
    if n == 0 {
        return Rc::clone(painter);
    }
    let smaller = right_split(painter, n - 1);
    beside(painter, &below(&smaller, &smaller))
}

/// The `up-split` of exercise 2.44: the mirror of [`right_split`] that
/// branches upward. Shipped here because the book's own `corner-split`
/// listing below is defined in terms of it; exercise 2.44's solution
/// rebuilds it independently and proves the two agree.
#[must_use]
pub fn up_split(painter: &Painter, n: u32) -> Painter {
    if n == 0 {
        return Rc::clone(painter);
    }
    let smaller = up_split(painter, n - 1);
    below(painter, &beside(&smaller, &smaller))
}

/// Places an `up-split` and the painter side by side over a painter and
/// a `right-split`, recursing into the corner: the book's
/// `corner-split`, which branches up and to the right at once.
#[must_use]
pub fn corner_split(painter: &Painter, n: u32) -> Painter {
    if n == 0 {
        return Rc::clone(painter);
    }
    let top_left = beside(&up_split(painter, n - 1), painter);
    let bottom_right = below(painter, &right_split(painter, n - 1));
    let corner = corner_split(painter, n - 1);
    beside(&below(painter, &top_left), &below(&bottom_right, &corner))
}

/// A painter operation: a function from a painter to a painter, the
/// higher-order element the `square-of-four` abstraction manipulates.
pub type PainterOp = Rc<dyn Fn(&Painter) -> Painter>;

/// Lifts an ordinary painter-to-painter function into a [`PainterOp`],
/// the `Rc` handle `square_of_four` stores.
#[must_use]
pub fn painter_op(f: impl Fn(&Painter) -> Painter + 'static) -> PainterOp {
    Rc::new(f)
}

/// The identity painter operation: leaves a painter unchanged.
#[must_use]
pub fn identity_op() -> PainterOp {
    painter_op(|p| Rc::clone(p))
}

/// Rotates a painter by 180 degrees, composed out of the two flips
/// exactly as the book's footnote suggests (`compose flip-vert
/// flip-horiz`); exercise 2.50 builds it directly as a transform and
/// proves the two agree.
#[must_use]
pub fn rotate_180(painter: &Painter) -> Painter {
    flip_vert(&flip_horiz(painter))
}

/// Flips a painter's image left-to-right: the `flip-horiz` of exercise
/// 2.50, shipped here because the book's `square-limit` listing uses
/// it. Exercise 2.50's solution constructs it independently.
#[must_use]
pub fn flip_horiz(painter: &Painter) -> Painter {
    transform_painter(
        painter,
        Vect::new(1.0, 0.0),
        Vect::new(0.0, 0.0),
        Vect::new(1.0, 1.0),
    )
}

/// Arranges four copies of a painter in a square, transformed by the
/// four given operations: the book's `square-of-four`, the
/// higher-order means of combination over painter operations.
#[must_use]
pub fn square_of_four(tl: PainterOp, tr: PainterOp, bl: PainterOp, br: PainterOp) -> PainterOp {
    Rc::new(move |painter: &Painter| {
        let top = beside(&tl(painter), &tr(painter));
        let bottom = beside(&bl(painter), &br(painter));
        below(&bottom, &top)
    })
}

/// Four copies of a painter, each pair flipped vertically: the book's
/// `flipped-pairs`, built on [`square_of_four`].
#[must_use]
pub fn flipped_pairs(painter: &Painter) -> Painter {
    (square_of_four(
        identity_op(),
        painter_op(flip_vert),
        identity_op(),
        painter_op(flip_vert),
    ))(painter)
}

/// The square limit of a painter: the book's `square-limit`, four
/// `corner-split` copies arranged by [`square_of_four`].
#[must_use]
pub fn square_limit(painter: &Painter, n: u32) -> Painter {
    (square_of_four(
        painter_op(flip_horiz),
        identity_op(),
        painter_op(rotate_180),
        painter_op(flip_vert),
    ))(&corner_split(painter, n))
}

// ---------------------------------------------------------------------
// Sinks: the capture sink for tests, the SVG sink for figures
// ---------------------------------------------------------------------

/// Collects the segments a painter draws instead of rendering them, so
/// tests and exercises can compare painter geometry exactly.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CaptureSink {
    lines: Vec<(Vect, Vect)>,
}

impl CaptureSink {
    /// An empty sink.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The captured segments, in draw order.
    #[must_use]
    pub fn lines(&self) -> &[(Vect, Vect)] {
        &self.lines
    }

    /// The captured segments as one display line, `a -> b` per segment,
    /// the form the exercise tests pin geometry with.
    #[must_use]
    pub fn to_text(&self) -> String {
        self.lines
            .iter()
            .map(|(a, b)| format!("{a} -> {b}"))
            .collect::<Vec<_>>()
            .join("; ")
    }
}

impl Sink for CaptureSink {
    fn line(&mut self, a: Vect, b: Vect) {
        self.lines.push((a, b));
    }
}

/// Renders line segments into SVG text: the edition stands in for the
/// book's graphics terminal. Frame coordinates land in the unit square,
/// the `y` axis flips (SVG grows downward), and the output is plain,
/// deterministic markup so the checked-in figures under
/// `book/figures/generated/` never drift between runs.
#[derive(Debug)]
pub struct SvgSink {
    size: f64,
    lines: Vec<String>,
}

impl SvgSink {
    /// A sink for a `size x size` canvas.
    #[must_use]
    pub fn new(size: u32) -> Self {
        Self {
            size: f64::from(size),
            lines: Vec::new(),
        }
    }

    /// The complete SVG document as text.
    #[must_use]
    pub fn svg(&self) -> String {
        use std::fmt::Write as _;

        let size = self.size;
        let mut out = String::new();
        let _ = writeln!(
            out,
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {size:.0} {size:.0}\" \
             width=\"{size:.0}\" height=\"{size:.0}\""
        );
        let _ = writeln!(
            out,
            "<rect width=\"{size:.0}\" height=\"{size:.0}\" fill=\"white\"/>"
        );
        out.push_str(
            "<g stroke=\"#1a1a1a\" stroke-width=\"1.2\" stroke-linecap=\"round\" fill=\"none\">\n",
        );
        for line in &self.lines {
            out.push_str(line);
            out.push('\n');
        }
        out.push_str("</g>\n</svg>\n");
        out
    }

    fn px(&self, value: f64) -> String {
        format!("{:.2}", value * self.size)
    }
}

impl Sink for SvgSink {
    fn line(&mut self, a: Vect, b: Vect) {
        // The frame map lands points in the unit square; the SVG `y`
        // axis points down, so the flip happens here, once.
        let (x1, y1) = (self.px(a.x), self.px(1.0 - a.y));
        let (x2, y2) = (self.px(b.x), self.px(1.0 - b.y));
        self.lines.push(format!(
            "<line x1=\"{x1}\" y1=\"{y1}\" x2=\"{x2}\" y2=\"{y2}\"/>"
        ));
    }
}

/// Paints a painter into a `size x size` unit-square frame and returns
/// the SVG document: the convenience the figure generators and the
/// exercise tests render with.
#[must_use]
pub fn render_svg(painter: &Painter, size: u32) -> String {
    let mut sink = SvgSink::new(size);
    painter(&Frame::unit_square(), &mut sink);
    sink.svg()
}
