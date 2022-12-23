use std::collections::HashMap;
use super::vec2::Vec2;
use super::tools;

#[derive( Eq, PartialEq, Debug,  Clone)]
struct Elf
{
    id      : usize,
    pos     : Vec2,
    moves   : Vec<char>,
    move_to : Vec2,
    move_ok : bool
}

impl Elf {
    fn new(pos:Vec2,id:usize)->Self
    {
        Self {
            id,
            pos,
            moves  : vec!['n','s','w','e'],
            move_to: Vec2::zero(),
            move_ok: false
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

    fn update(&mut self)
    {
        self.field.clear();

        for elf in self.elfs
        {
            self.field.insert(elf.pos, elf.id);
        }        
    }

    fn neighbours(&self,p:Vec2)->usize
    {
        let mut res = 0;
        for v in p.around8()
        {
            if self.is_empty(v)
            {
                res+=1;
            }
        }
        res
    }

    fn first_half(&mut self)
    {
        for e in self.elfs.iter_mut()
        {
            e.move_ok = true;
            if self.neighbours(e.pos)==0
            {
                e.move_ok = false;
            }

        }



    }

    fn second_half(&mut self)
    {
        
    }

    fn moving(&mut self)
    {
        self.update();
        self.first_half();
        self.second_half();
    }

    fn is_empty(&self,p:Vec2)->bool
    {
        *self.field.get(&p).unwrap_or(&usize::MAX)==usize::MAX
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
            _ => panic!("wrong dir");
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

  



    #[allow(unused)]
    fn print(&self,xx:usize,yy:usize)
    {
        for y in 0..=yy
        {
            for x in 0..=xx
            {
                let pos = Vec2::new(x as i64,y as i64);                
                let t = self.field.get(&pos);

                if t.is_some() 
                { 
                    print!("#",t.unwrap().1); 
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
        let mut minx = self.elfs
                           .iter()
                           .map(|e| e.pos.x).min().unwrap();
        let mut miny = self.elfs
                                .iter()
                                .map(|e| e.pos.y).min().unwrap();
        let mut maxx = self.elfs
                                .iter()
                                .map(|e| e.pos.x).max().unwrap();
        let mut maxy = self.elfs
                                .iter()
                                .map(|e| e.pos.y).max().unwrap();

        ((maxx-minx+1)*(maxy-miny+1) - self.elfs.len() as i64) as usize
   }
  
    fn compute(&mut self)->usize
    {    
        for i in 0..5
        {
            self.moving();
        }
        self.count_empty()
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

pub fn part1(data:&[String])->usize
{
    World::new(data,false).compute()
}

pub fn part2(data:&[String])->usize
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
