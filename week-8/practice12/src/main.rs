fn main() {

    let mut colors = ["red","green","yellow","white"];

    println!("\noriginal array = {:?}", colors);

    let sliced_colors = &mut colors[1..3];

    print!("first slice = {:?}", sliced_colors);

    sliced_colors[1] = "purple";
    println!("changed slice = {:?}", sliced_colors);


}