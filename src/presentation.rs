use crate::datatypes::custum_struct::CustomStruct;

pub fn say_hello(){
    println!("Hi Nerds!")
}

pub fn usage_of_types() -> CustomStruct {
    let _custom_structure = CustomStruct{ title:"Ceva", age:2 };
    _custom_structure
}