(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.12 *)

(** Exercise 3.12: [append] builds a fresh chain of pairs, [append_bang]
    splices [y] onto the last pair of [x]. [ex_3_12] replays the
    statement's interaction and answers the two missing responses. *)

open Sicp_ch3.Sec_3_3.Mpairs

(* It is an error to call [append_bang] with an empty [x]. *)
let append_bang x y =
  match x with
  | Nil -> invalid_arg "append!: the first list is empty"
  | _ ->
    set_cdr (last_pair x) y;
    x
;;

let ex_3_12 () =
  let x = from_symbols [ "a"; "b" ] in
  let y = from_symbols [ "c"; "d" ] in
  let z = append x y in
  let z_shown = show z in
  let cdr_x_after_append = show (cdr x) in
  let w = append_bang x y in
  let w_shown = show w in
  let cdr_x_after_append_bang = show (cdr x) in
  z_shown, cdr_x_after_append, w_shown, cdr_x_after_append_bang
;;
