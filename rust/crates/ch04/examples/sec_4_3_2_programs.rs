// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.3

//! Section 4.3.2: parsing and constraint problems as searches. The
//! multiple-dwelling puzzle is a choice per occupant with one guard
//! per constraint; the ambiguous sentence is a recursive grammar over
//! a token list with one choice per attachment, and the alternative
//! parse is what the next answer of the same search yields. Parse
//! trees travel as pre-order node streams, each category's arity
//! fixing the tree.

use ch04::sec_4_3::{AnswerTerm, AnswerValue, Search, SearchEngine, reference_model};
use sicp_runtime::host::query::{Predicate, Term};

fn node(name: &str) -> AnswerTerm {
    AnswerTerm::Atom(name.to_owned())
}

fn var(name: &str) -> AnswerTerm {
    AnswerTerm::Var(name.to_owned())
}

fn cell(name: &str) -> Term {
    Term::Variable(name.to_owned())
}

fn run(program: &Search) -> ch04::sec_4_3::SearchOutcome {
    let outcome = SearchEngine::new().run(program);
    assert_eq!(outcome, reference_model(program));
    outcome
}

/// Nests one guard per conjunct over the answer.
fn under(guards: Vec<Predicate>, answer: Search) -> Search {
    guards
        .into_iter()
        .rev()
        .fold(answer, |body, guard| Search::Guard(guard, Box::new(body)))
}

/// Binds the named constants before running the body.
fn with_constants(constants: [(&str, i64); 4], body: Search) -> Search {
    constants
        .into_iter()
        .rev()
        .fold(body, |body, (name, value)| {
            Search::Set(name.to_owned(), value, Box::new(body))
        })
}

/// One range choice per name over `1..=5`, outermost first.
fn over(names: [&str; 5], body: Search) -> Search {
    names
        .into_iter()
        .rev()
        .fold(body, |body, name| Search::ChooseRange {
            var: name.to_owned(),
            lo: 1,
            hi: 5,
            body: Box::new(body),
        })
}

#[derive(Clone)]
struct ParseTree {
    label: String,
    children: Vec<ParseTree>,
}

fn leaf(word: &str) -> ParseTree {
    ParseTree {
        label: word.to_owned(),
        children: Vec::new(),
    }
}

fn branch(label: &str, children: Vec<ParseTree>) -> ParseTree {
    ParseTree {
        label: label.to_owned(),
        children,
    }
}

/// Matches the word at `position` against one grammar category.
fn parse_word(
    words: &[&str],
    position: usize,
    category: &str,
    choices: &[&str],
) -> Option<(ParseTree, usize)> {
    let word = *words.get(position)?;
    choices
        .contains(&word)
        .then(|| (branch(category, vec![leaf(word)]), position + 1))
}

fn parse_simple_noun_phrase(words: &[&str], position: usize) -> Option<(ParseTree, usize)> {
    let (article, after_article) = parse_word(words, position, "article", &["a", "the"])?;
    let (noun, after_noun) = parse_word(
        words,
        after_article,
        "noun",
        &["professor", "student", "cat", "class"],
    )?;
    Some((
        branch("simple-noun-phrase", vec![article, noun]),
        after_noun,
    ))
}

fn parse_prepositional_phrases(words: &[&str], position: usize) -> Vec<(ParseTree, usize)> {
    let Some((preposition, after_preposition)) =
        parse_word(words, position, "prep", &["for", "to", "in", "by", "with"])
    else {
        return Vec::new();
    };

    parse_noun_phrases(words, after_preposition)
        .into_iter()
        .map(|(noun_phrase, end)| {
            (
                branch("prep-phrase", vec![preposition.clone(), noun_phrase]),
                end,
            )
        })
        .collect()
}

fn extend_noun_phrase(
    words: &[&str],
    base: &ParseTree,
    position: usize,
) -> Vec<(ParseTree, usize)> {
    let mut extended_phrases = Vec::new();
    for (prepositional_phrase, end) in parse_prepositional_phrases(words, position) {
        let phrase = branch("noun-phrase", vec![base.clone(), prepositional_phrase]);
        extended_phrases.push((phrase.clone(), end));
        extended_phrases.extend(extend_noun_phrase(words, &phrase, end));
    }
    extended_phrases
}

fn parse_noun_phrases(words: &[&str], position: usize) -> Vec<(ParseTree, usize)> {
    let Some((simple, end)) = parse_simple_noun_phrase(words, position) else {
        return Vec::new();
    };
    let mut phrases = vec![(simple.clone(), end)];
    phrases.extend(extend_noun_phrase(words, &simple, end));
    phrases
}

fn extend_verb_phrase(
    words: &[&str],
    base: &ParseTree,
    position: usize,
) -> Vec<(ParseTree, usize)> {
    let mut extended_phrases = Vec::new();
    for (prepositional_phrase, end) in parse_prepositional_phrases(words, position) {
        let phrase = branch("verb-phrase", vec![base.clone(), prepositional_phrase]);
        extended_phrases.push((phrase.clone(), end));
        extended_phrases.extend(extend_verb_phrase(words, &phrase, end));
    }
    extended_phrases
}

fn parse_verb_phrases(words: &[&str], position: usize) -> Vec<(ParseTree, usize)> {
    let Some((verb, end)) = parse_word(words, position, "verb", &["eats", "sleeps", "lectures"])
    else {
        return Vec::new();
    };
    let mut phrases = vec![(verb.clone(), end)];
    phrases.extend(extend_verb_phrase(words, &verb, end));
    phrases
}

