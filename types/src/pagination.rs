use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub struct LinkedPages {
  pub previous: Option<i64>,
  pub next: Option<i64>,
  pub first: Option<i64>,
  pub last: Option<i64>,
}

impl<T: AsRef<str>> From<T> for LinkedPages {
  fn from(header: T) -> Self {
    let mut linked_pages = Self::default();
    let sections = header.as_ref().split(",").map(|v| v.trim());

    for section in sections {
      let Some((wrapped_url, raw_rel)) = section.split_once("; ") else {
        continue;
      };

      let Some(url) = wrapped_url
        .replace("<", "")
        .replace(">", "")
        .parse::<Url>()
        .ok()
      else {
        continue;
      };

      let query: HashMap<_, _> = url.query_pairs().into_owned().collect();
      let page = query.get("page").and_then(|page| page.parse().ok());
      let quoteless_rel = raw_rel.replace("\"", "");

      let Some((_, rel)) = quoteless_rel.split_once("rel=") else {
        continue;
      };

      match rel {
        "prev" => {
          linked_pages.previous = page;
        }
        "next" => {
          linked_pages.next = page;
        }
        "first" => {
          linked_pages.first = page;
        }
        "last" => {
          linked_pages.last = page;
        }
        _ => continue,
      }
    }

    linked_pages
  }
}

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct PaginatedData<Data> {
  pub data: Data,
  pub pages: LinkedPages,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_pagination_from_string() {
    let header_value = r#"<https://api.github.com/repositories/1300192/issues?page=2>; rel="prev", <https://api.github.com/repositories/1300192/issues?page=4>; rel="next", <https://api.github.com/repositories/1300192/issues?page=515>; rel="last", <https://api.github.com/repositories/1300192/issues?page=1>; rel="first""#;
    let pages = LinkedPages::from(header_value);

    assert_eq!(
      pages,
      LinkedPages {
        previous: Some(2),
        next: Some(4),
        first: Some(1),
        last: Some(515)
      }
    )
  }
}
