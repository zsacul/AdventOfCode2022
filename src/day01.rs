pub fn part1(data:&[String])->i32
{
    let mut cnt=0;
    let mut max=0;

    for i in 0..data.len()
    {
        if data[i].is_empty()
        {
            cnt=0;
        }
          else
        {
            let v = data[i].parse::<i32>().unwrap();
            cnt+=v;
        }
        
        if cnt>max 
        {
            max = cnt;
        }
    }

    max
}

pub fn part2(data:&[String])->i32
{
    let mut cnt=0;
    let mut tab = vec![];

    for i in 0..data.len()
    {
        if data[i].is_empty()
        {
            tab.push(cnt);
            cnt = 0;
        }
          else
        {
            cnt+=data[i].parse::<i32>().unwrap();
        }
    }
    tab.push(cnt);

    tab.sort();
    tab.iter().rev().take(3).sum()
    
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day1");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = 
    vec![
        "1000".to_string(),
        "2000".to_string(),
        "3000".to_string(),
        "".to_string(),
        "4000".to_string(),
        "".to_string(),
        "5000".to_string(),
        "6000".to_string(),
        "".to_string(),
        "7000".to_string(),
        "8000".to_string(),
        "9000".to_string(),
        "".to_string(),
        "10000".to_string(),
        ];
    assert_eq!(part1(&v),24000);
}

#[test]
fn test2()
{
    let v = vec![
        "1000".to_string(),
        "2000".to_string(),
        "3000".to_string(),
        "".to_string(),
        "4000".to_string(),
        "".to_string(),
        "5000".to_string(),
        "6000".to_string(),
        "".to_string(),
        "7000".to_string(),
        "8000".to_string(),
        "9000".to_string(),
        "".to_string(),
        "10000".to_string(),        
    ];
    assert_eq!(part2(&v),45000);
}