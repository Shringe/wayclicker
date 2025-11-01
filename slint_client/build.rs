use slint_build::CompilerConfiguration;

fn main() {
    let config = CompilerConfiguration::new().with_style("qt".to_string());
    slint_build::compile_with_config("ui/main.slint", config).unwrap();
}
