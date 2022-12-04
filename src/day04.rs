struct Range{
    a : i32,
    b : i32,
    c : i32,
    d : i32,
}

impl Range {
    fn new(s:&str)->Self
    {
        let tab   : Vec<&str> =      s.split(',').collect(); 
        let left  : Vec<&str> = tab[0].split('-').collect(); 
        let right : Vec<&str> = tab[1].split('-').collect(); 

        Self 
        {
            a :  left[0].parse::<i32>().unwrap(),
            b :  left[1].parse::<i32>().unwrap(),
            c : right[0].parse::<i32>().unwrap(),
            d : right[1].parse::<i32>().unwrap(),
        }
    }

    fn contains(&self)->bool
    {
        self.a>=self.c && self.b<=self.d || 
        self.c>=self.a && self.d<=self.b
    }

    fn inside(x:i32,a:i32,b:i32)->bool
    {
        x>=a && x<=b
    }

    fn overlap(&self)->bool
    {
        Range::inside(self.a,self.c,self.d) ||
        Range::inside(self.b,self.c,self.d) ||
        Range::inside(self.c,self.a,self.b) ||
        Range::inside(self.d,self.a,self.b) 
    }
}

pub fn part1(data:&[String])->i32
{
    data.iter().map(|s| Range::new(s) )
               .filter(|r| r.contains())
               .count() as i32
}

pub fn part2(data:&[String])->i32
{
    data.iter().map(|s| Range::new(s) )
               .filter(|r| r.overlap())
               .count() as i32
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day4");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "2-4,6-8".to_string(),
        "2-3,4-5".to_string(),
        "5-7,7-9".to_string(),
        "2-8,3-7".to_string(),
        "6-6,4-6".to_string(),
        "2-6,4-8".to_string(),
    ];
    assert_eq!(part1(&v),2);
}

#[test]
fn test2()
{
    let v = vec![
        "2-4,6-8".to_string(),
        "2-3,4-5".to_string(),
        "5-7,7-9".to_string(),
        "2-8,3-7".to_string(),
        "6-6,4-6".to_string(),
        "2-6,4-8".to_string(),
    ];
    assert_eq!(part2(&v),4);
}
