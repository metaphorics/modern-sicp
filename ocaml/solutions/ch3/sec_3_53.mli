(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.53 *)

(** Exercise 3.53: what is [s = 1 : (add-streams s s)]? The map class
    is [T]: the self-referential definition translates directly, and
    the answer is read off the computed prefix instead of predicted
    only. *)

(** [s] is 1 followed by the stream added to itself: the powers of
    two, 1, 2, 4, 8, 16, ... -- each element doubles the one before
    because the tail adds the stream to itself. *)
val s : int Sicp_ch3.Sec_3_5.Streams.stream

(** [ex_3_53 ()] is the first eight elements of [s]:
    [1; 2; 4; 8; 16; 32; 64; 128]. *)
val ex_3_53 : unit -> int list
