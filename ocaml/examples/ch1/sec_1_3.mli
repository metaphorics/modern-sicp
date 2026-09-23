(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** The named definitions behind the listings of section 1.3, grouped by
    subsection. Each value a listing displays is asserted by
    [run_sec_1_3] through [Replay], so the book's result comments are
    true by construction.

    The section's hard spot is [sum]: OCaml's [+] and [+.] are two
    different operators, so one generic [sum] cannot mix integer and
    float terms the way Scheme's untyped arithmetic does. This edition
    resolves it by keeping [sum]'s index, term, and accumulator all in
    [float]; [sum_cubes] and [sum_integers] promote their integer
    bounds with [Float.of_int] and return [float], while [integral]'s
    already-real step reuses [sum] with no conversion at all. *)

(** The three template procedures of subsection 1.3.1, before
    abstraction: [sum_integers], [sum_cubes], and [pi_sum], each its
    own linear recursion differing only in the term added and the step
    to the next index. *)
module Sum_templates : sig
  val sum_integers : int -> int -> int
  val cube : int -> int
  val sum_cubes : int -> int -> int
  val pi_sum : int -> int -> float
end

(** The single [sum] abstraction of subsection 1.3.1 and its four
    reformulated uses. *)
module Sum_abstraction : sig
  (** [sum term a next b] is [term a + term (next a) + ... + term k]
      for the largest [k] the [next] chain reaches without passing
      [b]. *)
  val sum : (float -> float) -> float -> (float -> float) -> float -> float

  val inc : float -> float
  val cube : float -> float

  (** [sum_cubes a b] is [sum] applied to [cube], promoted to [float]
      at the call boundary. *)
  val sum_cubes : int -> int -> float

  val identity : float -> float
  val sum_integers : int -> int -> float
  val pi_sum : int -> int -> float

  (** [integral f a b dx] approximates [∫ₐᵇ f] by summing [f] at the
      midpoint of each width-[dx] slice; unlike [sum_cubes] and
      [sum_integers], its index is already [float], so it calls [sum]
      directly. *)
  val integral : (float -> float) -> float -> float -> float -> float
end

(** Subsection 1.3.2: the [lambda] and [let] reformulations. [pi_sum]
    and [integral] drop their named auxiliary procedures in favor of
    anonymous functions passed straight to [Sum_abstraction.sum];
    [f_via_helper], [f_via_lambda], and [f_via_let] are the book's
    three equivalent ways to bind the local names [a] and [b] inside
    [f(x,y)]; [let_shadows_outer] and [let_binds_from_outer_scope]
    are the two scoping examples the section reasons about by hand. *)
module Lambda_and_let : sig
  val pi_sum : int -> int -> float
  val integral : (float -> float) -> float -> float -> float -> float
  val square : int -> int
  val plus4 : int -> int
  val plus4_via_lambda : int -> int

  (** [lambda_as_operator ()] is the book's
      [((lambda (x y z) (+ x y (square z))) 1 2 3)]. *)
  val lambda_as_operator : unit -> int

  val f_via_helper : float -> float -> float
  val f_via_lambda : float -> float -> float
  val f_via_let : float -> float -> float

  (** [let_shadows_outer outer_x] is the Scheme
      [(+ (let ((x 3)) (+ x ( * x 10))) outer_x)]: the inner [x] never
      sees [outer_x]. *)
  val let_shadows_outer : float -> float

  (** [let_inner_value ()] is the [let] body alone, 33, so the book's
      arithmetic (33 plus an outer 5 is 38) is visible in the test. *)
  val let_inner_value : unit -> float

  (** [let_binds_from_outer_scope outer_x] is the Scheme
      [(let ((x 3) (y (+ outer_x 2))) ( * x y))]: both bindings read
      [outer_x], never each other. *)
  val let_binds_from_outer_scope : float -> float
end

(** The section's one closed error: [Half_interval]'s mismatched-sign
    endpoints, or a [Fixed_point] search that runs past its iteration
    cap without settling. Every search in this section is a genuine
    numerical method, so both can really happen; neither is a
    programming error. *)
module Numeric_error : sig
  type t =
    | Values_not_of_opposite_sign
    | Not_converged

  val pp : Format.formatter -> t -> unit
end

(** Subsection 1.3.3, first half: locating a zero by halving the
    interval where the function changes sign. *)
module Half_interval : sig
  val close_enough : float -> float -> bool
  val search : (float -> float) -> float -> float -> float

  (** [half_interval_method f a b] is a zero of [f] between [a] and
      [b], found by halving; [Error Values_not_of_opposite_sign] when
      [f a] and [f b] do not have opposite signs. *)
  val half_interval_method
    :  (float -> float)
    -> float
    -> float
    -> (float, Numeric_error.t) result
end

(** Subsection 1.3.3, second half: locating a fixed point of a
    function by repeated application. [fixed_point]'s [max_iterations]
    is this edition's addition (default 100,000): the book's own
    example of a search that never converges (the unaveraged
    square-root iteration two sections later) would otherwise spin
    forever, which a batch build cannot tolerate the way an
    interactive REPL can. *)
module Fixed_point : sig
  val tolerance : float

  val fixed_point
    :  ?max_iterations:int
    -> (float -> float)
    -> float
    -> (float, Numeric_error.t) result
end

(** Subsection 1.3.4, first part: [average_damp] as a procedure that
    returns a procedure, and the square-root and cube-root searches it
    tames. *)
module Average_damping : sig
  val average : float -> float -> float
  val average_damp : (float -> float) -> float -> float
  val sqrt : float -> (float, Numeric_error.t) result
  val cube_root : float -> (float, Numeric_error.t) result
end

(** Subsection 1.3.4, "Newton's Method": [deriv] as a numerical
    derivative, [newton_transform], and the square root it finds. *)
module Newtons_method : sig
  val dx : float
  val deriv : (float -> float) -> float -> float
  val cube : float -> float
  val newton_transform : (float -> float) -> float -> float
  val newtons_method : (float -> float) -> float -> (float, Numeric_error.t) result
  val sqrt : float -> (float, Numeric_error.t) result
end

(** Subsection 1.3.4, "Abstractions and first-class procedures":
    [fixed_point_of_transform] unifies the average-damped and
    Newton-transformed searches into one more general procedure. *)
module First_class_procedures : sig
  val fixed_point_of_transform
    :  (float -> float)
    -> ((float -> float) -> float -> float)
    -> float
    -> (float, Numeric_error.t) result

  val sqrt_via_average_damp : float -> (float, Numeric_error.t) result
  val sqrt_via_newton_transform : float -> (float, Numeric_error.t) result
end
