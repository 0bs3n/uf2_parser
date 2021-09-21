pub fn str_encode(data: &Vec<u8>) -> String {
    data.iter().map(|c| *c as char).collect::<String>()
}


pub fn hex_encode<'a>(bs: &[u8]) -> String {
    let mut s = String::with_capacity(bs.len() *2);
    for b in bs {
        let bh = b >> 4;
        let bl = b & 0x0f;
        for bn in [bh, bl].iter() {
            if *bn < 0xa { s.push((bn + 0x30) as char) }
            else { s.push((bn + 0x57) as char) }
        }
    }
    return s;
}


pub fn hex_decode(s: &str) -> Vec<u8> {
    assert!(s.len() % 2 == 0, "hex_decode(): Odd number of hex nibbles!");
    s.chars()
     .collect::<Vec<char>>()
     .chunks(2)
     .map(|c| u8::from_str_radix(&c.iter().collect::<String>(), 0x10).unwrap())
     .collect::<Vec<u8>>()
}


pub fn interlace_vec(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    a.iter().zip(b).flat_map(|(x, y)| {
        std::iter::once(*x).chain(std::iter::once(*y))
    }).collect::<Vec<u8>>()
}
