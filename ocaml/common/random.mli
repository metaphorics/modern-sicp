(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** The seeded xorshift64* generator behind the book's [random]. Every
    edition ships the same generator, so exercise 1.24 and the Monte Carlo
    sections are deterministic under test. *)

(** A generator whose state advances as values are drawn. *)
type t

(** [create seed] is a generator seeded with [seed], or [Error Zero_seed]
    when [seed] is zero: xorshift64* cannot run from the zero state. *)
val create : int64 -> (t, Error.t) result

(** [next_u64 t] advances [t] and returns the next unsigned 64-bit word. *)
val next_u64 : t -> Int64.t

(** [random t n] is a value in [0 .. n - 1]; [n] must be positive. *)
val random : t -> int -> int
