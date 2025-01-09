/**
 * Copyright 2025 The MITRE Corporation

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
 */



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
