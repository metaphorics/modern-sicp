(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.52 *)

(** Exercise 3.52: [accum] adds each mapped element into a shared
    [sum], and the transcript of sums reveals the interleaving of
    laziness and assignment. The map class is [A]: Scheme's [set!] sum
    becomes one [int ref], and the "would the answers differ without
    memoization" question is answered by running the same transcript
    over plain thunks, not only argued. *)

(** The non-memoized stream of the exercise's last question: a plain
    thunk for the tail, re-evaluated on every access. *)
type 'a thunk_stream =
  | TCons of 'a * (unit -> 'a thunk_stream)
  | TEmpty

val thunk_map : ('a -> 'b) -> 'a thunk_stream -> 'b thunk_stream
val thunk_filter : ('a -> bool) -> 'a thunk_stream -> 'a thunk_stream
val thunk_ref : 'a thunk_stream -> int -> 'a
val thunk_take : int -> 'a thunk_stream -> 'a list

(** [ex_3_52 ()] is the memoized transcript and the plain-thunk
    transcript, each in the order of the statement: the sum after
    [seq], after [y], after [z], the eighth even element [y7], the sum
    after [stream-ref y 7], the elements [display-stream z] shows, and
    the sum after the display. The two transcripts differ in the sums
    and in the displayed list, which is the exercise's answer. *)
val ex_3_52
  :  unit
  -> (int * int * int * int * int * int list * int)
     * (int * int * int * int * int * int list * int)
