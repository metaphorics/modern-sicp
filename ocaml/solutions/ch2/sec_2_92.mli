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

(** [var_less a b] is the ordering the exercise asks to impose on
    variables: plain string order, so ["x"] outranks ["y"]. *)
val var_less : string -> string -> bool

(** A dominant-variable term's coefficient: either a plain number, or
    -- once a polynomial in a lower-priority variable is promoted to
    a constant coefficient -- that whole polynomial. *)
type coeff =
  | Num of float
  | Sub_poly of poly

val coeff_as_float : coeff -> float
val coeff_as_poly : coeff -> poly

(** [add_across_variables p1 p2] adds two polynomials in different
    variables: the lower-priority one (by [var_less]) is promoted to
    a degree-0 coefficient of the higher-priority polynomial's
    variable, then adjoined to that polynomial's own terms. It
    refuses (rather than silently mis-combine) when the
    higher-priority polynomial already carries its own degree-0 term,
    since reconciling two same-order coefficients -- one a number,
    one a promoted polynomial -- needs the full multivariate
    addition the exercise itself calls "not easy"; this edition
    scopes down to the case with no such collision. *)
val add_across_variables : poly -> poly -> string * (int * coeff) list

(** [ex_2_92 ()] adds [x] (variable [x], a bare first-order term) to
    [3y + 2] (variable [y]): ["x"] outranks ["y"], so the result is
    [x]'s variable with two terms, order 1 (coefficient 1) and order
    0 (coefficient the promoted polynomial [3y + 2]). *)
val ex_2_92 : unit -> string * (int * coeff) list
