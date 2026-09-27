(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.87 *)

type coeff =
  | Flt of float
  | Poly of poly

and term =
  { order : int
  ; coeff : coeff
  }

and poly =
  { var : string
  ; term_list : term list
  }

val make_term : int -> coeff -> term
val make_poly : string -> term list -> poly
val is_zero_coeff : coeff -> bool
val adjoin_term : term -> term list -> term list
val ex_2_87 : unit -> int * bool
