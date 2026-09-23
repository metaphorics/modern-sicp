(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program make-interval in SICP section 2.1
   exercise 2.7 *)

type interval = float * float

let make_interval a b : interval = a, b
let lower_bound ((a, _) : interval) = a
let upper_bound ((_, b) : interval) = b

let ex_2_07 () =
  lower_bound (make_interval 6.12 7.48), upper_bound (make_interval 6.12 7.48)
;;
