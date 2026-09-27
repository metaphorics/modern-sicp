(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

(** Exercise 5.27: the recursive factorial on the monitored stack, for
    comparison with 5.26; both counters are linear in n, and the
    measured constants fill the book's table. *)

(** [recursive_source] is the 1.2.1 recursive factorial the exercise
    defines. *)
val recursive_source : string

(** [ex_5_27 ()] measures the recursive factorial for n = 1 to 6, fits
    both counters, and verifies the linear formulas on every point.
    The maximum depth of 5.26's iterative version is constant, this
    one's grows by a fixed amount per n -- the book's table's two
    rows. *)
val ex_5_27 : unit -> (string list, Sicp_ch5.Sec_5_4.error) result
