(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.91 *)

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

(** [div_terms l1 l2] divides [l1] by [l2] via the long-division
    algorithm the exercise states: divide the leading terms to get
    the quotient's next term, subtract that term times the divisor
    from the dividend, and recurse on the difference. Returns
    [(quotient, remainder)]. *)
val div_terms : term list -> term list -> term list * term list

val div_poly : poly -> poly -> poly * poly

(** [ex_2_91 ()] divides [x^5 - 1] by [x^2 - 1], the exercise's own
    example, returning [(quotient term list, remainder term list)] as
    [(order, coeff)] pairs. The answer is [x^3 + x] remainder
    [x - 1]. *)
val ex_2_91 : unit -> (int * float) list * (int * float) list
