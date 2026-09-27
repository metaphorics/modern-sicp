(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.93 *)

type term =
  { order : int
  ; coeff : float
  }

type poly =
  { var : string
  ; term_list : term list
  }

val make_term : int -> float -> term
val make_poly : string -> term list -> poly
val adjoin_term : term -> term list -> term list
val add_terms : term list -> term list -> term list
val mul_terms : term list -> term list -> term list
val add_poly : poly -> poly -> poly
val mul_poly : poly -> poly -> poly

(** A numerator and a denominator, both polynomials, kept exactly as
    given -- [make_rational_function] does not call [reduce] the way
    2.1.1's [make-rat] did, per the exercise's instruction. *)
type rational_function =
  { numer : poly
  ; denom : poly
  }

val make_rational_function : poly -> poly -> rational_function

(** [add_rational_function rf1 rf2] cross-multiplies: [n1*d2 + n2*d1]
    over [d1*d2], with no reduction step. *)
val add_rational_function : rational_function -> rational_function -> rational_function

(** [ex_2_93 ()] builds [p1 = x^2 + 1], [p2 = x^3 + 1], [rf = p2/p1],
    and adds [rf] to itself, returning the sum's numerator and
    denominator term lists as [(order, coeff)] pairs. Since nothing
    reduces, the denominator is [p1^2], not [p1] -- the exercise's
    own observation about this addition procedure. *)
val ex_2_93 : unit -> (int * float) list * (int * float) list
