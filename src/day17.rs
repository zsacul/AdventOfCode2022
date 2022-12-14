use std::collections::HashMap;
use super::vec2::Vec2;

#[derive(Eq, PartialEq,  Debug, Clone)]
struct Part
{
    points : Vec<Vec2>,
    types  : usize,
}

impl Part 
{
    fn new(num:usize)->Self
    {
        let templates = vec![
            //....
            //....
            //....
            //####
            (vec![0,0, 1,0, 2,0, 3,0], 0,0),

            //....
            //.#..
            //###.
            //.#..
            (vec![1,0, 0,1, 1,1, 2,1, 1,2], 0,0),

            //....
            //..#.
            //..#.
            //###.
            (vec![0,0, 1,0, 2,0, 2,1, 2,2],0,0),

            //#...
            //#...
            //#...
            //#...
            (vec![0,0, 0,1, 0,2, 0,3],0,0),

            //....
            //....
            //##..
            //##..
            (vec![0,0, 1,0, 0,1, 1,1],0,0),
        ];

        let (numbers,offx,offy) = &templates[num];
        let number = templates.len();
        let mut pts = vec![];
        
        for i in (0..numbers.len()).step_by(2) 
        {
            pts.push(Vec2::new(numbers[i  ] + offx,
                               numbers[i+1] + offy));
        }

        Self {
            points : pts,
            types  : number 
        }
    }
}

#[derive(Eq, PartialEq,  Debug, Clone)]
struct World
{
    well      : HashMap<Vec2,char>,
    top       : i64,
    data      : Vec<char>,
    data_pos  : usize,
    part_id   : usize,    
    part      : Vec<Part>,
}

impl World {
    fn new()->Self
    {
        Self
        {
            well     : HashMap::new(),
            top      : 0,
            data     : vec![],
            data_pos : 0,
            part_id  : 0,
            part     : vec![Part::new(0),Part::new(1),Part::new(2),Part::new(3),Part::new(4)],
        }
    }

    fn get_next_part(&mut self)->usize
    {
        self.part_id = (self.part_id + 1usize) % self.part.len();
        self.part_id
    }

    fn get_next_dir(&mut self)->char
    {
        let res = self.data[self.data_pos]        ;
        self.data_pos = (self.data_pos+1)%self.data.len();
        res        
    }

    fn load(&mut self,commands:&str)
    {
        self.data = commands.chars().collect::<Vec<char>>();
        self.draw(Vec2::new(0,0), Vec2::new(8,   0));
    }

    fn placement_ok(&self,part_id:usize,pos:&Vec2)->bool
    {
        for p in &self.part[part_id].points
        {
            if self.get(p.x + pos.x,p.y + pos.y)!='.' { return false; }
        }
        true
    }

    fn placement_char(&self,part_id:usize,pos:&Vec2,c:char)->bool
    {
        for p in &self.part[part_id].points
        {
            if self.get(p.x + pos.x,p.y + pos.y)==c
            {
                return true;
            }
        }
        false
    }

    fn place(&mut self,part_id:usize,pos:&Vec2)
    {
        for p in &self.part[part_id].points
        {
            self.well.insert(Vec2::new(p.x+pos.x,p.y+pos.y),'@');
            self.top = self.top.max(p.y+pos.y);
        }
    }

    fn get(&self,x:i64,y:i64)->char
    {
        if x<=0 || x>=8 { return '#'; }
        *self.well.get(&Vec2::new(x,y)).unwrap_or(&'.')
    }

    #[allow(unused)]
    fn set(&mut self,x:i64,y:i64,v:char)
    {
        self.well.insert(Vec2::new(x,y),v);
    }

    fn draw(&mut self,p1:Vec2,p2:Vec2)
    {
        if p1.x==p2.x
        {
            for y in p1.y.min(p2.y)..=p1.y.max(p2.y)
            {
                self.well.insert(Vec2::new(p1.x,y),'#');
            }
        }
          else
        {
            for x in p1.x.min(p2.x)..=p1.x.max(p2.x)
            {
                self.well.insert(Vec2::new(x,p1.y),'#');
            }
        }
    }

    #[allow(unused)]
    fn print(&self,x0:usize,x1:usize,y0:usize,y1:usize)
    {
        for y in y0..=y1 {
        for x in x0..=x1 { print!("{}",self.get(x as i64,y1 as i64-y as i64)); }
            println!();
        }
        println!();
    }

    #[allow(unused)]
    fn printh(&self,y0:usize,y1:usize)
    {
        self.print(0, 8, y0, y1)
    }

    #[allow(unused)]
    fn printvis(&self)
    {
        self.printh(0, self.top as usize);
    }

    fn count(&mut self,n:usize)->usize 
    {
        let mut part_num = 0;
        let mut n = n;

        while n>0
        {
            let mut placed = false;
            let mut part_pos = Vec2::new(3,4 + self.top);

            while !placed            
            {
                let c = self.get_next_dir();
                
                let offset_x:i64 = match c {
                                               '<' => -1,
                                               '>' =>  1,
                                                _  => panic!("wrong code"),
                                           };

                let right = part_pos.add(offset_x,0);                
    
                if !self.placement_char(part_num, &right,'@') && !self.placement_char(part_num, &right,'#')
                {
                    part_pos = right;                   
                }

                let down = part_pos.add(0,-1);
                
                if !self.placement_ok(part_num,&down)
                {
                    self.place(part_num, &part_pos);
                    placed = true;
                }
                  else 
                {
                    part_pos = down;
                }              
            }
            part_num = self.get_next_part();
            n-=1;
        }
        self.top as usize
    }
}

fn calc(commands:&str,iters:usize)->usize
{
    let mut w  = World::new();
    w.load(commands);
    w.count(iters)
}

fn find_offset(commands:&str)->Option<(usize,usize,usize)>
{
    for offset in 100..10000
    {
        for step in 1..500
        {
            let     scores       = calc(commands,offset+(step*5));
            let mut last         = scores;
            let mut stable_delta = 0;

            for i in 2..15
            {
                let res   = calc(commands,offset + i*step*5);
                let delta = res-last;

                if stable_delta==0
                {
                    stable_delta = delta;
                }
                else if delta!=stable_delta
                {
                    stable_delta = 0;
                    break;
                }
      
                last = res;
            }
      
            if stable_delta!=0
            {
                return Some((offset,step*5,stable_delta));
            }
        }
    }
    None
}

pub fn part1(data:&[String])->usize
{
    calc(&data[0][..],2022)
}

pub fn part2(data:&[String])->usize
{
    let commands = &data[0][..];
    let t = 1_000_000_000_000_usize;
    let (offset,step,delta) = find_offset(commands).expect("failure");
    let count = (t-offset)/step; 
    let left  =  t-count*step;

    calc(commands,left) + count*delta
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day 17");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".to_string()];
    assert_eq!(part1(&v),3068);
}

#[test]
fn test2()
{
    let v = vec![">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".to_string()];
    assert_eq!(part2(&v),1514285714288);
}