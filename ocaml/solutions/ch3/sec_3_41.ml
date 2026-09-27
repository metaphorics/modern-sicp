(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.4 exercise 3.41 *)

(** Exercise 3.41: Ben Bitdiddle wants [balance] to answer through the
    account's serializer. The scenario that would demonstrate his
    worry is a reader who watches the balance while a writer deposits;
    if the reader could ever observe a value that is not one the
    writer actually wrote, unserialized reads would be unsafe. An
    OCaml [int ref] write is not torn: any concurrent read sees either
    the value before or the value after one write, never a mixture, so
    no such value exists. *)

open Sicp_ch3.Sec_3_4.Account

let collect_reads rounds =
  let cap = rounds * 20 in
  let account = make_account 0 in
  let deposits_done = Atomic.make false in
  let reads = ref [] in
  let writer =
    Domain.spawn (fun () ->
      for _ = 1 to rounds do
        ignore (account (Deposit 1))
      done;
      Atomic.set deposits_done true)
  in
  let rec read_until_done count =
    (match account Balance with
     | New_balance n -> reads := n :: !reads
     | _ -> invalid_arg "collect_reads: not a balance");
    if (not (Atomic.get deposits_done)) && count < cap then read_until_done (count + 1)
  in
  read_until_done 0;
  Domain.join writer;
  List.rev !reads
;;

let anomalous_reads total reads =
  List.length (List.filter (fun v -> v < 0 || v > total) reads)
;;

let ex_3_41 () =
  let reads = collect_reads 2000 in
  List.length reads, anomalous_reads 2000 reads
;;
