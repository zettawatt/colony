fn main() {
    let config =
        slint_build::CompilerConfiguration::new()
        .with_style("fluent-dark".into());
    slint_build::compile_with_config("ui/main.slint", config).expect("Slint build failed");
    //slint_build::compile("ui/main.slint").expect("Slint build failed");
}
