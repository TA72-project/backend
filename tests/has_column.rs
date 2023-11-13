mod models {
    pub use backend::models::HasColumn;
}

mod simple {
    use backend::models::HasColumn;
    use backend_derive::HasColumn;

    #[derive(HasColumn)]
    struct Foo {
        #[allow(dead_code)]
        bar: i32,
    }

    #[test]
    fn contain_field() {
        assert!(Foo::has_column("bar"));
    }

    #[test]
    fn not_contain_field() {
        assert!(!Foo::has_column("nope"));
    }
}

mod multiple {
    use backend::models::HasColumn;
    use backend_derive::HasColumn;

    #[derive(HasColumn)]
    struct Foo {
        #[allow(dead_code)]
        bar: i32,
        #[allow(dead_code)]
        pub baz: i32,
    }

    #[test]
    fn contain_field() {
        assert!(Foo::has_column("bar"));
        assert!(Foo::has_column("baz"));
    }

    #[test]
    fn not_contain_field() {
        assert!(!Foo::has_column("nope"));
    }
}

mod serde_skip {
    use backend::models::HasColumn;
    use backend_derive::HasColumn;
    use serde::Serialize;

    #[derive(Serialize, HasColumn)]
    struct Foo {
        #[allow(dead_code)]
        bar: i32,
        #[allow(dead_code)]
        #[serde(skip)]
        pub baz: i32,
    }

    #[test]
    fn contain_field() {
        assert!(Foo::has_column("bar"));
    }

    #[test]
    fn not_contain_field() {
        assert!(!Foo::has_column("nope"));
    }

    #[test]
    fn not_contain_skipped() {
        assert!(!Foo::has_column("baz"));
    }
}
