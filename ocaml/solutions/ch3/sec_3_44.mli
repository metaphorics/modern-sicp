(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.4 exercise 3.44 *)

(** Exercise 3.44: Ben Bitdiddle's [transfer] withdraws from one
    account and deposits into another, without a joint lock. Louis
    Reasoner insists on the heavier exchange machinery. The exercise
    asks who is right; the edition answers with a concurrent transfer
    workload whose total never moves. *)

open Sicp_ch3.Sec_3_4.Account

(** [transfer from_account to_account amount] withdraws [amount] from
    [from_account] and deposits it into [to_account], each step
    serialized by its own account. *)
val transfer : account -> account -> int -> unit

(** [total_preserved_under_concurrent_transfers rounds per_transfer]
    runs [rounds] rounds of two concurrent transfers over three
    accounts holding 1000 each and answers whether the three balances
    sum to 3000 after every round. *)
val total_preserved_under_concurrent_transfers : int -> int -> bool

(** [ex_3_44 ()] is the preservation verdict and the final total over
    all accounts. *)
val ex_3_44 : unit -> bool * int
