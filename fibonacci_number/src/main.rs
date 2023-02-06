fn main() {
    const EARLY_INDEX: i32 = 1;
    const FIB_NUM: i32 = 25;
    let answer1 = fibonacci(FIB_NUM);
    let answer2 = fib(FIB_NUM-EARLY_INDEX);
    println!("Answer1 = {:?}", answer1);
    println!("Answer2 = {:?}", answer2);
}

fn fibonacci(n: i32) -> Vec<i32>{
    let mut sequence: Vec<i32> = Vec::new();
    for i in 0..n{
        if i == 0 {
            sequence.push(0);
        } 
        else if i == 1 {
            sequence.push(1);
        }
        else {
            sequence.push(sequence[(i-1) as usize]+sequence[(i-2) as usize])
        }
    }
    sequence
}

fn fib(n: i32) -> Vec<i32>{
    let mut sequence: Vec<i32> = Vec::new();
    if n == 0 {
        sequence.push(0);
        return sequence
    }
    else if n == 1 {
        sequence.push(0);
        sequence.push(1);
        return sequence 
    }

    sequence = fib(n-1);
    if sequence.len() >= 2{
        sequence.push(sequence[(n-1) as usize]+sequence[(n-2) as usize]);
    }
    sequence
}