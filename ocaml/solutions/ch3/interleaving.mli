(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Every interleaving of [lists] that keeps each list's own order,
    built by choosing which list contributes the next event and
    recursing on what remains. *)

val interleavings : 'a list list -> 'a list list
