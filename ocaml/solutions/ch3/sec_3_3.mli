(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.1 exercise 3.3 *)

(** Exercise 3.3: a password-protected account. Scheme's two-level
    application [((acc password op) amount)] flattens into one
    three-argument curried call, since OCaml never needs a returned
    procedure just to carry [amount]. *)

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

(** [make_account balance password] is a password-protected account:
    calling it with [password] and a [message] performs the operation;
    any other password answers [Incorrect_password]. *)
val make_account : int -> string -> string -> message -> response

(** [ex_3_03 ()] is the pair of responses the statement's calls to
    [((acc 'secret-password 'withdraw) 40)] and
    [((acc 'some-other-password 'deposit) 50)] produce, in order. *)
val ex_3_03 : unit -> response * response
