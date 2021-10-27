use std::{cmp, str::FromStr};

use semver::Semver;

fn github_print(output_name: &str, value: String) {
    println!("::set-output name={}::{}", output_name, value);
}

fn short_sha(github_sha: &str) -> &str {
    let sha_len = github_sha.chars().count();
    let sha_len = cmp::min(sha_len, 8);

    let mut end: usize = 0;
    github_sha
        .chars()
        .into_iter()
        .take(sha_len)
        .for_each(|x| end += x.len_utf8());

    &github_sha[..end]
}

fn get_branch(github_ref: &str) -> Option<String> {
    None
}

fn get_tag(github_ref: &str) -> Option<String> {
    None
}

fn main() -> anyhow::Result<()> {
    // should be passed as environment variables
    let default_branch = "main";
    let github_ref = "refs/heads/feature-branch-1";
    let github_run_id = "42";
    let github_sha = "abcdefghijklmnopqrstuvwxyz";
    let images = "example.com/org/image,example.com/org/image2";

    // some computed values
    let sha = short_sha(github_sha);
    let branch = get_branch(github_ref);
    let tag = get_tag(github_ref);
    let is_latest = match branch {
        Some(b) => b == default_branch,
        None => false,
    };

    // results will be stored there
    let mut tags = String::from("");

    // loop over all images
    let images = images.split(',');
    for image in images {
        let image = image.trim().to_lowercase();
        let prefix = format!("{}:", image);
        let v_prefix = format!("{}v", prefix);

        if !tags.is_empty() {
            tags.push(',');
        }

        tags.push_str(&format!(
            "{}pipeline-{},{}git-{}",
            prefix, github_run_id, prefix, sha
        ));
        if is_latest {
            tags.push_str(&format!(",{}latest", prefix));
        }

        if let Some(ref tag) = tag {
            if let Ok(version) = Semver::from_str(tag) {
                let version_tags = version.single_line(v_prefix);
                if !version_tags.is_empty() {
                    tags.push_str(&format!(",{}", version_tags));
                }
            }
        }
    }

    // print output
    github_print("tags", tags);

    Ok(())
}
