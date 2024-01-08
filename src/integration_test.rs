use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, PartialEq, Eq, Deserialize)]
pub struct WrappedU8(pub u8);

#[derive(Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Struct1 {
    pub u8v: Vec<WrappedU8>,
    pub u16v: Vec<u16>,
    pub u32v: Vec<u32>,
    pub u64v: Vec<u64>,
    pub i8v: Vec<i8>,
    pub i16v: Vec<i16>,
    pub i32v: Vec<i32>,
    pub i64v: Vec<i64>,
    pub u8s: u8,
    pub bs: bool,
    pub some_s: Option<u16>,
    pub none_s: Option<u32>,
    pub strings: Vec<u8>,
    pub stringv: Vec<Vec<u8>>,
}

#[test]
fn test_struct_1() {
    let mut test_s = Struct1::default();
    test_s.u8v = vec![WrappedU8(1u8), WrappedU8(231u8), WrappedU8(123u8)];
    test_s.u16v = vec![124u16, 41374u16];
    test_s.u32v = vec![14710471u32, 3590275702u32, 1u32, 2u32];
    test_s.u64v = vec![352905235952532u64, 2147102974910410u64];
    test_s.i8v = vec![-1i8, 120i8, -22i8];
    test_s.i16v = vec![-7932i16];
    test_s.i32v = vec![-4327i32, 35207277i32];
    test_s.i64v = vec![-1i64, 1i64];
    test_s.u8s = 3u8;
    test_s.bs = true;
    test_s.some_s = Some(5u16);
    test_s.none_s = None;
    test_s.strings = b"Here is a string.".to_vec();
    test_s.stringv = vec![b"string a".to_vec(), b"34720471290497230".to_vec()];

    let mut res = Vec::<u32>::new();
    let mut serializer = crate::Serializer::new(&mut res);
    let _ = test_s.serialize(&mut serializer);

    let answer = vec![
        3u32, 8120065, 2, 124, 41374, 4, 14710471, 3590275702, 1, 2, 2, 658142100, 82167,
        1578999754, 499911, 3, 4294967295, 120, 4294967274, 1, 4294959364, 2, 4294962969, 35207277,
        2, 4294967295, 4294967295, 1, 0, 259, 1, 5, 0, 17, 1701995848, 544434464, 1953701985,
        1735289202, 46, 2, 8, 1769108595, 1629513582, 17, 842478643, 825701424, 875575602,
        858928953, 48,
    ];
    assert_eq!(answer, res);

    let recovered: Struct1 = crate::deserializer::from_slice(&res).unwrap();
    assert_eq!(test_s, recovered);
}

#[test]
fn test_struct_2() {
    type TestType = (Vec<u8>, u32, u8, u8, Vec<u8>, u8);
    let serialize_value: TestType = (vec![0u8, 1], 8u32, 7u8, 222u8, vec![1, 1, 1, 1, 1], 5u8);
    let mut res = Vec::<u32>::new();
    let mut serializer = crate::Serializer::new(&mut res);
    serialize_value.serialize(&mut serializer).unwrap();

    assert_eq!([2, 256, 8, 56839, 5, 16843009, 1281].as_slice(), res);

    let recovered: TestType = crate::deserializer::from_slice(&res).unwrap();
    assert_eq!(serialize_value, recovered);
}
