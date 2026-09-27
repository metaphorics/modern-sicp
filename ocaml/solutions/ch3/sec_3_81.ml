(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.81 *)

(** Exercise 3.81: the random-number generator as a stream process over
    requests, with no assignment. The map class is [T]; the tailored
    note in the map column asks for replayable tests, which the fixed
    seed delivers. *)

open Sicp_ch3.Sec_3_5

type request =
  | Generate
  | Reset of Int64.t

let rand_stream requests seed0 =
  let rec go requests current =
    let value =
      match Streams.stream_car requests with
      | Generate -> Random_streams.rand_update current
      | Reset value -> value
    in
    Streams.Cons (value, lazy (go (Streams.stream_cdr requests) value))
  in
  go requests seed0
;;

let sample_requests =
  List.fold_right
    (fun r acc -> Streams.cons_stream r (fun () -> acc))
    [ Generate; Generate; Reset 42L; Generate; Generate; Reset 7L; Generate ]
    Streams.the_empty_stream
;;

let ex_3_81 () =
  (* stream_ref, not stream_take: take forces one request past the
     last, and the script has none. *)
  let first =
    List.init 7 (fun k -> Streams.stream_ref (rand_stream sample_requests 42L) k)
  in
  let replay =
    List.init 7 (fun k -> Streams.stream_ref (rand_stream sample_requests 42L) k)
  in
  first, first = replay
;;
