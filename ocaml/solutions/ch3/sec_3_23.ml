(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.23 *)

(** Exercise 3.23: a deque as doubly linked cells with front and rear
    pointers. Every operation is a constant number of pointer writes;
    nothing here ever prints a structure, so the cycle footnote of the
    book never bites. *)

open Sicp_ch3.Sec_3_3.Mpairs

type deque =
  { mutable front : node option
  ; mutable rear : node option
  }

and node =
  { mutable prev : node option
  ; mutable item : mobj
  ; mutable next : node option
  }

let make_deque () = { front = None; rear = None }

let empty_deque d =
  match d.front with
  | None -> true
  | Some _ -> false
;;

let front_deque d =
  match d.front with
  | None -> invalid_arg "FRONT called with an empty deque"
  | Some n -> n.item
;;

let rear_deque d =
  match d.rear with
  | None -> invalid_arg "REAR called with an empty deque"
  | Some n -> n.item
;;

let front_insert_deque d item =
  let n = { prev = None; item; next = d.front } in
  (match d.front with
   | Some old -> old.prev <- Some n
   | None -> ());
  d.front <- Some n;
  match d.rear with
  | None -> d.rear <- Some n
  | Some _ -> ()
;;

let rear_insert_deque d item =
  let n = { prev = d.rear; item; next = None } in
  (match d.rear with
   | Some old -> old.next <- Some n
   | None -> ());
  d.rear <- Some n;
  match d.front with
  | None -> d.front <- Some n
  | Some _ -> ()
;;

let front_delete_deque d =
  match d.front with
  | None -> invalid_arg "FRONT-DELETE called with an empty deque"
  | Some n ->
    d.front <- n.next;
    (match d.front with
     | Some f -> f.prev <- None
     | None -> d.rear <- None)
;;

let rear_delete_deque d =
  match d.rear with
  | None -> invalid_arg "REAR-DELETE called with an empty deque"
  | Some n ->
    d.rear <- n.prev;
    (match d.rear with
     | Some r -> r.next <- None
     | None -> d.front <- None)
;;

let items d =
  let rec go n acc =
    let acc = n.item :: acc in
    match n.next with
    | Some m -> go m acc
    | None -> List.rev acc
  in
  match d.front with
  | None -> []
  | Some n -> go n []
;;

let ex_3_23 () =
  let d = make_deque () in
  rear_insert_deque d (msym "b");
  front_insert_deque d (msym "a");
  rear_insert_deque d (msym "c");
  let after_inserts = List.map (fun o -> show o) (items d) in
  front_delete_deque d;
  rear_delete_deque d;
  let after_deletes = List.map (fun o -> show o) (items d) in
  let front_now = show (front_deque d) in
  let rear_now = show (rear_deque d) in
  ( String.concat " " after_inserts
  , String.concat " " after_deletes
  , front_now
  , rear_now
  , empty_deque d )
;;
