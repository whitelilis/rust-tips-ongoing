#[cfg(test)]
mod test {
    #[test]
    fn subtle_index() {
        let x = [1, 2, 3, 4, 5, 6];
        let y = &x;
        assert_eq!(3, x[..3].len());
        assert_eq!(3, x[0..3].len());
        assert_eq!(3, y[0..3].len());
        assert_eq!(3, y[..3].len());
    }
}
