/// A macros to get a single record from a given pool.
///
/// # Parameters
///
/// - The schema to execute the query against
/// - The database connections pool
/// - The record id
///
/// There are optional parameters that represents joined tables. See examples for more info.
///
/// # Examples
///
/// A simple `SELECT`
/// ```ignore
/// let skill: Skill = macros::get!(skills, pool, *id);
/// ```
///
/// A `SELECT` on `centers` with a join on `addresses`
/// ```ignore
/// let center: CenterWithAddresses = macros::get!(centers, pool, *id, addresses);
/// ```
///
/// You can specify multiple tables to join if necessary
/// ```ignore
/// let res: Nurse = macros::get!(nurses, pool, *id, users, addresses);
/// ```
#[macro_export]
macro_rules! get {
    ($schema:ident, $pool:expr, $id:expr $( ,$join:ident )*) => {
        actix_web::web::block(move || {
            $schema::table
                $(
                    .inner_join($join::table)
                )*
                .filter($schema::id.eq($id))
                .first(&mut $pool.get().unwrap())
        })
        .await??;
    };
}
