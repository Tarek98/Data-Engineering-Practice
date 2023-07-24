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

/*
// TODO: Come back to this when you get to traits & generics.
// E7.2 Bonus
// We use the std::convert::AsRef trait to abstract over anything that can be referenced as a slice.
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
*/

/*
// Copying & Cloning: int type implements copy by default instead of move semantics - however composite type Point does not implement copy by default.
// Copying refers to bitwise copies of memory regions and does not work on arbitrary objects. Cloning is a more general operation and also allows for custom behavior by implementing the Clone trait.
#[derive(Copy, Clone, Debug)]
struct Point(i32, i32, /* String */);
fn main() {
    let p1 = Point(3, 4);
    let p2 = p1; // let p2 = p1.clone();
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
}
*/


// E11.1
#[derive(Debug)]
struct Book {
    title: String,
    year: u16,
}
impl Book {
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}
#[derive(Debug)]
struct Library {
    books: Vec<Book>,
}
impl Library {
    fn new() -> Library {
        Library {
            books: Vec::new(),
        }
    }

    fn len(&self) -> usize {
        return self.books.len();
    }

    fn is_empty(&self) -> bool {
        return self.books.is_empty();
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn print_books(&self) {
       println!("{:?}",self);
    }

    fn oldest_book(&self) -> Option<&Book> {
        if self.books.is_empty() {
            return None;
        }
        let mut oldest = &self.books[0];
        let oldest_year = &oldest.year;
        for book in &self.books[1..] {
            if book.year < *oldest_year {
                oldest = book;
            }
        }
        return Some(oldest);
    }
}
fn main() {
    let mut library = Library::new();

    println!("The library is empty: library.is_empty() -> {}", library.is_empty());

    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));

    println!("The library is no longer empty: library.is_empty() -> {}", library.is_empty());

    library.print_books();

    match library.oldest_book() {
       Some(book) => println!("The oldest book is {}", book.title),
       None => println!("The library is empty!"),
    }

    println!("The library has {} books", library.len());
    library.print_books();
}

