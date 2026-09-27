(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.97 *)

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
val negate_terms : term list -> term list
val sub_terms : term list -> term list -> term list
val mul_term_by_all_terms : term -> term list -> term list
val mul_terms : term list -> term list -> term list
val mul_poly : poly -> poly -> poly
val div_terms : term list -> term list -> term list * term list
val remainder_terms : term list -> term list -> term list
val integerizing_power : float -> int -> float
val pseudoremainder_terms : term list -> term list -> term list
val gcd_terms : term list -> term list -> term list
val int_gcd : int -> int -> int
val coefficients_content : term list -> int

(** [reduce_content_pair numer denom] divides both term lists by the
    integer GCD of their combined coefficients. *)
val reduce_content_pair : term list -> term list -> term list * term list

val max_order : term list -> int
val leading_term : term list -> term

(** [reduce_terms numer denom] is [(numer', denom')], the numerator and
    denominator scaled by the GCD's integerizing factor, divided by the
    GCD, and stripped of their remaining common integer factor: a
    numerator/denominator pair in lowest terms with integer coefficients. *)
val reduce_terms : term list -> term list -> term list * term list

type rational_function =
  { numer : poly
  ; denom : poly
  }

val make_rational_function : poly -> poly -> rational_function

(** [reduce_poly p1 p2] is the rational function p1/p2 reduced to lowest
    terms via {!reduce_terms}. Raises [Invalid_argument] if [p1] and [p2]
    are not in the same variable. *)
val reduce_poly : poly -> poly -> rational_function

val ex_2_97 : unit -> (int * float) list * (int * float) list * bool
