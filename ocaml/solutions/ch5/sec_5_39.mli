(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.39: the lexical addressing operations. *)

(** The shadow store: cells whose content was overridden, keyed
    physically by the cell. *)
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

(** [lexical_operations shadow global] is the lexical machine's
    operation table; [global] is the top-level chain
    [global_chain (global_frame shadow prims)] builds. *)
val lexical_operations
  :  shadow
  -> Sicp_common.Value.t
  -> (string * Sicp_ch5.Sec_5_4.op) list

(** [global_frame shadow prims] is the primed global frame cell. *)
val global_frame
  :  shadow
  -> (string * Sicp_common.Value.primitive) list
  -> Sicp_common.Value.t

(** [global_chain cell] is the top-level environment of the chain
    [cell]. *)
val global_chain : Sicp_common.Value.t -> Sicp_common.Value.t

(** [values_of transcript] is the printed values: every line that
    follows a value announcement. *)
val values_of : string list -> string list

val run_lexical : string -> (string list, Sicp_ch5.Sec_5_5.error) result
val ex_5_39 : unit -> (string list, Sicp_ch5.Sec_5_5.error) result
