#[cfg(test)]
mod tokio_test_m {

    // with multiple thread

    #[tokio::test(threaded_scheduler)]
    async fn my_test() {
        assert!(true);
    }

    // with single thread
    #[tokio::test]
    async fn my_test_single() {
        assert!(true);
    }
}
