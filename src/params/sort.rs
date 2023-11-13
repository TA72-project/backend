/// Holds the parameters a route can receive to sort data.
use std::{
    fmt::{self, Display},
    str::FromStr,
};

use serde::de;
use utoipa::IntoParams;

/// Direction in which to sort data.
#[derive(Default, Debug, PartialEq, Eq, Clone)]
enum OrderByDirection {
    #[default]
    Asc,
    Desc,
}

impl Display for OrderByDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Asc => "asc",
                Self::Desc => "desc",
            }
        )
    }
}

impl FromStr for OrderByDirection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err(format!("invalid direction '{}'", s)),
        }
    }
}

/// Parameter to sort data.
///
/// `sort` is for the query while `col` and `direction` are the validated data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SortParam {
    /// Field and direction, `:` separated.
    sort: String,
    /// Column to sort by.
    col: String,
    /// Direction by which to sort.
    direction: OrderByDirection,
}

impl SortParam {
    /// Returns the end of an `ORDER BY` statement.
    ///
    /// # Example
    ///
    /// ```
    /// let param = SortParam::default();
    /// assert_eq(param.value(), "id asc");
    /// ```
    #[must_use]
    pub fn value(&self) -> String {
        format!("{} {}", self.col, self.direction)
    }

    /// Returns an sql statement usable by [`diesel::query_dsl::QueryDsl::order`].
    pub fn raw_sql(&self) -> diesel::expression::SqlLiteral<diesel::sql_types::Text> {
        diesel::dsl::sql::<diesel::sql_types::Text>(&self.value())
    }
}

impl Default for SortParam {
    fn default() -> Self {
        Self {
            sort: format!("1:{}", OrderByDirection::default()),
            col: String::from("1"),
            direction: OrderByDirection::default(),
        }
    }
}

impl IntoParams for SortParam {
    fn into_params(
        parameter_in_provider: impl Fn() -> Option<utoipa::openapi::path::ParameterIn>,
    ) -> Vec<utoipa::openapi::path::Parameter> {
        vec![utoipa::openapi::path::ParameterBuilder::new()
            .name("sort")
            .description(Some(
                "Field and direction to sort by.\n\nTakes the form `<column>[:asc|desc]`.",
            ))
            .parameter_in(parameter_in_provider().unwrap_or_default())
            .required(utoipa::openapi::Required::False)
            .schema(Some(
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::SchemaType::String)
                    .pattern(Some(r"[a-z_\.0-9]+(:(asc|desc))?")),
            ))
            .build()]
    }
}

impl<'de> de::Deserialize<'de> for SortParam {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct SortParamVisitor;

        impl<'de> de::Visitor<'de> for SortParamVisitor {
            type Value = SortParam;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("sort parameters")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                if let Some("sort") = map.next_key::<&str>()? {
                    let value: String = map.next_value()?;
                    let mut data = value.splitn(2, ':');

                    let col: String = data
                        .next()
                        .expect("sort cannot be empty, expected column name")
                        .into();

                    if col.is_empty() {
                        return Err(de::Error::custom("column is empty"));
                    }

                    let direction = if let Some(dir) = data.next() {
                        OrderByDirection::from_str(dir).map_err(|e| {
                            de::Error::custom(format!("{}, expected 'asc' or 'desc'", e))
                        })?
                    } else {
                        OrderByDirection::default()
                    };

                    Ok(SortParam {
                        sort: value.clone(),
                        col,
                        direction,
                    })
                } else {
                    Ok(SortParam::default())
                }
            }
        }

        deserializer.deserialize_map(SortParamVisitor)
    }
}

#[cfg(test)]
mod tests {
    use actix_web::web::Query;

    use super::*;

    /// No query parameter should give default sort order
    #[test]
    fn no_param_default() {
        let params = Query::<SortParam>::from_query("").unwrap();
        assert_eq!(params.into_inner(), SortParam::default());
    }

    /// Only a column name sort by column name and default direction.
    #[test]
    fn only_column_name() {
        let params = Query::<SortParam>::from_query("sort=name").unwrap();
        assert_eq!(
            params.into_inner(),
            SortParam {
                sort: "name".into(),
                col: "name".into(),
                direction: OrderByDirection::default()
            }
        );
    }

    /// A column name and direction gets both.
    #[test]
    fn column_and_direction() {
        let params = Query::<SortParam>::from_query("sort=name:desc").unwrap();
        assert_eq!(
            params.into_inner(),
            SortParam {
                sort: "name:desc".into(),
                col: "name".into(),
                direction: OrderByDirection::Desc
            }
        );
    }

    /// No value gives error
    #[test]
    fn param_no_value() {
        assert!(Query::<SortParam>::from_query("sort=").is_err());
    }

    /// Incorrect direction gives error
    #[test]
    fn incorrect_direction() {
        assert!(Query::<SortParam>::from_query("sort=name:incorrect_direction").is_err());
    }
}
