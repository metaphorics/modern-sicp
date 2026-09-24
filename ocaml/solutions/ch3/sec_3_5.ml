(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.1 exercise 3.5 *)

(** Exercise 3.5: Monte Carlo integration, estimating pi by the area
    of a unit circle. Following @ref{3.1}'s note, the generator is a
    [Sicp_common.Random.t] passed explicitly, not hidden behind
    OCaml's own global [Random] state. *)

let scale = 1_000_000

let random_in_range generator low high =
  let range = high -. low in
  let fraction =
    float_of_int (Sicp_common.Random.random generator scale) /. float_of_int scale
  in
  low +. (range *. fraction)
;;

let monte_carlo trials experiment =
  let rec iter trials_remaining trials_passed =
    if trials_remaining = 0
    then float_of_int trials_passed /. float_of_int trials
    else if experiment ()
    then iter (trials_remaining - 1) (trials_passed + 1)
    else iter (trials_remaining - 1) trials_passed
  in
  iter trials 0
;;

let estimate_integral predicate x1 x2 y1 y2 trials generator =
  let experiment () =
    let x = random_in_range generator x1 x2 in
    let y = random_in_range generator y1 y2 in
    predicate x y
  in
  monte_carlo trials experiment *. (x2 -. x1) *. (y2 -. y1)
;;

let fixed_seed = 42L

let fresh_generator () =
  match Sicp_common.Random.create fixed_seed with
  | Ok generator -> generator
  | Error Sicp_common.Error.Zero_seed -> failwith "the fixed seed 42 is nonzero"
;;

let ex_3_05 () =
  let unit_circle x y = (x *. x) +. (y *. y) <= 1.0 in
  estimate_integral unit_circle (-1.0) 1.0 (-1.0) 1.0 5000 (fresh_generator ())
;;
