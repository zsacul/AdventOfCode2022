use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[allow(unused)]
pub fn read_1d_i32(path:&str)->Vec<i32>
{
    let mut res:Vec<i32> = vec![];
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() 
        {
            res.push(line.parse::<i32>().unwrap());
        }
    }
    res
}

#[allow(unused)]
pub fn read_1d_i64(path:&str)->Vec<i64>
{
    let mut res:Vec<i64> = vec![];
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() 
        {
            res.push(line.parse::<i64>().unwrap());
        }
    }
    res
}

#[allow(unused)]
pub fn read_1d_string(path:&str)->Vec<String>
{
    let mut res:Vec<String> = vec![];
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() 
        {
            res.push(line);
        }
    }
    res
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[allow(unused)]
pub fn get_between(str:&str,from:&str,to:&str)->String
{
    if from.is_empty()
    {
        let e =          str.find(to).unwrap();
        return str[..e].to_string();
    }

    if to.is_empty()
    {
        let s =          str.find(from).unwrap();
        return str[s+from.len()..].to_string();
    }

        let s =          str.find(from).unwrap() + from.len();
        let e = s + str[s..].find(to  ).unwrap();
        
    str[s..e].to_string()
}

#[allow(unused)]
pub fn i32_get_between(str:&str,from:&str,to:&str)->i32
{
    get_between(str, from, to).parse::<i32>().unwrap()
}

#[allow(unused)]
pub fn f32_get_between(str:&str,from:&str,to:&str)->f32
{
    get_between(str, from, to).parse::<f32>().unwrap()
}
