(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** The named definitions behind the listings of section 3.4, grouped by
    subsection. Concurrency is real here: [Parallel.parallel] runs two
    procedures on two OCaml 5 domains, and the serializers wrap one
    [Mutex.t] each. Every deterministic value a listing displays is
    asserted by [run_sec_3_4] through [Replay]; the interleaving demos
    are asserted by membership in their possible-outcome sets, never by
    one specific schedule. *)
type withdraw_result = Sec_3_1.withdraw_result =
  | Balance of int
  | Insufficient_funds

(** 3.4's opening: the account of 3.1.1 recalled as one shared [ref]
    that any number of processes can reach. *)
module Shared_withdraw = struct
  let balance = ref 100

  let withdraw amount =
    if !balance >= amount
    then (
      balance := !balance - amount;
      Sec_3_1.Balance !balance)
    else Sec_3_1.Insufficient_funds
  ;;
end

(** The edition's [parallel]: the first procedure runs on a freshly
    spawned domain, the second on the calling domain, and the call
    returns only when both have finished. *)
module Parallel = struct
  let parallel left right =
    let domain = Domain.spawn left in
    let right_result = right () in
    let left_result = Domain.join domain in
    left_result, right_result
  ;;
end

(** 3.4.2: serializers. A serializer wraps one mutex and answers with
    the polymorphic [protect] field, so a serialized procedure works
    for every result type. *)
module Serializers = struct
  type serializer = { protect : 'a. (unit -> 'a) -> 'a }

  let make_serializer () =
    let mutex = Mutex.create () in
    { protect = (fun thunk -> Mutex.protect mutex thunk) }
  ;;
end

(** The five-outcome increment/square race of the text and its
    serialized counterpart. [run_unserialized] and [run_serialized]
    each return whatever value one particular schedule left in [x]. *)
module X_race = struct
  let possible_values = [ 101; 121; 110; 11; 100 ]
  let serialized_values = [ 101; 121 ]

  let run_unserialized () =
    let x = ref 10 in
    ignore (Parallel.parallel (fun () -> x := !x * !x) (fun () -> incr x));
    !x
  ;;

  let run_serialized () =
    let x = ref 10 in
    let s = Serializers.make_serializer () in
    ignore
      (Parallel.parallel
         (fun () -> s.protect (fun () -> x := !x * !x))
         (fun () -> s.protect (fun () -> incr x)));
    !x
  ;;
end

(** The bank account of 3.1.1 with serialized deposits and withdrawals.
    Scheme's message dispatch becomes a function from a [message] to a
    [response]; the [Serializer] message exists for the version of the
    account that exports its serializer. *)
module Account = struct
  type message =
    | Withdraw of int
    | Deposit of int
    | Balance
    | Serializer

  type response =
    | Withdrawn of withdraw_result
    | New_balance of int
    | Serialized of Serializers.serializer

  type account = message -> response

  let make_account initial =
    let balance = ref initial in
    let withdraw amount =
      if !balance >= amount
      then (
        balance := !balance - amount;
        Sec_3_1.Balance !balance)
      else Sec_3_1.Insufficient_funds
    in
    let deposit amount =
      balance := !balance + amount;
      !balance
    in
    let protected = Serializers.make_serializer () in
    fun m ->
      match m with
      | Withdraw amount -> Withdrawn (protected.protect (fun () -> withdraw amount))
      | Deposit amount -> New_balance (protected.protect (fun () -> deposit amount))
      | Balance -> New_balance !balance
      | _ -> invalid_arg "Unknown request: MAKE-ACCOUNT"
  ;;

  (** Ben's variant of exercise 3.41: the balance message answers
      through the serializer too (the commented line of the exercise). *)
  let make_account_serialized_balance initial =
    let balance = ref initial in
    let withdraw amount =
      if !balance >= amount
      then (
        balance := !balance - amount;
        Sec_3_1.Balance !balance)
      else Sec_3_1.Insufficient_funds
    in
    let deposit amount =
      balance := !balance + amount;
      !balance
    in
    let protected = Serializers.make_serializer () in
    fun m ->
      match m with
      | Withdraw amount -> Withdrawn (protected.protect (fun () -> withdraw amount))
      | Deposit amount -> New_balance (protected.protect (fun () -> deposit amount))
      | Balance -> New_balance (protected.protect (fun () -> !balance))
      | _ -> invalid_arg "Unknown request: MAKE-ACCOUNT"
  ;;

  (** Ben's variant of exercise 3.42: the serialized procedures are
      created once, when the account is made, instead of once per
      message. *)
  let make_account_hoisted_serialization initial =
    let balance = ref initial in
    let withdraw amount =
      if !balance >= amount
      then (
        balance := !balance - amount;
        Sec_3_1.Balance !balance)
      else Sec_3_1.Insufficient_funds
    in
    let deposit amount =
      balance := !balance + amount;
      !balance
    in
    let protected = Serializers.make_serializer () in
    let protected_withdraw amount = protected.protect (fun () -> withdraw amount) in
    let protected_deposit amount = protected.protect (fun () -> deposit amount) in
    fun m ->
      match m with
      | Withdraw amount -> Withdrawn (protected_withdraw amount)
      | Deposit amount -> New_balance (protected_deposit amount)
      | Balance -> New_balance !balance
      | _ -> invalid_arg "Unknown request: MAKE-ACCOUNT"
  ;;

  (** [n] deposits of 1, half on a spawned domain and half on the
      calling domain; the serialized deposits conserve every unit. *)
  let concurrent_deposits account count =
    let half = count / 2 in
    ignore
      (Parallel.parallel
         (fun () ->
            for _ = 1 to half do
              ignore (account (Deposit 1))
            done)
         (fun () ->
            for _ = 1 to count - half do
              ignore (account (Deposit 1))
            done));
    match account Balance with
    | New_balance n -> n
    | _ -> invalid_arg "concurrent_deposits: not a balance"
  ;;
