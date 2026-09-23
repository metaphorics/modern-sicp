(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program f in SICP section 1.3
   exercise 1.34 *)

(** Exercise 1.34: [f g = g 2], applied to procedures, then to itself.
    The stub raises [Sicp_common.Pending.Pending_solution] until it is
    solved; [ex_1_34.md] carries the answer to "what happens with
    [f f]", since that call itself must never appear in compiled
    source. *)

(** [f g] is [g 2]; OCaml infers the principal type
    [(int -> 'a) -> 'a], since nothing in the body constrains ['a]. *)
val f : (int -> 'a) -> 'a

(** [ex_1_34 ()] is [(f square, f (fun z -> z * (z + 1)))]. *)
val ex_1_34 : unit -> int * int
