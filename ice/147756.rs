fn array() {
    #[rustc_layout_scalar_valid_range_start(1)]
    struct NonZero<T>([T; 4]);
    let nums = [1, 2, 3, 4];
    let mut foo = unsafe { NonZero(nums) };
}