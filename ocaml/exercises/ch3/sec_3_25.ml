(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.25 *)

(* The pending scaffold of the solution with the same name under
   solutions/ch3: every entry raises the pending marker until the
   exercise is solved. *)

(** Exercise 3.25: tables under an arbitrary number of keys. *)

(** [lookup keys table] is the value stored under the key list, or
    [None]. *)

(** [insert keys value table] stores [value] under the key list,
    building any missing subtables. *)

(** [ex_3_25 ()] is [(the value under [a; b; c], under [a; b; d], under
    [a; e], under [x], the lookup of a never-inserted path under
    [a; b], the value under [a; b; c] after an overwrite, and the
    value under [a; b; d] which the overwrite left alone)]; the
    never-inserted lookup answers [None]. *)
let lookup _keys _table = raise Sicp_common.Pending.Pending_solution

let insert _keys _value _table = raise Sicp_common.Pending.Pending_solution
let ex_3_25 () = raise Sicp_common.Pending.Pending_solution
