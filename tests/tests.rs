use dir_test::dir_test;
use insta::assert_snapshot;
use pest::Parser;
use vibe_grammars::{elements::{Parser as ElementsParser, Rule as ElementsRule}, util::to_sexpr};

#[dir_test(
    dir: "$CARGO_MANIFEST_DIR/tests/elements",
    glob: "chord*.txt",
)]
fn test_chords(fixture: dir_test::Fixture<&str>) {
    let input = *fixture.content();
    let result = ElementsParser::parse(ElementsRule::chords, input).unwrap();
    let result = to_sexpr(result, 0);
    let path = fixture.path();
    use std::path::Path;
    let path = Path::new(&path);
    let stem = path.file_stem().unwrap().to_str().unwrap();
    println!("Stem: {}", stem);
    assert_snapshot!(stem, result);
}