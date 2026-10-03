(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.27: lazy [id] with a counter.  Evaluating [id (id 10)]
    applies the outer [id] at once, so its assignment runs and [count]
    becomes 1, while the operand [id 10] stays a thunk.  Demanding [w]
    forces that thunk: the inner [id] advances [count] to 2 and its
    result 10 fills the memo cell, so demanding [w] again answers 10
    without running [id] and [count] stays 2. *)

(** [ex_4_27 ()] answers the interaction's printed lines in order:
    [count], [w], [count], [w] again, [count], then the experiment's
    four thunk counts. *)
val ex_4_27 : unit -> string list
