/// Initializes an array [T; N] dynamically with the elements produced using the `producer` method
pub fn array_init<const N: usize, T, F: FnMut(usize) -> T>(mut producer: F) -> [T; N] {
    let mut idx = 0;

    [(); N].map(|_| {
        let result = producer(idx);
        idx += 1;

        result
    })
}


#[macro_export]
macro_rules! arr {
    ($producer:expr) => {
        array_init($producer)
    };
    ($producer:expr; $N:literal) => {
        array_init::<$N, _, _>($producer)
    };
    ($($val:expr),+) => {
        [$($val),+]
    }
}