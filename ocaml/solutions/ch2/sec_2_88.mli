(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.88 *)

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

(** [add_terms l1 l2] is 2.5.3's narrative [add_terms], restated for
    plain [float] coefficients. *)
val add_terms : term list -> term list -> term list

(** [negate_terms l] negates every coefficient in [l] -- the "generic
    negation operation" the exercise's hint suggests. *)
val negate_terms : term list -> term list

(** [sub_terms l1 l2] is [add_terms l1 (negate_terms l2)]: subtraction
    needs no new merge logic, only negation plus the addition already
    on hand. *)
val sub_terms : term list -> term list -> term list

val sub_poly : poly -> poly -> poly

(** [ex_2_88 ()] subtracts [x^2 + 1] from [x^2 + 3x + 1], returning
    the result's term list as [(order, coeff)] pairs; the [x^2] terms
    must cancel to leave [3x]. *)
val ex_2_88 : unit -> (int * float) list
