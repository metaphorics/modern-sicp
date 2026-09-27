(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.21 *)

(** Exercise 3.21: what Ben saw is the pair-of-pointers rendering of the
    queue, not the queue's items. [ben_view] reproduces the interpreter's
    response -- the printed front and rear pointers -- and [print_queue]
    is Eva Lu's procedure: the sequence of items alone. *)

open Sicp_ch3.Sec_3_3
module Q = Queue
module M = Mpairs

(* What a naive printer shows for the queue: the cons of the two
   pointers, exactly the book's ((a) a) responses. *)
let ben_view q = M.show (M.mcons (Q.front_ptr q) (Q.rear_ptr q))

(* Eva Lu's print-queue: the sequence of items. *)
let print_queue q =
  "(" ^ String.concat " " (List.map (fun o -> M.show o) (Q.items q)) ^ ")"
;;

let ex_3_21 () =
  let q = Q.make_queue () in
  ignore (Q.insert_queue q (M.msym "a"));
  let v1 = ben_view q in
  ignore (Q.insert_queue q (M.msym "b"));
  let v2 = ben_view q in
  ignore (Q.delete_queue q);
  let v3 = ben_view q in
  ignore (Q.delete_queue q);
  let v4 = ben_view q in
  let items_now = print_queue q in
  (* the rear pointer still names the deleted pair; empty-queue? looks
     only at the front, so the queue is empty all the same *)
  let rear_after = ben_view q in
  v1, v2, v3, v4, items_now, Q.empty_queue q, rear_after
;;
