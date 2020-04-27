use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use slug::slugify;
use std::cmp::Ord;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct Blog {
    pub conf: BlogConf,
    pub posts: HashMap<String, Post>,
    pub ordered_posts: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BlogConf {
    title: String,
    page_size: Option<u16>,
    enable_drafts: Option<bool>,
    posts_dir: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    metadata: PostMetadata,
    content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostMetadata {
    title: String,
    slug: Option<String>,
    date: Option<DateTime<Utc>>,
}

impl Blog {
    pub fn from_conf(conf: BlogConf) -> Result<Self> {
        let posts = Post::read_all_from_dir(Path::new(conf.get_post_dir().as_ref().unwrap()))?;
        let mut values: Vec<&Post> = posts
            .values()
            .filter(|p| p.get_metadata().get_date().is_some())
            .collect();

        values.sort_by(|a, b| {
            a.get_metadata()
                .get_date()
                .as_ref()
                .unwrap()
                .cmp(b.get_metadata().get_date().as_ref().unwrap())
        });

        let ordered_posts: Vec<String> = values
            .into_iter()
            .map(|p| {
                let slug = &p.get_metadata().get_slug().as_ref().unwrap().to_owned();
                slug.to_string()
            })
            .collect();

        Ok(Blog {
            conf,
            posts,
            ordered_posts,
        })
    }

    pub fn get_all_posts(&self) -> &[String] {
        &self.ordered_posts
    }

    pub fn get_post(&self, key: &str) -> Option<&Post> {
        self.posts.get(key)
    }
}

impl BlogConf {
    pub fn new_from_file(path: &Path) -> Result<Self> {
        if !path.exists() {
            bail!("File not found - {:?}", &path);
        }

        let conf_contents = fs::read_to_string(&path)?;
        let mut conf: BlogConf = serde_yaml::from_str(&conf_contents)?;

        if let None = &conf.posts_dir {
            conf.posts_dir = Some(String::from("./posts"));
        }

        if !Path::new(&conf.posts_dir.as_ref().unwrap()).exists() {
            bail!(
                "Directory not found - {:?}",
                conf.posts_dir.as_ref().unwrap()
            );
        }

        Ok(conf)
    }

    pub fn get_post_dir(&self) -> &Option<String> {
        &self.posts_dir
    }
}

impl Post {
    pub fn read_all_from_dir(path: &Path) -> Result<HashMap<String, Post>> {
        let mut map = HashMap::new();

        let paths = fs::read_dir(path)?;
        for path in paths {
            let path = path?.path();
            let metadata = fs::metadata(&path)?;
            if metadata.is_file() {
                let post = Post::new_from_file(&path)?;
                let key = post.metadata.slug.as_ref().unwrap();
                if map.contains_key(key) {
                    bail!("Post {:?} already exists", &path);
                } else {
                    map.insert(String::from(key), post);
                }
            }
        }

        Ok(map)
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

        let post = Post {
            metadata,
            content: String::from(content.trim()),
        };

        Ok(post)
    }

    pub fn new_from_file(path: &Path) -> Result<Post> {
        let raw = fs::read_to_string(path)?;
        Post::new_from_str(&raw)
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn get_metadata(&self) -> &PostMetadata {
        &self.metadata
    }
}

impl PostMetadata {
    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_slug(&self) -> &Option<String> {
        &self.slug
    }

    pub fn get_date(&self) -> &Option<DateTime<Utc>> {
        &self.date
    }
}
