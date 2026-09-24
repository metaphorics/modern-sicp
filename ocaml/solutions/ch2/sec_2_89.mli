(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.89 *)

(** A dense term list: coefficients only, highest order first, one
    slot per order down to 0 -- the book's own example represents
    [x^5 + 2x^4 + 3x^2 - 2x - 5] as [(1 2 0 3 -2 -5)]. The order of
    the first coefficient is the list's length minus one; dropping
    the head both removes the highest-order term and correctly
    lowers every remaining term's implied order by one, with no
    separate order field to keep in step. *)
type dense = int list

val is_empty_termlist : dense -> bool

(** [first_term l] is [(order, coeff)] of [l]'s leading coefficient. *)
val first_term : dense -> int * int

val rest_terms : dense -> dense

(** [of_terms sparse] builds the dense representation of the sparse
    [(order, coeff)] list [sparse], filling every missing order with
    0 down to order 0. *)
val of_terms : (int * int) list -> dense

(** [to_terms l] is the sparse [(order, coeff)] list of [l]'s nonzero
    coefficients, highest order first -- [of_terms] undone. *)
val to_terms : dense -> (int * int) list

(** [ex_2_89 ()] builds the dense representation of
    [x^5 + 2x^4 + 3x^2 - 2x - 5], its [first_term], and its
    [to_terms], confirming the dense list, the leading term, and the
    round trip back to sparse form all match the book's own example. *)
val ex_2_89 : unit -> dense * (int * int) * (int * int) list
