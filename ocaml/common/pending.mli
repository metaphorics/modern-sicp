(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** The pending marker of an unsolved exercise: [dune build @scaffold]
    surfaces it and [dune runtest] never runs the stubs that raise it. *)

exception Pending_solution
