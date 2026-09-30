(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.34: printing lazy pairs.  A lazy list of the experiment
    is a list of the subset whose cells hold thunks, so the typed value
    already identifies it and no tag is needed.  The lazy-printing
    rule prints a list as its elements, each forced only as it is
    printed, the walk continuing through delayed tails up to a budget
    of ten elements: a list still going at the budget prints an
    ellipsis, so an infinite list prints a bounded prefix and the
    driver terminates.  Printing is itself a forcing site, so an
    element whose forcing fails surfaces its error at print time. *)

(** [ex_4_34 ()] answers the printed form of, in order: a finite lazy
    list, a lazy list of lazy lists, the infinite list of ones, the
    first element of that list, and a lazy list whose second element
    is armed. *)
val ex_4_34 : unit -> string list
