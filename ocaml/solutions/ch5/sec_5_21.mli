(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.3 *)

(** Exercise 5.21: the two [count_leaves] machines over the
    list-structure memory, with the host oracle beside them. *)

(** The object-language trees the machines count. *)
type tree =
  | Leaf of int
  | Node of tree list

(** [plant mem tree] builds the tree's cells in [mem]. *)
val plant
  :  Sicp_ch5.Sec_5_3.memory
  -> tree
  -> (Sicp_ch5.Sec_5_3.word, Sicp_ch5.Sec_5_3.error) result

(** [count tree] is the host oracle over the same trees. *)
val count : tree -> int

(** The machine of Exercise 5.21a (pure recursion, answers combined by
    [+]) and of Exercise 5.21b (the explicit counter). *)
val recursive_controller : Sicp_ch5.Sec_5_3.word Sicp_ch5.Sec_5_1.instruction list

val iterative_controller : Sicp_ch5.Sec_5_3.word Sicp_ch5.Sec_5_1.instruction list

(** [run controller answer tree] plants the tree, runs the machine, and
    answers the result word with the monitored stack counters. *)
val run
  :  Sicp_ch5.Sec_5_3.word Sicp_ch5.Sec_5_1.instruction list
  -> string
  -> tree
  -> (string * string, Sicp_ch5.Sec_5_3.error) result

(** [ex_5_21 ()] runs the recursive machine, the iterative machine, and
    the oracle over three planted trees; one line per tree, named by the
    tree's list notation, with the machines' stack use. *)
val ex_5_21 : unit -> (string list, Sicp_ch5.Sec_5_3.error) result
