(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program sub-interval in SICP section 2.1
   exercise 2.8 *)

(** The smallest [a - b] comes from the smallest [a] and the largest
    [b]; the largest comes from the largest [a] and the smallest [b],
    the same reasoning [add_interval] uses with [b]'s sign flipped. *)

type interval = float * float

let make_interval a b : interval = a, b
let lower_bound ((a, _) : interval) = a
let upper_bound ((_, b) : interval) = b

let sub_interval x y =
  make_interval (lower_bound x -. upper_bound y) (upper_bound x -. lower_bound y)
;;

let ex_2_08 () = sub_interval (make_interval 6.12 7.48) (make_interval 4.465 4.935)
