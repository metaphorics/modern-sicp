(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.39: the lexical addressing operations. *)

(** The mutation shadow of [lexical-address-set!]. *)
type shadow = (Sicp_common.Value.t, Sicp_common.Value.t) Hashtbl.t

val lexical_lookup
  :  shadow
  -> int
  -> int
  -> Sicp_common.Value.t
  -> (Sicp_common.Value.t, Sicp_ch5.Sec_5_5.error) result

val lexical_set
  :  shadow
  -> int
  -> int
  -> Sicp_common.Value.t
  -> Sicp_common.Value.t
  -> (Sicp_common.Value.t, Sicp_ch5.Sec_5_5.error) result

val lexical_operations
  :  shadow
  -> Sicp_common.Value.env
  -> (string * Sicp_ch5.Sec_5_4.op) list

val run_lexical : string -> (string list, Sicp_ch5.Sec_5_5.error) result
val ex_5_39 : unit -> (string list, Sicp_ch5.Sec_5_5.error) result
