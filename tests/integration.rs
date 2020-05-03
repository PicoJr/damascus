#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use assert_fs::fixture::{FileWriteStr, FixtureError, PathChild};
    use assert_fs::TempDir;

    #[test]
    fn test_simple() -> Result<(), FixtureError> {
        let temp_dir = TempDir::new().unwrap();
        let stream0_path = temp_dir.child("stream0");
        stream0_path.write_str("AA")?;
        let stream1_path = temp_dir.child("stream1");
        stream1_path.write_str("BBBB")?;
        let default_path = temp_dir.child("default");
        default_path.write_str("CC")?;
        let mut cmd = Command::cargo_bin("damascus").unwrap();
        cmd.arg(stream0_path.path().to_str().unwrap())
            .arg(stream1_path.path().to_str().unwrap())
            .arg(default_path.path().to_str().unwrap())
            .assert()
            .stdout("AABBCCBB")
            .success();
        Ok(())
    }
}
