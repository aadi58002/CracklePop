fn main() {
    for i in 1..=100{
        let mut print_number = true;
        if i%3==0{
            print!("Crackle");
            print_number = false;
        }
        if i%5==0{
            print!("Pop");
            print_number = false;
        }
        if print_number{
            print!("{}",i);
        }
        println!();
    }
}
