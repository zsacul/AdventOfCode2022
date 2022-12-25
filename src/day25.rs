fn part1(data:&[String])->String
{
    tosna(data.iter()
              .map(|line|todec(line.to_string()) )
              .sum())
}

//2=020-===0-1===2=020

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day 25");
    println!("part1: {}",part1(data));
}

fn tosna(s:i128)->String
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

    let mut ll = res.len()-1;

    while res[ll]==*"0"
    {
        ll-=1;
        res.pop();
    }

    res.reverse();
    res.join("")
}

fn todec(s:String)->i128
{
    let digts = "=-012";
    let mut res = 0i128;
    let mut pow = 1;

    for c in s.chars().rev() 
    {
        let ss = (digts.find(c).unwrap() as i128) - 2;
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
    assert_eq!(todec("2=-01".to_string()),976);
}

#[test]
fn test5()
{
    assert_eq!((1),          todec("1".to_string()));
    assert_eq!((2),          todec("2".to_string()));
    assert_eq!((3),          todec("1=".to_string()));
    assert_eq!((4),          todec("1-".to_string()));
    assert_eq!((5),          todec("10".to_string()));
    assert_eq!((6),          todec("11".to_string()));
    assert_eq!((7),          todec("12".to_string()));
    assert_eq!((8),          todec("2=".to_string()));
    assert_eq!((9),          todec("2-".to_string()));
    assert_eq!((10),         todec("20".to_string()));
    assert_eq!((15),         todec("1=0".to_string()));
    assert_eq!((20),         todec("1-0".to_string()));
    assert_eq!((2022),       todec("1=11-2".to_string()));
    assert_eq!((12345),      todec("1-0---0".to_string()));
    assert_eq!((314159265),  todec("1121-1110-1=0".to_string()));    
 }

 #[test]
fn test10()
{
    assert_eq!(tosna(1),"1".to_string());
}

#[test]
fn test11()
{
    assert_eq!(tosna(2),"2".to_string());
}

#[test]
fn test12()
{
   assert_eq!(tosna(3),"1=".to_string());
}

#[test]
fn test13()
{
    assert_eq!(tosna(4),"1-".to_string());
}

#[test]
fn test14()
{
   assert_eq!(tosna(5),"10".to_string());
}

#[test]
fn test15()
{
   assert_eq!(tosna(6),"11".to_string());
}

#[test]
fn test16()
{
   assert_eq!(tosna(7),"12".to_string());
}

#[test]
fn test17()
{
   assert_eq!(tosna(8),"2=".to_string());
}

#[test]
fn test18()
{
   assert_eq!(tosna(9),"2-".to_string());
}

#[test]
fn test19()
{
   assert_eq!(tosna(10),"20".to_string());
}

#[test]
fn test20()
{
   assert_eq!(tosna(15),"1=0".to_string());
}

#[test]
fn test21()
{
   assert_eq!(tosna(20),"1-0".to_string());
}

#[test]
fn test22()
{
   assert_eq!(tosna(2022),"1=11-2".to_string());
}


#[test]
fn test23()
{
  assert_eq!(tosna(12345),"1-0---0".to_string());
}

#[test]
fn test24()
{
    assert_eq!(tosna(314159265),"1121-1110-1=0".to_string());
}
