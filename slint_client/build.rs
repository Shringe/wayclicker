// use slint_build::CompilerConfiguration;

fn main() {
    // let config = CompilerConfiguration::new().with_style("natvi".to_string());
    slint_build::compile("ui.slint").unwrap();
}
