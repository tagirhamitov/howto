mod openai;

fn get_query() -> String {
    let args: Vec<String> = std::env::args().skip(1).collect();
    args.join(" ")
}

fn main() {
    let query = get_query();
    let result = openai::make_request(&query);
    println!("{result}")
}
