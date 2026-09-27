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

module Sparse = struct
  type t = (int * int) list

  let the_empty_termlist = raise Sicp_common.Pending.Pending_solution
  let is_empty _a0 = raise Sicp_common.Pending.Pending_solution
  let first_term _a0 = raise Sicp_common.Pending.Pending_solution
  let rest_terms _a0 = raise Sicp_common.Pending.Pending_solution
  let adjoin_term _a0 _a1 _a2 = raise Sicp_common.Pending.Pending_solution
  let of_pairs _a0 = raise Sicp_common.Pending.Pending_solution
  let to_pairs _a0 = raise Sicp_common.Pending.Pending_solution
end

module Dense = struct
  type t = int list

  let the_empty_termlist = raise Sicp_common.Pending.Pending_solution
  let is_empty _a0 = raise Sicp_common.Pending.Pending_solution
  let first_term _a0 = raise Sicp_common.Pending.Pending_solution
  let rest_terms _a0 = raise Sicp_common.Pending.Pending_solution
  let adjoin_term _a0 _a1 _a2 = raise Sicp_common.Pending.Pending_solution
  let of_pairs _a0 = raise Sicp_common.Pending.Pending_solution
  let to_pairs _a0 = raise Sicp_common.Pending.Pending_solution
end

module Make_ops (T : Term_list) = struct
  let add_terms (_a0 : T.t) (_a1 : T.t) : T.t = raise Sicp_common.Pending.Pending_solution
end

let ex_2_90 () = raise Sicp_common.Pending.Pending_solution
