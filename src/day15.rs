use std::collections::HashMap;
use super::vec2::Vec2;

#[derive(Eq, PartialEq,  Debug, Clone)]
struct World
{
    field  : HashMap<Vec2,char>,
    b_dist : HashMap<Vec2,i64>,
}

impl World {
    fn new()->Self
    {
        Self
        {
            field : HashMap::new(),
            b_dist : HashMap::new(),
        }
    }

    fn dist(p1:&Vec2,p2:&Vec2)->i64
    {
        (p2.x-p1.x).abs()+
        (p2.y-p1.y).abs()
    }

    fn load(&mut self,data:&[String])
    {
        for line in data 
        {
            let tab : Vec<&str> = line.split(": closest beacon is at x=").collect(); 
            let t2  : Vec<&str> = tab[0].split("Sensor at x=").collect(); 
            let p1  : Vec<&str> =  t2[1].split(", y=").collect();
            let p1x = p1[0].parse::<i64>().unwrap();
            let p1y = p1[1].parse::<i64>().unwrap();

            let p2 : Vec<&str>= tab[1].split(", y=").collect();
            let p2x = p2[0].parse::<i64>().unwrap();
            let p2y = p2[1].parse::<i64>().unwrap();

            
            let s = Vec2::new(p1x,p1y);
            let b = Vec2::new(p2x,p2y);
            let d  = World::dist(&b,&s);

            self.field.insert(s,'S');
            self.field.insert(b,'B');
            
            self.b_dist.insert(s,d);
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

    fn count(&self,y_pos:i64)->i64
    {
        let mut count = 0;
        let yy = y_pos;
        
        for xx in -5091374..5091374
        {
            let mut was = false;
            let x = Vec2::new(xx,yy);

            if self.field.get(&x).unwrap_or(&'.')==&'.'
            {
                for (sensor_pos,sensor_range) in self.b_dist.iter() 
                {
                    let dist = World::dist(&x, sensor_pos);

                    if dist<=*sensor_range
                    {                        
                        was=true;
                        break;
                    }
                }

                if was { count+=1 }
            }            
        }
       
        count
        
    }

    fn countfx(&self,y_pos:i64,range:i64)->i64
    {        
        let yy = y_pos;
        let mut xx=0;

        while xx<=range
        {
            let mut was = false;
            let x = Vec2::new(xx,yy);

            let &c = self.field.get(&x).unwrap_or(&'.');

            if c=='.'
            {
                for (sensor_pos,&sensor_range) in self.b_dist.iter() 
                {
                    let dist = World::dist(&x, sensor_pos);

                    if dist<=sensor_range
                    {                        
                        was = true;
                        xx += sensor_range-dist;
                        break;
                    }
                }

                if !was
                {
                   return xx as i64; 
                } 
            }
            
            xx+=1;
        }
        -1
    }

    fn count2(&self,range:i64)->(i64,i64)
    {
        for y in 1..=range 
        {
            let x = self.countfx(y,range);
            if x!=-1 { return (x,y); }
        }
        (0,0)
    }

    fn val(&self,x:i64,y:i64)->char
    {
        *self.field.get(&Vec2::new(x as i64,y as i64)).unwrap_or(&'.')
    }
}

pub fn part1(data:&[String],y:i64)->i64
{
    let mut w = World::new();
    w.load(data);
    w.count(y)
}

pub fn part2(data:&[String],range:i64)->i64
{
    let mut w = World::new();
    w.load(data);
    let (x,y) = w.count2(range);
    x*4000000+y
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day 15");
    println!("part1: {}",part1(data,2000000));
    println!("part2: {}",part2(data,4000000));
}

#[test]
fn test1()
{
    let v = 
    vec![
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15".to_string(),
        "Sensor at x=9, y=16: closest beacon is at x=10, y=16".to_string(),
        "Sensor at x=13, y=2: closest beacon is at x=15, y=3".to_string(),
        "Sensor at x=12, y=14: closest beacon is at x=10, y=16".to_string(),
        "Sensor at x=10, y=20: closest beacon is at x=10, y=16".to_string(),
        "Sensor at x=14, y=17: closest beacon is at x=10, y=16".to_string(),
        "Sensor at x=8, y=7: closest beacon is at x=2, y=10".to_string(),
        "Sensor at x=2, y=0: closest beacon is at x=2, y=10".to_string(),
        "Sensor at x=0, y=11: closest beacon is at x=2, y=10".to_string(),
        "Sensor at x=20, y=14: closest beacon is at x=25, y=17".to_string(),
        "Sensor at x=17, y=20: closest beacon is at x=21, y=22".to_string(),
        "Sensor at x=16, y=7: closest beacon is at x=15, y=3".to_string(),
        "Sensor at x=14, y=3: closest beacon is at x=15, y=3".to_string(),
        "Sensor at x=20, y=1: closest beacon is at x=15, y=3".to_string(),
        ];
    assert_eq!(part1(&v,10),26);
}

#[test]
fn test2()
{
    let v = 
    vec![
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15".to_string(),
        "Sensor at x=9, y=16: closest beacon is at x=10, y=16".to_string(),
        "Sensor at x=13, y=2: closest beacon is at x=15, y=3".to_string(),
        "Sensor at x=12, y=14: closest beacon is at x=10, y=16".to_string(),
        "Sensor at x=10, y=20: closest beacon is at x=10, y=16".to_string(),
        "Sensor at x=14, y=17: closest beacon is at x=10, y=16".to_string(),
        "Sensor at x=8, y=7: closest beacon is at x=2, y=10".to_string(),
        "Sensor at x=2, y=0: closest beacon is at x=2, y=10".to_string(),
        "Sensor at x=0, y=11: closest beacon is at x=2, y=10".to_string(),
        "Sensor at x=20, y=14: closest beacon is at x=25, y=17".to_string(),
        "Sensor at x=17, y=20: closest beacon is at x=21, y=22".to_string(),
        "Sensor at x=16, y=7: closest beacon is at x=15, y=3".to_string(),
        "Sensor at x=14, y=3: closest beacon is at x=15, y=3".to_string(),
        "Sensor at x=20, y=1: closest beacon is at x=15, y=3".to_string(),
       ];
    assert_eq!(part2(&v,20),56000011);
}