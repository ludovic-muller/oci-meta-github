use std::str::FromStr;

use semver::Semver;

fn github_print(output_name: &str, value: String) {
    println!("::set-output name={}::{}", output_name, value);
}

fn main() -> anyhow::Result<()> {
    let pipeline = "42";
    let sha = "12345678";
    let is_latest = true;
    let version = Semver::from_str("v1.2.3")?;
    let prefix = "example.com/org/image:";
    let v_prefix = format!("{}v", prefix);
    let mut tags = version.single_line(v_prefix);
    let ci_tags = format!("{}pipeline-{},{}git-{}", prefix, pipeline, prefix, sha);
    if !tags.is_empty() {
        tags.push(',');
    }
    tags.push_str(&ci_tags);
    if is_latest {
        tags.push_str(&format!(",{}latest", prefix));
    }

    github_print("tags", tags);

    Ok(())
}
