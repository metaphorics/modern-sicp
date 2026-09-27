(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.94 *)

(** A single term of a sparse univariate polynomial. *)
type term =
  { order : int
  ; coeff : float
  }

(** A polynomial in one variable, as a dense-in-spirit but sparse-in-storage
    list of terms ordered from highest to lowest [order]. *)
type poly =
  { var : string
  ; term_list : term list
  }

val make_term : int -> float -> term
val make_poly : string -> term list -> poly

(** [adjoin_term term term_list] is [term_list] with [term] prepended, unless
    [term]'s coefficient is zero, in which case [term_list] is unchanged. *)
val adjoin_term : term -> term list -> term list

val add_terms : term list -> term list -> term list
val negate_terms : term list -> term list
val sub_terms : term list -> term list -> term list
val mul_term_by_all_terms : term -> term list -> term list

(** [div_terms dividend divisor] is [(quotient, remainder)], the result of
    dividing [dividend] by [divisor] one leading term at a time. *)
val div_terms : term list -> term list -> term list * term list

(** [remainder_terms dividend divisor] is the remainder component of
    {!div_terms}. *)
val remainder_terms : term list -> term list -> term list

(** [gcd_terms a b] is the term-list GCD of [a] and [b], found by Euclid's
    algorithm using {!remainder_terms}. *)
val gcd_terms : term list -> term list -> term list

(** [gcd_poly p1 p2] is the polynomial GCD of [p1] and [p2]. Raises
    [Invalid_argument] if the two polys are not in the same variable. *)
val gcd_poly : poly -> poly -> poly

val int_gcd : int -> int -> int

(** A value that {!greatest_common_divisor} can dispatch on: either a
    polynomial or an ordinary integer. *)
type numeric =
  | Poly_value of poly
  | Int_value of int

(** [greatest_common_divisor a b] reduces to {!gcd_poly} when both arguments
    are polynomials and to {!int_gcd} when both are integers. Raises
    [Invalid_argument] on a mismatched pair. *)
val greatest_common_divisor : numeric -> numeric -> numeric

val ex_2_94 : unit -> (int * float) list * bool * bool
