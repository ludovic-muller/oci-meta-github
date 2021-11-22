use anyhow::Context;
use lazy_static::lazy_static;
use regex::Regex;
use std::{cmp, str::FromStr};

use semver::Semver;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^refs/(?P<kind>[a-z]+)/(?P<name>.+)$").unwrap();
}

#[derive(Debug, Default, Clone)]
struct GitHubRef {
    kind: String,
    name: String,
}

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

fn parse_github_ref(github_ref: &str) -> anyhow::Result<GitHubRef> {
    let caps = RE.captures(github_ref).context("invalid GitHub ref")?;

    Ok(GitHubRef {
        kind: caps["kind"].to_string().trim().to_lowercase(),
        name: caps["name"].to_string().trim().to_lowercase(),
    })
}

fn get_github_ref_type(github_ref: GitHubRef, kind: &str) -> Option<String> {
    if github_ref.kind == kind {
        Some(github_ref.name)
    } else {
        None
    }
}

fn get_branch(github_ref: &str) -> Option<String> {
    let gh_ref = parse_github_ref(github_ref).ok()?;
    get_github_ref_type(gh_ref, "heads")
}

fn get_tag(github_ref: &str) -> Option<String> {
    let gh_ref = parse_github_ref(github_ref).ok()?;
    get_github_ref_type(gh_ref, "tags")
}

fn main() -> anyhow::Result<()> {
    // should be passed as environment variables
    let default_branch = "main";
    let github_ref = "refs/tags/v1.2.3";
    let github_run_id = "42";
    let github_sha = "abcdefghijklmnopqrstuvwxyz";
    let enable_git_branch_tag = false;
    let images = "example.com/org/image,example.com/org/image2";

    // some computed values
    let sha = short_sha(github_sha);
    let branch = get_branch(github_ref);
    let tag = get_tag(github_ref);
    let is_latest = match branch {
        Some(ref b) => b == &default_branch.to_lowercase(),
        None => false,
    };

    // results will be stored there
    let mut tags = String::from("");
    let labels = branch
        .as_deref()
        .or_else(|| tag.as_deref())
        .unwrap_or("latest");
    let labels = format!("org.opencontainers.image.version={}", labels);

    // loop over all images
    let images = images.split(',');
    for image in images {
        let image = image.trim().to_lowercase();
        let prefix = format!("{}:", image);
        let v_prefix = format!("{}v", prefix);

        if !tags.is_empty() {
            tags.push(',');
        }

        // those tags will always be present
        tags.push_str(&format!(
            "{}pipeline-{},{}git-{}",
            prefix, github_run_id, prefix, sha
        ));

        // case for branch
        if let Some(ref b) = branch {
            let b = &b.replace("/", "-");

            tags.push_str(&format!(",{}branch-{}", prefix, &b));
            if enable_git_branch_tag {
                tags.push_str(&format!(",{}git-{}-{}", prefix, &b, sha));
            }

            if is_latest {
                tags.push_str(&format!(",{}latest", prefix));
            }
        }

        // case for tag
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
    github_print("labels", labels);

    Ok(())
}
