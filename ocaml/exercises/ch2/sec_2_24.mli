(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.2 exercise 2.24:
   the tree reading of 1, (2, (3, 4)) *)

type tree =
  | Leaf of int
  | Node of tree list

(** The tree value whose printed form, box-and-pointer structure, and
    tree interpretation the exercise asks for. *)
val ex_2_24 : unit -> tree
