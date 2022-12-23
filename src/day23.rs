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
        let         t = self.moves[0];
        self.moves[0] = self.moves[1];
        self.moves[1] = self.moves[2];
        self.moves[2] = self.moves[3];
        self.moves[3] = t;

        /*
        if self.moves[0]==c || self.moves[0]==' ' 
        {
            let         t = self.moves[0];
            self.moves[0] = self.moves[1];
            self.moves[1] = self.moves[2];
            self.moves[2] = self.moves[3];
            self.moves[3] = t;
        }
        else if self.moves[1]==c
        {
            let         t = self.moves[1];
            self.moves[1] = self.moves[2];
            self.moves[2] = self.moves[3];
            self.moves[3] = t;
        }
        else if self.moves[2]==c
        {
            let         t = self.moves[2];
            self.moves[2] = self.moves[3];
            self.moves[3] = t;
        }
        else if self.moves[3]==c
        {
            let         t = self.moves[3];
            self.moves[3] = t;
        }
         */
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
        let mut res = 0;
        for v in p.around8()
        {
            if self.is_empty(v)
            {
                res+=1;
            }
        }
        8-res
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

            let num = self.neighbours(self.elfs[i].pos);

            //println!("id={} {}",i,num);
            if num==0
            {
                self.elfs[i].move_ok = false;
                self.elfs[i].nothing = true;
            }
        }

        for i in 0..self.elfs.len()
        {
            //self.elfs[i].first   = self.elfs[i].moves[0];

            if self.elfs[i].move_ok
            {
                let mut good = false;

                for m in self.moves.clone().iter()
                {
                    let mo = *m;
                    //self.elfs[i].first   = mo;

                    if self.good_move(self.elfs[i].pos, *m)
                    {
                        self.elfs[i].move_to = Self::move_to(self.elfs[i].pos, mo);    
                        //println!("elf:{} move:{}",self.elfs[i].id,mo);
                        good = true;
                        break;
                    }
                }

                if !good
                {
                    self.elfs[i].move_to = Vec2::new(60000,60000);
                    self.elfs[i].move_ok = false;
                }
            }
        }

       // println!("{:#?}",self.elfs);

        let mut cnt:HashMap<Vec2,usize> = HashMap::new();

        for i in 0..self.elfs.len()
        {
            if self.elfs[i].move_ok
            {
                let c = cnt.get(&self.elfs[i].move_to).unwrap_or(&0);
                cnt.insert(self.elfs[i].move_to, *c+1);
                //(*cnt.get_mut(&self.elfs[i].move_to).unwrap_or(&mut 0))+=1;
            }
        }

        //println!("h:{:?}",cnt);

        for i in 0..self.elfs.len()
        {
            if self.elfs[i].move_ok
            {
                let c = *cnt.get(&self.elfs[i].move_to).unwrap_or(&0);

                if c>1 
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
       // let res = 
        self.second_half()
//        self.update();


       // res
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
  
    fn compute(&mut self,rounds:usize)->usize
    {    
        let mut res = true;
        let mut id  = 0;

        //self.print(s,s);
        println!("rr {}",rounds);
        for i in 0..rounds
        {            
            //println!("i:{}",i);
            id+=1;
            res = self.moving();
            println!("round:{} move:{} {}",id,res,rounds);
            if !res && rounds==100_000 { return id;}
           // println!("round {}",id);
            //self.print(s,s);
        }
        self.count_empty()
 
    }
}

pub fn part1(data:&[String],rounds:usize)->usize
{
    World::new(data,false).compute(rounds)
}

pub fn part2(data:&[String],rounds:usize)->usize
{
    World::new(data,false).compute(100000)
}

//< 15231
//< 15230


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

#[test]
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


