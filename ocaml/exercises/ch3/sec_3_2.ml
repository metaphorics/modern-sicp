(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.1 exercise 3.2 *)

(** Exercise 3.2: a monitored wrapper that counts and resets calls to
    an underlying procedure. *)

type 'a message =
  | How_many_calls
  | Reset_count
  | Input of 'a

type 'a response =
  | Value of 'a
  | Count of int
  | Reset

let make_monitored _f = raise Sicp_common.Pending.Pending_solution
let ex_3_02 () = raise Sicp_common.Pending.Pending_solution
