use std::collections::HashMap;

struct YamlParser {
    x: int
}

impl YamlParser {
    fn parse(file_content: &str) -> HashMap<&str, uint> {
           let mut h = HashMap::new();

           h.insert("key", 123u);

           h
       }
}
