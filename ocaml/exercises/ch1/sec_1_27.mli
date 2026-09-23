(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program expmod in SICP section 1.2 exercise
   1.27 *)

(** Exercise 1.27: demonstrate that the Carmichael numbers of
    @ref{Footnote 47} really do fool the Fermat test. The stubs raise
    [Sicp_common.Pending.Pending_solution] until they are solved. *)

val expmod : int -> int -> int -> int

(** [fools_fermat n] tests every [a] from 1 to [n - 1], exhaustively,
    for [a ** n = a (mod n)]. *)
val fools_fermat : int -> bool

(** [ex_1_27 ()] pairs each of the six listed Carmichael numbers with
    [fools_fermat] applied to it. *)
val ex_1_27 : unit -> (int * bool) list
