use cli::run;
use dev;
use jrest::expect;

#[test]
fn runs_returns_the_expected_output() {
    // TODO Add Windows test.
    // TODO Add macOS test.
    if cfg!(linux) {
        let command = "ls".to_string();
        let args = vec![
            "-al".to_string(),
            dev::get_sample_directory_absolute_path_option().unwrap(),
        ];

        let stdout_callback = |index: usize, line: String| match index {
            0 => {
                expect!(&line).to_be(&("total 20".to_string()));
            }
            1 => {
                expect!(&line).to_end_with(&(".".to_string()));
            }
            2 => {
                expect!(&line).to_end_with(&("..".to_string()));
            }
            3 => {
                expect!(&line).to_end_with(&("Da".to_string()));
            }
            4 => {
                expect!(&line).to_end_with(&("Db".to_string()));
            }
            5 => {
                expect!(&line).to_end_with(&("F1.txt".to_string()));
            }
            6 => {
                expect!(&line).to_end_with(&("F2.txt".to_string()));
            }
            7 => {
                expect!(&line).to_end_with(&("INFECTED.eicar.com.txt".to_string()));
            }
            8 => {
                expect!(&line).to_end_with(&("S_Broken -> Broken".to_string()));
            }
            9 => {
                expect!(&line).to_end_with(&("S_Daa -> Da/Daa".to_string()));
            }
            10 => {
                expect!(&line).to_end_with(&("S_DaaF1.txt -> Da/Daa/DaaF1.txt".to_string()));
            }
            _ => {
                panic!("Unexpected index.")
            }
        };

        let stderr_callback = |_index: usize, _line: String| {};

        run(command, args, stdout_callback, stderr_callback);
    }
}
