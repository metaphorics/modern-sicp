(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.92 *)

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
val var_less : string -> string -> bool

type coeff =
  | Num of float
  | Sub_poly of poly

val coeff_as_float : coeff -> float
val coeff_as_poly : coeff -> poly
val add_across_variables : poly -> poly -> string * (int * coeff) list
val ex_2_92 : unit -> string * (int * coeff) list
