trait Ttt {
    fn three() -> i64 {
        3
    }
}

struct X {}
impl Ttt for X {}

#[cfg(test)]
mod test {
    use crate::using_method_in_trait::Ttt;
    use crate::using_method_in_trait::X;
    #[test]
    fn try_it() {
        /* below: cannot infer type
        let x = Ttt::three(); // cannot infer type


        using like below
        */
        let x = X::three();
        println!("{:?}", x);
    }
}
