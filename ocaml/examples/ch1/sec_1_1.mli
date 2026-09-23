(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** The named definitions behind the listings of section 1.1, grouped by
    subsection. Each value a listing displays is asserted by
    [run_sec_1_1] through [Replay], so the book's result comments are
    true by construction. *)

(** The interactions of subsection 1.1.1: arithmetic expressions over
    [int] and [float], nested combination trees, and the deep expression
    the book pretty-prints. *)
module Expressions : sig
  val forty_eight_six : int
  val sum : int
  val difference : int
  val product : int
  val quotient : int
  val mixed : float
  val chained_sum : int
  val chained_product : int
  val nested : int
  val essential_parens : int
  val deep : int
end

(** The named values of subsection 1.1.2: [size], then the circle
    quantities computed across the [int]/[float] boundary with an
    explicit [Float.of_int]. *)
module Naming : sig
  val size : int
  val five_times_size : int
  val pi : float
  val radius : int
  val area : float
  val circumference : float
end

(** The compound procedures of subsection 1.1.4: [square] as a building
    block under [sum_of_squares] and [f]. *)
module Compound : sig
  val square : int -> int
  val sum_of_squares : int -> int -> int
  val f : int -> int
end

(** The conditional forms of subsection 1.1.6: [abs] as a case analysis
    spelled three ways, and the two equivalent [greater_or_equal]
    definitions. *)
module Conditionals : sig
  val abs_cases : int -> int
  val abs_two_way : int -> int
  val abs_match : int -> int
  val in_range : int -> bool
  val greater_or_equal : int -> int -> bool
  val greater_or_equal_not : int -> int -> bool
end

(** The square-root program of subsection 1.1.7, flat first and then in
    block structure with internal definitions, as the book presents it.
    [square] here is the [float] version; it shadows the [int] [square]
    of [Compound], the same redefinition the book makes. *)
module Sqrt : sig
  val square : float -> float
  val average : float -> float -> float
  val improve : float -> float -> float
  val good_enough : float -> float -> bool
  val sqrt_iter : float -> float -> float
  val sqrt : float -> float
  val sqrt_block : float -> float
end

(** The black-box point of subsection 1.1.8: two definitions of
    [square] on [float] with the same type and contract. They agree
    exactly on some inputs and differ by a final-bit rounding on
    others, which the section's prose now has to say. *)
module Black_box : sig
  val square : float -> float
  val double : float -> float
  val square_via_exp : float -> float
end
