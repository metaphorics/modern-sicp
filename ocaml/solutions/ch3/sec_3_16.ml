(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.16 *)

(** Exercise 3.16: Ben's [count_pairs] counts paths, not pairs, so the
    same three physical pairs can be counted as 3, as 4, as 7, or never
    finish counting at all, depending on how they are shared.
    [ex_3_16] builds the four structures and records Ben's answers;
    [count_pairs_bounded] is the bounded wrapper whose [None] shows the
    cyclic structure never finishes. *)

open Sicp_ch3.Sec_3_3.Mpairs

let rec count_pairs x =
  if is_pair x then count_pairs (car x) + count_pairs (cdr x) + 1 else 0
;;

(* [count_pairs_bounded fuel x] is Ben's count, or [None] when the walk
   would visit more than [fuel] pairs. *)
let count_pairs_bounded fuel x =
  let steps = ref 0 in
  let exceeded = ref false in
  let rec go x =
    if is_pair x
    then (
      incr steps;
      if !steps > fuel
      then (
        exceeded := true;
        0)
      else go (car x) + go (cdr x) + 1)
    else 0
  in
  let n = go x in
  if !exceeded then None else Some n
;;

let ex_3_16 () =
  (* exactly three pairs, no sharing: Ben answers 3. *)
  let three = mcons (msym "a") (mcons (msym "b") (mcons (msym "c") mnil)) in
  (* three pairs, one shared between a car slot and a cdr slot: 4. *)
  let shared = mcons (msym "b") mnil in
  let four = mcons shared (mcons shared mnil) in
  (* three pairs, each shared into both slots of its parent: 7. *)
  let p = mcons (msym "a") mnil in
  let q = mcons p p in
  let seven = mcons q q in
  (* three pairs in a ring: never returns. *)
  let ring = mcons (msym "a") (mcons (msym "b") (mcons (msym "c") mnil)) in
  set_cdr (last_pair ring) ring;
  ( count_pairs three
  , count_pairs four
  , count_pairs seven
  , count_pairs_bounded 1000 three
  , count_pairs_bounded 1000 four
  , count_pairs_bounded 1000 seven
  , count_pairs_bounded 1000 ring )
;;
