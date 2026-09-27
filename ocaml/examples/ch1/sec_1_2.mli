(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** The named definitions behind the listings of section 1.2, grouped by
    subsection. Each value a listing displays is asserted by
    [run_sec_1_2] through [Replay], so the book's result comments are
    true by construction. Listings that call a later definition keep the
    book's presentation order in the text; here every definition precedes
    its use, as OCaml requires. *)

(** The factorials of subsection 1.2.1: the linear recursive process, the
    linear iterative process carried by [fact_iter], and the
    block-structured variant the book's footnote hides inside
    [factorial]. [recursive] is exact through 20 and wraps at 21, the
    boundary the section's margin states. *)
module Factorial : sig
  val recursive : int -> int
  val fact_iter : int -> int -> int -> int
  val iterative : int -> int
  val hidden : int -> int
end

(** The Fibonacci procedures of subsection 1.2.2: the tree-recursive
    [recursive], the linear iteration carried by [fib_iter] whose
    self-call is its whole result, and the [count_change] tree the
    section counts a dollar with. *)
module Fibonacci : sig
  val recursive : int -> int
  val fib_iter : int -> int -> int -> int
  val iterative : int -> int
end

(** Counting change, subsection 1.2.2. [first_denomination] reads the
    number of kinds of coins; [cc] runs the reduction rule; the book's
    interaction is [count_change 100]. *)
module Counting_change : sig
  val first_denomination : int -> int
  val cc : int -> int -> int
  val count_change : int -> int
end

(** Exponentiation, subsection 1.2.4: the linear recursive [expt], the
    linear iterative [expt_iter] behind [expt_iterative], and
    successive squaring in [fast_expt]. [is_even] is the book's
    [even?] over the remainder operator. *)
module Exponentiation : sig
  val square : int -> int
  val is_even : int -> bool
  val expt : int -> int -> int
  val expt_iter : int -> int -> int -> int
  val expt_iterative : int -> int -> int
  val fast_expt : int -> int -> int
end

(** Euclid's Algorithm, subsection 1.2.5. The self-call is the entire
    result, so OCaml runs it in constant space. *)
module Gcd : sig
  val gcd : int -> int -> int
end

(** Testing for primality, subsection 1.2.6: trial division up to the
    square root, then the Fermat test over modular successive squaring.
    [fermat_test] and [fast_prime] take the edition's seeded generator
    as an explicit argument, so a fixed seed reproduces a run. *)
module Primality : sig
  val square : int -> int
  val divides : int -> int -> bool
  val is_even : int -> bool
  val find_divisor : int -> int -> int
  val smallest_divisor : int -> int
  val is_prime : int -> bool
  val expmod : int -> int -> int -> int
  val fermat_test : int -> Sicp_common.Random.t -> bool
  val fast_prime : int -> int -> Sicp_common.Random.t -> bool
end
