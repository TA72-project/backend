/// A macros to get a single record from a given pool.
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
/// let skill: Skill = macros::get!(skills, pool, *id);
/// ```
#[macro_export]
macro_rules! get {
    ($schema:ident, $pool:expr, $id:expr) => {
        actix_web::web::block(move || {
            $schema::table
                .find($id)
                .get_result(&mut $pool.get().unwrap())
        })
        .await??;
    };
}
