(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.17: the extra frame of the scanned-out let. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** [frame_count_sequential] and [frame_count_scanned] count the
      frames live at the marked point of the text example, under
      sequential definition and under the scan-out. *)
val frame_count_sequential : unit -> int

val frame_count_scanned : unit -> int

(** [ex_4_17 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_17 : unit -> string list
