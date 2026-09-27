(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.4 exercise 3.42 *)

(** Exercise 3.42: Ben Bitdiddle hoists the creation of the serialized
    withdraw and deposit procedures out of the account's message
    dispatch, so they are built once instead of once per message.
    Both versions still let only one protected operation run at a
    time on a given account -- the serializer's mutex is the same
    object either way -- so the change is safe, and a concurrent
    deposit workload conserves every unit under both. *)

open Sicp_ch3.Sec_3_4.Account

let with_per_message_serialization initial = make_account initial
let with_hoisted_serialization initial = make_account_hoisted_serialization initial
let run_concurrent_deposits account n = concurrent_deposits account n

let ex_3_42 () =
  let a = with_per_message_serialization 0 in
  let b = with_hoisted_serialization 0 in
  run_concurrent_deposits a 400, run_concurrent_deposits b 400
;;
