# -*- coding: utf-8 -*-
import os
OUT = "/home/alpha/book/modern-sicp/ocaml/solutions/ch3"

R = {}

R["3_50"] = """The map class is `A`. Scheme's `stream-map` is variadic: it takes a
procedure and any number of argument streams. OCaml procedures have
fixed arity, so this edition's generalized map takes the argument
streams as one list and applies the procedure to the list of current
heads, ending as soon as any argument stream runs dry. `ex_3_50`
adds the integers to the integers from 2 for six elementwise sums and
confirms that a drained argument stream ends the whole mapped stream.

Sample: `ex_3_50 ()` answers `([3; 5; 7; 9; 11; 13], true)`."""

R["3_51"] = """The map class is `T`. Printing becomes appending to a log, so the
exercise's transcript is data instead of console noise. Building `x`
logs nothing but the first element; the two `stream_ref` calls extend
the log to 0..5 and then 0..7, and the memoized delay is why nothing
is ever recomputed. `ex_3_51` answers the log after the definition,
after `stream_ref x 5`, and after `stream_ref x 7`.

Sample: `ex_3_51 ()` answers `([0], [0; 1; 2; 3; 4; 5],
[0; 1; 2; 3; 4; 5; 6; 7])`."""

R["3_51a"] = """This edition's addition. The transcript of `show` makes memoization
visible only through what was logged, and logged evidence is easy to
misread, so 3.51a rebuilds the same stream over an instrumented delay:
one counter increments on every tail access and a second only when a
tail thunk actually runs. `ex_3_51a` reports the `show` counts after
the definition, after `stream_ref x 5`, and after `stream_ref x 7`
followed by the two counters -- twelve tail accesses ran only seven
tail bodies, which is memoization made arithmetic.

Sample: `ex_3_51a ()` answers `(1, 6, 8, 12, 7)`."""

R["3_52"] = """The map class is `A`. Scheme's `set!` sum becomes one `int ref`, and
the "would the answers differ without memoization" question is
answered by running the same transcript twice: once over the
memoized streams and once over plain-thunk streams that re-evaluate
their tails on every access. `ex_3_52` reports both transcripts in
the statement's order: the sum after `seq` (1), after `y` (6, since
the even filter scans 1, 3, 6), after `z` (10), the eighth even
element (136), the sum after the ref (136), the eight multiples of
five `display_stream` shows (10 15 45 55 105 120 190 210), and the
final sum (210). The plain-thunk pass re-adds elements as their tails
are re-evaluated, so its sums (1, 6, 15, 162, 162, a four-element
display, 362) differ from the memoized ones -- the exercise's last
question, answered by execution rather than argument.

Sample: `ex_3_52 ()` answers the memoized transcript and the
plain-thunk transcript as described above."""

R["3_53"] = """The map class is `T`. The self-referential definition translates
directly: `s` is 1 followed by the stream added to itself, and each
element doubles the one before because the tail adds the stream to
itself. `ex_3_53` reads the computed prefix instead of predicting it.

Sample: `ex_3_53 ()` answers `[1; 2; 4; 8; 16; 32; 64; 128]`."""

R["3_54"] = """The map class is `T`. `mul_streams` is the elementwise product, and
the factorial stream pairs the integers from 1 with the factorials
themselves, aligned: 1x1, 2x1, 3x2, 4x6, and so on. Pairing with the
stream's own shifted tail instead would demand the tail's head from
the very promise being forced -- a promise cycle this edition's lazy
raises rather than loops on. `ex_3_54` reads the first eight
elements.

Sample: `ex_3_54 ()` answers
`[1; 1; 2; 6; 24; 120; 720; 5040]`."""

R["3_55"] = """The map class is `T`. `partial_sums` is defined the way the text
defines it: the head of the input followed by the input's tail added
into the partial sums themselves. `ex_3_55` checks the statement's
own example and the constant ones stream.

Sample: `ex_3_55 ()` answers `([1; 3; 6; 10; 15], [1; 2; 3; 4; 5])`."""

