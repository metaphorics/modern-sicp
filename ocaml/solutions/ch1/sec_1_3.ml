(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program ex_1_03 in SICP section 1.1 *)

(** Exercise 1.3: drop the smallest of the three, then square and
    add. The [min] route avoids the three-way case split. *)

let ex_1_03 a b c =
  let smallest = min a (min b c) in
  let square x = x * x in
  if a = smallest
  then square b + square c
  else if b = smallest
  then square a + square c
  else square a + square b
;;
