(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.52: the compiler's C backend. *)

(** A label name as a C identifier. *)
val c_ident : string -> string

(** The runtime the emitted program links against. *)
val runtime_c : string

(** [compile_to_c source] is the whole C file for the compiled source. *)
val compile_to_c : string -> (string, Sicp_ch5.Sec_5_5.error) result

(** [build_and_run c_source] builds and runs the program with the
    system C compiler. *)
val build_and_run : string -> (string, Sicp_ch5.Sec_5_5.error) result

val ex_5_52 : unit -> (string list, Sicp_ch5.Sec_5_5.error) result
