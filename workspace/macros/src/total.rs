/// A highly specialized macro that queries `COUNT` on a table inside an `actix_web::web::block`.
///
/// # Parameters
///
/// - The table to execute the `COUNT` on
/// - The database connections pool
///
/// # Example
///
/// ```ignore
/// use macros::total;
///
/// let total = total!(schema::skills::table, pool);
/// ```
#[macro_export]
macro_rules! total {
    ($schema:ident, $pool:expr) => {
        actix_web::web::block(move || {
            $schema::table
                .count()
                .get_result::<i64>(&mut $pool.get().unwrap())
        })
        .await?? as u32
    };
}
