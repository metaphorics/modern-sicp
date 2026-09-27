(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.4 exercise 3.45 *)

(** Exercise 3.45: Louis wants [make-account] to serialize its own
    withdrawals and deposits with the very serializer it exports.
    [serialized-exchange] then protects the whole exchange with
    [account1]'s serializer before calling [exchange], which itself
    calls [account1]'s already-protected withdrawal -- a second
    acquisition of a mutex the calling domain already holds. Stdlib's
    [Mutex] does not support that: the acquisition blocks forever. *)

open Sicp_ch3.Sec_3_4.Account

let make_louis_account initial =
  let balance = ref initial in
  let withdraw amount =
    if !balance >= amount
    then (
      balance := !balance - amount;
      Sicp_ch3.Sec_3_1.Balance !balance)
    else Sicp_ch3.Sec_3_1.Insufficient_funds
  in
  let deposit amount =
    balance := !balance + amount;
    !balance
  in
  let serializer = Sicp_ch3.Sec_3_4.Serializers.make_serializer () in
  fun m ->
    match m with
    | Withdraw amount -> Withdrawn (serializer.protect (fun () -> withdraw amount))
    | Deposit amount -> New_balance (serializer.protect (fun () -> deposit amount))
    | Balance -> New_balance !balance
    | Serializer -> Serialized serializer
;;

let serializer_of account =
  match account Serializer with
  | Serialized s -> s
  | _ -> invalid_arg "serializer_of: not a serializer"
;;

let balance_of account =
  match account Balance with
  | New_balance n -> n
  | _ -> invalid_arg "balance_of: not a balance"
;;

let louis_serialized_exchange account1 account2 =
  let s1 = serializer_of account1 in
  let s2 = serializer_of account2 in
  s1.protect (fun () ->
    s2.protect (fun () ->
      let difference = balance_of account1 - balance_of account2 in
      ignore (account1 (Withdraw difference));
      ignore (account2 (Deposit difference))))
;;

let inner_acquire_is_impossible () =
  let mutex = Mutex.create () in
  not (Mutex.protect mutex (fun () -> Mutex.try_lock mutex))
;;

let exchange_stalls () =
  let a1 = make_louis_account 20 in
  let a2 = make_louis_account 10 in
  (* 0 = the exchange is still grinding, 1 = it finished, 2 = the
     runtime refused the second acquisition of a held mutex, which is
     this host's way of saying the exchange cannot complete. *)
  let status = Atomic.make 0 in
  let domain =
    Domain.spawn (fun () ->
      (try louis_serialized_exchange a1 a2 with
       | Sys_error _ -> Atomic.set status 2);
      Atomic.set status (if Atomic.get status = 2 then 2 else 1))
  in
  let rec spin budget =
    if Atomic.get status <> 0
    then false
    else if budget = 0
    then true
    else (
      Domain.cpu_relax ();
      spin (budget - 1))
  in
  let stalled = spin 20_000_000 in
  if not stalled then Domain.join domain;
  if stalled then Atomic.set status 2;
  Atomic.get status = 2
;;

let ex_3_45 () = inner_acquire_is_impossible (), exchange_stalls ()
