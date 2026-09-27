(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.2 exercise 2.29:
   binary mobiles *)

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
    mobile], the sample checks the statement's parts (b) and (c) call
    for. *)
val ex_2_29 : mobile -> int * bool

(** [total_weight mobile] is the sum of the weights hanging anywhere
    inside [mobile]. *)
val total_weight : mobile -> int

(** [balanced mobile] holds when the torques of the two top branches
    are equal and every submobile is balanced. *)
val balanced : mobile -> bool

(** The pair-based representation of part (d): only the shape of the
    data changes, and the selectors with it. *)
module Cons_repr : sig
  type structure =
    | Weight of int
    | Hanging of mobile

  and mobile = branch * branch
  and branch = int * structure

  val total_weight : mobile -> int
  val balanced : mobile -> bool
end
