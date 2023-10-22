use trybuild::TestCases;

#[test]
fn test() {
	let tb = TestCases::new();
	tb.pass("./tests/snippets/triple.rs");
}
