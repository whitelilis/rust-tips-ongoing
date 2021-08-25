//!
//! By default, function call will get the owner of args, so if you want use many times:
//! - in sync runtime: just use `&mut` , both borrow and changeable
//! - in async runtime: just use std::sync::Arc<tokio::sync::Mutex<SomeThing>>, when want to use,
//! ```rust, not_run
//!     let raw = ....
//!     let x = Arc<Mutex<SomeThing>>::new(raw);
//!
//!     x.lock().await.yyyyyyy;
//! ```
//!
//!

#[cfg(test)]
mod test {
    use crate::types::Big;
    fn foo1(x: &mut Big) {
        x.x = 15;
    }

    fn foo2(x: &mut Big) {
        x.y = "dkkd".to_string();
    }

    #[test]
    fn ttt() {
        let mut kk = Big {
            x: 3,
            y: "init".to_string(),
        };
        foo1(&mut kk);
        foo2(&mut kk);
        assert_eq!(kk.x, 15);
        assert_eq!(kk.y, "dkkd");
    }
}
