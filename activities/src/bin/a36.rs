// Topic: Arrays & Slices
//
// Requirements:
// * Print pairs of numbers and their sums as they are streamed from a data source
// * If only one number is received, then print "Unpaired value: V",
//   where V is the value
// * If no numbers are received, print "Data stream complete"
//
// Notes:
// * A simulated data stream is already configured in the code
// * See the stdlib docs for the "chunks" method on "slice" for more info

fn data() -> &'static [u64] {
    &[5, 5, 4, 4, 3, 3, 1]
}

fn main() {
    // `stream` is an iterator of Option<&[u64]>
    let stream = data().chunks(2);

    stream.for_each(|chunk| match chunk {
      [one, two] => println!("Number 1: {one:?}, Number 2: {two:?}, Sum: {:?}\n", one + two),
      [one] => println!("Unpaired value: {one:?}\n"),
      [] => println!("Data stream completed"),
      [..] => unreachable!("chunk size is only 2"),
    })
}
