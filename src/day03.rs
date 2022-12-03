fn get_code(i:usize)->i32
{
    let c = i as u8 as char;
    
    if c.is_uppercase() { i as i32 - 'A' as i32 + 27 }
                   else { i as i32 - 'a' as i32 + 1  }    
}

fn get_vec(s1:&str)->Vec<bool> 
{
    let mut vec = vec![false;255];
    for c in s1.as_bytes() { vec[*c as usize] = true; }
    vec
}

fn compute1(s:&str)->i32
{   
    let left =  &s[         ..s.len()/2];
    let right = &s[s.len()/2..];
    
    let v1 = get_vec(left);
    let v2 = get_vec(right);

    for i in 0..255
    {
        if v1[i] && v2[i] { return get_code(i); }
    }

    0
}
fn compute2(s1:&str,s2:&str,s3:&str)->i32
{   
    let v1 = get_vec(s1);
    let v2 = get_vec(s2);
    let v3 = get_vec(s3);

    for i in 0..255
    {
        if v1[i] && v2[i] && v3[i] { return get_code(i); }
    }

    0
}

pub fn part1(data:&[String])->i32
{
    data.iter()
        .map( |l| compute1(l) )
        .sum()
}

pub fn part2(data:&[String])->i32
{
    (0..data.len()).step_by(3)
                   .map( |i| compute2(&data[i],&data[i+1],&data[i+2]) )
                   .sum()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day3");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = 
    vec![
        "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
        "PmmdzqPrVvPwwTWBwg".to_string(),
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
        "ttgJtRGJQctTZtZT".to_string(),
        "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
        ];
    assert_eq!(part1(&v),157);
}

#[test]
fn test2()
{
    let v = vec![
        "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
        "PmmdzqPrVvPwwTWBwg".to_string(),
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
        "ttgJtRGJQctTZtZT".to_string(),
        "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
    ];
    assert_eq!(part2(&v),70);
}