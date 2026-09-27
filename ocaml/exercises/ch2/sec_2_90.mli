(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.90 *)

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

module Sparse : Term_list
module Dense : Term_list

module Make_ops (T : Term_list) : sig
  val add_terms : T.t -> T.t -> T.t
end

val ex_2_90 : unit -> (int * int) list * (int * int) list
