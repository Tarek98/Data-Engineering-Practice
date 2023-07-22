/*
// E7.1
fn main() {
    let y: i8 = 15;
    let x: i16 = 1000;

    // println!("{x} * {y} = {}", multiply(x, y.into()));
    println!("{x} * {y} = {}", multiply(x, i16::from(y)));
}
fn multiply(x: i16, y: i16) -> i16 {
    x * y
}
*/

/*
// E7.2
fn main() {
    let matrix = [
        [101, 102, 103],
        [201, 202, 203],
        [301, 302, 303]
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}
fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0;3];3];

    for i in 0..3 {
        for j in 0..3 {
            result[j][i] = matrix[i][j];
        }
    }

    return result;
}
fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for row in matrix {
        println!("{:?}", row);
    }
}
#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], //
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}
*/


// TODO: Come back to this when you get to traits & generics.
// E7.2 Bonus
// Once we get to traits and generics, we’ll be able to use the std::convert::AsRef trait to abstract over anything that can be referenced as a slice.
use std::convert::AsRef;
use std::fmt::Debug;
fn pretty_print<T, Line, Matrix>(matrix: Matrix)
where
    T: Debug,
    // A line references a slice of items
    Line: AsRef<[T]>,
    // A matrix references a slice of lines
    Matrix: AsRef<[Line]>
{
    for row in matrix.as_ref() {
        println!("{:?}", row.as_ref());
    }
}
fn main() {
    // &[&[i32]] --> 2 dimensional slice of slices
    pretty_print(&[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]]);
    // [[&str; 2]; 2]
    pretty_print([["a", "b"], ["c", "d"]]);
    // Vec<Vec<i32>>
    pretty_print(vec![vec![1, 2], vec![3, 4]]);
}

