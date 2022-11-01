use std::io;

fn main() {
    println!("Number of Slices");
    //Get slices from user and store it in an int
    let mut n_string = String::new();
    io::stdin().read_line(&mut n_string).expect("failed to read from stdin");
    let n = n_string.trim().parse::<u32>().expect("invalid input");
    
    
    println!("Number of Dimensions");
    //Get dimensions from user and store it in an int
    let mut k_string = String::new();
    io::stdin().read_line(&mut k_string).expect("failed to read from stdin");
    let k = k_string.trim().parse::<u32>().expect("invalid input");

    println!("{:?}", hypercake(n, k));
}

fn hypercake(n: u32, k: u32) -> u32 {
    let mut pieces:u32 = 0;
    if n == 0 {
        return 1;
    } else {
        for i in 0..k+1 {
            pieces+= combinations(n, i);
        }
        return pieces;
    }
}
fn combinations(n: u32, r: u32) -> u32 {
    if r <= n {
       let result:u32 = factorial(n) / (factorial(r) * factorial(n - r));
    return result; 
    } else {
        return 0;
    }
    
}
fn factorial(mut n: u32) -> u32 {
    if n <= 1 {
        return 1;
    } else {
        n = n * factorial(n-1);
        return n;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn zero_two() {
        let pieces = hypercake(0, 2);
        assert_eq!(pieces, 1);
    }

    #[test]
    fn zero_three() {
        let pieces = hypercake(0, 3);
        assert_eq!(pieces, 1);
    }
    #[test]
    fn zero_four() {
        let pieces = hypercake(0, 4);
        assert_eq!(pieces, 1);
    }

    #[test]
    fn zero_five() {
        let pieces = hypercake(0, 5);
        assert_eq!(pieces, 1);
    }
    #[test]
    fn one_two() {
        let pieces = hypercake(1, 2);
        assert_eq!(pieces, 2);
    }

    #[test]
    fn one_three() {
        let pieces = hypercake(1, 3);
        assert_eq!(pieces, 2);
    }
    #[test]
    fn one_four() {
        let pieces = hypercake(1, 4);
        assert_eq!(pieces, 2);
    }
    #[test]
    fn one_five() {
        let pieces = hypercake(1, 5);
        assert_eq!(pieces, 2);
    }
    #[test]
    fn two_two() {
        let pieces = hypercake(2, 2);
        assert_eq!(pieces, 4);
    }

    #[test]
    fn two_three() {
        let pieces = hypercake(2, 3);
        assert_eq!(pieces, 4);
    }
    #[test]
    fn two_four() {
        let pieces = hypercake(2, 4);
        assert_eq!(pieces, 4);
    }
    #[test]
    fn two_five() {
        let pieces = hypercake(2, 5);
        assert_eq!(pieces, 4);
    }
    #[test]
    fn three_two() {
        let pieces = hypercake(3, 2);
        assert_eq!(pieces, 7);
    }
    #[test]
    fn three_three() {
        let pieces = hypercake(3, 3);
        assert_eq!(pieces, 8);
    }
    #[test]
    fn three_four() {
        let pieces = hypercake(3, 4);
        assert_eq!(pieces, 8);
    }
    #[test]
    fn three_five() {
        let pieces = hypercake(3, 5);
        assert_eq!(pieces, 8);
    }
    #[test]
    fn four_two() {
        let pieces = hypercake(4, 2);
        assert_eq!(pieces, 11);
    }
    #[test]
    fn four_three() {
        let pieces = hypercake(4, 3);
        assert_eq!(pieces, 15);
    }
    #[test]
    fn four_four() {
        let pieces = hypercake(4, 4);
        assert_eq!(pieces, 16);
    }
    #[test]
    fn four_five() {
        let pieces = hypercake(4, 5);
        assert_eq!(pieces, 16);
    }
    #[test]
    fn five_two() {
        let pieces = hypercake(5, 2);
        assert_eq!(pieces, 16);
    }
    #[test]
    fn five_three() {
        let pieces = hypercake(5, 3);
        assert_eq!(pieces, 26);
    }
    #[test]
    fn five_four() {
        let pieces = hypercake(5, 4);
        assert_eq!(pieces, 31);
    }
    #[test]
    fn five_five() {
        let pieces = hypercake(5, 5);
        assert_eq!(pieces, 32);
    }
    #[test]
    fn six_two() {
        let pieces = hypercake(6, 2);
        assert_eq!(pieces, 22);
    }
    #[test]
    fn six_three(){
        let pieces = hypercake(6, 3);
        assert_eq!(pieces, 42)
    }
    #[test]
    fn six_four(){
        let pieces = hypercake(6, 4);
        assert_eq!(pieces, 57)
    }
    #[test]
    fn six_five(){
        let pieces = hypercake(6, 5);
        assert_eq!(pieces, 63)
    }
    #[test]
    fn seven_two(){
        let pieces = hypercake(7, 2);
        assert_eq!(pieces, 29)
    }
    #[test]
    fn seven_three(){
        let pieces = hypercake(7, 3);
        assert_eq!(pieces, 64)
    }
    #[test]
    fn seven_four(){
        let pieces = hypercake(7, 4);
        assert_eq!(pieces, 99)
    }
    #[test]
    fn seven_five(){
        let pieces = hypercake(7, 5);
        assert_eq!(pieces, 120)
    }
    #[test]
    fn eight_two(){
        let pieces = hypercake(8, 2);
        assert_eq!(pieces, 37)
    }
    #[test]
    fn eight_three(){
        let pieces = hypercake(8, 3);
        assert_eq!(pieces, 93)
    }
    #[test]
    fn eight_four(){
        let pieces = hypercake(8, 4);
        assert_eq!(pieces, 163)
    }
    #[test]
    fn eight_five(){
        let pieces = hypercake(8, 5);
        assert_eq!(pieces, 219)
    }
    #[test]
    fn nine_two(){
        let pieces = hypercake(9, 2);
        assert_eq!(pieces, 46)
    }
    #[test]
    fn nine_three(){
        let pieces = hypercake(9, 3);
        assert_eq!(pieces, 130)
    }
    #[test]
    fn nine_four(){
        let pieces = hypercake(9, 4);
        assert_eq!(pieces, 256)
    }
    #[test]
    fn nine_five(){
        let pieces = hypercake(9, 5);
        assert_eq!(pieces, 382)
    }
    #[test]
    fn ten_two(){
        let pieces = hypercake(10, 2);
        assert_eq!(pieces, 56)
    }
    #[test]
    fn ten_three(){
        let pieces = hypercake(10, 3);
        assert_eq!(pieces, 176)
    }
    #[test]
    fn ten_four(){
        let pieces = hypercake(10, 4);
        assert_eq!(pieces, 386)
    }
    #[test]
    fn ten_five(){
        let pieces = hypercake(10, 5);
        assert_eq!(pieces, 638)
    }
}
