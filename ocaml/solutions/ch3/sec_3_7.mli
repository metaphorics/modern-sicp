(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.1 exercise 3.7 *)

(** Exercise 3.7: [make-joint] shares one account under a second
    password, following @ref{Exercise 3.3}'s password-protected
    account. *)

type withdraw_result =
  | Balance of int
  | Insufficient_funds

type message =
  | Withdraw of int
  | Deposit of int

type response =
  | Balance_response of withdraw_result
  | Deposit_response of int
  | Incorrect_password

(** [make_account balance password] is @ref{Exercise 3.3}'s
    password-protected account. *)
val make_account : int -> string -> string -> message -> response

(** [make_joint account account_password new_password] is a second
    access to [account], the same underlying closure, that requires
    [new_password] instead of [account]'s own password; [account]'s
    own password still works, unchanged, for direct calls to
    [account]. *)
val make_joint
  :  (string -> message -> response)
  -> string
  -> string
  -> string
  -> message
  -> response

(** [ex_3_07 ()] is the withdrawal Paul's joint access makes, and
    Peter's own view of the balance afterward (read by depositing 0,
    the account having no dedicated balance query): the two responses
    agree, showing the joint account is one shared object, not two. *)
val ex_3_07 : unit -> response * response

(** Addition 3.7a: a read-only capability over the unpassworded
    account of @ref{3.1.1}, projected out of the full account record
    rather than gated by a password check. *)

type account =
  { withdraw : int -> withdraw_result
  ; deposit : int -> int
  ; balance : unit -> int
  }

type read_only_account = { balance : unit -> int }

(** [make_full_account balance] is the unpassworded account of
    @ref{3.1.1}, a record of closures over one captured [ref]. *)
val make_full_account : int -> account

(** [read_only_capability account] is a capability exposing only
    [account]'s [balance] field: no [withdraw] or [deposit] closure is
    reachable through it, a guarantee the type checker enforces
    instead of a message dispatch refusing at run time. *)
val read_only_capability : account -> read_only_account

(** [ex_3_07a ()] is the balance the read-only capability observes
    before and after a deposit made through the full account: the
    second is larger, since the capability still sees the account's
    live state, only through a narrower interface. *)
val ex_3_07a : unit -> int * int
