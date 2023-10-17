//! Contains everything related to pagination, the query parameters and the response.

use serde::{Deserialize, Serialize};

/// Defines the query parameters that can be received for pagination.
#[derive(Clone, Deserialize)]
#[serde(default)]
pub struct PaginationParam {
    /// Page number to get
    pub page: u32,

    /// Number of record per page
    #[serde(alias = "perPage")]
    pub per_page: u8,
}

const DEFAULT_PAGE: u32 = 1;
const DEFAULT_PER_PAGE: u8 = 15;

impl Default for PaginationParam {
    fn default() -> Self {
        Self {
            page: DEFAULT_PAGE,
            per_page: DEFAULT_PER_PAGE,
        }
    }
}

impl PaginationParam {
    /// Computes the offset to get this pagination.
    pub fn offset(&self) -> u32 {
        self.per_page as u32 * (self.page - 1)
    }

    /// Get the limit of records to retrieve
    pub fn limit(&self) -> u8 {
        self.per_page
    }
}

/// A paginated response format.
///
/// Contains paging metadata and the inner data. `page` and `per_page` can be set from
/// [PaginationParam]. `total` and `total_page` can be set using [Self::total].
#[derive(Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    data: T,
    page: u32,
    per_page: u8,
    total: Option<u32>,
    total_page: Option<u32>,
}

impl<T: Serialize> PaginatedResponse<T> {
    /// Creates a new paginated response from inner data and query parameters.
    pub fn new(data: T, params: &PaginationParam) -> Self {
        Self {
            data,
            page: params.page,
            per_page: params.per_page,
            total: None,
            total_page: None,
        }
    }

    /// Sets the total number of records.
    ///
    /// The total number of pages is computed from this value.
    pub fn total(mut self, total: u32) -> Self {
        self.total = Some(total);
        self.total_page = Some((total as f32 / self.per_page as f32).ceil() as u32);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::{PaginatedResponse, PaginationParam};

    #[test]
    fn pagination_total() {
        let pag = PaginatedResponse::new(
            "",
            &PaginationParam {
                page: 1,
                per_page: 15,
            },
        )
        .total(30);

        assert_eq!(pag.total_page, Some(2));
    }

    #[test]
    fn pagination_total_round() {
        let pag = PaginatedResponse::new(
            "",
            &PaginationParam {
                page: 1,
                per_page: 15,
            },
        )
        .total(32);

        assert_eq!(pag.total_page, Some(3));
    }
}
