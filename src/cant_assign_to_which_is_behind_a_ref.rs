#[cfg(test)]
mod test {
    #[allow(dead_code)]
    use crate::cant_assign_to_which_is_behind_a_ref::test::RunningStatus::Pending;
    use std::collections::HashMap;

    enum RunningStatus {
        New,
        Pending,
    }

    struct JobInfo {
        pub status: RunningStatus,
    }

    fn mut_or_not() {
        let mut job_map: HashMap<i64, JobInfo> = HashMap::new();
        job_map.insert(3, JobInfo { status: Pending });
        job_map.insert(
            2,
            JobInfo {
                status: RunningStatus::New,
            },
        );
        // WRONG: for mut job_info in job_map.iter()
        // at the same time, if job_map in another struct, it must derive Copy
        for mut job_info in job_map.into_iter() {
            if let RunningStatus::New = job_info.1.status {
                job_info.1.status = RunningStatus::Pending;
            }
        }
    }

    fn right() {}
}
