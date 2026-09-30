(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.3 *)

(** Exercise 5.22: [append] and [append!] as register machines over the
    list-structure memory, with the before/after dumps. *)

(** [list_word mem ints] plants a proper list of numbers in [mem]. *)
val list_word
  :  Sicp_ch5.Sec_5_3.memory
  -> int list
  -> (Sicp_ch5.Sec_5_3.word, Sicp_ch5.Sec_5_3.error) result

(** [append_controller] copies [x] and shares [y];
    [append_bang_controller] splices [y] into [x] with one
    [set-cdr!]. *)
val append_controller : Sicp_ch5.Sec_5_3.word Sicp_ch5.Sec_5_1.instruction list

val append_bang_controller : Sicp_ch5.Sec_5_3.word Sicp_ch5.Sec_5_1.instruction list

(** [run controller result mem x y] runs one machine over the planted
    lists and reads the result register ([z] for [append], [x] for
    [append!]). *)
val run
  :  Sicp_ch5.Sec_5_3.word Sicp_ch5.Sec_5_1.instruction list
  -> string
  -> Sicp_ch5.Sec_5_3.memory
  -> Sicp_ch5.Sec_5_3.word
  -> Sicp_ch5.Sec_5_3.word
  -> (Sicp_ch5.Sec_5_3.word, Sicp_ch5.Sec_5_3.error) result

(** [ex_5_22 ()] runs both machines over [x = [1; 2; 3]] and
    [y = [4; 5]]: the [append] answer with its three fresh cells and
    [x] untouched, then the [append!] before/after memory dumps and the
    shared-pointer answer. *)
val ex_5_22 : unit -> (string list, Sicp_ch5.Sec_5_3.error) result
