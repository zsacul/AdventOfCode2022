use std::collections::HashMap;
use super::vec2::Vec2;

#[derive(Eq, PartialEq,  Debug, Clone)]
struct World
{
    field : HashMap<Vec2,char>,
}

impl World {
    fn new()->Self
    {
        Self
        {
            field : HashMap::new(),
        }
    }

    fn load(&mut self,data:&[String])
    {
        for line in data 
        {
            let tab : Vec<&str> = line.split(" -> ").collect(); 
            let mut last = Vec2::zero();

            let mut first = true;
            for ps in tab
            {
                let pt : Vec<&str> = ps.split(',').collect(); 
                let x = pt[0].parse::<i64>().unwrap();
                let y = pt[1].parse::<i64>().unwrap();
                let p = Vec2::new(x,y);       
                
                if first
                {
                    last = Vec2::new(x,y);
                    first = false;
                }
                  else
                {
                    self.draw(last,p);
                    last = p;
                }
            }
        }
    }

    fn draw(&mut self,p1:Vec2,p2:Vec2)
    {
        if p1.x==p2.x
        {
            for y in p1.y.min(p2.y)..=p1.y.max(p2.y)
            {
                self.field.insert(Vec2::new(p1.x,y),'#');
            }
        }
          else
        {
            for x in p1.x.min(p2.x)..=p1.x.max(p2.x)
            {
                self.field.insert(Vec2::new(x,p1.y),'#');
            }
        }
    }

    #[allow(unused)]
    fn print(&self,x0:usize,x1:usize,y0:usize,y1:usize)
    {
        for y in y0..=y1 {
        for x in x0..=x1 { print!("{}",self.val(x as i64,y as i64)); }
                           println!();
        }
    }

    fn plum(&mut self,start:Vec2,lim:i64)->bool
    {
        let mut      p = Vec2::newv(&start);
        let mut count  = 0;
        let mut moving = true;

        while moving 
        {
            count+=1;
                 if self.val(p.x  ,p.y+1)=='.' {         p.y+=1; }
            else if self.val(p.x-1,p.y+1)=='.' { p.x-=1; p.y+=1; }
            else if self.val(p.x+1,p.y+1)=='.' { p.x+=1; p.y+=1; }
            else                               { moving = false; }            
            
            if lim!=i64::MIN
            {
                if p.y==lim { break; }
            }
            else if count>1000 
            { 
                return false; 
            }
        }
        
        self.field.insert(p,'o');
        !(lim!=-99999 && p==start)      
    }

    fn count(&self,c:char)->usize
    {
        self.field.values().filter(|&x| x==&c).count()
    }

    fn val(&self,x:i64,y:i64)->char
    {
        *self.field.get(&Vec2::new(x as i64,y as i64)).unwrap_or(&'.')
    }

    fn max_y(&self)->i64
    {
        self.field.keys().map(|p| p.y).max().unwrap()
    }
}

pub fn part1(data:&[String])->usize
{
    let mut w  = World::new();
    w.load(data);

    while w.plum(Vec2::new(500, 0),i64::MIN) {}

    w.count('o')
}

pub fn part2(data:&[String])->usize
{
    let mut w  = World::new();
    w.load(data);
    let limit = w.max_y()+1;

    while w.plum(Vec2::new(500, 0),limit) {}

    w.count('o')
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day 14");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = 
    vec![
        "498,4 -> 498,6 -> 496,6".to_string(),
        "503,4 -> 502,4 -> 502,9 -> 494,9".to_string(),
        ];
    assert_eq!(part1(&v),24);
}

#[test]
fn test2()
{
    let v = 
    vec![
        "498,4 -> 498,6 -> 496,6".to_string(),
        "503,4 -> 502,4 -> 502,9 -> 494,9".to_string(),
        ];
    assert_eq!(part2(&v),93);
}