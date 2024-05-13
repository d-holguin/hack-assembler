use std::{env, error::Error, fs, path::PathBuf, process::Command};

#[test]
fn test_asm() {
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let test_asm_input_path = PathBuf::from(&project_dir).join("tests/input/test.asm");
    let expected_test_asm_output_path =
        PathBuf::from(&project_dir).join("tests/expected/test.hack");

    // Run the assembler with the test input file
    let test_asm_result = run_assembler(&test_asm_input_path.to_string_lossy());
    assert!(test_asm_result.is_ok(), "Assembler failed to run");
    let mut test_asm_result_contents =
        fs::read_to_string(test_asm_input_path.with_extension("hack"))
            .expect("Failed to read output file");
    let mut expected_test_asm_content = fs::read_to_string(expected_test_asm_output_path)
        .expect("Failed to read expected output file");

    test_asm_result_contents = test_asm_result_contents.replace("\r\n", "\n");
    expected_test_asm_content = expected_test_asm_content.replace("\r\n", "\n");

    assert_eq!(
        test_asm_result_contents.trim(),
        expected_test_asm_content.trim(),
        "Assembler output did not match expected output"
    );

    // Clean up
    fs::remove_file(test_asm_input_path.with_extension("hack"))
        .expect("Failed to clean up output file");
}

#[test]
fn pong_asm() {
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let test_asm_input_path = PathBuf::from(&project_dir).join("tests/input/Pong.asm");
    let expected_test_asm_output_path =
        PathBuf::from(&project_dir).join("tests/expected/Pong.hack");

    let test_asm_result = run_assembler(&test_asm_input_path.to_string_lossy());
    assert!(test_asm_result.is_ok(), "Assembler failed to run");
    let mut test_asm_result_contents =
        fs::read_to_string(test_asm_input_path.with_extension("hack"))
            .expect("Failed to read output file");
    let mut expected_test_asm_content = fs::read_to_string(expected_test_asm_output_path)
        .expect("Failed to read expected output file");

    test_asm_result_contents = test_asm_result_contents.replace("\r\n", "\n");
    expected_test_asm_content = expected_test_asm_content.replace("\r\n", "\n");

    assert_eq!(
        test_asm_result_contents.trim(),
        expected_test_asm_content.trim(),
        "Assembler output did not match expected output"
    );

    // Clean up
    fs::remove_file(test_asm_input_path.with_extension("hack"))
        .expect("Failed to clean up output file");
}

fn run_assembler(input: &str) -> Result<(), Box<dyn Error>> {
    let output = Command::new("cargo")
        .args(["run", "--", "-f", input])
        .output()?;

    if !output.status.success() {
        let errmsg = String::from_utf8_lossy(&output.stderr);
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            errmsg,
        )));
    }
    Ok(())
}
