/// A highly specialized macro that queries a table for a list of entries.
///
/// # Parameters
///
/// - The schema to execute the query against
/// - The database connections pool
/// - The query parameters (used for paging)
///
/// # Example
///
/// ```ignore
/// use macros::list;
///
/// let skills: Vec<Skill> = list!(skills, pool, query);
/// ```
#[macro_export]
macro_rules! list {
    ($schema:ident, $pool:expr, $query:expr) => {
        actix_web::web::block(move || {
            $schema::table
                .select($schema::all_columns)
                .offset($query.offset().into())
                .limit($query.limit().into())
                .get_results(&mut $pool.get().unwrap())
        })
        .await??
    };
}
