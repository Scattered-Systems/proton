/*
    Appellation: default <test>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[cfg(test)]
#[test]
fn lib_compiles() {
    let f = |x: usize, y: usize, z: usize| (x * y) + z;
    let a = f(10, 2, 10);
    let b = 30;
    assert_eq!(a, b)
}
