#[cfg(test)]
mod test {

    // the below derive PartiaEq  is needed

    #[derive(PartialEq)]
    enum MyOpt {
        Opt1,
        Opt2,
    }

    #[test]
    fn cmp_test() {
        assert!(MyOpt::Opt1 != MyOpt::Opt2);
        assert!(MyOpt::Opt1.eq(&MyOpt::Opt1));
        assert!(!matches!(MyOpt::Opt2, MyOpt::Opt1));
    }
}
