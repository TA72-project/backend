/// A macros to delete a single record from a given pool.
///
/// # Parameters
///
/// - The schema to execute the query against
/// - The database connections pool
/// - The record id
///
/// # Example
///
/// ```ignore
/// let skill: Skill = macros::delete!(skills, pool, *id);
/// ```
#[macro_export]
macro_rules! delete {
    ($schema:ident, $pool:expr, $id:expr) => {
        actix_web::web::block(move || {
            diesel::delete($schema::table)
                .filter($schema::id.eq($id))
                .get_result(&mut $pool.get().unwrap())
        })
        .await??;
    };
}
