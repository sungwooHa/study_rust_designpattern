mod deser;
mod element;

use crate::deser::deserializer::Deserializer;
use crate::deser::string_deserializer::StringDeserializer;
use crate::deser::vec_deserializer::VecDeserializer;
use crate::element::{two_values_array, two_values_struct};

pub fn active() {
    //String Deserializer 생성 ()
    //TwoValuesStruct
    {
        let deserializer =
            StringDeserializer::create(two_values_struct::TwoValuesStruct::default());
        let result = deserializer.parse_str("123 456");
        println!("{:?}", result);
    }

    //Vector Deserializer 생성
    //TwoValuesStruct 대상
    {
        let deserializer = VecDeserializer::create(two_values_struct::TwoValuesStruct::default());
        let result = deserializer.parse_vec(vec![123, 456]);
        println!("{:?}", result);
    }

    //Vector Deserializer 생성
    //TwoValuesArray 대상
    {
        let deserializer = VecDeserializer::create(two_values_array::TwoValuesArray::default());
        let result = deserializer.parse_vec(vec![123, 456]);
        println!("{:?}", result);

        //array 용 deserializer에 struct를 넣어 보기
        println!(
            "Error: {}",
            deserializer.parse_str("123 456").err().unwrap()
        )
    }
}
