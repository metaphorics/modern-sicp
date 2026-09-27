(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** The edition's typed error, returned through [result] by the runtime. *)

type t = Zero_seed (** [Zero_seed] is the rejected seed of a random generator. *)
