use ArclightSyntaxTree::ArclightSyntaxTree;

pub fn generate(ast: ArclightSyntaxTree) -> Result<(),ArclightGeneratorError> {
    let mut compiled = "".to_string();
    compiled.push_str("fn main() {\n");

    for photon in ast.iter() {
        if photon.left.is_none() {
            compiled.push_str(&photon.token);
            compiled.push_str("(");
        } else {
            if compiled.last() != "(" {
                compiled.push_str(",");
            }
            compiled.push_str(&photon.token);
        }
    }
    
    compiled.push_str(");\n}");
    
    println!("{}", compiled);
    Ok(())
}

enum ArclightGeneratorError {
    Unknown,
}