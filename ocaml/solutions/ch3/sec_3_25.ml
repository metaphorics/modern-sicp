(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.25 *)

(** Exercise 3.25: one table machinery for any number of keys. A lookup
    of [k1; ...; kn] descends the backbone one subtable per key and
    reads the record under [kn]; an insert builds the missing subtables
    on the way down. *)

open Sicp_ch3.Sec_3_3
module T = Table
module M = Mpairs

let rec lookup keys table =
  match keys with
  | [] -> None
  | [ key ] -> T.lookup key table
  | key :: rest ->
    (match T.assoc key (M.cdr table) with
     | None -> None
     | Some subtable -> lookup rest (M.cdr subtable))
;;

let rec insert keys value table =
  match keys with
  | [] -> ()
  | [ key ] -> T.insert key value table
  | key :: rest ->
    (match T.assoc key (M.cdr table) with
     | Some subtable -> insert rest value (M.cdr subtable)
     | None ->
       let subtable = T.make_table () in
       M.set_cdr table (M.mcons (M.mcons key subtable) (M.cdr table));
       insert rest value subtable)
;;

let ex_3_25 () =
  let t = T.make_table () in
  insert [ M.msym "a"; M.msym "b"; M.msym "c" ] (M.mint 1) t;
  insert [ M.msym "a"; M.msym "b"; M.msym "d" ] (M.mint 2) t;
  insert [ M.msym "a"; M.msym "e" ] (M.mint 3) t;
  insert [ M.msym "x" ] (M.mint 4) t;
  let abc = lookup [ M.msym "a"; M.msym "b"; M.msym "c" ] t in
  let abd = lookup [ M.msym "a"; M.msym "b"; M.msym "d" ] t in
  let ae = lookup [ M.msym "a"; M.msym "e" ] t in
  let x = lookup [ M.msym "x" ] t in
  let missing = lookup [ M.msym "a"; M.msym "b"; M.msym "z" ] t in
  (* overwriting through the same key path leaves siblings alone *)
  insert [ M.msym "a"; M.msym "b"; M.msym "c" ] (M.mint 5) t;
  let abc_again = lookup [ M.msym "a"; M.msym "b"; M.msym "c" ] t in
  let abd_after = lookup [ M.msym "a"; M.msym "b"; M.msym "d" ] t in
  abc, abd, ae, x, missing, abc_again, abd_after
;;
