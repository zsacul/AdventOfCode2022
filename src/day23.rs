use std::collections::HashMap;
use super::vec2::Vec2;
use super::tools;

#[derive(Eq, PartialEq, Debug, Clone)]
struct Elf
{
    id      : usize,
    pos     : Vec2,
    moves   : Vec<char>,
    move_to : Vec2,
    move_ok : bool,
    nothing : bool
}

impl Elf 
{
    fn new(pos:Vec2,id:usize)->Self
    {
        Self 
        {
            id,
            pos,
            moves   : vec!['n','s','w','e'],
            move_to : Vec2::zero(),
            move_ok : false,
            nothing : false
        }
    }

    fn shift_moves(&mut self)
    {
        let         t = self.moves[0];
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

        for elf in self.elfs.iter()
        {
            self.field.insert(elf.pos, elf.id);
        }        
    }

    fn neighbours(&mut self,p:Vec2)->usize
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
        //for e in self.elfs.iter_mut()
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

                for m in self.elfs[i].moves.iter()
                {
                    if self.good_move(self.elfs[i].pos, *m)
                    {
                        self.elfs[i].move_to = Self::move_to(self.elfs[i].pos, *m);                        
                        good = true;
                        break;
                    }
                }

                if !good
                {
                    self.elfs[i].move_ok = true;
                }
            }
        }

       // println!("{:#?}",self.elfs);

        let mut cnt:HashMap<Vec2,usize> = HashMap::new();

        for i in 0..self.elfs.len()
        {
            if self.elfs[i].move_ok
            {
                let c = cnt.get(&self.elfs[i].pos).unwrap_or(&0);
                cnt.insert(self.elfs[i].pos, *c+1);
                //(*cnt.get_mut(&self.elfs[i].move_to).unwrap_or(&mut 0))+=1;
            }
        }

        println!("h:{:?}",cnt);

        for i in 0..self.elfs.len()
        {
            if self.elfs[i].move_ok
            {
                let c = *cnt.get(&self.elfs[i].pos).unwrap_or(&0);

                if c>1 
                {
                    self.elfs[i].move_ok = false;
                }
                //(*cnt.get_mut(&self.elfs[i].move_to).unwrap_or(&mut 0))+=1;
            }

        }
    }

    fn second_half(&mut self)
    {
        for e in self.elfs.iter_mut()
        {
            if e.move_ok && !e.nothing
            {
                e.pos = e.move_to;
            }
        }       

        for e in self.elfs.iter_mut()
        {
            if !e.nothing
            {
                e.shift_moves();
            }
        }
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
            _ => panic!("wrong dir")
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
                    elfs.push(Elf::new(pos, id));
                                        
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
        for y in -2..=yy as i64
        {
            for x in -2..=xx as i64
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
        self.print(11,11);

        for i in 0..5
        {
            self.moving();
            println!("round {}",i);
            self.print(11,11);
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


#[test]
fn test2()
{
    let v = vec![
                        ".....".to_string(),
                        "..##.".to_string(),
                        "..#..".to_string(),
                        ".....".to_string(),
                        "..##.".to_string(),
                        ".....".to_string(),
                        ];
    assert_eq!(part1(&v),10);
}
