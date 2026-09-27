// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Section 0.4: Product and sum data.

/// A shape: the fields inside one variant are a product (both present at
/// once), and the choice of variant is a sum (exactly one active at a
/// time); [`Shape`] is both together, checked exhaustively by `match`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
    /// A circle of the given radius.
    Circle {
        /// The radius.
        radius: f64,
    },
    /// A rectangle of the given width and height.
    Rectangle {
        /// The width.
        width: f64,
        /// The height.
        height: f64,
    },
}

impl Shape {
    /// The area of the shape.
    #[must_use]
    pub fn area(&self) -> f64 {
        match *self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
        }
    }
}

/// The first even number in `xs`, or [`None`] if every element is odd.
#[must_use]
pub fn first_even(xs: &[i64]) -> Option<i64> {
    xs.iter().copied().find(|n| n % 2 == 0)
}

/// A singly linked list of `i64`, a recursive enum: `Cons` holds another
/// `List` inside itself, so without an indirection the type would need
/// infinite size. `Box<List>` gives the recursive branch a fixed size (one
/// pointer) regardless of how long the list underneath it is.
#[derive(Debug, Clone, PartialEq)]
pub enum List {
    /// An element followed by the rest of the list.
    Cons(i64, Box<List>),
    /// The empty list.
    Nil,
}

/// The sum of every element of `list`.
#[must_use]
pub fn list_sum(list: &List) -> i64 {
    match list {
        List::Cons(value, rest) => value + list_sum(rest),
        List::Nil => 0,
    }
}