R["3_56"] = """The map class is `T`. `merge` combines two ordered streams into one
ordered stream, dropping repetitions, and `hamming` is 1 followed by
the merge of twice, three times, and five times itself -- the four
facts of the statement hold by construction. `ex_3_56` reads the
first twelve elements.

Sample: `ex_3_56 ()` answers
`[1; 2; 3; 4; 5; 6; 8; 9; 10; 12; 15; 16]`."""

R["3_57"] = """The map class is `T`. Both counts are measured by running the fibs
definition over a counting adder, memoized and unmemoized. With the
memoized delay each element past the first two costs exactly one
addition, never repeated; over plain thunks the recomputation grows
like the Fibonacci recursion itself. The memoized count is forced
with `stream_ref` rather than `stream_take`, whose one-element
overshoot would spend the count's last addition on an element the
question never asks about. `ex_3_57` answers the count at fifteen
elements with both verdicts.

Sample: `ex_3_57 ()` answers `(15, 13, 2567, true)`."""

R["3_58"] = """The map class is `T`. `quotient` and `remainder` become `/` and
`mod` on integers. `expand` yields the successive digits of the
fraction in the given radix: each element is the scaled quotient and
the recursion continues on the remainder. `ex_3_58` reads both
fractions the statement asks about.

Sample: `ex_3_58 ()` answers
`([1; 4; 2; 8; 5; 7; 1; 4], [3; 7; 5; 0; 0])` -- the repeating
142857... of 1/7 and the 0.37500... of 3/8."""

R["3_59"] = """The map class is `T`. `integrate_series` divides each coefficient by
its new index, `exp_series` is the constant 1 consed onto its own
integral, and the sine and cosine series are each the integral of the
other with cosine negated. `ex_3_59` reads the first six coefficients
of all three series.

Sample: `ex_3_59 ()` answers the coefficients of e^x
(`[1; 1; 0.5; 1/6; 1/24; 1/120]`), of cos x
(`[1; 0; -0.5; 0; 1/24; 0]`), and of sin x
(`[0; 1; 0; -1/6; 0; 1/120]`)."""

R["3_60"] = """The map class is `T`. `mul_series` is the Cauchy product: the head
product followed by the head of the first series times the rest of
the second, added into the product of the first's tail with all of
the second. The identity sin^2 + cos^2 = 1 holds to floating-point
dust -- products and sums of the series leave a residue near 1e-16 in
the zero terms, so the verdict compares against a 1e-12 tolerance.
`ex_3_60` answers the first five coefficients and the verdict.

Sample: `ex_3_60 ()` answers `([1; 0; 0; 0; 0], true)`."""

R["3_61"] = """The map class is `T`. `invert_unit_series` implements the algebra of
the statement: X = 1 - S_R * X recursively, so the reciprocal is the
constant 1 followed by the negated tail of S times X. `ex_3_61`
multiplies e^x by its reciprocal and reads the unit series.

Sample: `ex_3_61 ()` answers `[1; 0; 0; 0; 0]`."""

R["3_62"] = """The map class is `T`. `div_series` divides numerator by denominator
through 3.60's product and 3.61's reciprocal, refusing a denominator
whose constant term is zero; the tangent series is sin x divided by
cos x. `ex_3_62` reads the first six tangent coefficients and checks
the refusal.

Sample: `ex_3_62 ()` answers the first six tangent coefficients
(x + x^3/3 + 2x^5/15 + ...) with `true` for the refusal check."""

R["3_63"] = """The map class is `T`. Both versions run with a counting improvement
step. The local `guesses` binding computes each Newton refinement
exactly once; the open version rebuilds its prefix from the initial
guess on every demand, so it spends the same five guesses for more
than twice the improvements -- the measured answer to Alyssa's claim.
`ex_3_63` answers both prefixes and both counts.

Sample: `ex_3_63 ()` answers the five local guesses, 4 local calls,
the five open guesses, and 10 open calls."""

R["3_64"] = """The map class is `T`. `stream_limit` walks the stream until two
successive elements differ by less than the tolerance and returns the
second. `ex_3_64` computes the square root of 2 at tolerance 1e-4 and
at a finer tolerance of 1e-8.

Sample: `ex_3_64 ()` answers two root-2 approximations, the coarser
within 1e-4 and the finer within 1e-8 of 1.4142135623730951."""

