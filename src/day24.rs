use std::collections::HashMap;
use std::collections::HashSet;
use super::vec2::Vec2;

#[derive(Eq, PartialEq, Debug, Clone)]
struct Blizzard 
{
    position : Vec2,
    dir      : char,
}

impl Blizzard  
{
    fn new(position:Vec2,dir:char)->Self
    {
        Self 
        {
            position,
            dir
        }
    }
}

#[derive( Eq, PartialEq, Debug,  Clone)]
struct World
{
    bliz  : Vec<Blizzard>,
    field : HashMap<Vec2,char>,
    moves : Vec<char>,
    size  : Vec2,
    time  : usize
}

impl World {  


    fn next_pos(p:Vec2,c:char)->Vec2
    {
        let n = 
        match c {
            '^'=> { 
                Vec2::new(p.x  ,p.y-1)
            },
            'v'=> { 
                Vec2::new(p.x  ,p.y+1)
            },
            '<'=> { 
                Vec2::new(p.x-1,p.y  )
            },
            '>'=> { 
                Vec2::new(p.x+1,p.y  )
            },
            _ => panic!("wrong dir")
        };
        n
    }

    fn update(&mut self)
    {
        field.clear();
        for b in self.bliz.iter_mut()
        {
            b.position = self.next_pos(b.position);
            self.field.insert(b.position);
        }        
    }

    fn is_empty(&self,p:Vec2)->bool
    {
        self.field.get(&p).is_none()
    }

 
    fn new(data:&[String])->Self
    {
        let mut field = HashSet::new();
        let mut bliz  = vec![];
        let size = Vec2::new(data[0].len(),data.len());
    
        for (py, line) in data.iter().enumerate()
        {
            for (px ,c) in line.chars().enumerate()
            {         
                if c!='#' && if c!='.'
                {
                    if c!='.'
                    {
                        let position = Vec2::new(px as i64,py as i64);
                        bliz.push(blizzard::new(position,c));
                    }
                    if c=='#' { field.insert('#'); }
                         else { field.insert('.'); }
                }        
            }
        }
    
        Self {
            bliz,
            field,            
            size     
        }
    }

    #[allow(unused)]
    fn print(&self,xx:usize,yy:usize)
    {
        for y in 0..=yy as i64
        {
            for x in 0..=xx as i64
            {
                let mut c = '.';
                for b in self.bliz
                {
                    if x==b.pos.x && y==b.pos.y
                    {
                        c = b.dir;
                    }
                }

                //for b
                //any.get(&Vec2::new(x,y)).is_some() 
                if self.field.get(&Vec2::new(x,y))=='#'
                { 
                    print!("#"); 
                }
                  else 
                { 
                    print!(c);
                }
            }
            println!();
        }
    }

  
    fn compute1(&mut self,rounds:usize)->usize
    {    
//        for id in 1..usize::MAX
        for id in 1..10
        {                        
            self.moving();
            self.print();
        }
        0        
    }

    fn compute2(&mut self)->usize
    {    
        for id in 1..usize::MAX
        {                        
            if !self.moving() { return id; }
        }
        0
    }

}

fn part1(data:&[String],rounds:usize)->usize
{
    World::new(data).compute1(rounds)
}

fn part2(data:&[String])->usize
{
    0
    //World::new(data).compute2()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day 24");
    println!("part1: {}",part1(data,10));
    println!("part2: {}",part2(data   ));
}

#[test]
fn test1()
{
    let v = vec![
        "#.#####".to_string(),
        "#.....#".to_string(),
        "#>....#".to_string(),
        "#.....#".to_string(),
        "#...v.#".to_string(),
        "#.....#".to_string(),
        "#####.#".to_string(),
    ];
    assert_eq!(part1(&v,10),110);
}

#[test]
fn test2()
{
    let v = vec![
        "#.#####".to_string(),
        "#.....#".to_string(),
        "#>....#".to_string(),
        "#.....#".to_string(),
        "#...v.#".to_string(),
        "#.....#".to_string(),
        "#####.#".to_string(),
    ];
    assert_eq!(part2(&v),20);
}
