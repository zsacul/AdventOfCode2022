fn distinct(s:&[u8])->bool
{ 
    s.iter()
     .try_fold(0usize, |acc, &l| {  let code = 1<<(l - b'a');
                                    if acc&code > 0 { None           } 
                                               else { Some(acc|code) }   
                                 }
              )
     .unwrap_or(0) > 0    
}

#[allow(unused)]
fn distinct_old(s:&[u8])->bool
{ 
    let mut vec = vec![false;255];
    let mut acc = 0u32;
    for c in s 
    { 
        vec[*c as usize - 'a' as usize] = true; 
    }
    
    vec.iter()
       .filter(|&&p| p)
       .count()==s.len()
}

fn first_distinct(data:&str,n:usize)->Option<usize>
{
    data.as_bytes()
        .windows(n)
        .enumerate()
        .find_map(|(id,st)| if distinct(st) { Some(id+n) } else { None } )
}

pub fn part1(data:&str)->usize
{
    first_distinct(data,4).unwrap_or(0)
}

pub fn part2(data:&str)->usize
{
    first_distinct(data,14).unwrap_or(0)
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