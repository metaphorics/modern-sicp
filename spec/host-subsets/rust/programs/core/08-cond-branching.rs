// SPDX-License-Identifier: GPL-3.0-only
// Case: core/08-cond-branching. Provenance: derived from teaching execution;
// parent confirms natively. Never native-proven.

/// The lesson of the section's branching: a closed enum and an
/// exhaustive match replace the cond chain.
enum Shape {
    Circle(i64),
    Square(i64),
    Rectangle(i64, i64),
}

fn classify(shape: Shape) -> i64 {
    match shape {
        Shape::Circle(radius) => 3 * radius * radius,
        Shape::Square(side) => side * side,
        Shape::Rectangle(width, height) => width * height,
    }
}

fn main() {
    println!("{}", classify(Shape::Circle(2)));
    println!("{}", classify(Shape::Square(3)));
    println!("{}", classify(Shape::Rectangle(2, 4)));
}
