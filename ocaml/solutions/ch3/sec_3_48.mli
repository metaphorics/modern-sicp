(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.4 exercise 3.48 *)

(** Exercise 3.48: number the accounts and have every serialized
    exchange protect the lower-numbered account first. The exercise
    asks why the ordering removes the deadlock and to rewrite
    [serialized-exchange] to use it. *)

open Sicp_ch3.Sec_3_4.Account

(** An account carrying the unique number the deadlock-avoidance
    scheme needs. *)
type numbered_account = private
  { id : int
  ; dispatch : account
  ; serializer : Sicp_ch3.Sec_3_4.Serializers.serializer
  }

(** [make_numbered_account id initial] opens the account with the
    given number. *)
val make_numbered_account : int -> int -> numbered_account

val balance_of : numbered_account -> int

(** [ordered_serialized_exchange a1 a2] runs the exchange under both
    accounts' serializers, always entering the lower-numbered one
    first, so no two exchanges can hold each other's first lock. *)
val ordered_serialized_exchange : numbered_account -> numbered_account -> unit

(** [survives_reversed_concurrent_exchanges rounds] repeats the text's
    deadlock scenario, one process exchanging accounts 1 and 2 while
    another exchanges 2 and 1, and answers whether every round
    completed with the multiset of balances intact. *)
val survives_reversed_concurrent_exchanges : int -> bool

(** [ex_3_48 ()] is the survival verdict and the final sorted balances
    of the three accounts. *)
val ex_3_48 : unit -> bool * int list
