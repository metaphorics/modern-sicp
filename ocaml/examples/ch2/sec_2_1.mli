(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** The named definitions behind the listings of section 2.1, grouped by
    subsection. Each value a listing displays is asserted by
    [run_sec_2_1] through [Replay], so the book's result comments are
    true by construction.

    The section's hard spot is order: the book states [add_rat] and
    friends before it says how [make_rat], [numer], and [denom] are
    implemented, pure wishful thinking that a Scheme file tolerates
    because nothing calls [add-rat] until every [define] above it has
    already run. OCaml resolves each name where it is written, so a
    file cannot state the wish before granting it. This edition keeps
    the wish first anyway, as a module type ([Rational_number]) with
    no implementation attached; [Rational_arithmetic] is a functor
    that turns any module satisfying that wish into the five
    operations, written exactly once. The book's other claim, that
    changing the representation ``does not have to modify'' [add_rat]
    at all, is then not just asserted but checked by the compiler:
    [Rational_arithmetic] is applied to three different
    representations below without changing one line of its body. *)

(** Subsection 2.1.1, first half: pairs, via OCaml's built-in tuple.
    [cons]/[car]/[cdr] become tuple literals and [fst]/[snd]; nesting a
    pair inside a pair needs no new mechanism. *)
module Pairs : sig
  val x : int * int

  (** [car_x] and [cdr_x] are [fst x] and [snd x]. *)
  val car_x : int

  val cdr_x : int

  (** [nested_z] is [(cons (cons 1 2) (cons 3 4))]: a pair of pairs. *)
  val nested_z : (int * int) * (int * int)

  (** [car_car_z] and [car_cdr_z] are [(car (car z))] and [(car (cdr z))]. *)
  val car_car_z : int

  val car_cdr_z : int
end

