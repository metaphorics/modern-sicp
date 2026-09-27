(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.2 exercise 2.27:
   deep-reverse *)

type tree =
  | Leaf of int
  | Node of tree list

let ex_2_27 _tree = raise Sicp_common.Pending.Pending_solution
