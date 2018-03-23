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

pub enum ArclightGeneratorError {
    Unknown,
}