end

(** Exchanging balances: the account that exports its serializer, the
    plain exchange, and the exchange serialized by both accounts. *)
module Exchange = struct
  type account_and_serializer =
    { dispatch : Account.account
    ; serializer : Serializers.serializer
    }

  let make_account_and_serializer initial =
    let balance = ref initial in
    let withdraw amount =
      if !balance >= amount
      then (
        balance := !balance - amount;
        Sec_3_1.Balance !balance)
      else Sec_3_1.Insufficient_funds
    in
    let deposit amount =
      balance := !balance + amount;
      !balance
    in
    let serializer = Serializers.make_serializer () in
    let dispatch m =
      match m with
      | Account.Withdraw amount -> Account.Withdrawn (withdraw amount)
      | Account.Deposit amount -> Account.New_balance (deposit amount)
      | Account.Balance -> Account.New_balance !balance
      | Account.Serializer -> Account.Serialized serializer
    in
    { dispatch; serializer }
  ;;

  let balance_of account =
    match account.dispatch Account.Balance with
    | Account.New_balance n -> n
    | _ -> invalid_arg "balance_of: not a balance"
  ;;

  (** Deposits and withdrawals are no longer serialized by the account
      itself; each user of the object manages serialization through the
      exported serializer. *)
  let deposit account amount =
    let s = account.serializer in
    match s.protect (fun () -> account.dispatch (Account.Deposit amount)) with
    | Account.New_balance n -> n
    | _ -> invalid_arg "deposit: not a balance"
  ;;

  let exchange account1 account2 =
    let difference = balance_of account1 - balance_of account2 in
    ignore (account1.dispatch (Account.Withdraw difference));
    ignore (account2.dispatch (Account.Deposit difference))
  ;;

  let serialized_exchange account1 account2 =
    let s1 = account1.serializer in
    let s2 = account2.serializer in
    s1.protect (fun () -> s2.protect (fun () -> exchange account1 account2))
  ;;
end

(** Implementing serializers: the cell, the mutex built on it, and the
    serializer built on that mutex -- the section's own account of what
    [Mutex.protect] does underneath. The naive cell is a plain [bool
    ref]: reading and then writing it are two separate memory
    operations, which is exactly what exercise 3.46 exploits. The
    working cell is a [bool Atomic.t], whose [compare_and_set] is one
    indivisible operation. *)
module Mutex_impl = struct
  (** The text's ordinary-procedure version: it tests the cell and
      returns the result of the test, setting the cell to true first if
      the test was false. Atomicity is the whole question (exercise
      3.46). *)
  let test_and_set cell =
    if !cell
    then true
    else (
      cell := true;
      false)
  ;;

  (** What the host actually owes test-and-set: a compare-and-swap. It
      answers [true] exactly when [test_and_set] would, that is, when
      the cell was already set -- [compare_and_set] answers [true] on
      the opposite event, a successful change from [false] to [true],
      so this negates it. *)
  let atomic_test_and_set cell = not (Atomic.compare_and_set cell false true)

  type mutex =
    { acquire : unit -> unit
    ; release : unit -> unit
    }

  let clear cell = Atomic.set cell false

  (** The working mutex: [acquire] retries until the atomic
      test-and-set reports the cell was free. *)
  let make_mutex () =
    let cell = Atomic.make false in
    let rec acquire () = if atomic_test_and_set cell then acquire () in
    { acquire; release = (fun () -> clear cell) }
  ;;

  let make_serializer () =
    let mutex = make_mutex () in
    { Serializers.protect =
        (fun thunk ->
          mutex.acquire ();
          let value = thunk () in
          mutex.release ();
          value)
    }
  ;;
end
