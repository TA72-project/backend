/// A highly specialized macro that queries a table for a list of entries.
///
/// # Parameters
///
/// - The schema to execute the query against
/// - The database connections pool
/// - The query parameters (used for paging)
///
/// There are optional parameters to specify joined tables.
///
/// # Example
///
/// A simple `SELECT`
/// ```ignore
/// use macros::list;
///
/// let skills: Vec<Skill> = list!(skills, pool, query);
/// ```
///
/// A `SELECT` on `centers` with a join on `addresses`
/// ```ignore
/// use macros::list;
///
/// let centers: Vec<CentersWithAddresses> = list!(centers, pool, query, addresses);
/// ```
///
/// You can specify multiple tables to join on if needed
/// ```ignore
/// use macros::list;
///
/// let skills: Vec<Nurse> = list!(nurses, pool, query, users, addresses);
/// ```
#[macro_export]
macro_rules! list {
    ($schema:ident, $pool:expr, $query:expr $( ,$join:ident)*) => {
        actix_web::web::block(move || {
            $schema::table
                $(
                    .inner_join($join::table)
                )*
                .offset($query.offset().into())
                .limit($query.limit().into())
                .load(&mut $pool.get().unwrap())
        })
        .await??
    };
}
