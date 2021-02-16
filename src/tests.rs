
#[derive(Clone)]
struct Example {
    pub value: i8
}

struct OtherExample {
    pub example: Example
}

fn mutate(arg: &Example) -> OtherExample {
    OtherExample {
        example: arg.clone()
    }
}

fn main() {
    mutate(
        &Example {
            value: 1
        }
    );
    println!("all good");
}