(** The wish of subsection 2.1.1: a rational number is whatever three
    procedures [make_rat], [numer], and [denom] agree it is. Every
    representation below satisfies this signature; none of them is
    named in [Rational_arithmetic]'s body. *)
module type Rational_number = sig
  type t

  val make_rat : int -> int -> t
  val numer : t -> int
  val denom : t -> int
end

(** The five operations of subsection 2.1.1, stated once against the
    wish above and reused unchanged against every representation this
    section builds. *)
module Rational_arithmetic (R : Rational_number) : sig
  val add_rat : R.t -> R.t -> R.t
  val sub_rat : R.t -> R.t -> R.t
  val mul_rat : R.t -> R.t -> R.t
  val div_rat : R.t -> R.t -> R.t
  val equal_rat : R.t -> R.t -> bool

  (** [print_rat x] prints [numer x], a slash, and [denom x], followed
      by a newline. *)
  val print_rat : R.t -> unit
end

(** The book's first representation: a pair, reduced to nothing. *)
module Unreduced : Rational_number

module Unreduced_ops : sig
  val add_rat : Unreduced.t -> Unreduced.t -> Unreduced.t
  val sub_rat : Unreduced.t -> Unreduced.t -> Unreduced.t
  val mul_rat : Unreduced.t -> Unreduced.t -> Unreduced.t
  val div_rat : Unreduced.t -> Unreduced.t -> Unreduced.t
  val equal_rat : Unreduced.t -> Unreduced.t -> bool
  val print_rat : Unreduced.t -> unit
end

(** The book's second representation: [make_rat] divides out the
    @ref{1.2.5} [gcd] before building the pair, so [numer]/[denom]
    need no change. This edition takes the [gcd] of absolute values
    first; @ref{1.2.5}'s own [gcd], read literally on a negative
    argument, is not guaranteed non-negative the way Scheme's built-in
    [gcd] is, and a negative divisor would flip a sign [make_rat] never
    promised to touch. *)
module Reduced : Rational_number

module Reduced_ops : sig
  val add_rat : Reduced.t -> Reduced.t -> Reduced.t
  val sub_rat : Reduced.t -> Reduced.t -> Reduced.t
  val mul_rat : Reduced.t -> Reduced.t -> Reduced.t
  val div_rat : Reduced.t -> Reduced.t -> Reduced.t
  val equal_rat : Reduced.t -> Reduced.t -> bool
  val print_rat : Reduced.t -> unit
end

(** Subsection 2.1.2's closed error: the one way this section's
    [Rational.make] can fail. *)
module Rational_error : sig
  type t = Zero_denominator of { n : int } (** the rejected numerator, denominator 0 *)

  val to_string : t -> string
end

(** Subsection 2.1.2's sealed representation, the section's
    abstraction barrier drawn as an actual module boundary: [t] is
    abstract outside this signature, and [make] is the section's
    representative program, refusing a zero denominator instead of
    building a pair no [numer]/[denom] call can make sense of. *)
module Rational : sig
  type t

  val make : int -> int -> (t, Rational_error.t) result
  val numer : t -> int
  val denom : t -> int
end

(** Subsection 2.1.2's alternate representation: [make_rat] builds the
    raw pair and [numer]/[denom] divide out the [gcd] at access time
    instead of at construction time. Applying [Rational_arithmetic]
    here a third time, unchanged, is this edition's check on the
    book's claim that the choice of when to reduce is confined to
    [make_rat], [numer], and [denom] alone. *)
module Lazy_reduced : Rational_number

module Lazy_reduced_ops : sig
  val add_rat : Lazy_reduced.t -> Lazy_reduced.t -> Lazy_reduced.t
  val sub_rat : Lazy_reduced.t -> Lazy_reduced.t -> Lazy_reduced.t
  val mul_rat : Lazy_reduced.t -> Lazy_reduced.t -> Lazy_reduced.t
  val div_rat : Lazy_reduced.t -> Lazy_reduced.t -> Lazy_reduced.t
  val equal_rat : Lazy_reduced.t -> Lazy_reduced.t -> bool
  val print_rat : Lazy_reduced.t -> unit
end

(** Subsection 2.1.3: pairs represented by nothing but a dispatch
    function. Unlike Scheme's [cons], which can pair values of two
    different types because its dispatch is untyped, [cons] here
    requires [x] and [y] to share one type: [dispatch] must return the
    same type on either branch. Every pair this section actually
    builds already satisfies that (a rational's numerator and
    denominator are both [int]), so the restriction costs nothing
    here, but this [cons] cannot stand in for OCaml's built-in tuple
    in general. *)
module Procedural_pairs : sig
  (** [cons x y] is a function that, applied to [0], yields [x], to
      [1], yields [y], and raises [Invalid_argument] on any other
      input, matching the book's own [(error "Argument not 0 or 1:
      CONS" m)]. *)
  val cons : 'a -> 'a -> int -> 'a

  val car : (int -> 'a) -> 'a
  val cdr : (int -> 'a) -> 'a
end

(** The wish of subsection 2.1.4: an interval is whatever
    [make_interval], [lower_bound], and [upper_bound] agree it is.
    Exercise 2.7 supplies the first representation. *)
module type Interval_number = sig
  type t

  val make_interval : float -> float -> t
  val lower_bound : t -> float
  val upper_bound : t -> float
end

(** Alyssa's three operations, stated once against the wish above. *)
module Interval_arithmetic (I : Interval_number) : sig
  val add_interval : I.t -> I.t -> I.t
  val mul_interval : I.t -> I.t -> I.t
  val div_interval : I.t -> I.t -> I.t
end

(** The center-width alternate constructor and selectors that close
    subsection 2.1.4's narrative, ahead of Exercise 2.12's
    center-percent extension. *)
module Interval_center_width (I : Interval_number) : sig
  val make_center_width : float -> float -> I.t
  val center : I.t -> float
  val width : I.t -> float
end

(** Lem E. Tweakit's two algebraically equivalent parallel-resistance
    programs, the setup for exercises 2.14 through 2.16. *)
module Parallel_resistors (I : Interval_number) : sig
  val par1 : I.t -> I.t -> I.t
  val par2 : I.t -> I.t -> I.t
end
