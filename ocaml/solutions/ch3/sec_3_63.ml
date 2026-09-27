(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.63 *)

(** Exercise 3.63: why does [sqrt-stream] bind [guesses] locally? The
    map class is [T]: both versions run with a counting improvement
    step, and the two prefixes answer the question -- and show it is
    about more than efficiency. *)

open Sicp_ch3.Sec_3_5

let sqrt_improve guess x = Convergence.average guess (x /. guess)

(* Alyssa's version: one shared memoized guesses stream. *)
let sqrt_stream_local x calls =
  let rec guesses =
    Streams.Cons
      ( 1.0
      , lazy
          (Streams.stream_map
             (fun g ->
                incr calls;
                sqrt_improve g x)
             guesses) )
  in
  guesses
;;

(* The version without the local binding: every tail builds a fresh
   stream from the initial guess. *)
let sqrt_stream_open x calls =
  let rec go () =
    Streams.Cons
      ( 1.0
      , lazy
          (Streams.stream_map
             (fun g ->
                incr calls;
                sqrt_improve g x)
             (go ())) )
  in
  go ()
;;

let ex_3_63 () =
  let local_calls = ref 0 in
  let open_calls = ref 0 in
  (* stream_ref, not stream_take: take forces one tail past the last
     requested element, which would spend one call on an element the
     question never asks about. *)
  let prefix s =
    let rec go k acc =
      if k < 0 then acc else go (k - 1) (Streams.stream_ref s k :: acc)
    in
    go 4 []
  in
  let local = prefix (sqrt_stream_local 2.0 local_calls) in
  let open_ = prefix (sqrt_stream_open 2.0 open_calls) in
  local, !local_calls, open_, !open_calls
;;