R["3_65"] = """The map class is `T`. The ln 2 summands alternate sign over the
reciprocal integers: 1, -1/2, 1/3, -1/4, and so on. The plain stream,
its Euler transform, and the accelerated sequence answer the
convergence question: eight terms of the plain stream straddle ln 2,
while the accelerated sequence reaches it to 13 places. `ex_3_65`
answers the first eight elements of each sequence.

Sample: `ex_3_65 ()` answers three eight-element lists; the
accelerated one ends within 1e-12 of 0.6931471805599453."""

R["3_66"] = """The map class is `T`. The exact positions that are enumerable are
measured against the closed forms. The enumeration confirms that
(1, j) sits at position 2j - 2 -- the first row owns every even slot;
the formula holds from j = 2 up, (1, 1) being position 1 -- and that
(k, k) sits at position 2^k - 1, the diagonal doubling; it also
reports where (2, 10), (9, 10), and (10, 10) landed. `ex_3_66`
answers the two verdicts and the three measured positions.

Sample: `ex_3_66 ()` answers `(true, true, Some 33, Some 767,
Some 1023)`."""

R["3_67"] = """The map class is `T`. `pairs_all` mixes in the reflected lower half,
so every ordered pair eventually appears. `ex_3_67` takes a prefix
and checks that every (i, j) with i, j at most 5 appears in both
orders within it.

Sample: `ex_3_67 ()` answers the first twenty-four pairs and whether
the 3x3 corner is fully covered there: `(24, true)`."""

R["3_68"] = """The map class is `T`. Louis's definition interleaves the whole first
row with the recursively defined remainder -- and the recursive call
is evaluated before `interleave` can return its first cons, so on
infinite streams the construction diverges before producing anything.
The executable demonstration runs the recursion under a fuel budget:
each descent spends one unit and no pair ever comes out, while the
book's decomposition yields its sixteen pairs freely. `ex_3_68`
answers Louis's first pair within a 200,000-step budget, the budget
it burned, and the book's first sixteen pairs.

Sample: `ex_3_68 ()` answers `(None, 200000, the book's first
sixteen pairs)`."""

R["3_69"] = """The map class is `T`. `triples` interleaves the elementwise heads of
the three streams with the recursively defined remainder, and
`pythagorean` filters the triples by i^2 + j^2 = k^2. `ex_3_69` reads
the first three Pythagorean triples.

Sample: `ex_3_69 ()` answers `[(3, 4, 5); (6, 8, 10); (5, 12, 13)]`."""

R["3_70"] = """The map class is `T`. `merge_weighted` merges two ordered streams by
their weights, keeping both heads on equal weight -- unlike 3.56's
deduplicating merge, since 3.71 and 3.72 need consecutive equal
weights to survive. `weighted_pairs` is the on-or-above-diagonal
stream ordered by any weighting function. `ex_3_70` answers the first
ten pairs by the sum i + j and by 2i + 3j + 5ij over the integers
coprime to 2, 3, and 5.

Sample: `ex_3_70 ()` answers
`([(1, 1); (1, 2); (1, 3); (2, 2); (1, 4); (2, 3); (1, 5); (2, 4);
(3, 3); (1, 6)], [(1, 1); (1, 7); (1, 11); (1, 13); (1, 17); (1, 19);
(1, 23); (1, 29); (1, 31); (7, 7)])`."""

R["3_71"] = """The map class is `T`. The cube-weighted pair stream is searched for
runs of two consecutive equal weights -- which only works because
`merge_weighted` keeps equal weights side by side. `ex_3_71` reads
1729 and the next five Ramanujan numbers.

Sample: `ex_3_71 ()` answers
`[1729; 4104; 13832; 20683; 32832; 39312]`."""

