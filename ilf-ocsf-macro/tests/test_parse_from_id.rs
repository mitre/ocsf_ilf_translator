use ilf_ocsf_macro::ParseFromUid;

#[derive(PartialEq, Eq, Debug)]
struct A;
#[derive(PartialEq, Eq, Debug)]
struct B;
trait Parsable {
    const NAME: &'static str;
    fn new() -> Self;
}
impl Parsable for A {
    const NAME: &'static str = "A";
    fn new() -> Self {
        A
    }
}
impl Parsable for B {
    const NAME: &'static str = "B";
    fn new() -> Self {
        B
    }
}

fn my_parser<T: Parsable>(input: &str) -> Result<T, Box<dyn std::error::Error>> {
    if input == T::NAME {
        Ok(T::new())
    } else {
        Err("failed to parse".into())
    }
}

#[test]
fn test_macro() {
    #[derive(PartialEq, Eq, Debug, ParseFromUid)]
    #[Parser(self::my_parser)]
    enum MyEnum {
        #[Uid(1)]
        TypeA(A),
        #[Uid(2)]
        TypeB(B),
    }

    assert_eq!(
        MyEnum::parse_from_id(1, "A").expect("parses properly"),
        MyEnum::TypeA(A)
    );

    let Err(e) = MyEnum::parse_from_id(1, "B") else {
        panic!("B shouldn't parse")
    };
    println!("{}", e);
}
