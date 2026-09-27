(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.50 *)

(** Exercise 3.50: the generalized [stream-map]. The map class is [A]:
    Scheme's variadic procedure over any number of argument streams
    becomes a function from a list of streams to a stream, since OCaml
    procedures have fixed arity; [proc] receives the list of current
    heads. *)

open Sicp_ch3.Sec_3_5

let rec stream_map_multi proc streams =
  if List.exists Streams.stream_null streams
  then Streams.the_empty_stream
  else
    Streams.cons_stream
      (proc (List.map Streams.stream_car streams))
      (fun () -> stream_map_multi proc (List.map Streams.stream_cdr streams))
;;

let ex_3_50 () =
  let add_heads heads = List.fold_left ( + ) 0 heads in
  let sums =
    stream_map_multi add_heads [ Infinite.integers; Infinite.integers_starting_from 2 ]
  in
  let drained =
    stream_map_multi add_heads [ Infinite.integers; Streams.the_empty_stream ]
  in
  Streams.stream_take 6 sums, Streams.stream_null drained
;;
