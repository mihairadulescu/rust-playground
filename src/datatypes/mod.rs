use crate::datatypes::custum_struct::CustomStruct;
pub mod custum_struct;

pub fn show_types(){
    let _number1:u8 = 2;
    let _number2:u16 = 2;
    let _number3:u32 = 2;
    let _number4:u64 = 2;
    let _number5:u128 = 2;
    let _number6:i128 = 2;

    let _number7:f32 = 2.2;
    let _number8:f64 = 2.2;

    let _char:char = '3';

    let _string:&str = "Some string";

    let _custom_struct = CustomStruct { age: 1, title:"Mihai"};
    let _custom_struct2 = CustomStruct::new(2,"mihai");

    println!("{}",_number7);
    println!("{:?}", _custom_struct);
}