(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.24 *)

(** Exercise 3.24: a two-dimensional table whose keys are compared by a
    caller-supplied [same_key] predicate instead of structural
    equality. The constructor returns the record-of-closures dispatch
    this edition uses for message passing. *)

open Sicp_ch3.Sec_3_3
module T = Table
module M = Mpairs

type table =
  { lookup : M.mobj -> M.mobj -> M.mobj option
  ; insert : M.mobj -> M.mobj -> M.mobj -> unit
  }

let make_table same_key =
  let local_table = T.make_table () in
  let assoc key records =
    let rec go = function
      | M.Pair p when same_key (M.car p.car) key -> Some p.car
      | M.Pair p -> go p.cdr
      | _ -> None
    in
    go records
  in
  let lookup key1 key2 =
    match assoc key1 (M.cdr local_table) with
    | None -> None
    | Some subtable ->
      (match assoc key2 (M.cdr subtable) with
       | Some record -> Some (M.cdr record)
       | None -> None)
  in
  let insert key1 key2 value =
    match assoc key1 (M.cdr local_table) with
    | Some subtable ->
      (match assoc key2 (M.cdr subtable) with
       | Some record -> M.set_cdr record value
       | None -> M.set_cdr subtable (M.mcons (M.mcons key2 value) (M.cdr subtable)))
    | None ->
      M.set_cdr
        local_table
        (M.mcons (M.mcons key1 (M.mcons (M.mcons key2 value) M.mnil)) (M.cdr local_table))
  in
  { lookup; insert }
;;

(* The statement's numeric example: a first-level key matches when it
   lies within the tolerance of the stored key; second-level keys are
   symbols and match exactly, since [make_table] uses one predicate for
   both levels of the two-dimensional table. *)
let near tol =
  M.(
    fun a b ->
      match a, b with
      | Int x, Int y -> abs (x - y) <= tol
      | Sym x, Sym y -> String.equal x y
      | _ -> false)
;;

let ex_3_24 () =
  let t = make_table (near 1) in
  t.insert (M.mint 10) (M.msym "a") (M.mint 1);
  t.insert (M.mint 20) (M.msym "b") (M.mint 2);
  let exact = t.lookup (M.mint 10) (M.msym "a") in
  let within_tolerance = t.lookup (M.mint 11) (M.msym "a") in
  let outside_tolerance = t.lookup (M.mint 12) (M.msym "a") in
  ( exact
  , within_tolerance
  , outside_tolerance
  , match t.lookup (M.mint 21) (M.msym "b") with
    | Some v -> M.show v
    | None -> "false" )
;;
