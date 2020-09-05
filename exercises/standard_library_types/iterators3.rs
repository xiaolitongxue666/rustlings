// iterators3.rs
// This is a bigger exercise than most of the others! You can do it!
// Here is your mission, should you choose to accept it:
// 1. Complete the divide function to get the first four tests to pass
// 2. Uncomment the last two tests and get them to pass by filling in
//    values for `x` using `division_results`.
// Execute `rustlings hint iterators3` to get some hints!
// Have fun :-)

// hint
// Minor hint: In each of the two cases in the match in main, you can create x with either
// a 'turbofish' or by hinting the type of x to the compiler. You may try both.
//
// Major hint: Have a look at the Iter trait and at the explanation of its collect function.
// Especially the part about Result is interesting.

#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}

// This function should calculate `a` divided by `b` if `a` is
// evenly divisible by b.
// Otherwise, it should return a suitable error.
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {

    if b==0 {
        Err(DivisionError::DivideByZero)
    }else {
        if a%b == 0 {
            println!("a:{} divided by b:{}", a, b);
            Ok(a/b)
        }
        else {
            let mut result_err = NotDivisibleError{dividend: a, divisor: b};
            Err(DivisionError::NotDivisible(result_err))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests that verify your `divide` function implementation
    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(
            divide(81, 6),
            Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: 81,
                divisor: 6
            }))
        );
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    // Iterator exercises using your `divide` function
    #[test]
    fn result_with_list() {
        let numbers = vec![27, 297, 38502, 81];
        let division_results = numbers.into_iter().map(|n| divide(n, 27));
        let x : Result<Vec<i32>,Vec<DivisionError>> = Ok(division_results.flatten().collect::<Vec<i32>>());//... Fill in here!
        assert_eq!(format!("{:?}", x), "Ok([1, 11, 1426, 3])");
    }

    #[test]
    fn list_of_results() {
        let numbers = vec![27, 297, 38502, 81];
        let division_results = numbers.into_iter().map(|n| divide(n, 27));
        // let x: Vec<Result<i32, DivisionError>> = division_results.collect();//... Fill in here! First solution
        let x: Vec<_> = division_results.collect();//... Fill in here! Second solution
        assert_eq!(format!("{:?}", x), "[Ok(1), Ok(11), Ok(1426), Ok(3)]");
    }

}
