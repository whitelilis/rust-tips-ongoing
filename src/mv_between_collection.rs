#[cfg(test)]
mod test {

    struct Big {
        m1: String,
        m2: String,
    }

    #[test]
    fn move_between_collections() {
        let mut src: Vec<Big> = Vec::new();
        src.push(Big {
            m1: "hello".to_string(),
            m2: "what".to_string(),
        });
        src.push(Big {
            m1: "hello".to_string(),
            m2: "rust".to_string(),
        });

        let mut dst: Vec<Big> = Vec::new();
        /*
         Use .into_iter() instead of .iter() to get owned elements without making a copy of them.
        .iter() gives you a borrow, and if you’ve borrowed something, you can’t take ownership of
         it. *x happens to work on borrowed Copy types, because the impossibility of taking
         ownership of borrowed value is solved by making a copy instead. But String is not copy,
         so it won’t be copied, so *x means you’re trying to steal a borrowed reference.
         https://users.rust-lang.org/t/easier-way-to-move-elements-out-from-an-owned-collection/25205
         */
        for m in src.into_iter() {
            dst.push(m);
        }
    }
}
