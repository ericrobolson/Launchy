/// Maps a value from one numerical range to another.
#[allow(unused_variables)]
pub fn index_1d_to_2d<N>(index: N, width: N, height: N) -> (N, N)
where
    N: core::ops::Sub<Output = N>
        + Copy
        + core::ops::Div<Output = N>
        + core::ops::Mul<Output = N>
        + core::ops::Rem<Output = N>,
{
    let x = index % width;
    let y = index / width;

    (x, y)
}

/// Maps a value from one numerical range to another.
#[allow(unused_variables)]
pub fn index_2d_to_1d<N>(x: N, y: N, width: N, height: N) -> N
where
    N: core::ops::Sub<Output = N>
        + Copy
        + core::ops::Div<Output = N>
        + core::ops::Mul<Output = N>
        + core::ops::Rem<Output = N>
        + core::ops::Add<Output = N>,
{
    let x = x % width;
    let y = y % height;

    y * width + x
}
