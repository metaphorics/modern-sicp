(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.21 *)

(** Exercise 3.21: the naive pointer rendering versus [print_queue]. *)

open Sicp_ch3.Sec_3_3

(** [ben_view q] is the printed form of the pair-of-pointers a naive
    printer shows for [q]: the book's ((a) a) responses. *)
val ben_view : Queue.t -> string

(** [print_queue q] is the sequence of items in [q], in parentheses;
    the empty queue prints as [()]. *)
val print_queue : Queue.t -> string

(** [ex_3_21 ()] is [(the naive view after each of Ben's four
    operations, the print_queue answer afterwards, whether the queue is
    empty, the naive view at the end)], reproducing the statement's
    transcript and Eva Lu's explanation. *)
val ex_3_21 : unit -> string * string * string * string * string * bool * string
