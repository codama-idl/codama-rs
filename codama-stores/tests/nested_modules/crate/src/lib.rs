// Item index: 0
pub type Foo = ();

// Item index: 1
pub mod nested_1 {
    // Item index: 0
    pub mod nested_2 {
        // Item index: 0
        pub mod person;
    }

    // Item index: 1
    pub type Foo = ();

    // Item index: 2
    pub mod membership;
}
