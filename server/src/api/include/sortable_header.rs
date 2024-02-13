use std::fmt::{Debug, Display};

use askama::Template;
use tap::Tap;
use tracing::Span;
use tracing_unwrap::ResultExt;
use url::Url;

use crate::crud::Direction;
use crate::crud::SORT_QUERY_PARAM_DELIMITER;

const SORT_PARAM: &str = "sort";
const DUMMY_BASE_FOR_URI: &str = "http://example.com";

#[derive(Debug, Template)]
#[template(path = "includes/sortable-header.html")]
pub struct SortableHeaderTemplate<T>
where
    T: Debug + Display,
{
    pub text: String,
    pub sort: T,
    pub direction: Option<Direction>,
    pub link: String,
}

impl<T> SortableHeaderTemplate<T>
where
    T: Debug + Display,
{
    pub fn new(text: String, sort: T, direction: Option<Direction>, link: String) -> Self {
        Self {
            text,
            sort,
            direction,
            link,
        }
        .tap(|sortable_header| {
            Span::current().record("sortable_header", format!("{sortable_header:?}"));
        })
    }

    pub fn link_with_sort_params(&self) -> String {
        // TODO use URL parsing for pager and breadcrumbs query params too, make util functions for dummy url base and replacing as well
        let mut path = Url::parse(&format!("{}{}", DUMMY_BASE_FOR_URI, &self.link))
            .expect_or_log("sortable header link parsing failed");
        {
            let path_clone = path.clone();
            let filtered_query_params = path_clone
                .query_pairs()
                .filter(|(_, value)| !value.contains(&self.sort.to_string()))
                .collect::<Vec<_>>();

            let mut query_params = path.query_pairs_mut();
            query_params.clear();
            query_params.extend_pairs(filtered_query_params);
            match self.direction {
                Some(ref direction) => match direction {
                    Direction::Asc => {
                        query_params.append_pair(
                            SORT_PARAM,
                            &format!(
                                "{}{SORT_QUERY_PARAM_DELIMITER}{}",
                                self.sort,
                                Direction::Desc
                            ),
                        );
                    }
                    Direction::Desc => (),
                },
                None => {
                    query_params.append_pair(
                        SORT_PARAM,
                        &format!(
                            "{}{SORT_QUERY_PARAM_DELIMITER}{}",
                            self.sort,
                            Direction::Asc
                        ),
                    );
                }
            };
        }
        path.to_string().replace(DUMMY_BASE_FOR_URI, "")
    }
}
