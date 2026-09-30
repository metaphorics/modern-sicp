// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.1: the iterative factorial
//! machine, its diagrams, and its run.

use ch05::sec_5_1::factorial_iterative;
use ch05::sec_5_2::{Fault, Machine, assemble};

mod ex_5_01 {
    //! Exercise 5.1: design a register machine for the iterative
    //! factorial, drawing the data-path and controller diagrams.

    use super::*;

    /// The machine of the exercise, transcribed in
    /// [`ch05::sec_5_1::factorial_iterative`]. The two drawings the
    /// book asks for, in the book's notation:
    ///
    /// ```text
    /// Data paths: three registers, three operation boxes, and the
    /// button the controller pushes for each assignment.
    ///
    ///     counter ------+
    ///                   v
    ///                 (>)------> to the controller   test: counter > n
    ///                   ^
    ///     n ------------+
    ///
    ///     counter, product --> (*) --> product   button p<-c*p
    ///     counter, 1 --------> (+) --> counter   button c<-c+1
    ///
    /// Controller diagram: the loop the buttons are pushed in.
    ///
    ///        product<-1, counter<-1
    ///                 |
    ///                 v
    ///        +-> (counter > n)? --yes--> factorial-done
    ///        |         | no
    ///        +---- [p<-c*p, then c<-c+1]
    /// ```
    fn run(n: i64) -> Result<(Machine, u64), Fault> {
        let mut machine = Machine::new(assemble(&factorial_iterative())?);
        machine.set_register("n", n)?;
        machine.run()?;
        let pushes = machine.stack_statistics().pushes;
        Ok((machine, pushes))
    }

    #[test]
    fn ex_5_01() -> Result<(), Fault> {
        // the machine computes the iterative factorial...
        let (one, one_pushes) = run(1)?;
        assert_eq!(one.get_register("product")?, 1);
        let (six, six_pushes) = run(6)?;
        assert_eq!(six.get_register("product")?, 720);
        // ...iteratively: the stack is never touched.
        assert_eq!(one_pushes, 0);
        assert_eq!(six_pushes, 0);
        Ok(())
    }
}
