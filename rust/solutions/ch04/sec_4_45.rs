// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.45: typed grammar parses are
//! compiled into explicit search paths with guarded token consumption.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_3::{AnswerTerm, AnswerValue, Predicate, Search, SearchEngine};
use support::{int, var};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Word {
    The,
    Professor,
    Lectures,
    To,
    Student,
    In,
    Class,
    With,
    Cat,
    Eats,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Part {
    Article,
    Noun,
    Verb,
    Preposition,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Tree {
    SimpleNoun {
        article: Word,
        noun: Word,
    },
    NounPhraseSimple(Box<Tree>),
    NounPhraseExtended {
        base: Box<Tree>,
        prep: Box<Tree>,
    },
    PrepPhrase {
        prep: Word,
        object: Box<Tree>,
    },
    VerbSingle {
        verb: Word,
    },
    VerbExtended {
        base: Box<Tree>,
        prep: Box<Tree>,
    },
    Sentence {
        subject: Box<Tree>,
        predicate: Box<Tree>,
    },
}

fn part(word: Word) -> Part {
    match word {
        Word::The => Part::Article,
        Word::Professor | Word::Student | Word::Class | Word::Cat => Part::Noun,
        Word::Lectures | Word::Eats => Part::Verb,
        Word::To | Word::In | Word::With => Part::Preposition,
    }
}

fn word_code(word: Word) -> i64 {
    match word {
        Word::The => 0,
        Word::Professor => 1,
        Word::Lectures => 2,
        Word::To => 3,
        Word::Student => 4,
        Word::In => 5,
        Word::Class => 6,
        Word::With => 7,
        Word::Cat => 8,
        Word::Eats => 9,
    }
}

fn parse_word(input: &[Word], position: usize, expected: Part) -> Vec<(Word, usize)> {
    match input.get(position) {
        Some(word) if part(*word) == expected => vec![(*word, position + 1)],
        _ => Vec::new(),
    }
}

fn parse_simple_noun(input: &[Word], position: usize) -> Vec<(Tree, usize)> {
    let mut parses = Vec::new();
    for (article, after_article) in parse_word(input, position, Part::Article) {
        for (noun, after_noun) in parse_word(input, after_article, Part::Noun) {
            parses.push((Tree::SimpleNoun { article, noun }, after_noun));
        }
    }
    parses
}

fn parse_noun_extensions(base: &Tree, input: &[Word], position: usize) -> Vec<(Tree, usize)> {
    let mut parses = vec![((*base).clone(), position)];
    for (prep, after_prep) in parse_word(input, position, Part::Preposition) {
        for (object, after_object) in parse_noun(input, after_prep) {
            let extended = Tree::NounPhraseExtended {
                base: Box::new((*base).clone()),
                prep: Box::new(Tree::PrepPhrase {
                    prep,
                    object: Box::new(object),
                }),
            };
            parses.extend(parse_noun_extensions(&extended, input, after_object));
        }
    }
    parses
}

fn parse_noun(input: &[Word], position: usize) -> Vec<(Tree, usize)> {
    let mut parses = Vec::new();
    for (simple, after_simple) in parse_simple_noun(input, position) {
        parses.extend(parse_noun_extensions(
            &Tree::NounPhraseSimple(Box::new(simple)),
            input,
            after_simple,
        ));
    }
    parses
}

fn parse_verb_extensions(base: &Tree, input: &[Word], position: usize) -> Vec<(Tree, usize)> {
    let mut parses = vec![((*base).clone(), position)];
    for (prep, after_prep) in parse_word(input, position, Part::Preposition) {
        for (object, after_object) in parse_noun(input, after_prep) {
            let extended = Tree::VerbExtended {
                base: Box::new((*base).clone()),
                prep: Box::new(Tree::PrepPhrase {
                    prep,
                    object: Box::new(object),
                }),
            };
            parses.extend(parse_verb_extensions(&extended, input, after_object));
        }
    }
    parses
}

fn parse_verb(input: &[Word], position: usize) -> Vec<(Tree, usize)> {
    let mut parses = Vec::new();
    for (verb, after_verb) in parse_word(input, position, Part::Verb) {
        parses.extend(parse_verb_extensions(
            &Tree::VerbSingle { verb },
            input,
            after_verb,
        ));
    }
    parses
}

fn parse_sentence(input: &[Word]) -> Vec<(Tree, usize)> {
    let mut parses = Vec::new();
    for (subject, after_subject) in parse_noun(input, 0) {
        for (predicate, end) in parse_verb(input, after_subject) {
            if end == input.len() {
                parses.push((
                    Tree::Sentence {
                        subject: Box::new(subject.clone()),
                        predicate: Box::new(predicate.clone()),
                    },
                    end,
                ));
            }
        }
    }
    parses
}

fn render_tree(tree: &Tree) -> String {
    match tree {
        Tree::SimpleNoun { article, noun } => format!("Noun({article:?},{noun:?})"),
        Tree::NounPhraseSimple(base) => format!("NP({})", render_tree(base)),
        Tree::NounPhraseExtended { base, prep } => {
            format!("NP({}, {})", render_tree(base), render_tree(prep))
        }
        Tree::PrepPhrase { prep, object } => {
            format!("PP({prep:?}, {})", render_tree(object))
        }
        Tree::VerbSingle { verb } => format!("Verb({verb:?})"),
        Tree::VerbExtended { base, prep } => {
            format!("VP({}, {})", render_tree(base), render_tree(prep))
        }
        Tree::Sentence { subject, predicate } => {
            format!("S({}, {})", render_tree(subject), render_tree(predicate))
        }
    }
}

fn search_path(input: &[Word], tree: &Tree, start: usize, end: usize) -> Search {
    let end_index = i64::try_from(end).expect("small parse span");
    let mut body = Search::Guard(
        Predicate::Eq(var("unparsed"), int(end_index)),
        Box::new(Search::Success(vec![AnswerTerm::Atom(render_tree(tree))])),
    );
    for index in (start..end).rev() {
        let position = i64::try_from(index).expect("small parse span");
        body = Search::Guard(
            Predicate::Eq(var("unparsed"), int(position)),
            Box::new(Search::Set(
                "unparsed".to_owned(),
                position + 1,
                Box::new(Search::Set(
                    "token".to_owned(),
                    word_code(input[index]),
                    Box::new(body),
                )),
            )),
        );
    }
    let start_index = i64::try_from(start).expect("small parse span");
    Search::Set("unparsed".to_owned(), start_index, Box::new(body))
}

const SENTENCE: &[Word] = &[
    Word::The,
    Word::Professor,
    Word::Lectures,
    Word::To,
    Word::The,
    Word::Student,
    Word::In,
    Word::The,
    Word::Class,
    Word::With,
    Word::The,
    Word::Cat,
];

const MALFORMED: &[Word] = &[Word::The, Word::Cat, Word::Eats, Word::Cat];

fn program_for(input: &[Word]) -> Search {
    let paths = parse_sentence(input)
        .into_iter()
        .map(|(tree, end)| search_path(input, &tree, 0, end))
        .collect();
    Search::Choose(paths)
}

fn ambiguous_program() -> Search {
    program_for(SENTENCE)
}

fn malformed_program() -> Search {
    program_for(MALFORMED)
}

#[test]
fn ex_4_45() {
    let outcome = SearchEngine::new().run(&ambiguous_program());
    let labels: Vec<String> = outcome
        .answers
        .iter()
        .map(|answer| match answer.first() {
            Some(AnswerValue::Sym(label)) => label.clone(),
            _ => panic!("parse path answers a rendered tree"),
        })
        .collect();
    assert_eq!(labels.len(), 5);
    assert_eq!(
        labels
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len(),
        5
    );
    assert!(labels.iter().all(|label| label.starts_with("S(")));
    assert!(
        SearchEngine::new()
            .run(&malformed_program())
            .answers
            .is_empty()
    );
}
