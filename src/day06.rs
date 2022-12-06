fn distinct(s:&str)->bool
{ 
    let mut vec = vec![false;255];
    for c in s.as_bytes() { vec[*c as usize] = true; }    
    
    vec.iter()
       .filter(|&&p| p)
       .count()==s.len()
}

fn first_distinct(data:&str,n:usize)->usize
{
    for i in 0..data.len()-n 
    {
        if distinct(&data[i..i+n]) { return i+n; }        
    }
    0
}

pub fn part1(data:&str)->usize
{
    first_distinct(data,4)
}

pub fn part2(data:&str)->usize
{
    first_distinct(data,14)
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day6");
    println!("part1:{}",part1(&data[0]));
    println!("part2:{}",part2(&data[0]));
}

#[test]
fn test1()
{
    assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz")     ,  5);
    assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg")     ,  6);
    assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw") , 11);
}

#[test]
fn test2()
{
    assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb")   , 19);
    assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz")     , 23);
    assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg")     , 23);
    assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw") , 26);
}