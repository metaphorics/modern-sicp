(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.19 *)

(** Exercise 3.19: Floyd's tortoise-and-hare. Two pointers walk the cdr
    chain, the hare twice as fast; if a cycle exists the hare laps the
    tortoise and the two meet, all in constant space. *)

open Sicp_ch3.Sec_3_3.Mpairs

let contains_cycle_constant x =
  if not (is_pair x)
  then false
  else (
    let rec go tortoise hare =
      if is_pair hare && is_pair (cdr hare)
      then (
        let tortoise = cdr tortoise in
        let hare = cdr (cdr hare) in
        tortoise == hare || go tortoise hare)
      else false
    in
    go x (cdr x))
;;

let ex_3_19 () =
  let plain = from_symbols [ "a"; "b"; "c" ] in
  (* fresh chain, not an alias of [plain]: see exercise 3.18 *)
  let ring = from_symbols [ "a"; "b"; "c" ] in
  set_cdr (last_pair ring) ring;
  let self = { car = msym "a"; cdr = Nil } in
  set_cdr (Pair self) (Pair self);
  let shared = mcons (msym "b") mnil in
  (* a shared suffix without a cycle: both lists walk into the same
     (b) pair and then end *)
  let suffix_list = mcons (msym "a") shared in
  ( contains_cycle_constant plain
  , contains_cycle_constant ring
  , contains_cycle_constant (Pair self)
  , contains_cycle_constant suffix_list )
;;
