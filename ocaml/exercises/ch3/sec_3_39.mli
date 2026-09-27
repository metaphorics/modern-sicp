(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.4 exercise 3.39 *)

(** Exercise 3.39: which of the five outcomes of the text's
    increment/square race remain when only the square's computation is
    serialized and the assignment is not, while the increment is
    serialized entirely. *)

(** The five possible values of the unserialized race shown in the
    text. *)
val possibilities_before : int list

(** [possibilities_after ()] is the sorted list of values the partially
    serialized race can still leave in [x]. *)
val possibilities_after : unit -> int list

(** [sample_runs n] really runs the partially serialized race [n]
    times and answers the sorted distinct finals observed. *)
val sample_runs : int -> int list

(** [ex_3_39 ()] is the answer list; every sampled run lands in it. *)
val ex_3_39 : unit -> int list
