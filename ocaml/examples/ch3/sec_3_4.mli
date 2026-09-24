(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** The named definitions behind the listings of section 3.4, grouped by
    subsection. Concurrency is real here: [Parallel.parallel] runs two
    procedures on two OCaml 5 domains, and the serializers wrap one
    [Mutex.t] each. Every deterministic value a listing displays is
    asserted by [run_sec_3_4] through [Replay]; the interleaving demos
    are asserted by membership in their possible-outcome sets, never by
    one specific schedule. *)

(** The result of a checked withdrawal, shared with section 3.1's
    account: either the new [Balance], or [Insufficient_funds]. *)
type withdraw_result = Sec_3_1.withdraw_result =
  | Balance of int
  | Insufficient_funds

(** 3.4's opening: the account of 3.1.1 recalled as one shared [ref]
    that any number of processes can reach. *)
module Shared_withdraw : sig
  val balance : int ref
  val withdraw : int -> withdraw_result
end

(** The edition's [parallel]: the first procedure runs on a freshly
    spawned domain, the second on the calling domain, and the call
    returns only when both have finished. *)
module Parallel : sig
  val parallel : (unit -> 'a) -> (unit -> 'b) -> 'a * 'b
end

(** 3.4.2: the serializer. A serializer wraps one mutex and answers
    with the polymorphic [protect] field, so a serialized procedure
    works for every result type. [Mutex.protect] guarantees the unlock
    even when the procedure raises. *)
module Serializers : sig
  type serializer = { protect : 'a. (unit -> 'a) -> 'a }

  val make_serializer : unit -> serializer
end

(** The five-outcome increment/square race of the text and its
    serialized counterpart. Each runner returns whatever value one
    particular schedule left in [x]; only membership in the outcome
    sets is assertable, never one specific schedule. *)
module X_race : sig
  val possible_values : int list
  val serialized_values : int list
  val run_unserialized : unit -> int
  val run_serialized : unit -> int
end

(** The bank account of 3.1.1 with serialized deposits and withdrawals.
    Scheme's message dispatch becomes a function from a [message] to a
    [response]; the [Serializer] message exists for the version of the
    account that exports its serializer. *)
module Account : sig
  type message =
    | Withdraw of int
    | Deposit of int
    | Balance
    | Serializer

  type response =
    | Withdrawn of withdraw_result
    | New_balance of int
    | Serialized of Serializers.serializer

  type account = message -> response

  val make_account : int -> account

  (** Ben's variant of exercise 3.41: the balance message answers
      through the serializer too. *)
  val make_account_serialized_balance : int -> account

  (** Ben's variant of exercise 3.42: the serialized procedures are
      created once, when the account is made. *)
  val make_account_hoisted_serialization : int -> account

  (** [concurrent_deposits account n] makes [n] deposits of 1, half on
      a spawned domain and half on the calling domain, and answers the
      resulting balance. *)
  val concurrent_deposits : account -> int -> int
end

(** Exchanging balances: the account that exports its serializer, the
    plain exchange, and the exchange serialized by both accounts. *)
module Exchange : sig
  type account_and_serializer =
    { dispatch : Account.account
    ; serializer : Serializers.serializer
    }

  val make_account_and_serializer : int -> account_and_serializer
  val balance_of : account_and_serializer -> int

  (** [deposit account amount] is the user-managed serialization the
      text shows: the exported serializer protects a raw deposit. *)
  val deposit : account_and_serializer -> int -> int

  val exchange : account_and_serializer -> account_and_serializer -> unit
  val serialized_exchange : account_and_serializer -> account_and_serializer -> unit
end

(** Implementing serializers: the cell, the mutex built on it, and the
    serializer built on that mutex -- the section's own account of what
    [Mutex.protect] does underneath. [test_and_set] is the text's
    ordinary, non-atomic procedure; [atomic_test_and_set] is the
    compare-and-swap the host actually provides. *)
module Mutex_impl : sig
  val test_and_set : bool ref -> bool
  val atomic_test_and_set : bool Atomic.t -> bool

  type mutex =
    { acquire : unit -> unit
    ; release : unit -> unit
    }

  val clear : bool Atomic.t -> unit
  val make_mutex : unit -> mutex
  val make_serializer : unit -> Serializers.serializer
end
