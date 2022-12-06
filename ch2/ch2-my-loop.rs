
fn main() {
    println!("start!!");
    let mut counter = 0;
    'outer: for x in 0..200 {
        counter += 1;
        // println!("x: {}", x);
        for y in 0..400 {
            counter += 1;
            // println!("x: {}, y: {}", x, y);
            for z in 0..500 {
                counter += 1;
                // println!("x: {}, y: {}, z {}", x, y, z);
                if x + y + z > 1000 {
                    println!("count: {}", counter);
                    break 'outer;
                }
            }
        }
    }
}
