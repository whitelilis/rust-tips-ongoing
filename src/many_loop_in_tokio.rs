#[cfg(test)]
mod test {
    use tokio::time;
    use tokio::time::Duration;

    async fn loop1() {
        let mut interval_step = time::interval(Duration::from_secs(5));
        loop {
            interval_step.tick().await;
            eprintln!("loop1");
        }
    }

    async fn loop2() {
        let mut interval_step = time::interval(Duration::from_secs(3));
        loop {
            interval_step.tick().await;
            eprintln!("loop2");
        }
    }

    #[tokio::test(threaded_scheduler)]
    async fn mm() {
        //sim().await;
        tokio::spawn(loop1());
        tokio::spawn(loop2());

        // NOTICE 1: without some loop, the main thread will exit.
        // NOTICE 2: int 'test' runtime, spawned println! not work, should use debugger to see it worked
        //loop {}
    }
}
