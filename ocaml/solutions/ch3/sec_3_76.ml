(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.76 *)

(** Exercise 3.76: [smooth] as a reusable combinator, and the
    zero-crossing detector rebuilt on top of it. The map class is
    [T]. *)

open Sicp_ch3.Sec_3_5

let smooth s = Infinite.stream_map2 (fun a b -> (a +. b) /. 2.0) s (Streams.stream_cdr s)

let zero_crossings sense_data =
  let smoothed = smooth sense_data in
  Infinite.stream_map2
    Sec_3_74.sign_change_detector
    smoothed
    (Streams.cons_stream 0.0 (fun () -> smoothed))
;;

let rec zeros_float = Streams.Cons (0.0, lazy zeros_float)

(* A finite prefix read into an infinite stream, held at 0.0 forever
   after: real sensor signals never end, and the combinators of the
   section demand one element past any finite prefix. *)
let extending xs =
  let rec append a b =
    match a with
    | Streams.Cons (x, tail) ->
      Streams.cons_stream x (fun () -> append (Lazy.force tail) b)
    | Streams.Empty -> b
  in
  append
    (List.fold_right
       (fun x acc -> Streams.cons_stream x (fun () -> acc))
       xs
       Streams.the_empty_stream)
    zeros_float
;;

let ex_3_76 () =
  ( Streams.stream_take 3 (smooth (extending [ 1.0; 2.0; 3.0; 4.0 ]))
  , Streams.stream_take
      14
      (zero_crossings
         (extending
            [ 1.0; 2.0; 1.5; 1.0; 0.5; -0.1; -2.0; -3.0; -2.0; -0.5; 0.2; 3.0; 4.0 ])) )
;;
