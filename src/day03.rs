
fn get_code(i:usize)->i32
{
    let cc = i as u8 as char;
    
    if cc.is_uppercase()
    {
        i as i32 - 'A' as i32 + 27
    }
      else
    {
        i as i32 - 'a' as i32 + 1
    }    
}

fn compute2(s1:&String,s2:&String,s3:&String)->i32
{   
    let mut v1 = vec![0u8;255];
    let mut v2 = vec![0u8;255];
    let mut v3 = vec![0u8;255];

    for c in s1.as_bytes() { v1[*c as usize] = 1; }
    for c in s2.as_bytes() { v2[*c as usize] = 1; }
    for c in s3.as_bytes() { v3[*c as usize] = 1; }

    for i in 0..255 
    {
        if v1[i]>0 && v2[i]>0 && v3[i]>0
        {
            return get_code(i);
        }
    }

    return 0;
}


fn compute1(l:&String)->i32
{   
    let mut left =  &l[0..l.len()/2];
    let mut right = &l[l.len()/2..];
    println!("{} {}",left,right);
    let mut lv = vec![0u8;255];
    let mut rv = vec![0u8;255];
    for c in  left.as_bytes() { lv[*c as usize] = 1; }
    for c in right.as_bytes() { rv[*c as usize] = 1; }

    for i in 0..255 
    {
        if lv[i]>0 && rv[i]>0
        {
            return get_code(i);
        }
    }

    return 0;

}

pub fn part1(data:&[String])->i32
{
    data.iter().map( |l| { compute1(l) }).sum()
}

pub fn part2(data:&[String])->i32
{
    let mut res=0;
    for i in (0..data.len()).step_by(3)
    {
        res+=compute2(&data[i],&data[i+1],&data[i+2]);
    }
    res
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