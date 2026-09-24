(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.51 *)

(** Exercise 3.51: [show] prints its argument and returns it, and the
    transcript reveals when memoized [delay] evaluates what. The map
    class is [T]: printing becomes appending to a log the exercise
    answers, so the transcript is data rather than console noise. The
    tailored addition 3.51a counts force operations explicitly. *)

(** [ex_3_51 ()] is what [show] has logged after the stream is built,
    after [stream_ref x 5], and after [stream_ref x 7], in that order:
    [0], then [0..5], then [0..7]. *)
val ex_3_51 : unit -> int list * int list * int list

(** One instrumented-stream experiment: two counters, [forces] for
    every tail access and [bodies] for every tail thunk that ran. *)
type counters

(** [counted_cons ctr head tail] pairs [head] with one counted memoized
    promise for [tail ()], and [counted_cdr]/[counted_ref] force only
    through the counters. *)
val counted_cons
  :  counters
  -> 'a
  -> (unit -> 'a Sicp_ch3.Sec_3_5.Streams.stream)
  -> 'a Sicp_ch3.Sec_3_5.Streams.stream

val counted_cdr
  :  counters
  -> 'a Sicp_ch3.Sec_3_5.Streams.stream
  -> 'a Sicp_ch3.Sec_3_5.Streams.stream

val counted_ref : counters -> 'a Sicp_ch3.Sec_3_5.Streams.stream -> int -> 'a

(** [ex_3_51a ()] is the show counts after definition, after [ref 5],
    and after [ref 7] ([1], [6], [8]), followed by the counters: 12
    tail accesses ran only 7 tail bodies, which is memoization made
    arithmetic. *)
val ex_3_51a : unit -> int * int * int * int * int
