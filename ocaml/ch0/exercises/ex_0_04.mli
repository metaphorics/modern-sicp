(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 0.4's statement is in the book, section 0.7; the account
    record and [make_account] below are what it asks you to build. *)

(** An account is a record of three closures over the same private
    balance: deposit an amount and get the new balance, withdraw an
    amount and get the new balance, or read the balance. *)
type account =
  { deposit : float -> float
  ; withdraw : float -> float
  ; balance : unit -> float
  }

(** [make_account initial] is a fresh account holding [initial]. *)
val make_account : float -> account

(** [make_shared_accounts initial] is the broken variant: two accounts
    built over one shared [ref], so each observes the other's
    transactions. *)
val make_shared_accounts : float -> account * account
