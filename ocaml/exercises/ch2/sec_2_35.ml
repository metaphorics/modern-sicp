(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.2 exercise 2.35:
   count-leaves as an accumulation *)

type tree =
  | Leaf of int
  | Node of tree list

let ex_2_35 _tree = raise Sicp_common.Pending.Pending_solution
