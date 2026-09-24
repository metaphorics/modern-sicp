(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.87 *)

(** A coefficient is a plain number or a nested polynomial -- the
    representational fact behind "coefficients that are themselves
    polynomials," which [is_zero_coeff] must handle recursively. *)
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

(** [is_zero_coeff c] is [=zero?] extended to a coefficient that may
    itself be a polynomial: a [Flt] is zero exactly when the float is,
    and a [Poly] is zero exactly when its term list is empty (every
    term already installed through [adjoin_term] has a nonzero
    coefficient, so an empty list is the only zero polynomial). *)
val is_zero_coeff : coeff -> bool

(** [adjoin_term term term_list] drops [term] when
    [is_zero_coeff term.coeff], exactly as 2.5.3's narrative
    [adjoin_term] does for plain numbers. *)
val adjoin_term : term -> term list -> term list

(** [ex_2_87 ()] adjoins two [x]-terms to an empty term list: one
    whose coefficient is the empty (zero) polynomial in [y], one
    whose coefficient is a nonzero polynomial in [y]. Only the
    nonzero-coefficient term survives. Returns [(surviving term
    count, is_zero_coeff of the empty y-polynomial)]. *)
val ex_2_87 : unit -> int * bool