R["3_72"] = """The map class is `T`. The square-weighted pair stream is searched
for runs of three consecutive equal weights, each reported with its
three writings. `ex_3_72` reads the first five numbers, beginning at
325 = 1^2 + 18^2 = 6^2 + 17^2 = 10^2 + 15^2.

Sample: `ex_3_72 ()` answers
`([325; 425; 650; 725; 845], the three writings for each)`."""

R["3_73"] = """The map class is `T`. `rc` composes the signal processors of the
statement's diagram: the current scaled by R added to the integral of
the current scaled by 1/C. `ex_3_73` models the statement's circuit
(R = 5 ohms, C = 1 farad, dt = 0.5 s) over a constant one-amp current
and reads the first five voltages -- the capacitor charging toward
the 5-volt ceiling.

Sample: `ex_3_73 ()` answers `[5; 5.5; 6; 6.5; 7]`."""

R["3_74"] = """The map class is `T`. `sign_change_detector` compares signs with 0
counted positive, and `zero_crossings` pairs each point with the
previous through the generalized map. The statement's own sample
signal pins the answer.

Sample: `ex_3_74 ()` answers
`[0; 0; 0; 0; 0; -1; 0; 0; 0; 0; 1; 0]`."""

R["3_75"] = """The map class is `T`. Louis's construction averages the current
point with the carried value and then compares the average with that
same carried value -- the averaging stirs the raw point into the
smoothed one, so his crossings trail the signal's swings. The repair
does what Alyssa's plan says: average raw with raw first, then
extract crossings from the averages alone. `ex_3_75` runs both over a
noisy version of the 3.74 sample -- one dip below zero and one
recovery, with sensor noise around both swings -- and answers both
fourteen-crossing prefixes with their flip counts: Louis's -1 and +1
land one element late; the repaired detector's sit at the raw swings.

Sample: `ex_3_75 ()` answers `([0; 0; 0; 0; 0; 0; 0; 0; -1; 0; 0; 0;
0; 1], [0; 0; 0; 0; 0; 0; 0; -1; 0; 0; 0; 0; 1; 0], 2, 2)`."""

R["3_76"] = """The map class is `T`. `smooth` is the reusable two-point averager,
and the modular detector is the crossing detector applied to the
smoothed stream -- exactly Eva Lu Ator's criticism answered. The
sample signal is the 3.74 sample, held at 0.0 forever after, so the
streams stay infinite the way real sensor signals are.

Sample: `ex_3_76 ()` answers the successive averages of 1, 2, 3, 4
and the crossings of the smoothed sample, whose fourteen elements
contain the signal's two crossings at the raw swings."""

R["3_77"] = """The map class is `T`. The cons-stream-style `integral` conses the
initial value and recurses on the tail; the repair takes the integrand
as a delayed argument and forces it only past the first element, which
is what lets feedback loops close. `ex_3_77` solves dy/dt = y at
dt = 0.001, reads y(1), and checks the empty-integrand case.

Sample: `ex_3_77 ()` answers y(1) within 0.002 of e, and the initial
value alone for an empty integrand."""

R["3_78"] = """The map class is `T`. The three streams -- y, its derivative, and
the second derivative -- are knotted through shared promises: dy
integrates ddy, y integrates dy, and ddy is the elementwise a*dy +
b*y over the two streams it feeds. The promises are planted before
either integrator runs and backpatched before anything is forced, so
every demand advances all three streams one element in lockstep.
`ex_3_78` solves the harmonic oscillator (a = 0, b = -1, y(0) = 0,
y'(0) = 1, dt = 0.001) and reads y at t = 1.57.

Sample: `ex_3_78 ()` answers y(1.57) = 1.00078..., within one step
of sin(pi/2) = 1."""

R["3_79"] = """The map class is `T`. The general solver is 3.78's knot with f in
place of the a/b combination. `ex_3_79` runs f(dy, y) = -y, the
harmonic oscillator of 3.78, and checks the two solvers agree at
several indices.

Sample: `ex_3_79 ()` answers `true`."""

