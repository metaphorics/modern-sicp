(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.75 *)

(** Exercise 3.75: Louis's smoothed zero-crossing detector, its bug,
    and the fix that compares each average with the previous average.
    The map class is [T]. *)

open Sicp_ch3.Sec_3_5

let sign value = if value < 0.0 then -1 else 1

let sign_change_detector value last_value =
  if sign value = sign last_value then 0 else sign value
;;

(* Louis's version: the smoothed point is compared with the raw last
   value, and the smoothed point itself is carried as the last value. *)
let rec make_zero_crossings_louis input_stream last_value =
  let avpt = (Streams.stream_car input_stream +. last_value) /. 2.0 in
  Streams.Cons
    ( sign_change_detector avpt last_value
    , lazy (make_zero_crossings_louis (Streams.stream_cdr input_stream) avpt) )
;;

(* The repair the hint points at: average each raw value with the
   previous raw value first, then extract crossings from the averages
   alone -- the detector never sees a half-raw, half-smoothed point. *)
let make_zero_crossings_smoothed input_stream =
  let smoothed =
    Infinite.stream_map2
      (fun a b -> (a +. b) /. 2.0)
      input_stream
      (Streams.stream_cdr input_stream)
  in
  let rec go s prev =
    Streams.Cons
      ( sign_change_detector (Streams.stream_car s) prev
      , lazy (go (Streams.stream_cdr s) (Streams.stream_car s)) )
  in
  go smoothed 0.0
;;

let rec ones_float = Streams.Cons (1.0, lazy ones_float)

(* A noisy version of the 3.74 sample: the signal dips below zero
    once and comes back up, with sensor noise around both swings. *)
let noisy_signal =
  List.fold_right
    (fun x acc -> Streams.cons_stream x (fun () -> acc))
    [ 1.0; 2.0; 1.5; 1.0; 0.6; -0.1; 0.3; -0.2; -2.0; -3.0; -2.0; -0.5; 0.2; 3.0; 4.0 ]
    ones_float
;;

let ex_3_75 () =
  let louis = Streams.stream_take 14 (make_zero_crossings_louis noisy_signal 0.0) in
  let fixed = Streams.stream_take 14 (make_zero_crossings_smoothed noisy_signal) in
  let flips crossings = List.length (List.filter (fun c -> c <> 0) crossings) in
  louis, fixed, flips louis, flips fixed
;;
