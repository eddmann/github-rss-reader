use rss::Channel;
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
    let feed = Channel::read_from(&response[..]).ok()?;

    Some(
        feed.items
            .iter()
            .filter_map(|post| {
                Some(Post {
                    feed_title: feed.title().to_string(),
                    feed_url: feed.link().to_string(),
                    title: post.title()?.to_string(),
                    date: post.pub_date().unwrap_or_default().to_string(),
                    description: post.description().unwrap_or_default().to_string(),
                    url: post.link()?.to_string(),
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
        .filter_map(|&feed_url| fetch_posts(feed_url))
        .flatten()
        .collect();

    let mut context = Context::new();
    context.insert("posts", &posts);
    let output =
        Tera::one_off(template, &context, false).or_else(|_| Err("Failed to generate output."))?;
    print!("{}", output);

    Ok(())
}
