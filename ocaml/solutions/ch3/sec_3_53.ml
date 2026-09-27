(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.53 *)

(** Exercise 3.53: what is [s = 1 : (add-streams s s)]? The map class
    is [T]: the self-referential definition translates directly, and
    the answer is read off the computed prefix instead of predicted
    only. *)

open Sicp_ch3.Sec_3_5

let rec s = Streams.Cons (1, lazy (Infinite.add_streams s s))
let ex_3_53 () = Streams.stream_take 8 s
