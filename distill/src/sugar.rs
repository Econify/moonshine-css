
use serde_yaml as yaml;
use serde::Deserialize;
use std::collections::HashMap;


const SRC: &str = r#"

bg-[$colors as $color]:
  background-color: $color

text-[$colors as $color]:
  color: $color

"#;

type Rules = HashMap<String, HashMap<String, String>>;

#[test]
fn test() {
    let rules: Rules = yaml::from_str(SRC).unwrap();
    println!("{:#?}", rules);
}