#[allow(dead_code)]
mod assignments;
#[allow(dead_code)]
mod tkl;

use tkl::math::vector2::*;

#[allow(dead_code)]
fn main() {
    // println!();
    // println!("Assignment 1: Bit Craziness");
    // assignments::a01::run();
    // println!("Assignment 2: idk");
    // assignments::a02::run();

    let vec1 = Vector2::from(1.1, 1.1);
    let mut vec2 = Vector2::from(2.2, 2.2);
    let flo1 = vec2 * vec1;

    // let lol = tkl::math::vector2::dot(vec1, vec2);

    println!();
    println!("vec1 {:?} ", vec1);
    println!("vec2 {:?} ", vec2);
    println!("flo1 {:?} ", flo1);
    println!();
    println!("vec1 + vec2 {:?} ", vec1 + vec2);
    println!("vec1 - vec2 {:?} ", vec1 - vec2);
    println!("vec1 * vec2 {:?} ", vec1 * vec2);
    println!("vec1 * flo1 {:?} ", vec1 * flo1);
    println!("vec1 / flo1 {:?} ", vec1 / flo1);
    println!();
    println!("vec2 len() {:?} ", vec2.len());
    println!("vec2 len() {:?} ", vec2.nor());
    vec2 = vec2.nor();
    println!("vec2 len() {:?} ", vec2.len());
    println!("vec2 len() {:?} ", vec2.nor());
    println!();

    let vec4 = Vector2::from(1.0, 0.0);
    let vec5 = Vector2::from(0.0, 1.0);
    let vec6 = Vector2::from(1.0, 1.0);

    println!("vec4 ang vec5 {:?} ", ang(vec4, vec5).to_degrees());
    println!("vec4 ang vec6 {:?} ", ang(vec4, vec6).to_degrees());
    println!("vec5 ang vec6 {:?} ", ang(vec5, vec6).to_degrees());

    println!("vec4 ang_floor {:?} ", angf(vec4).to_degrees());
    println!("vec5 ang_floor {:?} ", angf(vec5).to_degrees());
    println!("vec6 ang_floor {:?} ", angf(vec6).to_degrees());

    let asd = (1.2, 1.3);
    let vec7: Vector2 = asd.into();
    println!("vec7 {:?} ", vec7);
}
