(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.29: binary mobiles, both representations. *)

type structure =
  | Weight of int
  | Hanging of mobile

and mobile = Mobile of branch * branch
and branch = Branch of int * structure

val left_branch : mobile -> branch
val right_branch : mobile -> branch
val branch_length : branch -> int
val branch_structure : branch -> structure

(** [ex_2_29 mobile] pairs [total_weight mobile] with [balanced
    mobile]. *)
val ex_2_29 : mobile -> int * bool

val total_weight : mobile -> int
val balanced : mobile -> bool

module Cons_repr : sig
  type structure =
    | Weight of int
    | Hanging of mobile

  and mobile = branch * branch
  and branch = int * structure

  val total_weight : mobile -> int
  val balanced : mobile -> bool
end
