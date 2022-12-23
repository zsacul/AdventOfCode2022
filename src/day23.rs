use std::collections::HashMap;
use super::vec2::Vec2;

#[derive(Eq, PartialEq, Debug, Clone)]
struct Elf
{
    id      : usize,
    pos     : Vec2,
    move_to : Vec2,
    move_ok : bool,
    nothing : bool,
}

impl Elf 
{
    fn new(pos:Vec2,id:usize)->Self
    {
        Self 
        {
            id,
            pos,
            move_to : Vec2::new(70000,70000),
            move_ok : false,
            nothing : false,
        }
    }
}

#[derive( Eq, PartialEq, Debug,  Clone)]
struct World
{
    elfs  : Vec<Elf>,
    field : HashMap<Vec2,usize>,
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
            self.field.insert(elf.pos, elf.id);
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
            self.elfs[i].move_ok = true;
            self.elfs[i].nothing = false;
        }

        for i in 0..self.elfs.len()
        {
            self.elfs[i].move_ok = true;

            if self.neighbours(self.elfs[i].pos)==0
            {
                self.elfs[i].move_ok = false;
                self.elfs[i].nothing = true;
            }
        }

        for i in 0..self.elfs.len()
        {
            if self.elfs[i].move_ok
            {
                let mut good = false;

                for m in self.moves.clone().iter()
                {
                    let mo = *m;

                    if self.good_move(self.elfs[i].pos, *m)
                    {
                        self.elfs[i].move_to = Self::move_to(self.elfs[i].pos, mo);    
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

        let mut common_moves:HashMap<Vec2,usize> = HashMap::new();

        for i in 0..self.elfs.len()
        {
            if self.elfs[i].move_ok
            {
                let count = common_moves.get(&self.elfs[i].move_to).unwrap_or(&0);
                common_moves.insert(self.elfs[i].move_to, *count+1);
            }
        }

        for i in 0..self.elfs.len()
        {
            if self.elfs[i].move_ok
            {
                if *common_moves.get(&self.elfs[i].move_to).unwrap_or(&0)>1 
                {
                    self.elfs[i].move_ok = false;
                }
            }
        }
    }

    fn second_half(&mut self)->bool
    {
        let mut res = false;

        for e in self.elfs.iter_mut()
        {
            if e.move_ok && !e.nothing
            {
                e.pos = e.move_to;
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
            _ => panic!("wrong dir")
        }
    }

 
    fn new(data:&[String])->Self
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
                    elfs.push(Elf::new(pos, id));
                    field.insert(pos, id);
                    id+=1;
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
                let pos = Vec2::new(x as i64,y as i64);                
                let t = self.field.get(&pos);

                if t.is_some() 
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
        let  minx = self.elfs
                           .iter()
                           .map(|e| e.pos.x).min().unwrap();
        let  miny = self.elfs
                                .iter()
                                .map(|e| e.pos.y).min().unwrap();
        let  maxx = self.elfs
                                .iter()
                                .map(|e| e.pos.x).max().unwrap();
        let  maxy = self.elfs
                                .iter()
                                .map(|e| e.pos.y).max().unwrap();

        ((maxx-minx+1)*(maxy-miny+1) - self.elfs.len() as i64) as usize
   }
  
    fn compute1(&mut self,rounds:usize)->usize
    {    
        for i in 0..rounds
        {            
            self.moving();                        
        }
        self.count_empty()
    }

    fn compute2(&mut self)->usize
    {    
        let mut id  = 0;

        loop
        {            
            id+=1;
            if !self.moving() { return id;}
        }
    }

}

pub fn part1(data:&[String],rounds:usize)->usize
{
    World::new(data).compute1(rounds)
}

pub fn part2(data:&[String],rounds:usize)->usize
{
    World::new(data).compute2()
}


#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day 23");
    println!("part1: {}",part1(data,10));
    println!("part2: {}",part2(data,10));
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

/* 
fn test2()
{
    let v = vec![
                                ".......#......".to_string(),
                                "....#......#..".to_string(),
                                "..#.....#.....".to_string(),
                                "......#.......".to_string(),
                                "...#....#.#..#".to_string(),
                                "#.............".to_string(),
                                "....#.....#...".to_string(),
                                "..#.....#.....".to_string(),
                                "....#.#....#..".to_string(),
                                ".........#....".to_string(),
                                "....#......#..".to_string(),
                                ".......#......".to_string(),
                             ];
    assert_eq!(part2(&v,10),20);
}


*/