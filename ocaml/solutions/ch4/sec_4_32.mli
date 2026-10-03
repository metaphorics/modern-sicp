(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.32: chapter 3 streams versus the lazier lazy lists.
    Under the lazy experiment the program's own [cons] is a compound
    procedure, so it delays the element as well as the tail, and no
    element of a lazy list is produced until something forces it --
    never, for a skipped element.  A chapter 3 stream under the strict
    core evaluates its element when the cell is built and delays only
    the tail, so the same construction with an armed first element
    dies before the stream exists.  The armed tail of a two-element
    lazy list is skipped just as freely.  Delayed versions of general
    structures, such as lazy trees, fall out of the same non-strict
    [cons]. *)

(** [ex_4_32 ()] answers, in order: the second element of a lazy list
    whose first element is armed; the same construction as a chapter 3
    stream; and the first element of a lazy list whose tail is
    armed. *)
val ex_4_32 : unit -> string list
