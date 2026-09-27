(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** The named definitions behind the listings of section 3.5, grouped by
    subsection. Streams here keep the book's memoized [delay] semantics:
    [cons_stream] takes the tail as a thunk and stores it behind exactly
    one [Lazy.t], so a tail is computed at most once no matter how often
    it is forced. Every deterministic value a listing displays is
    asserted by [run_sec_3_5] through [Replay]; the book's result
    comments are true by construction.

    The section's hard spot is accidental self-forcing: a recursively
    defined stream such as [Convergence.sqrt_stream] must hold the
    recursion behind the same memoized delay that [cons_stream] builds,
    or each access rebuilds the stream from scratch (exercise 3.63
    measures exactly that). OCaml's own [Lazy.force] computes once and
    reuses the value, which is what makes the whole section work. *)

(** 3.5.1: the stream type. A stream is either the empty stream or one
    evaluated element paired with a memoized promise for the rest. The
    promise is where the book's [delay] lives: [cons_stream] receives
    the tail as a thunk and wraps it in one [lazy], so the special form
    of the original becomes an ordinary function whose second argument
    the caller writes as a [fun () -> ...]. *)
module Streams = struct
  type 'a stream =
    | Cons of 'a * 'a stream Lazy.t
    | Empty

  (** The distinguishable object that cannot result from any
      [cons_stream] call. *)
  let the_empty_stream : 'a stream = Empty

  (** [stream_null s] asks whether [s] is the empty stream. *)
  let stream_null = function
    | Empty -> true
    | Cons _ -> false
  ;;

  (** [cons_stream head tail] pairs [head] with one memoized delay for
      [tail ()]; the tail thunk runs at most once, whatever the number
      of later accesses. *)
  let cons_stream head tail = Cons (head, lazy (tail ()))

  let stream_car = function
    | Cons (head, _) -> head
    | Empty -> invalid_arg "stream_car: the empty stream"
  ;;

  let stream_cdr = function
    | Cons (_, tail) -> Lazy.force tail
    | Empty -> invalid_arg "stream_cdr: the empty stream"
  ;;

  let rec stream_ref s n =
    if n = 0 then stream_car s else stream_ref (stream_cdr s) (n - 1)
  ;;

  let rec stream_map f s =
    if stream_null s
    then the_empty_stream
    else cons_stream (f (stream_car s)) (fun () -> stream_map f (stream_cdr s))
  ;;

  let rec stream_for_each f s =
    if stream_null s
    then ()
    else (
      f (stream_car s);
      stream_for_each f (stream_cdr s))
  ;;

  (** [stream_enumerate_interval low high] is the stream analog of the
      list [enumerate_interval] of 2.2.3: only the first element is
      computed up front. *)
  let rec stream_enumerate_interval low high =
    if low > high
    then the_empty_stream
    else cons_stream low (fun () -> stream_enumerate_interval (low + 1) high)
  ;;

  let rec stream_filter pred s =
    if stream_null s
    then the_empty_stream
    else if pred (stream_car s)
    then cons_stream (stream_car s) (fun () -> stream_filter pred (stream_cdr s))
    else stream_filter pred (stream_cdr s)
  ;;

  (** [stream_take n s] collects the first [n] elements as a list; the
      observability helper behind every finite prefix a listing shows. *)
  let rec stream_take n s =
    if n <= 0 || stream_null s
    then []
    else stream_car s :: stream_take (n - 1) (stream_cdr s)
  ;;

  (** [display_line] writes a newline and then the shown element, in
      the book's order. *)
  let display_line out x =
    Buffer.add_char out '\n';
    Buffer.add_string out x
  ;;

  (** [display_stream show out s] walks the stream, displaying each
      element through [show]; a listing's visible output without
      committing this edition to one printing channel. *)
  let display_stream show out s = stream_for_each (fun x -> display_line out (show x)) s
end

(** 3.5.1: implementing [delay] and [force]. The plain delay is the
    host's own thunk; the memoized delay is a cached thunk, and
    [Lazy.t] is the standard-library form of exactly that cache. *)
module Memo = struct
  (** The book's [memo_proc]: the first call computes and remembers,
      every later call returns the remembered value. *)
  let memo_proc f =
    let cache = ref None in
    fun () ->
      match !cache with
      | Some value -> value
      | None ->
        let value = f () in
        cache := Some value;
        value
  ;;
end

(** 3.5.2: infinite streams, both by generating procedures and by
    implicit self-reference. *)
module Infinite = struct
  open Streams

  let rec integers_starting_from n =
    cons_stream n (fun () -> integers_starting_from (n + 1))
  ;;

  let integers = integers_starting_from 1
  let divisible x y = x mod y = 0
  let no_sevens = stream_filter (fun x -> not (divisible x 7)) integers
  let rec fibgen a b = cons_stream a (fun () -> fibgen b (a + b))
  let fibs = fibgen 0 1

  let rec sieve s =
    cons_stream (stream_car s) (fun () ->
      sieve (stream_filter (fun x -> not (divisible x (stream_car s))) (stream_cdr s)))
  ;;

  let primes = sieve (integers_starting_from 2)

  (** Defining streams implicitly: [ones] refers to itself, and the
      promise behind its tail makes the reference terminate one element
      at a time. [Cons (1, lazy (ones))] is [cons_stream 1 ones] with
      the constructor written out: a self-referential value must place
      the promise itself in the tail, since a call to [cons_stream]
      would have to be evaluated before [ones] exists. *)
  let rec ones = Cons (1, lazy ones)

  (** The elementwise sum, built on the two-stream map this typed
      edition names [stream_map2]; the n-ary generalization is
      exercise 3.50. *)
  let rec stream_map2 f s1 s2 =
    cons_stream
      (f (stream_car s1) (stream_car s2))
      (fun () -> stream_map2 f (stream_cdr s1) (stream_cdr s2))
  ;;

  let add_streams = stream_map2 ( + )

  (** The elementwise sum for floating-point streams; the typed
      edition carries one adder per arithmetic type, since [+] and [+.]
      are distinct. *)
  let add_streams_float = stream_map2 ( +. )

  (** The implicit integers: 1 followed by the sum of [ones] and the
      stream itself. [integers] above and this stream agree element
      for element; both exist because each tail is forced at most
      once. *)
  let rec integers_implicit = Cons (1, lazy (add_streams ones integers_implicit))

  let rec fibs_implicit =
    Cons (0, lazy (Cons (1, lazy (add_streams (stream_cdr fibs_implicit) fibs_implicit))))
  ;;

  (** [scale_stream stream factor] multiplies each item by a constant;
      the integer edition of the book's procedure, which this section
      uses where weights and factors are whole numbers. *)
  let scale_stream stream factor = stream_map (fun x -> x * factor) stream

  let rec double = Cons (1, lazy (scale_stream double 2))

  (** The alternate primes: filter the integers by a primality test
      that itself consults [primes_alt]. Enough of the stream is always
      generated before the test needs it. *)
  let rec primes_alt = Cons (2, lazy (stream_filter is_prime (integers_starting_from 3)))

  and is_prime n =
    let rec iter ps =
      if stream_car ps * stream_car ps > n
      then true
      else if divisible n (stream_car ps)
      then false
      else iter (stream_cdr ps)
    in
    iter primes_alt
  ;;
end

(** 3.5.3: power series as streams of coefficients. *)
module Series = struct
  open Streams
  open Infinite

  (** The integral of a_0 + a_1 x + a_2 x^2 + ... is 0 + a_0 x + a_1 x^2/2
      + a_2 x^3/3 + ...; the non-constant coefficients divide by their
      new index. *)
  let integrate_series s =
    stream_map2 (fun a k -> a /. float_of_int k) s (integers_starting_from 1)
  ;;

  (** e^x is its own derivative, so the series equals its own integral
      except for the constant term 1. *)
  let rec exp_series = Cons (1.0, lazy (integrate_series exp_series))
end

(** 3.5.3: formulating iterations as stream processes, and sequence
    acceleration. *)
module Convergence = struct
  open Streams
  open Infinite

  let average x y = (x +. y) /. 2.0
  let sqrt_improve guess x = average guess (x /. guess)

  (** The local [guesses] is essential: the stream refers to itself
      through one shared memoized delay, so each better guess is
      computed exactly once (exercise 3.63 measures the version that
      omits the local binding). *)
  let sqrt_stream x =
    let rec guesses =
      Cons (1.0, lazy (stream_map (fun guess -> sqrt_improve guess x) guesses))
    in
    guesses
  ;;

  let rec pi_summands n =
    Cons (1.0 /. float_of_int n, lazy (stream_map (fun x -> -.x) (pi_summands (n + 2))))
  ;;

  let rec partial_sums (s : float stream) : float stream =
    Cons (stream_car s, lazy (add_streams_float (stream_cdr s) (partial_sums s)))
  ;;

  let pi_stream = stream_map (fun x -> 4.0 *. x) (partial_sums (pi_summands 1))

  (** The Euler accelerator for partial sums of alternating series. *)
  let rec euler_transform s =
    let s0 = stream_ref s 0 in
    let s1 = stream_ref s 1 in
    let s2 = stream_ref s 2 in
    let square x = x *. x in
    Cons
      ( s2 -. (square (s2 -. s1) /. (s0 +. (-2.0 *. s1) +. s2))
      , lazy (euler_transform (stream_cdr s)) )
  ;;

  (** A tableau: a stream of streams, each row the transform of the
      one above it. *)
  let rec make_tableau transform s = Cons (s, lazy (make_tableau transform (transform s)))

  (** The first term of every row, super-accelerated. *)
  let accelerated_sequence transform s = stream_map stream_car (make_tableau transform s)
end

(** 3.5.3: infinite streams of pairs. *)
module Pairs = struct
  open Streams

  (** The stream analog of append; unsuitable for infinite streams,
      which the text shows before reaching for [interleave]. *)
  let rec stream_append s1 s2 =
    if stream_null s1
    then s2
    else cons_stream (stream_car s1) (fun () -> stream_append (stream_cdr s1) s2)
  ;;

  let rec interleave s1 s2 =
    if stream_null s1
    then s2
    else cons_stream (stream_car s1) (fun () -> interleave s2 (stream_cdr s1))
  ;;

  (** All pairs from the on-or-above-diagonal half of the infinite
      array, ordered by an [ad hoc] interleaving of the first row with
      the recursively defined remainder. *)
  let rec pairs s t =
    cons_stream
      (stream_car s, stream_car t)
      (fun () ->
         interleave
           (stream_map (fun x -> stream_car s, x) (stream_cdr t))
           (pairs (stream_cdr s) (stream_cdr t)))
  ;;
end

(** 3.5.3: streams as signals. *)
module Signals = struct
  open Streams
  open Infinite

  (** The integrator: C plus the running sum of the scaled input. The
      self-reference through [int] is the feedback loop of the text's
      signal-flow diagram, and it terminates one element at a time
      because of the memoized delay in [cons_stream]. *)
  let integral integrand initial_value dt =
    let rec int =
      Cons
        ( initial_value
        , lazy (add_streams_float (stream_map (fun x -> dt *. x) integrand) int) )
    in
    int
  ;;
end

(** 3.5.4: streams and delayed evaluation -- the integrator whose input
    is itself a promise. *)
module Solve = struct
  open Streams
  open Infinite

  (** [integral] with a delayed integrand: the first output element
      needs no input at all, so a feedback loop can close through this
      procedure. *)
  let integral_delayed delayed_integrand initial_value dt =
    let rec int =
      Cons
        ( initial_value
        , lazy
            (let integrand = Lazy.force delayed_integrand in
             add_streams_float (stream_map (fun x -> dt *. x) integrand) int) )
    in
    int
  ;;

  (** Solves dy/dt = f(y) by delaying [dy] in the definition of [y].
      The stream is one shared self-referential knot, as in exercises
      3.78 to 3.80: the integrand promise reads [y_cell] only when
      forced, and the cell is backpatched with the finished stream
      before anything is demanded, so every demand advances the one
      spine the loop closes through -- [delay dy] exactly as in the
      book, with no fresh copy built per element. *)
  let solve f y0 dt =
    let y_cell = ref (lazy the_empty_stream) in
    let y = integral_delayed (lazy (stream_map f (Lazy.force !y_cell))) y0 dt in
    y_cell := lazy y;
    y
  ;;
end

(** 3.5.5: modularity of functional programs -- random streams and the
    Monte Carlo estimate of [pi], with no assignment anywhere. *)
module Random_streams = struct
  open Streams

  (** [rand_update x] is the seeded generator of 3.1.2 made stateless:
      the state is a value, not a hidden [ref]. The fold through the
      same 1e9 bound as 3.1's [rand] keeps successive values usable as
      an independent-looking sample, which the Ces@`aro experiment
      needs. *)
  let random_init = 42L

  let rand_update x =
    let open Int64 in
    let x1 = logxor x (shift_right_logical x 12) in
    let x2 = logxor x1 (shift_left x1 25) in
    let x3 = logxor x2 (shift_right_logical x2 27) in
    unsigned_rem (mul x3 0x2545F4914F6CDD1DL) 1_000_000_000L
  ;;

  let rec random_numbers = Cons (random_init, lazy (stream_map rand_update random_numbers))

  let rec map_successive_pairs f s =
    cons_stream
      (f (stream_car s) (stream_car (stream_cdr s)))
      (fun () -> map_successive_pairs f (stream_cdr (stream_cdr s)))
  ;;

  let cesaro_stream =
    map_successive_pairs
      (fun r1 r2 -> Sicp_ch1.Sec_1_2.Gcd.gcd (Int64.to_int r1) (Int64.to_int r2) = 1)
      random_numbers
  ;;

  let rec monte_carlo experiment_stream passed failed =
    let next passed failed =
      cons_stream
        (float_of_int passed /. float_of_int (passed + failed))
        (fun () -> monte_carlo (stream_cdr experiment_stream) passed failed)
    in
    if stream_car experiment_stream
    then next (passed + 1) failed
    else next passed (failed + 1)
  ;;

  let pi_stream = stream_map (fun p -> sqrt (6.0 /. p)) (monte_carlo cesaro_stream 0 0)

  (** The withdrawal processor of 3.1.3 rebuilt as a function from a
      balance and a stream of amounts to a stream of balances: no
      assignment, no local state, and yet, from the user's seat, the
      same changing balance. *)
  let rec stream_withdraw balance amount_stream =
    cons_stream balance (fun () ->
      stream_withdraw (balance -. stream_car amount_stream) (stream_cdr amount_stream))
  ;;
end
