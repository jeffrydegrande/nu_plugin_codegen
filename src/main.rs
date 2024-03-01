use nu_plugin::{serve_plugin, JsonSerializer};

use nu_codegen::NuCodeGen;

mod nu_codegen;

fn main() {
    serve_plugin(&mut NuCodeGen {}, JsonSerializer {});
}
