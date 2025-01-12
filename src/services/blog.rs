use std::{collections::HashMap, path::Path, sync::Arc};

use crate::{app_config::AppConfig, models::posts::Post};
use anyhow::Result;

pub struct BlogService {
    pub posts: HashMap<String, Post>,
    pub ordered_posts: Vec<String>,
}

impl BlogService {
    pub fn new(app_config: Arc<AppConfig>) -> Result<Self> {
        let posts = Post::read_all_from_dir(&Path::new(&app_config.posts_dir).canonicalize()?)?;
        let mut values: Vec<&Post> = posts
            .values()
            .filter(|p| p.metadata.date.is_some())
            .collect();

        values.sort_by(|a, b| {
            b.metadata
                .date
                .as_ref()
                .unwrap()
                .cmp(a.metadata.date.as_ref().unwrap())
        });

        let ordered_posts: Vec<String> = values
            .into_iter()
            .map(|p| {
                let slug = &p.metadata.slug.as_ref().unwrap().to_owned();
                slug.to_string()
            })
            .collect();

        Ok(Self {
            posts,
            ordered_posts,
        })
    }

    pub fn get_all_posts(&self) -> impl Iterator<Item = &str> {
        self.ordered_posts.iter().map(|s| s.as_ref())
    }

    pub fn get_post(&self, key: &str) -> Option<&Post> {
        self.posts.get(key)
    }
}
