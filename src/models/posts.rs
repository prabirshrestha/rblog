use std::{
    collections::HashMap,
    ffi::OsStr,
    fs,
    path::Path,};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use slug::slugify;
use tracing::warn;

use crate::{
    models::{attachment::Attachment, post_metadata::PostMetadata},
    utils::markdown::markdown_to_html,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    pub metadata: PostMetadata,
    pub content: String,
    pub html_content: String,
    pub attachments: HashMap<String, Attachment>,
}

impl Post {
    pub fn read_all_from_dir(path: &Path) -> Result<HashMap<String, Post>> {
        let mut map = HashMap::new();

        for path in fs::read_dir(path)? {
            let path = path?.path();
            let metadata = fs::metadata(&path)?;
            let post = if metadata.is_file() {
                if path.extension().and_then(OsStr::to_str).unwrap_or("") != "md" {
                    continue;
                }
                Post::new_from_file(&path)?
            } else {
                Post::new_from_dir(&path)?
            };

            let key = post.metadata.slug.as_ref().unwrap();
            if map.contains_key(key) {
                bail!("Post {:?} already exists", &path);
            } else {
                map.insert(String::from(key), post);
            }
        }

        Ok(map)
    }

    pub fn new_from_file(path: &Path) -> Result<Post> {
        let raw = fs::read_to_string(path)?;
        Post::new_from_str(&raw)
    }

    pub fn new_from_dir(path: &Path) -> Result<Post> {
        // find the first *.md, rest of the files become attachments
        let mut all_md_files: Vec<fs::DirEntry> = fs::read_dir(path)?
            .filter_map(Result::ok)
            .filter_map(|d| {
                if d.path().extension()? == "md" {
                    Some(d)
                } else {
                    None
                }
            })
            .collect();
        all_md_files.sort_by_key(|p| p.path());
        let post_file = all_md_files
            .first()
            .with_context(|| format!("No markdown file found in directory {:?}", path))?;
        let mut post = Post::new_from_file(&post_file.path())?;

        for entry in fs::read_dir(path)? {
            let entry_path = entry?.path();
            let metadata = fs::metadata(&entry_path)?;
            if metadata.is_file() && entry_path.to_str() != post_file.path().to_str() {
                let name = match entry_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(|s| s.to_string())
                {
                    Some(n) => n,
                    None => {
                        warn!("Skipping attachment with non-UTF8 filename: {:?}", entry_path);
                        continue;
                    }
                };
                post.attachments.insert(name, Attachment { path: entry_path });
            }
        }

        Ok(post)
    }

    pub fn new_from_str(raw: &str) -> Result<Post> {
        let header_start = raw.find("---");
        if header_start.is_none() {
            bail!("--- header not found");
        }
        let header_start = header_start.unwrap();

        let content_start = &raw[header_start + 3..].find("---");
        if content_start.is_none() {
            bail!("--- content not found");
        }
        let content_start = content_start.unwrap();

        let header = &raw[header_start..content_start + 3];
        let content = &raw[header.len() + 3..];

        let mut metadata: PostMetadata = serde_yaml::from_str(header)?;

        metadata.slug = match &metadata.slug {
            Some(slug) => Some(slug.trim().to_lowercase()),
            None => Some(slugify(&metadata.title)),
        };

        let content = content.trim();

        let post = Post {
            metadata,
            content: content.into(),
            html_content: markdown_to_html(content),
            attachments: HashMap::new(),
        };

        Ok(post)
    }

    pub fn get_url(&self) -> String {
        format!("/posts/{}/", &self.metadata.slug.as_ref().unwrap())
    }
}
