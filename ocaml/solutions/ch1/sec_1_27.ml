(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program expmod in SICP section 1.2 exercise
   1.27 *)

(** Exercise 1.27: [fools_fermat] checks every witness [a] from 1 to
    [n - 1] exhaustively, unlike [Primality.fermat_test]'s random
    sample, so it proves the Carmichael property instead of merely
    suggesting it. *)

let is_even n = n mod 2 = 0
let square x = x * x

let rec expmod base exp m =
  if exp = 0
  then 1
  else if is_even exp
  then square (expmod base (exp / 2) m) mod m
  else base * expmod base (exp - 1) m mod m
;;

let fools_fermat n =
  let rec check a = a >= n || (expmod a n n = a mod n && check (a + 1)) in
  check 1
;;

let carmichael_numbers = [ 561; 1105; 1729; 2465; 2821; 6601 ]
let ex_1_27 () = List.map (fun n -> n, fools_fermat n) carmichael_numbers
