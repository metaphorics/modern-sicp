(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Closures and lexical scope: the named definitions behind the listings
    of section 0.7. *)

(** [make_adder n] is the function [fun x -> x + n]; the closure carries
    [n]'s binding with it. *)
val make_adder : int -> int -> int

(** [add_five] is [make_adder 5]; [add_five 1] is [6]. *)
val add_five : int -> int

(** [compose f g] is the function that applies [g] and then [f]. *)
val compose : ('b -> 'c) -> ('a -> 'b) -> 'a -> 'c

(** [make_withdraw initial] is the book's [make-withdraw]: a function over
    a private [ref] balance. Each call to [make_withdraw] builds a fresh
    cell, so the balance lives between calls to one withdrawer and apart
    from every other's. A withdrawal larger than the balance returns
    [None] and leaves the cell untouched. *)
val make_withdraw : int -> int -> int option

(** [withdrawal_sequence ()] is [(Some 40, None)]: one withdrawer's second
    attempt of [60] finds the balance at [40]. *)
val withdrawal_sequence : unit -> int option * int option

(** [independent_withdrawals ()] is [(Some 80, Some 70, Some 0)]: two
    withdrawers built from [100] keep separate balances. *)
val independent_withdrawals : unit -> int option * int option * int option
