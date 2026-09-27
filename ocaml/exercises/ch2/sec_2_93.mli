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

type rational_function =
  { numer : poly
  ; denom : poly
  }

val make_rational_function : poly -> poly -> rational_function
val add_rational_function : rational_function -> rational_function -> rational_function
val ex_2_93 : unit -> (int * float) list * (int * float) list
