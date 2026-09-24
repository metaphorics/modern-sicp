(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.90 *)

(** The shape [add_terms] needs from any term-list representation:
    empty, decompose into a leading [(order, coeff)] and the rest,
    build by adjoining a term (dropping a zero coefficient), and
    convert to and from the [(order, coeff)] list the tests read. *)
module type Term_list = sig
  type t

  val the_empty_termlist : t
  val is_empty : t -> bool
  val first_term : t -> int * int
  val rest_terms : t -> t
  val adjoin_term : int -> int -> t -> t
  val of_pairs : (int * int) list -> t
  val to_pairs : t -> (int * int) list
end

(** The list-of-nonzero-terms representation 2.5.3's narrative uses. *)
module Sparse : Term_list

(** The one-coefficient-per-order representation of Exercise 2.89. *)
module Dense : Term_list

(** [Make_ops(T)] is [add_terms], written once against [Term_list]
    and usable for either representation -- the "analogous to the
    complex-number example" 2.4 the exercise asks for. *)
module Make_ops (T : Term_list) : sig
  val add_terms : T.t -> T.t -> T.t
end

(** [ex_2_90 ()] adds [x^2 + 1] to [x + 1] through both [Sparse] and
    [Dense], returning each result as an [(order, coeff)] list; the
    two must agree. *)
val ex_2_90 : unit -> (int * int) list * (int * int) list
