use ArclightSyntaxTree::ArclightSyntaxTree;
use Property::PropertyErr;
use ActiveProperties::ActiveProperties;

#[derive(Debug)]
struct Compiler {
    compiled: String,
    active_properties: ActiveProperties,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            compiled: String::new(),
            active_properties: ActiveProperties::new(),
        }
    }

    pub fn generate(&mut self, ast: &mut ArclightSyntaxTree) -> Result<(),CompilerError> {
        const LIFETIME: &str = "lifetime";
        const FN: &str = "fn";
        const LET: &str = "let";
        const REF: &str = "&";
        const MUT: &str = "mut";
        const MUTREF: &str = "&mut";
        const META: &str = "meta";

        self.compiled.push_str("fn main() {\n");

        for photon in ast.iter() {

            match photon.token.as_str() {
                FN => {
                },
                LET => {
                },
                LIFETIME => {
                },
                REF => {
                },
                MUT => {
                },
                META => {
                    // execute generic_photon on the next one
                },
                _ => {
                    self.generic_photon(&photon, &mut compiled);
                },
            }
        }

        self.compiled.push_str(");\n}");
        Ok()
    }
    
    fn resolve_marker(&mut self, ast: &mut ArclightSyntaxTree) -> Result<(),PropertyErr> {
        for property in self.active_properties() {
            property.resolve()?;
        }
    }
}

#[derive(Debug)]
enum CompilerError {
}