R["3_80"] = """The map class is `T`. The RLC network knots two shared promises: v_C
integrates -i_L/C and i_L integrates v_C/L - R i_L/L, each through
exercise 3.77's delayed integrand. `ex_3_80` runs the statement's
circuit (R = 1 ohm, L = 1 henry, C = 0.2 farad, dt = 0.1 s, i_L0 = 0,
v_C0 = 10) and reads the first four values of both state streams.

Sample: `ex_3_80 ()` answers
`([10; 10; 9.5; 8.55], [0; 1; 1.9; 2.66])` -- the capacitor holds,
then gives way, while the inductor current builds."""

R["3_81"] = """The map class is `T`. The request stream is a value-level state
machine: each request is either `Generate` (apply `rand_update`) or
`Reset` (install the given value), and the answers stream conses one
value per request with the carried state threaded through -- no
assignment anywhere. `ex_3_81` runs the seven-request sample script
and replays it from the same seed. The seven answers are read with
`stream_ref` rather than `stream_take`, whose one-element overshoot
would demand an eighth request the script does not have.

Sample: `ex_3_81 ()` answers the seven values and `true`: the same
script from the same seed replays exactly."""

R["3_82"] = """The map class is `T`. `unit_pairs` draws successive pairs from the
seeded stateless generator scaled into the unit square --
`rand_update` folds each draw through the same 1e9 bound as 3.1, so
that bound is the scale -- and `estimate_integral` maps a predicate
over the points into the Monte Carlo process of the text, with no
trial-count argument anywhere. `pi_estimates` applies it to the
quarter disk. `ex_3_82` reads the estimates at 1,000 and 10,000
trials and checks both the sharpening and the error bound.

Sample: `ex_3_82 ()` answers estimates of about 3.136 and 3.146 with
`true`: ten thousand trials sit well within 0.1 of pi."""

TITLES = {
    "3_50": "the n-ary stream map as a stream-list function",
    "3_51": "show reveals when memoized delay evaluates",
    "3_51a": "an instrumented delay counts tail accesses against tail bodies",
    "3_52": "accum traces assignment plus laziness, memoized and not",
    "3_53": "the self-doubling stream",
    "3_54": "mul-streams and the factorial stream",
    "3_55": "partial-sums, the running total",
    "3_56": "Hamming numbers via a deduplicating merge",
    "3_57": "addition counts, memoized against plain thunks",
    "3_58": "expand computes long-division digits",
    "3_59": "integrate-series, and the sine and cosine series",
    "3_60": "mul-series and the sin^2 + cos^2 = 1 identity",
    "3_61": "invert-unit-series, the reciprocal of a series",
    "3_62": "div-series and the tangent series",
    "3_63": "why sqrt-stream binds guesses locally",
    "3_64": "stream-limit, the convergence helper",
    "3_65": "ln 2 at three accelerations",
    "3_66": "the order law of the pairs stream",
    "3_67": "all pairs, lower half included",
    "3_68": "Louis's whole-first-row pairs diverge",
    "3_69": "triples and the Pythagorean stream",
    "3_70": "merge-weighted and weighted-pairs",
    "3_71": "Ramanujan numbers from equal cube weights",
    "3_72": "sums of two squares, three ways",
    "3_73": "the RC circuit as a signal processor",
    "3_74": "zero crossings via the generalized map",
    "3_75": "Louis's smoothed detector and its repair",
    "3_76": "smooth as a reusable combinator",
    "3_77": "integral with a delayed integrand",
    "3_78": "solve-2nd, the second-order feedback network",
    "3_79": "the general second-order solver",
    "3_80": "the series RLC circuit as coupled streams",
    "3_81": "the request stream, replayable without assignment",
    "3_82": "streaming Monte Carlo integration",
}

count = 0
for name, body in sorted(R.items()):
    suffix = "a" if name.endswith("a") else ""
    num = name.replace("_", ".")
    kind = "(this edition's addition)" if name.endswith("a") else "(translate)"
    head = "# Exercise " + num + suffix + " \u2014 " + TITLES[name] + " " + kind + "\n\n"
    with open(os.path.join(OUT, "ex_" + name + ".md"), "w", encoding="utf-8") as f:
        f.write(head + body.strip() + "\n")
    count += 1
print("wrote", count, "rationales")
