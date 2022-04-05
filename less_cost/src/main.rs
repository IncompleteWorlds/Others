use rand::Rng;

/**
 * Return the cost of removing letters
 */
fn solution(in_string: &String, in_cost: &Vec<i32>) -> i32
{
    if in_string.is_empty() == true {
        return 0;
    }
    
    for c in in_string.chars() {
        print!("Char: {} ", c);
    }
    println!("");
    for v in in_cost {
        print!("{} ", v);
    }
    println!("");
    
    let mut minimum_cost: i32 = 0;
    let mut char_iter = in_string.chars();
    let mut c1 = char_iter.next();
    let mut c2;
    let mut i = 0;

    while c1 != None {
        c2 = char_iter.next();

        if c2 == None {
            break;
        }

        if c1 == c2 {
            if in_cost[i] < in_cost[i+1] {
                minimum_cost += in_cost[i];
            } else {
                minimum_cost += in_cost[i+1];
            }
        }

        i +=1;
        c1 = c2;
    }

    minimum_cost
}


fn buzz_fizz(in_string: &String) 
{
    if in_string.is_empty() {
        return;
    }

    let numbers : Vec<i32> = in_string.split_whitespace()
                .map(|v| v.parse::<i32>().unwrap_or(0) )
                .collect();

    for n in numbers {
        if n % 15 == 0 {
            println!("FizzBuzz");
        } else if n % 5 == 0 {
            println!("Fizz");
        } else if n % 3 == 0 {
            println!("Buzz");
        } else {
            println!("{}", n);
        }
    }
}


fn main() {

    let tmp_string = String::from("aabcdeeeffgh");
    let mut tmp_cost = Vec::new();

    for _ in 0..tmp_string.len() {
        tmp_cost.push( rand::thread_rng().gen_range(1..21) );
    }

    println!("First solution: {} ", solution(&tmp_string, &tmp_cost) );

    //tmp_string = "Other string".to_owned();

    // buzz_fizz(&"".to_owned());
    // println!("");

    // buzz_fizz(&"2 4 6 8".to_owned());
    // println!("");

    // buzz_fizz(&"0 0 0".to_owned());
    // println!("");

    // buzz_fizz(&"1 3 5 15 4 25 6 8 30".to_owned());
}
