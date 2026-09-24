(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.4 exercise 3.42 *)

(** Exercise 3.42: Ben Bitdiddle hoists the creation of the serialized
    procedures out of the account's message dispatch. The exercise
    asks whether the change is safe, that is, whether the two versions
    of the account allow different concurrency. *)

open Sicp_ch3.Sec_3_4.Account

(** [with_per_message_serialization initial] is the account of the
    text: each [Withdraw] or [Deposit] message is answered by a
    serialized procedure created for that message. *)
val with_per_message_serialization : int -> account

(** [with_hoisted_serialization initial] is Ben's version: the two
    serialized procedures are created once, when the account is made. *)
val with_hoisted_serialization : int -> account

(** [run_concurrent_deposits account n] makes [n] deposits of 1, half
    on a spawned domain and half on the calling domain, and answers
    the resulting balance. *)
val run_concurrent_deposits : account -> int -> int

(** [ex_3_42 ()] is the final balance under the two versions for the
    same concurrent deposit workload. *)
val ex_3_42 : unit -> int * int
