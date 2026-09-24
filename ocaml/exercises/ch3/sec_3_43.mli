(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.4 exercise 3.43 *)

(** Exercise 3.43: three accounts start at 10, 20, and 30 and
    processes run concurrent exchanges. The exercise asks why
    sequential runs preserve the multiset of balances, how the plain
    exchange violates it even with per-account serialization, and why
    the sum survives anyway. The edition's accounts here move balances
    without an insufficient-funds check, matching the text's own
    simplification that its deposit accepts negative amounts. *)

(** An account whose withdraw and deposit always succeed: the exchange
    experiments never want the insufficient-funds branch. *)
type raw_account = private
  { dispatch : Sicp_ch3.Sec_3_4.Account.account
  ; serializer : Sicp_ch3.Sec_3_4.Serializers.serializer
  }

val make_raw_account : int -> raw_account
val balance_of : raw_account -> int

(** [exchange a1 a2] is the text's first version: it reads both
    balances, withdraws the difference from [a1], and deposits it into
    [a2], with each step serialized only per account. *)
val exchange : raw_account -> raw_account -> unit

(** [serialized_exchange a1 a2] runs [exchange] under both accounts'
    serializers, so a whole exchange is atomic. *)
val serialized_exchange : raw_account -> raw_account -> unit

(** [multiset_preserved_by_serialized_exchange rounds balances] runs
    [rounds] rounds of two concurrent serialized exchanges over three
    accounts opened with [balances] (three amounts) and answers
    whether the sorted balances are the original amounts after every
    round. *)
val multiset_preserved_by_serialized_exchange : int -> int list -> bool

(** [sum_preserved_by_plain_exchange rounds balances] runs the same
    workload with the plain exchange and answers whether the sum of
    the balances is unchanged after every round. *)
val sum_preserved_by_plain_exchange : int -> int list -> bool

(** [ex_3_43 ()] is the serialized-exchange multiset verdict, the
    plain-exchange sum verdict, and the number of plain rounds whose
    multiset came out wrong (an observed count, never an asserted
    one). *)
val ex_3_43 : unit -> bool * bool * int
