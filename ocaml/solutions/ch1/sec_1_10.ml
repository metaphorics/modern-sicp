(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program A in SICP section 1.2 exercise 1.10 *)

(** Exercise 1.10: [f n] is [ackermann 0 n], which doubles [n]: [2n].
    [g n] is [ackermann 1 n], which is [2^n] for [n >= 1] ([g 0] is 0).
    [h n] is [ackermann 2 n], a tower of [n] twos for [n >= 1] ([h 0] is
    0): [h 1 = 2], [h 2 = 2^2 = 4], [h 3 = 2^(2^2) = 16],
    [h 4 = 2^(2^(2^2)) = 65536]. [k n] is [5 * n * n], stated by the
    book as the pattern the other three answer in kind. *)

let rec ackermann x y =
  if y = 0
  then 0
  else if x = 0
  then 2 * y
  else if y = 1
  then 2
  else ackermann (x - 1) (ackermann x (y - 1))
;;

let f n = ackermann 0 n
let g n = ackermann 1 n
let h n = ackermann 2 n
let k n = 5 * n * n
let ex_1_10 () = ackermann 1 10, ackermann 2 4, ackermann 3 3
