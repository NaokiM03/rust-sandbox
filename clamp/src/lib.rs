pub fn clamp(num: f32, bottom: f32, top: f32) -> f32 {
    num.min(top).max(bottom)
}

#[cfg(test)]
mod tests {
    use clamp;
    #[test]
    fn test_clamp() {
        assert_eq!(clamp(3., 0., 5.), 3.);
        assert_eq!(clamp(3., 0., 2.), 2.);
        assert_eq!(clamp(3., 4., 5.), 4.);
    }
}
