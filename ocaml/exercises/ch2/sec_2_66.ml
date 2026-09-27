(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.66 *)

type record =
  { key : int
  ; name : string
  }

type tree =
  | Empty
  | Node of tree * record * tree

let lookup _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let sample_tree = raise Sicp_common.Pending.Pending_solution
let ex_2_66 () = raise Sicp_common.Pending.Pending_solution
