(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 1.20: OCaml evaluates every call's arguments once, before
    the call, exactly as Scheme's applicative order does; wrapping
    [remainder] in a counter measures it directly. The normal-order
    count is an argument, not a measurement (OCaml has no
    substitute-then-evaluate mode to instrument), and is worked out
    step by step in [ex_1_20.md]: the four predicates containing
    [remainder]s force 1, 2, 4, 7 calls, then the returned [a] is
    forced once more for 4 — 18 in all. *)

let remainder_calls = ref 0

let remainder a b =
  incr remainder_calls;
  a mod b
;;

let rec gcd a b = if b = 0 then a else gcd b (remainder a b)

let gcd_traced a b =
  remainder_calls := 0;
  let value = gcd a b in
  value, !remainder_calls
;;

let ex_1_20 () = gcd_traced 206 40
