(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 0.4: build [make_account] returning a record of closures over
    one [ref]; then break it by sharing the [ref] between two accounts and
    explain the observed aliasing.

    What the reader observes in the broken variant: a deposit through one
    account raises the balance the other account reads, because the two
    records close over the same [ref] cell instead of each closing over
    its own. *)

(** An account is a record of three closures over one private [ref]. *)
type account =
  { deposit : float -> float
  ; withdraw : float -> float
  ; balance : unit -> float
  }

(** [make_account initial] is a fresh account holding [initial]; each
    call closes over its own cell, so two accounts stay independent. *)
val make_account : float -> account

(** [make_shared_accounts initial] is two accounts over one shared cell:
    depositing through one raises the balance the other reads. *)
val make_shared_accounts : float -> account * account
