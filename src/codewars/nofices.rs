// fn dont_give_me_five(start: isize, end: isize) -> isize {
//     let mut count = 0;
//     for i in start..=end {
//         if i.to_string().contains("5") {
//             continue
//         }
//         count += 1;
//     }
//     count
// }

fn dont_give_me_five(start: isize, end: isize) -> isize {
    let mut count = 0;
    for i in start..=end {
        if i % 10 == 5 {
            continue
        }
        count += 1;
    }
    count
}

fn main() {
    println!("{}", dont_give_me_five(4, 17));
}