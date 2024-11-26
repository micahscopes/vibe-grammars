use pest::iterators::Pairs;
use pest::RuleType;

pub fn to_sexpr<R: RuleType>(pairs: Pairs<R>, indent: usize) -> String {
    let mut output = String::new();
    let indentation = " ".repeat(indent);
    for pair in pairs {
      let rule = pair.as_rule();
      let inner_data = pair.as_str(); // Get the inner data of the pair
      let inner_data = inner_data.replace("\n", &format!("\n{indentation}  "));
      let inner_pairs = pair.into_inner();
      output.push_str(&format!("{indentation}( {rule:?}\n{indentation}  `{inner_data}`\n"));
      output.push_str(&to_sexpr(inner_pairs, indent + 2));
      output.push_str(&format!("{indentation})\n"));
    }
    output
}