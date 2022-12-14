use std::collections::HashMap;
use super::vec2::Vec2;

#[derive(Eq, PartialEq,  Debug, Clone)]
struct World
{
    field : HashMap<Vec2,char>,
    size  : Vec2,
    start : Vec2,
    end   : Vec2,    
}

impl World {
    fn new()->Self
    {
        Self
        {
            field : HashMap::new(),
            size  : Vec2::zero(),
            start : Vec2::zero(),
            end   : Vec2::zero(),
        }
    }

    fn load(&mut self,data:&[String])
    {
        for line in data 
        {
            let tab   : Vec<&str> = line.split(" -> ").collect(); 
            let mut last = Vec2::zero();
            let mut p = Vec2::zero();

            let mut first = true;
            for ps in tab
            {
                let pp : Vec<&str> = ps.split(',').collect(); 
                let x = pp[0].parse::<i64>().unwrap();
                let y = pp[1].parse::<i64>().unwrap();
                p = Vec2::new(x,y);       
                
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
                let p = Vec2::new(x as i64,p1.y as i64);
                self.field.insert(p,'#');
            }
        }
        //println!("{:?}",p1);
        //println!("{:?}",p2);
    }

    fn calc_sizes(&mut self)
    {
        self.start = Vec2::new(i64::MAX,i64::MAX);
        self.end   = Vec2::new(0,0);

        let c = self.field.keys().map(
            |&p|
            {
                self.start.x = self.start.x.min(p.x);
                self.start.y = self.start.y.min(p.y);
                self.end.x   = self.end.x.max(p.x);
                self.end.y   = self.end.y.max(p.y);
                0            
            }
        ).collect::<Vec<i32>>();
    }

    fn print(&self,x0:usize,x1:usize,y0:usize,y1:usize)
    {
        for y in y0..=y1
        {
            for x in x0..=x1
            {
                print!("{}",self.val(x as i64,y as i64));
            }
            println!();
        }
    }

    fn plum(&mut self,x:i64,y:i64,lim:i64)->bool
    {
        let mut p = Vec2::new(x,y);
        let mut cnt=0;

        let mut moving = true;

        while moving 
        {
            cnt+=1;
            moving = false;

            if self.val(p.x,p.y+1)=='.'
            {
                p.y+=1;
                moving = true;
            }
            else if self.val(p.x-1,p.y+1)=='.'
            {
                p.x-=1;
                p.y+=1;
                moving = true;
            }
            else if self.val(p.x+1,p.y+1)=='.'
            {
                p.x+=1;
                p.y+=1;
                moving = true;
            }

            if lim!=-99999
            {
                if p.y==lim
                {
                    moving = false;
                }
            }
            else
            if cnt>1000 
            {
                return true;
            }
        }
        
        self.field.insert(p,'o');

        if lim!=-99999 && p.y==0 && p.x==500 
        {
            return true;
        }

        false
    }

    fn count(&self,c:char)->usize
    {
        self.field.values().filter(|&x| x==&c).count()
    }

    fn val(&self,x:i64,y:i64)->char
    {
        *self.field.get(&Vec2::new(x as i64,y as i64)).unwrap_or(&'.')
    }
}

pub fn part1(data:&[String])->usize
{
    let mut w  = World::new();
    w.load(data);
    w.calc_sizes();

    while !w.plum(500, 0,-99999) {}

    w.print(494,503,0,9);
    w.count('o')
}

pub fn part2(data:&[String])->usize
{
    let mut w  = World::new();
    w.load(data);
    w.calc_sizes();

    let last = w.count('o');

    while !w.plum(500, 0,w.end.y+1)
    {
    }

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