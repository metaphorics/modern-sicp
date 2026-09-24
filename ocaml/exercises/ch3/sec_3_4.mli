(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.1 exercise 3.4 *)

(** Exercise 3.4: @ref{Exercise 3.3}'s account with a second local
    state variable, a consecutive-bad-password counter, gating
    [call-the-cops]. *)

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
  | Police_called

(** [make_account balance password] answers [Police_called] once the
    account has seen more than seven consecutive incorrect passwords;
    a correct password resets the count to zero before the requested
    operation runs. *)
val make_account : int -> string -> string -> message -> response

(** [ex_3_04 ()] is the responses to eight consecutive withdrawals
    made with the wrong password, in order: the first seven are
    [Incorrect_password] and the eighth is [Police_called]. *)
val ex_3_04 : unit -> response list