fn parse_sentences(words: &[&str]) -> Vec<ParseTree> {
    let mut sentences = Vec::new();
    for (subject, after_subject) in parse_noun_phrases(words, 0) {
        for (verb_phrase, end) in parse_verb_phrases(words, after_subject) {
            if end == words.len() {
                sentences.push(branch("sentence", vec![subject.clone(), verb_phrase]));
            }
        }
    }
    sentences
}

fn flatten(tree: &ParseTree, answer: &mut Vec<AnswerTerm>) {
    answer.push(node(&tree.label));
    for child in &tree.children {
        flatten(child, answer);
    }
}

fn parse_search(words: &[&str]) -> Search {
    let alternatives: Vec<Search> = parse_sentences(words)
        .iter()
        .map(|tree| {
            let mut answer = Vec::new();
            flatten(tree, &mut answer);
            Search::Success(answer)
        })
        .collect();
    Search::Choose(alternatives)
}

/// `multiple-dwelling`: one choice per occupant, one guard per
/// constraint. Floor differences of at least two are the enumerated
/// differences `2`, `3`, and `4` in either direction -- the floors
/// live in `1..=5`, so nothing else qualifies.
fn multiple_dwelling() -> Search {
    let distinct = [
        ("baker", "cooper"),
        ("baker", "fletcher"),
        ("baker", "miller"),
        ("baker", "smith"),
        ("cooper", "fletcher"),
        ("cooper", "miller"),
        ("cooper", "smith"),
        ("fletcher", "miller"),
        ("fletcher", "smith"),
        ("miller", "smith"),
    ];
    let mut guards: Vec<Predicate> = distinct
        .iter()
        .map(|(a, b)| Predicate::Ne(cell(a), cell(b)))
        .collect();
    guards.push(Predicate::Ne(cell("baker"), Term::Integer(5)));
    guards.push(Predicate::Ne(cell("cooper"), Term::Integer(1)));
    guards.push(Predicate::Ne(cell("fletcher"), Term::Integer(5)));
    guards.push(Predicate::Ne(cell("fletcher"), Term::Integer(1)));
    guards.push(Predicate::Gt(cell("miller"), cell("cooper")));
    for (high, low) in [("smith", "fletcher"), ("fletcher", "cooper")] {
        guards.push(Predicate::Or(
            [("c2", "c0"), ("c3", "c0"), ("c4", "c0")]
                .iter()
                .flat_map(|(plus, zero)| {
                    [
                        Predicate::DiffEq(
                            high.to_owned(),
                            low.to_owned(),
                            (*plus).to_owned(),
                            (*zero).to_owned(),
                        ),
                        Predicate::DiffEq(
                            low.to_owned(),
                            high.to_owned(),
                            (*plus).to_owned(),
                            (*zero).to_owned(),
                        ),
                    ]
                })
                .collect(),
        ));
    }
    let answer = Search::Success(vec![
        node("baker"),
        var("baker"),
        node("cooper"),
        var("cooper"),
        node("fletcher"),
        var("fletcher"),
        node("miller"),
        var("miller"),
        node("smith"),
        var("smith"),
    ]);
    with_constants(
        [("c0", 0), ("c2", 2), ("c3", 3), ("c4", 4)],
        over(
            ["baker", "cooper", "fletcher", "miller", "smith"],
            under(guards, answer),
        ),
    )
}

/// The book's ambiguous sentence: "the professor lectures to the
/// student with the cat"; its final prepositional phrase attaches to
/// the noun phrase or to the verb phrase.
fn ambiguous_sentence() -> Search {
    parse_search(&[
        "the",
        "professor",
        "lectures",
        "to",
        "the",
        "student",
        "with",
        "the",
        "cat",
    ])
}

/// A sentence without a prepositional phrase: one parse, no
/// attachment alternative.
fn simple_sentence() -> Search {
    parse_search(&["the", "cat", "eats"])
}

fn main() {
    // One answer per occupant: the book's listing.
    let outcome = run(&multiple_dwelling());
    assert_eq!(
        outcome.answers,
        [[
            AnswerValue::Sym(String::from("baker")),
            AnswerValue::Int(3),
            AnswerValue::Sym(String::from("cooper")),
            AnswerValue::Int(2),
            AnswerValue::Sym(String::from("fletcher")),
            AnswerValue::Int(4),
            AnswerValue::Sym(String::from("miller")),
            AnswerValue::Int(5),
            AnswerValue::Sym(String::from("smith")),
            AnswerValue::Int(1),
        ]]
    );

    // Parse the book's ambiguous sentence with the grammar above. The
    // parser reads each token once along each candidate, recursively
    // extending noun phrases and verb phrases by prepositional phrases.
    // `Search::Choose` then exposes the two resulting derivations in
    // grammar order, so requesting the next answer resumes at the
    // alternative attachment.
    let outcome = run(&ambiguous_sentence());
    assert_eq!(outcome.answers.len(), 2);
    assert_eq!(
        outcome.answers[0][7],
        AnswerValue::Sym(String::from("verb-phrase"))
    );
    assert_eq!(
        outcome.answers[1][12],
        AnswerValue::Sym(String::from("noun-phrase"))
    );

    // The same grammar parses a sentence without a prepositional
    // phrase once; no attachment alternative is introduced.
    let outcome = run(&simple_sentence());
    assert_eq!(outcome.answers.len(), 1);
    assert_eq!(
        outcome.answers[0],
        [
            AnswerValue::Sym(String::from("sentence")),
            AnswerValue::Sym(String::from("simple-noun-phrase")),
            AnswerValue::Sym(String::from("article")),
            AnswerValue::Sym(String::from("the")),
            AnswerValue::Sym(String::from("noun")),
            AnswerValue::Sym(String::from("cat")),
            AnswerValue::Sym(String::from("verb")),
            AnswerValue::Sym(String::from("eats")),
        ]
    );
}
