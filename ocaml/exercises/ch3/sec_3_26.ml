(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The pending scaffold of the solution with the same name under
   solutions/ch3: every entry raises the pending marker until the
   exercise is solved. *)

open Sicp_ch3.Sec_3_3
(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.26 *)

(** Exercise 3.26: a table whose records live in a binary tree. *)

type tree =
  | Leaf
  | Node of node

and node =
  { key : Mpairs.mobj
  ; mutable value : Mpairs.mobj
  ; mutable left : tree
  ; mutable right : tree
  }

type btable =
  { mutable root : tree
  ; key_compare : Mpairs.mobj -> Mpairs.mobj -> int
  }

(** [make_table ~key_compare] is an empty tree table ordered by
    [key_compare]. *)

(** [key_compare] orders integer and symbol keys; mixed key types are
    a programming error. *)

(** [ex_3_26 ()] is [(the values found under 10 30 50 70 80 after an
    out-of-order build, the value under 20 after an overwrite, and the
    lookup of the missing key 40)]. *)
let make_table = raise Sicp_common.Pending.Pending_solution

let lookup = raise Sicp_common.Pending.Pending_solution
let insert = raise Sicp_common.Pending.Pending_solution
let key_compare = raise Sicp_common.Pending.Pending_solution
let ex_3_26 = raise Sicp_common.Pending.Pending_solution
