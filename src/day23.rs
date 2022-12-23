use std::collections::HashMap;
use super::vec2::Vec2;

enum Move 
{
    N,S,E,W
}

#[derive( Eq, PartialEq, Debug,  Clone)]
struct Elf
{
    id   : usize,
    pos  : Vec2,
    moves: Vec<char>
}

impl Elf {
    fn new(pos:Vec2,id:usize)->Self
    {
        Self {
            id,
            pos,
            moves: vec!['n','s','w','e'],//Move::N,Move::S,Move::W,Move::E],
        }
    }

    fn shift_moves(&mut self)
    {
        let t = self.moves[0];
        self.moves[0] = self.moves[1];
        self.moves[1] = self.moves[2];
        self.moves[2] = self.moves[3];
        self.moves[3] = t;
    }
}

#[derive( Eq, PartialEq, Debug,  Clone)]
struct World
{
    elfs  : Vec<Elf>,
    field : HashMap<Vec2,usize>
    
}

impl World {
    
    fn get_point(&self,x:usize,y:usize)->Vec2
    {
        Vec2::new( (x*self.n) as i64, (y*self.n) as i64)
    }

    fn update(&mut self)
    {
        self.field.clear();

        for elf in self.elfs
        {
            self.field.insert(elf.pos, elf.id);
        }        
    }

    fn is_empty(&self,p:Vec2)->bool
    {
        true
    }

    fn good_move(&self,p:Vec2,m:char)->bool
    {
        match  {
            'n'=> { 
                self.is_empty(Vec2::new(p.x-1,p.y-1), m) &&
                self.is_empty(Vec2::new(p.x  ,p.y-1), m) &&
                self.is_empty(Vec2::new(p.x+1,p.y-1), m)
            },
            's'=> { 
                self.is_empty(Vec2::new(p.x-1,p.y+1), m) &&
                self.is_empty(Vec2::new(p.x  ,p.y+1), m) &&
                self.is_empty(Vec2::new(p.x+1,p.y+1), m)
            },
            'w'=> { 
                self.is_empty(Vec2::new(p.x-1,p.y-1), m) &&
                self.is_empty(Vec2::new(p.x-1,p.y  ), m) &&
                self.is_empty(Vec2::new(p.x-1,p.y+1), m)
            },
            'e'=> { 
                self.is_empty(Vec2::new(p.x+1,p.y-1), m) &&
                self.is_empty(Vec2::new(p.x+1,p.y  ), m) &&
                self.is_empty(Vec2::new(p.x+1,p.y+1), m)
            },
        }
    }

 
    fn new(data:&[String],part2:bool)->Self
    {
        let mut field = HashMap::new();
        let mut id=0usize;
        let mut elfs = vec![];
    
        for (py, line) in data.iter().enumerate()
        {
            for (px ,c) in line.chars().enumerate()
            {         
                if c=='#'
                {
                    let pos = Vec2::new(px as i64,py as i64);
                    elfs.push(Elf::new(pos, id))
                                        
                    field.insert(pos, id);
                    id+=1;
                }         
            }
        }
    
        Self {
            elfs,
            field
        }
    }

  


    fn moved(&self)->Vec2
    {
        self.movedir(self.dir)
    }

    fn movedir(&self,dir:u8)->Vec2
    {
        match dir 
        {
            0 => Vec2::new( 1, 0),
            1 => Vec2::new( 0, 1),
            2 => Vec2::new(-1, 0),
            3 => Vec2::new( 0,-1),
            _ => {panic!("wrong rotation"); },
        }
    }

    fn move_op(&self)->Vec2
    {
        let d = self.moved();
        Vec2::new(-d.x,-d.y)
    }

    fn get_final_code(&self)->i64
    {
        1000*(self.pos.y+1) + 4*(self.pos.x+1) + self.dir as i64
    }

    fn pos_ok(&self,x:i32,y:i32)->bool
    {
        !(x<0 || y<0 || x>=self.size.x as i32 || y>=self.size.y as i32)
    }

    fn pos_okv(&self,p: &Vec2)->bool
    {
        self.pos_ok(p.x as i32,p.y as i32)
    }

    #[allow(unused)]
    fn print(&self)
    {
        for y in -2..=self.size.y+2
        {
            for x in -2..=self.size.x+2
            {
                let pos = Vec2::new(x as i64,y as i64);
                
                let t = self.teleport.values().find(|(v,r)| v==&pos );

                if t.is_some() 
                { 
                    print!("{}",t.unwrap().1); 
                }
                else if !self.pos_okv(&pos) 
                { 
                    print!("?");                
                }
                else 
                {
                    print!("{}",self.get(pos));
                }
            }
            println!();
        }
    }

    fn get(&self,p:Vec2)->usize
    {     
        *self.field.get(&p).unwrap_or(0)
    }

  
    fn compute(&mut self)->i64
    {    
        0
        /*
        let moves = Self::get_path(self.path.clone());
        self.field[self.pos.y as usize][self.pos.x as usize] = self.mark();

        for m in moves.iter() 
        {
            match m.chars().last().unwrap()
            {
                'L' => { self.forwards(&m[..m.len()-1]); self.left()  },
                'R' => { self.forwards(&m[..m.len()-1]); self.right() },
                 _  => { self.forwards(&m[..         ]);              },
            }
            self.field[self.pos.y as usize][self.pos.x as usize] = self.mark();
        }
  
        //self.print();
        //println!{"{},{} rot:{}",self.pos.x,self.pos.y,self.dir};
        //println!{"{}",self.path};
        self.get_final_code()
        */
    }
}

pub fn part1(data:&[String])->i64
{
    World::new(data,false).compute()
}

pub fn part2(data:&[String])->i64
{
    0
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day 23");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
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
    assert_eq!(part1(&v),110);
}
