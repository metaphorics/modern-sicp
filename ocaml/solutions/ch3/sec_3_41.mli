(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.4 exercise 3.41 *)

(** Exercise 3.41: Ben Bitdiddle wants the account's balance message
    to answer through the serializer. The exercise asks whether any
    scenario demonstrates his concern; the OCaml edition answers with
    a concurrent reader that watches a balance while deposits land. *)

(** [collect_reads rounds] performs [rounds] deposits of 1 on a
    spawned domain while the calling domain reads the unserialized
    balance until the deposits are done; it answers every value the
    reader saw. *)
val collect_reads : int -> int list

(** [anomalous_reads total reads] counts the readings that are not
    balances the writer could have left, that is, values outside
    [0] to [total]. *)
val anomalous_reads : int -> int list -> int

(** [ex_3_41 ()] is the number of readings taken and the number of
    anomalous readings among them. *)
val ex_3_41 : unit -> int * int
