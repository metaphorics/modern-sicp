(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.4 exercise 3.45 *)

(** Exercise 3.45: Louis Reasoner would serialize the account's own
    deposits and withdrawals with the very serializer the account
    exports. The exercise asks what goes wrong when
    [serialized-exchange] is called: the exchange protects itself with
    a serializer and then calls a procedure protected by that same
    serializer. *)

open Sicp_ch3.Sec_3_4.Account

(** [make_louis_account initial] is Louis's account: withdrawals and
    deposits answer through the serializer, and the same serializer is
    exported by the [Serializer] message. *)
val make_louis_account : int -> account

(** [serializer_of account] sends the [Serializer] message and answers
    the exported serializer. *)
val serializer_of : account -> Sicp_ch3.Sec_3_4.Serializers.serializer

(** [louis_serialized_exchange a1 a2] runs the text's serialized
    exchange over Louis's accounts: [a1]'s serializer protects the
    whole exchange, [a2]'s the inner run, and the exchange body then
    calls [a1]'s already-protected withdrawal. *)
val louis_serialized_exchange : account -> account -> unit

(** [inner_acquire_is_impossible ()] demonstrates the fact Louis's
    design collides with: a mutex a domain already holds cannot be
    acquired again by that domain. *)
val inner_acquire_is_impossible : unit -> bool

(** [exchange_stalls ()] really runs [louis_serialized_exchange] on a
    spawned domain and answers whether it has failed to complete after
    a bounded spin; the call never blocks, and the stalled domain is
    left behind for the runtime to discard at exit. *)
val exchange_stalls : unit -> bool

(** [ex_3_45 ()] is the impossibility probe and the stall
    demonstration, in that order. *)
val ex_3_45 : unit -> bool * bool
