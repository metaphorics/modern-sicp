(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.82 *)

(** Exercise 3.82: Monte Carlo integration as a stream of estimates
    that sharpens as it is walked. The map class is [T]: like 3.81,
    the seeded stateless generator makes every estimate replayable. *)

open Sicp_ch3.Sec_3_5

(* Uniform points of the unit square, two draws per trial, from the
   seeded stateless generator. [rand_update] folds each draw through
   the same 1e9 bound as 3.1, so the scale is that bound. *)
let unit_pairs =
  let scale r = Int64.to_float r /. 1e9 in
  Random_streams.map_successive_pairs
    (fun r1 r2 -> scale r1, scale r2)
    Random_streams.random_numbers
;;

(* [estimate_integral predicate] is the stream of running estimates of
   the predicate's area over the unit square: experiment stream, then
   the monte-carlo process, with no trial-count argument anywhere. *)
let estimate_integral predicate =
  let experiments = Streams.stream_map predicate unit_pairs in
  Random_streams.monte_carlo experiments 0 0
;;

(* The quarter disk of exercise 3.5's pi measurement: the area is
   pi/4, so four times the ratio estimates pi. *)
let in_quarter_disk (x, y) = (x *. x) +. (y *. y) <= 1.0

let pi_estimates =
  Streams.stream_map (fun ratio -> 4.0 *. ratio) (estimate_integral in_quarter_disk)
;;

let ex_3_82 () =
  let at_1000 = Streams.stream_ref pi_estimates 999 in
  let at_10000 = Streams.stream_ref pi_estimates 9999 in
  at_1000, at_10000, Float.abs (at_10000 -. 3.141592653589793) < 0.1
;;
