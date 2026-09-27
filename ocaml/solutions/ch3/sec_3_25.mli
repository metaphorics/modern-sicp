(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.25 *)

(** Exercise 3.25: tables under an arbitrary number of keys. *)

open Sicp_ch3.Sec_3_3

(** [lookup keys table] is the value stored under the key list, or
    [None]. *)
val lookup : Mpairs.mobj list -> Mpairs.mobj -> Mpairs.mobj option

(** [insert keys value table] stores [value] under the key list,
    building any missing subtables. *)
val insert : Mpairs.mobj list -> Mpairs.mobj -> Mpairs.mobj -> unit

(** [ex_3_25 ()] is [(the value under [a; b; c], under [a; b; d], under
    [a; e], under [x], the lookup of a never-inserted path under
    [a; b], the value under [a; b; c] after an overwrite, and the
    value under [a; b; d] which the overwrite left alone)]; the
    never-inserted lookup answers [None]. *)
val ex_3_25
  :  unit
  -> Mpairs.mobj option
     * Mpairs.mobj option
     * Mpairs.mobj option
     * Mpairs.mobj option
     * Mpairs.mobj option
     * Mpairs.mobj option
     * Mpairs.mobj option
