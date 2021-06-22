use serde::Serialize;
use std::env;
use tera::Context;
use tera::Tera;

#[derive(Serialize)]
struct Post {
    feed_title: String,
    feed_url: String,
    title: String,
    date: String,
    description: String,
    url: String,
}

fn fetch_posts(feed_url: &str) -> Option<Vec<Post>> {
    let response = reqwest::blocking::get(feed_url).ok()?.bytes().ok()?;
    let feed = feed_rs::parser::parse(&response[..]).ok()?;

    Some(
        feed.entries
            .iter()
            .filter_map(|post| {
                Some(Post {
                    feed_title: feed.title.as_ref()?.content.to_string(),
                    feed_url: feed_url.to_string(),
                    title: post.title.as_ref()?.content.to_string(),
                    date: post
                        .published
                        .or_else(|| post.updated)
                        .as_ref()?
                        .to_rfc3339(),
                    description: if let Some(summary) = post.summary.as_ref() {
                        summary.content.to_string()
                    } else {
                        "".to_string()
                    },
                    url: post.links.first()?.href.to_string(),
                })
            })
            .collect(),
    )
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let template = args.get(1).ok_or("Missing template.")?;
    let feeds: Vec<&str> = args.get(2).ok_or("Missing feeds.")?.lines().collect();

    let posts: Vec<Post> = feeds
        .iter()
        .filter_map(|&feed_url| {
            let posts = fetch_posts(feed_url);
            if posts.is_none() || posts.as_ref().unwrap().is_empty() {
                eprintln!("No parseable posts found for: {}", feed_url)
            };
            posts
        })
        .flatten()
        .collect();

    let mut context = Context::new();
    context.insert("posts", &posts);
    let output =
        Tera::one_off(template, &context, false).or_else(|_| Err("Failed to generate output."))?;
    print!("{}", output);

    Ok(())
}
