use std::collections::HashMap;
use std::collections::HashSet;
use super::vec2::Vec2;

#[derive(Eq, PartialEq, Debug, Clone)]
struct Elf
{
    position : Vec2,
    move_to  : Vec2,
    move_ok  : bool,
}

impl Elf 
{
    fn new(position:Vec2)->Self
    {
        Self 
        {
            position,
            move_to : Vec2::new(i64::MAX,i64::MAX),
            move_ok : false,
        }
    }
}

#[derive( Eq, PartialEq, Debug,  Clone)]
struct World
{
    elfs  : Vec<Elf>,
    field : HashSet<Vec2>,
    moves : Vec<char>
}

impl World {  

    fn shift_moves(&mut self)
    {
        let      temp = self.moves[0];
        self.moves[0] = self.moves[1];
        self.moves[1] = self.moves[2];
        self.moves[2] = self.moves[3];
        self.moves[3] = temp;
    }

    fn update(&mut self)
    {
        self.field.clear();

        for elf in self.elfs.iter()
        {
            self.field.insert(elf.position);
        }        
    }

    fn neighbours(&mut self,p:Vec2)->usize
    {
        p.around8()
         .iter()
         .map(|p| !self.is_empty(*p) as usize)
         .sum()
    }

    fn move_to(p:Vec2,m:char)->Vec2
    {        
        match m {
            'n'=> { Vec2::new(p.x  ,p.y-1) },
            's'=> { Vec2::new(p.x  ,p.y+1) },
            'w'=> { Vec2::new(p.x-1,p.y  ) },
            'e'=> { Vec2::new(p.x+1,p.y  ) },
             _ => panic!("wrong dir")
        }    
    }

    fn first_half(&mut self)
    {
        for i in 0..self.elfs.len()
        {
            self.elfs[i].move_ok = self.neighbours(self.elfs[i].position)>0;
        }

        for i in 0..self.elfs.len()
        {
            if self.elfs[i].move_ok
            {
                let mut good = false;

                for m in self.moves.iter()
                {
                    let mo = *m;

                    if self.good_move(self.elfs[i].position, *m)
                    {
                        self.elfs[i].move_to = Self::move_to(self.elfs[i].position, mo);    
                        good = true;
                        break;
                    }
                }

                if !good
                {
                    self.elfs[i].move_ok = false;
                }
            }
        }

        let mut common_moves : HashMap<Vec2,usize> = HashMap::new();

        for i in 0..self.elfs.len()
        {
            if self.elfs[i].move_ok
            {
                let count = *common_moves.get(&self.elfs[i].move_to).unwrap_or(&0);
                common_moves.insert(self.elfs[i].move_to, count+1);
            }
        }

        for i in 0..self.elfs.len()
        {
            if *common_moves.get(&self.elfs[i].move_to).unwrap_or(&0)>1 
            {
                self.elfs[i].move_ok = false;
            }            
        }
    }

    fn second_half(&mut self)->bool
    {
        let mut res = false;

        for e in self.elfs.iter_mut()
        {
            if e.move_ok
            {
                e.position = e.move_to;
                res = true;
            }
        }       

        self.shift_moves();                
        res
    }

    fn moving(&mut self)->bool
    {
        self.update();
        self.first_half();
        self.second_half()
    }

    fn is_empty(&self,p:Vec2)->bool
    {
        self.field.get(&p).is_none()
    }

    fn good_move(&self,p:Vec2,m:char)->bool
    {
        match m {
            'n'=> { 
                self.is_empty(Vec2::new(p.x-1,p.y-1)) &&
                self.is_empty(Vec2::new(p.x  ,p.y-1)) &&
                self.is_empty(Vec2::new(p.x+1,p.y-1))
            },
            's'=> { 
                self.is_empty(Vec2::new(p.x-1,p.y+1)) &&
                self.is_empty(Vec2::new(p.x  ,p.y+1)) &&
                self.is_empty(Vec2::new(p.x+1,p.y+1))
            },
            'w'=> { 
                self.is_empty(Vec2::new(p.x-1,p.y-1)) &&
                self.is_empty(Vec2::new(p.x-1,p.y  )) &&
                self.is_empty(Vec2::new(p.x-1,p.y+1))
            },
            'e'=> { 
                self.is_empty(Vec2::new(p.x+1,p.y-1)) &&
                self.is_empty(Vec2::new(p.x+1,p.y  )) &&
                self.is_empty(Vec2::new(p.x+1,p.y+1))
            },
            _ => panic!("wrong dir")
        }
    }
 
    fn new(data:&[String])->Self
    {
        let mut field = HashSet::new();
        let mut elfs  = vec![];
    
        for (py, line) in data.iter().enumerate()
        {
            for (px ,c) in line.chars().enumerate()
            {         
                if c=='#'
                {
                    let position = Vec2::new(px as i64,py as i64);
                    elfs.push(Elf::new(position));
                    field.insert(position);
                }         
            }
        }
    
        Self {
            elfs,
            field,
            moves : vec!['n','s','w','e'],
        }
    }

    #[allow(unused)]
    fn print(&self,xx:usize,yy:usize)
    {
        for y in 0..=yy as i64
        {
            for x in 0..=xx as i64
            {
                if self.field.get(&Vec2::new(x,y)).is_some() 
                { 
                    print!("#"); 
                }
                  else 
                { 
                    print!(".");                
                }
            }
            println!();
        }
    }

    fn count_empty(&self)->usize
    {
        let min_x = self.elfs.iter().map(|e| e.position.x).min().unwrap();
        let min_y = self.elfs.iter().map(|e| e.position.y).min().unwrap();
        let max_x = self.elfs.iter().map(|e| e.position.x).max().unwrap();
        let max_y = self.elfs.iter().map(|e| e.position.y).max().unwrap();

        ((max_x - min_x + 1)*(max_y - min_y + 1) - self.elfs.len() as i64) as usize
   }
  
    fn compute1(&mut self,rounds:usize)->usize
    {    
        for _ in 0..rounds
        {            
            self.moving();                        
        }
        self.count_empty()
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
    World::new(data).compute2()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day 23");
    println!("part1: {}",part1(data,10));
    println!("part2: {}",part2(data   ));
}

#[test]
fn test1()
{
    let v = vec![
        "..............".to_string(),
        "..............".to_string(),
        ".......#......".to_string(),
        ".....###.#....".to_string(),
        "...#...#.#....".to_string(),
        "....#...##....".to_string(),
        "...#.###......".to_string(),
        "...##.#.##....".to_string(),
        "....#..#......".to_string(),
        "..............".to_string(),
        "..............".to_string(),
        "..............".to_string(),
    ];
    assert_eq!(part1(&v,10),110);
}

#[test]
fn test2()
{
    let v = vec![
        "..............".to_string(),
        "..............".to_string(),
        ".......#......".to_string(),
        ".....###.#....".to_string(),
        "...#...#.#....".to_string(),
        "....#...##....".to_string(),
        "...#.###......".to_string(),
        "...##.#.##....".to_string(),
        "....#..#......".to_string(),
        "..............".to_string(),
        "..............".to_string(),
        "..............".to_string(),
    ];
    assert_eq!(part2(&v),20);
}

#[test]
fn test1_2()
{
    let v = vec![
                    ".....".to_string(),
                    "..##.".to_string(),
                    "..#..".to_string(),
                    ".....".to_string(),
                    "..##.".to_string(),
                    ".....".to_string(),
                ];
    assert_eq!(part1(&v,10),25);
}
