use std::process::Command;


pub fn weather_query(city: &str)-> &str {
    let output: std::process::Output = Command::new("qw").args(["geo", "city-lookup", city]).output().expect("调用命令行工具时出现错误!");
    if output.status.success() {
        let stdout: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
        println!("{stdout}");
    } else {
        let stderr: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stderr);
        eprintln!("Command failed: {stderr}");
    }

    "abc"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qweather() {
        println!("将要测试weather_query函数:");
        weather_query("abc");
    }
}
