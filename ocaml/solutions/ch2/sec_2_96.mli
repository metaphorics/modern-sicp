(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.96 *)

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

(** [integerizing_power c exponent] is [c] raised to [exponent], used to
    build the integerizing factor c^(1+O1-O2) described in the text. *)
val integerizing_power : float -> int -> float

(** [pseudoremainder_terms dividend divisor] is like {!remainder_terms}
    except [dividend] is first scaled by the integerizing factor, so the
    division that follows never introduces a fraction. *)
val pseudoremainder_terms : term list -> term list -> term list

(** 2.96a: [gcd_terms] using {!pseudoremainder_terms}. *)
val gcd_terms : term list -> term list -> term list

val gcd_poly : poly -> poly -> poly
val int_gcd : int -> int -> int

(** [coefficients_content terms] is the integer GCD of every (rounded)
    coefficient in [terms], or [0] for the empty list. *)
val coefficients_content : term list -> int

(** 2.96b: [reduce_content terms] divides every coefficient by
    {!coefficients_content}, removing the redundant integer factor left
    behind by pseudodivision. *)
val reduce_content : term list -> term list

val gcd_terms_reduced : term list -> term list -> term list
val gcd_poly_reduced : poly -> poly -> poly
val ex_2_96 : unit -> bool * int
