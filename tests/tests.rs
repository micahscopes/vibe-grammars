use dir_test::dir_test;
use insta::assert_snapshot;
use pest::{Parser, RuleType};
use vibe_grammars::util::to_sexpr;

fn snapshot_sexpr<P: Parser<R>, R: RuleType>(fixture: dir_test::Fixture<&str>, rule: R) {
    let input = *fixture.content();
    let result = P::parse(rule, input).unwrap();
    let tree = to_sexpr(result, 0);
    let stem = std::path::Path::new(fixture.path()).file_stem().unwrap().to_str().unwrap();
    assert_snapshot!(stem, tree);
}

use vibe_grammars::chordmark::{Parser as ChordMarkParser, Rule as ChordMarkRule};
use vibe_grammars::elements::{Parser as ElementsParser, Rule as ElementsRule};
#[dir_test(
    dir: "$CARGO_MANIFEST_DIR/tests/elements",
    glob: "*.txt",
)]
fn test_elements(fixture: dir_test::Fixture<&str>) {
  snapshot_sexpr::<ElementsParser, ElementsRule>(fixture, ElementsRule::chords);
}

#[dir_test(
    dir: "$CARGO_MANIFEST_DIR/tests/chordmark",
    glob: "*.txt",
)]
fn test_chordmark_song(fixture: dir_test::Fixture<&str>) {
  snapshot_sexpr::<ChordMarkParser, ChordMarkRule>(fixture, ChordMarkRule::song);
}