(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.50 *)

(** Exercise 3.50: the generalized [stream-map]. The map class is [A]:
    Scheme's variadic procedure over any number of argument streams
    becomes a function from a list of streams to a stream, since OCaml
    procedures have fixed arity; [proc] receives the list of current
    heads. *)

(** [stream_map_multi proc streams] applies [proc] to the list of the
    streams' heads and continues with the tails, ending as soon as any
    stream is empty -- this edition's shape for the book's variadic
    [stream-map]. *)
val stream_map_multi
  :  ('a list -> 'b)
  -> 'a Sicp_ch3.Sec_3_5.Streams.stream list
  -> 'b Sicp_ch3.Sec_3_5.Streams.stream

(** [ex_3_50 ()] is the first six elementwise sums of the integers and
    the integers from 2, and whether a drained argument stream ends
    the whole mapped stream. *)
val ex_3_50 : unit -> int list * bool
