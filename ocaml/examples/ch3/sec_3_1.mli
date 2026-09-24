(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** The named definitions behind the listings of section 3.1, grouped by
    subsection. Each value a listing displays is asserted by
    [run_sec_3_1] through [Replay], so the book's result comments are
    true by construction.

    The section's hard spot is object identity: once [withdraw] and
    [deposit] close over a captured [ref], two accounts built from the
    same call are still two distinct closures over two distinct cells,
    while two names bound to one call share the one cell underneath.
    Structural equality on a closure is not available in OCaml (a
    function value cannot be compared with [=]), so this edition never
    reaches for it; the sections below show sameness by exhibiting
    shared state directly instead. *)

(** The result of a checked withdrawal: either the new [Balance], or
    [Insufficient_funds] when the account does not hold enough. *)
type withdraw_result =
  | Balance of int
  | Insufficient_funds

(** Subsection 3.1.1's first version: [balance] is an ordinary
    top-level [ref], reachable (and mutable) from anywhere in the
    module. *)
module Global_withdraw : sig
  val balance : int ref
  val withdraw : int -> withdraw_result
end

(** [new_withdraw] closes [balance] inside a [let]-bound [ref]: no
    other binding in the module can reach it directly. *)
module New_withdraw : sig
  val new_withdraw : int -> withdraw_result
end

(** [make_withdraw] makes [balance] a formal parameter turned local
    state; each call builds a fresh, independent [ref]. *)
module Make_withdraw : sig
  val make_withdraw : int -> int -> withdraw_result
end

(** The bank-account object of subsection 3.1.1: a record of closures
    over one captured [ref], replacing Scheme's message-passing
    [dispatch] procedure. *)
module Make_account : sig
  type account =
    { withdraw : int -> withdraw_result
    ; deposit : int -> int
    }

  val make_account : int -> account
end

(** Subsection 3.1.2's random-number generator. [rand] is a
    zero-argument closure over a [Sicp_common.Random.t]: the
    generator's own mutable field, not a separately captured [ref],
    carries the local state, since the generator is already a mutable
    cell. *)
module Rand : sig
  val random_init : int64
  val bound : int
  val rand : unit -> int
end

(** The Monte Carlo estimate of pi, encapsulating the random-number
    generator inside [Rand.rand]. *)
module Monte_carlo : sig
  val cesaro_test : unit -> bool
  val monte_carlo : int -> (unit -> bool) -> float
  val estimate_pi : int -> float
end

(** The alternate version that does not encapsulate the generator:
    every call in the trial loop takes it as an explicit argument
    instead. *)
module Random_gcd_test : sig
  val random_gcd_test : int -> Sicp_common.Random.t -> float
  val estimate_pi : int -> Sicp_common.Random.t -> float
end

(** Subsection 3.1.3's simplified withdrawer, with no
    insufficient-funds check. *)
module Make_simplified_withdraw : sig
  val make_simplified_withdraw : int -> int -> int
end

(** [make_decrementer], the [set!]-free contrast: no local state
    survives between calls. *)
module Make_decrementer : sig
  val make_decrementer : int -> int -> int
end

(** The imperative rewrite of @ref{1.2.1}'s iterative [factorial],
    using [ref] and explicit sequencing instead of extra loop
    parameters. *)
module Factorial_imperative : sig
  val factorial : int -> int
end
