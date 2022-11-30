pub fn part1(data:&[i32])->i32
{
    data.iter()
        .fold((0,i32::MAX), |(acc,prev), &x| 
           (acc + i32::from(x>prev), x)
         )
        .0
}

pub fn part2(data:&[i32])->i32
{
    let mut cnt=0;

    for i in 0..data.len()-3
    {
        if data[i+1..i+3+1].iter().sum::<i32>()>
           data[i  ..i+3  ].iter().sum::<i32>() { cnt+=1 }
    }

    cnt
}

#[allow(unused)]
pub fn solve(data:&[i32])
{    
    println!("Day1");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![199,    200,    208,    210,    200,    207,    240,    269,    260,    263];
    assert_eq!(part1(&v),7);
}

#[test]
fn test2()
{
    let v = vec![607,    618,    618,    617,    647,    716,    769,    792];
    assert_eq!(part2(&v),5);
}