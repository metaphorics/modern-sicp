(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program sine in SICP section 1.2 exercise
   1.15 *)

(** Exercise 1.15: [sine] divides its angle by 3 until it is at most
    0.1 in magnitude, so the recursion depth (and the number of [p]
    applications) is the smallest [k] with
    [angle / 3^k <= 0.1], hence @math{{\Theta(\log a)}} steps and, since
    each level waits for the one below it to return, @math{{\Theta(\log
    a)}} space as well: (a) *)

let cube x = x *. x *. x
let p x = (3.0 *. x) -. (4.0 *. cube x)

let sine_with_count angle =
  let count = ref 0 in
  let rec sine angle =
    if not (Float.abs angle > 0.1)
    then angle
    else (
      incr count;
      p (sine (angle /. 3.0)))
  in
  let result = sine angle in
  result, !count
;;

let ex_1_15 () = snd (sine_with_count 12.15)
