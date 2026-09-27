(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.14 *)

(** Exercise 3.14: [mystery] reverses a mutable list in place by
    walking it once, pointing each pair back at its predecessor.
    [ex_3_14] replays the statement's [v] and [w] and shows both. *)

open Sicp_ch3.Sec_3_3.Mpairs

let mystery x =
  let rec loop x y =
    match x with
    | Nil -> y
    | Pair _ ->
      let temp = cdr x in
      set_cdr x y;
      loop temp x
    | _ -> invalid_arg "mystery: not a list"
  in
  loop x mnil
;;

let ex_3_14 () =
  let v = from_symbols [ "a"; "b"; "c"; "d" ] in
  let v_before = show v in
  let w = mystery v in
  let v_after = show v in
  let w_shown = show w in
  v_before, v_after, w_shown
;;
