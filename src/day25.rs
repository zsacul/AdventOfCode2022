fn part1(data:&[String])->String
{
    to_snail(data.iter()
                 .map(|line|to_decimal(line.to_string()) )
                 .sum())
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day 25");
    println!("part1: {}",part1(data));
}

fn to_snail(s:i128)->String
{
    let digits = vec!['=','-','0','1','2'];
    let mut res = vec![];

    let mut off = 2;
    let m = (s+off)%5;
    res.push(digits[m as usize].to_string());
    let mut di = 5;

    for _ in 0..25
    {
        let m = (s+off)/di;
        let m = (m+2)%5;
        res.push(digits[m as usize].to_string());

        off+=2*di;   
        di*=5;
    }

    while res.last().unwrap()=="0"
    {    
        res.pop();
    }

    res.reverse();
    res.join("")
}

fn to_decimal(s:String)->i128
{
    let mut res = 0i128;
    let mut pow = 1;

    for c in s.chars().rev() 
    {
        let ss = ("=-012".find(c).unwrap() as i128) - 2;
        res+=pow*ss;
        pow*=5;
    }

    res
}

 #[test]
fn test1()
{
    let v = vec![
        "1=-0-2".to_string(),
        "12111".to_string(),
        "2=0=".to_string(),
        "21".to_string(),
        "2=01".to_string(),
        "111".to_string(),
        "20012".to_string(),
        "112".to_string(),
        "1=-1=".to_string(),
        "1-12".to_string(),
        "12".to_string(),
        "1=".to_string(),
        "122".to_string(),
    ];
    assert_eq!(part1(&v),"2=-1=0".to_string());
}

#[test]
fn test4()
{
    assert_eq!(to_decimal("2=-01".to_string()),976);
}

#[test]
fn test5()
{
    assert_eq!((1),          to_decimal("1".to_string()));
    assert_eq!((2),          to_decimal("2".to_string()));
    assert_eq!((3),          to_decimal("1=".to_string()));
    assert_eq!((4),          to_decimal("1-".to_string()));
    assert_eq!((5),          to_decimal("10".to_string()));
    assert_eq!((6),          to_decimal("11".to_string()));
    assert_eq!((7),          to_decimal("12".to_string()));
    assert_eq!((8),          to_decimal("2=".to_string()));
    assert_eq!((9),          to_decimal("2-".to_string()));
    assert_eq!((10),         to_decimal("20".to_string()));
    assert_eq!((15),         to_decimal("1=0".to_string()));
    assert_eq!((20),         to_decimal("1-0".to_string()));
    assert_eq!((2022),       to_decimal("1=11-2".to_string()));
    assert_eq!((12345),      to_decimal("1-0---0".to_string()));
    assert_eq!((314159265),  to_decimal("1121-1110-1=0".to_string()));    
 }

 #[test]
fn test10()
{
    assert_eq!(to_snail(1),"1".to_string());
}

#[test]
fn test11()
{
    assert_eq!(to_snail(2),"2".to_string());
}

#[test]
fn test12()
{
   assert_eq!(to_snail(3),"1=".to_string());
}

#[test]
fn test13()
{
    assert_eq!(to_snail(4),"1-".to_string());
}

#[test]
fn test14()
{
   assert_eq!(to_snail(5),"10".to_string());
}

#[test]
fn test15()
{
   assert_eq!(to_snail(6),"11".to_string());
}

#[test]
fn test16()
{
   assert_eq!(to_snail(7),"12".to_string());
}

#[test]
fn test17()
{
   assert_eq!(to_snail(8),"2=".to_string());
}

#[test]
fn test18()
{
   assert_eq!(to_snail(9),"2-".to_string());
}

#[test]
fn test19()
{
   assert_eq!(to_snail(10),"20".to_string());
}

#[test]
fn test20()
{
   assert_eq!(to_snail(15),"1=0".to_string());
}

#[test]
fn test21()
{
   assert_eq!(to_snail(20),"1-0".to_string());
}

#[test]
fn test22()
{
   assert_eq!(to_snail(2022),"1=11-2".to_string());
}


#[test]
fn test23()
{
  assert_eq!(to_snail(12345),"1-0---0".to_string());
}

#[test]
fn test24()
{
    assert_eq!(to_snail(314159265),"1121-1110-1=0".to_string());
}
