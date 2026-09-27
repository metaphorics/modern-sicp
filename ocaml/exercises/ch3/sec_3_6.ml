(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.1 exercise 3.6 *)

(** Exercise 3.6: a [rand] that can be reset to reproduce a sequence. *)

type rand_message =
  | Generate
  | Reset of int64

let make_rand _seed = raise Sicp_common.Pending.Pending_solution
let ex_3_06 () = raise Sicp_common.Pending.Pending_solution
