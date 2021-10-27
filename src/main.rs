use std::{cmp, str::FromStr};

use semver::Semver;

fn github_print(output_name: &str, value: String) {
    println!("::set-output name={}::{}", output_name, value);
}

fn short_sha(sha: &str) -> &str {
    let sha_len = sha.chars().count();
    let sha_len = cmp::min(sha_len, 8);

    let mut end: usize = 0;
    sha.chars()
        .into_iter()
        .take(sha_len)
        .for_each(|x| end += x.len_utf8());

    &sha[..end]
}

fn main() -> anyhow::Result<()> {
    let default_branch = "main";
    let github_ref = "refs/heads/feature-branch-1";
    let github_run_id = "42";
    let github_sha = "abcdefghijklmnopqrstuvwxyz";

    let sha = short_sha(github_sha);

    let is_latest = true;
    let version = Semver::from_str("v1.2.3")?;
    let prefix = "example.com/org/image:";
    let v_prefix = format!("{}v", prefix);
    let mut tags = version.single_line(v_prefix);
    let ci_tags = format!("{}pipeline-{},{}git-{}", prefix, github_run_id, prefix, sha);
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
