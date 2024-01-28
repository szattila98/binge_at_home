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
    pub link: String,
}

impl PagerTemplate {
    const PAGE_LIMIT: usize = 10;

    pub fn new(total_pages: usize, current_page: usize, link: String) -> Self {
        Self {
            total_pages,
            current_page,
            link,
        }
        .tap(|pager| {
            Span::current().record("pager", format!("{pager:?}"));
        })
    }

    pub fn shown_pages(&self) -> Vec<usize> {
        let lower_bound = (self.current_page.saturating_sub(Self::PAGE_LIMIT / 2)).max(1);
        let upper_bound =
            (self.current_page.saturating_add(Self::PAGE_LIMIT / 2)).min(self.total_pages);

        let mut result = (lower_bound..=upper_bound).collect::<Vec<_>>();

        while result.len() < Self::PAGE_LIMIT && result.first().map_or(false, |&first| first > 1) {
            result.insert(0, (result[0] - 1).max(1));
        }

        while result.len() < Self::PAGE_LIMIT
            && result.last().map_or(false, |&last| last < self.total_pages)
        {
            result.push(result[result.len() - 1] + 1);
        }

        result
    }

    pub const fn left_dots_visible(&self) -> bool {
        self.total_pages > Self::PAGE_LIMIT && self.current_page > Self::PAGE_LIMIT / 2 + 1
    }

    pub const fn right_dots_visible(&self) -> bool {
        self.total_pages > Self::PAGE_LIMIT
            && self.current_page < self.total_pages - Self::PAGE_LIMIT / 2
    }

    pub fn paged_link<T: Deref<Target = usize>>(&self, page: T) -> String {
        self.get_paged_link(*page)
    }

    pub fn first_page_link(&self) -> String {
        self.get_paged_link(1)
    }

    pub fn last_page_link(&self) -> String {
        self.get_paged_link(self.total_pages)
    }

    fn get_paged_link(&self, page: usize) -> String {
        return if self.link.contains("?") {
            format!("{}&page={page}", self.link)
        } else {
            format!("{}?page={page}", self.link)
        };
    }
}
