use ArclightSyntaxTree::ArclightSyntaxTree;

pub fn generate(ast: ArclightSyntaxTree) -> Result<String,ArclightGeneratorError> {
    let mut compiled = "".to_string();
    compiled.push_str("fn main() {\n");

    for photon in ast.iter() {
        if photon.left.is_none() {
            compiled.push_str(&photon.token);
            compiled.push_str("(");
        } else {
            if compiled.chars().last().unwrap() != '(' {
                compiled.push_str(",");
            }
            compiled.push_str(&photon.token);
        }
    }
    
    compiled.push_str(");\n}");
    Ok(compiled)
}

#[derive(Debug)]
pub enum ArclightGeneratorError {
    Unknown,
}

mod tests {
    use ArclightSyntaxTree::ArclightSyntaxTree;
    use Parser::parse;
    use Generator::generate;

    #[test]
    fn hello_world_test() {
        let mut hello_world = ArclightSyntaxTree::new();
        hello_world.build_at_marker(parse("println! \"Hello world!\"").expect("Testing hello_world_test, hello_world parse"));

        assert_eq!("fn main() {\nprintln!(\"Hello world!\");\n}".to_string(), generate(hello_world).expect("Testing hello_world_test, hello_world generate"));
    }
}