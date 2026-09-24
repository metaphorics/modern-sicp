(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.94 *)

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
val div_terms : term list -> term list -> term list * term list
val remainder_terms : term list -> term list -> term list
val gcd_terms : term list -> term list -> term list
val gcd_poly : poly -> poly -> poly
val int_gcd : int -> int -> int

type numeric =
  | Poly_value of poly
  | Int_value of int

val greatest_common_divisor : numeric -> numeric -> numeric
val ex_2_94 : unit -> (int * float) list * bool * bool
