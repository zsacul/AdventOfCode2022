fn ok(s:&str)->bool
{
    s.chars().nth(0).unwrap()!=s.chars().nth(1).unwrap() &&
    s.chars().nth(0).unwrap()!=s.chars().nth(2).unwrap() &&
    s.chars().nth(0).unwrap()!=s.chars().nth(3).unwrap() &&
    s.chars().nth(1).unwrap()!=s.chars().nth(2).unwrap() &&
    s.chars().nth(1).unwrap()!=s.chars().nth(3).unwrap() &&
    s.chars().nth(2).unwrap()!=s.chars().nth(3).unwrap()   
}

fn ok2(s:&str)->bool
{ 
    let mut vec = vec![false;255];
    for c in s.as_bytes() { vec[*c as usize] = true; }
    
    vec.into_iter()
       .filter(|&p| p==true)
       .count()==14
}

pub fn part1(data:&str)->usize
{
    for i in 0..data.len()-4 {
        if ok(&data[i..i+4]) {
            return i+4;
        }        
    }
    0
}

pub fn part2(data:&str)->usize
{
    for i in 0..data.len()-14 {
        if ok2(&data[i..i+14]) {
            return i+14;
        }        
    }
    0
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
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11 );
}

#[test]
fn test2()
{
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
}

