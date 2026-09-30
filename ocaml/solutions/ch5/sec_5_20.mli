(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.3 *)

(** Exercise 5.20: the two drawings and the pointer answers, and the
    edition's 5.20a allocator trace. *)

(** [ex_5_20 ()] is the memory-vector drawing of [let x = (1, 2)] and
    [let y = [x; x]] with [free] initially [p1]: the cells
    come out at [p1], [p2], [p3], [x] is [p1], [y] is [p3], and [free]
    ends at [p4]. *)
val ex_5_20 : unit -> (string list, Sicp_ch5.Sec_5_3.error) result

(** [ex_5_20a ()] is the allocator's trace over the same three conses:
    the free pointer before and after each, then the same pointer
    answers. *)
val ex_5_20a : unit -> (string list, Sicp_ch5.Sec_5_3.error) result
