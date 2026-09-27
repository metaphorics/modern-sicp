(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Records, mutable fields, and refs: the named definitions behind the
    listings of section 0.6. *)

type point =
  { x : float
  ; y : float
  }

let origin = { x = 0.0; y = 0.0 }
let right_three = { origin with x = 3.0 }

type counter = { mutable count : int }

let fresh_counter () = { count = 0 }
let bump counter = counter.count <- counter.count + 1

let after_deposit () =
  let balance = ref 100 in
  balance := !balance + 20;
  !balance
;;

let alias_cell () =
  let cell = ref 10 in
  let alias = cell in
  alias := !alias + 5;
  !cell
;;

let copy_point () =
  let original = { x = 1.0; y = 1.0 } in
  let moved = { original with x = 9.0 } in
  original.x, moved.x
;;

let cell_identity () =
  let a = ref 1 in
  let b = ref 1 in
  let alias = a in
  a == b, a == alias
;;
