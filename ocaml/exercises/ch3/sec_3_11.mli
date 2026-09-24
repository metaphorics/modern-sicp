(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.2 exercise 3.11 *)

(** Exercise 3.11 (and the edition's addition 3.11a): where an
    account's local state lives, and what identity and equality say
    about two accounts. *)

type withdraw_result =
  | Balance of int
  | Insufficient_funds

(** The record of closures over one captured ref cell that the
    statement's listing builds. *)
type account =
  { withdraw : int -> withdraw_result
  ; deposit : int -> int
  }

(** [make_account balance] is an account object whose [withdraw] and
    [deposit] fields operate on one cell allocated by this call. *)
val make_account : int -> account

(** [ex_3_11 ()] is
    [(acc.deposit 40 after make_account 50, acc.withdraw 60 after that
    deposit, the balance observed through a second account acc2 built
    with 100, whether acc and acc2 are physically distinct)], the
    observations the statement's structure drawing must explain. *)
val ex_3_11 : unit -> int * withdraw_result * int * bool

(** [ex_3_11a ()] is
    [(acc == twin, acc == alias, whether acc = twin raises
    Invalid_argument, whether the two accounts' withdrawals of 20
    answer the same, whether the accounts are still physically
    distinct after those withdrawals)] for twin built by a separate
    [make_account 100] and alias a second name for acc, the results
    the addition's identity-versus-equality checklist asks for. *)
val ex_3_11a : unit -> bool * bool * bool * bool * bool
