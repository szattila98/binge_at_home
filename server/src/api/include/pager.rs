use std::ops::Deref;

use askama::Template;
use serde::Serialize;
use tap::Tap;
use tracing::Span;

#[derive(Debug, Serialize, Template)]
#[template(path = "includes/pager.html")]
pub struct PagerTemplate {
    pub total_pages: usize,
    pub current_page: usize,
    pub limit: usize,
    pub link: String,
}

impl PagerTemplate {
    pub fn new(total_pages: usize, current_page: usize, limit: usize, link: String) -> Self {
        Self {
            total_pages,
            current_page,
            limit,
            link,
        }
        .tap(|pager| {
            Span::current().record("pager", format!("{pager:?}"));
        })
    }

    pub fn shown_pages(&self) -> Vec<usize> {
        let lower_bound = (self.current_page.saturating_sub(self.limit / 2)).max(1);
        let upper_bound = (self.current_page.saturating_add(self.limit / 2)).min(self.total_pages);

        let mut result = (lower_bound..=upper_bound).collect::<Vec<_>>();

        while result.len() < self.limit && result.first().map_or(false, |&first| first > 1) {
            result.insert(0, (result[0] - 1).max(1));
        }

        while result.len() < self.limit
            && result.last().map_or(false, |&last| last < self.total_pages)
        {
            result.push(result[result.len() - 1] + 1);
        }

        result
    }

    pub const fn left_dots_visible(&self) -> bool {
        self.total_pages > self.limit && self.current_page > self.limit / 2 + 1
    }

    pub const fn right_dots_visible(&self) -> bool {
        self.total_pages > self.limit && self.current_page < self.total_pages - self.limit / 2
    }

    pub fn paged_link<T: Deref<Target = usize>>(&self, page: T) -> String {
        format!("{}&page={}", self.link, *page)
    }

    pub fn first_page_link(&self) -> String {
        format!("{}&page=1", self.link)
    }

    pub fn last_page_link(&self) -> String {
        format!("{}&page={}", self.link, self.total_pages)
    }
}
