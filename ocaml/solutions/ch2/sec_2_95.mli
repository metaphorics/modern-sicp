(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.95 *)

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
val gcd_terms : term list -> term list -> term list
val gcd_poly : poly -> poly -> poly
val scalar_ratio_to : term list -> term list -> float option

(** [ex_2_95 ()] is [(gcd_term_list, ratio)]: the term list produced by
    the naive (non-pseudo) [gcd_poly] on Q1 = P1*P2 and Q2 = P1*P3, and
    the scalar ratio to P1 if the two term lists are proportional. The
    book's example demonstrates that [ratio] is [None]. *)
val ex_2_95 : unit -> (int * float) list * float option
