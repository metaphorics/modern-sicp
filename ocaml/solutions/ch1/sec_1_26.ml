(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program expmod in SICP section 1.2 exercise
   1.26 *)

(** Exercise 1.26: Eva is right. Calling [expmod] twice at each even
    level, instead of calling it once and squaring the result, doubles
    the tree's branching factor at every one of its @math{{\Theta(\log
    n)}} levels: the call tree that had one path down now has two,
    each splitting again, so the call count grows like
    @math{{2^{\Theta(\log n)}} = {\Theta(n)}} instead of staying
    @math{{\Theta(\log n)}}. Both counters count every call, including
    the base case; [ex_1_26.md] reads the two counts against that
    doubling argument. *)

let is_even n = n mod 2 = 0
let square x = x * x

let expmod_square_calls base exp m =
  let calls = ref 0 in
  let rec expmod base exp m =
    incr calls;
    if exp = 0
    then 1
    else if is_even exp
    then square (expmod base (exp / 2) m) mod m
    else base * expmod base (exp - 1) m mod m
  in
  ignore (expmod base exp m);
  !calls
;;

let expmod_double_calls base exp m =
  let calls = ref 0 in
  let rec expmod base exp m =
    incr calls;
    if exp = 0
    then 1
    else if is_even exp
    then expmod base (exp / 2) m * expmod base (exp / 2) m mod m
    else base * expmod base (exp - 1) m mod m
  in
  ignore (expmod base exp m);
  !calls
;;

let ex_1_26 () = expmod_square_calls 4 64 97, expmod_double_calls 4 64 97
