(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** The assertion helper behind the section replays. The executable of each
    chapter section prints the value its listings show and proves it with
    [expect], so the book's result comments are true by construction. *)

(** [expect actual expected] prints [actual] and exits nonzero when it
    differs from [expected]. *)
val expect : string -> string -> unit

(** [expect_float computed shown] prints [shown] and exits nonzero when
    [shown] does not parse back to exactly [computed] under
    [Float.equal], so the digits a listing displays are pinned to the
    computed value however a REPL formats floats. *)
val expect_float : float -> string -> unit
