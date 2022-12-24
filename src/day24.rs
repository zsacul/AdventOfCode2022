use std::collections::HashMap;
use std::collections::HashSet;
use crate::tools::usize_get_between;

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
    size  : Vec2,
    time  : usize,
    rec   : HashMap<Vec2,usize>,
    shot  : Vec<HashMap<Vec2,char>>
}

impl World 
{
    fn next_pos(&self,p:Vec2,c:char)->Vec2
    {
        let (mx,my) = (self.size.x-2,self.size.y-2);
        let n =
        match c {
            '^'=> { 
                Vec2::new(p.x               ,(p.y-1+my-1)%my+1)                
            },
            'v'=> { 
                Vec2::new(p.x               ,(p.y-1+1   )%my+1)
            },
            '<'=> { 
                Vec2::new((p.x-1+mx-1 )%mx+1,p.y              )
            },
            '>'=> { 
                Vec2::new((p.x-1+1    )%mx+1,p.y              )
            },
            _ => panic!("wrong dir {}",c)
        };
        n
    }

    fn update(&mut self)
    {
        self.field.clear();
        for i in 0..self.bliz.len()
        {
            self.bliz[i].position = self.next_pos(self.bliz[i].position.clone(),self.bliz[i].dir);
            self.field.insert(self.bliz[i].position,self.bliz[i].dir);
        }       

        for x in 0..self.size.x as usize
        {
            self.field.insert(Vec2::new(x as i64,0),'#');
            self.field.insert(Vec2::new(x as i64,self.size.y-1),'#');
        }
        for y in 0..self.size.y as usize
        {
            self.field.insert(Vec2::new(0            ,y as i64),'#');
            self.field.insert(Vec2::new(self.size.x-1,y as i64),'#');
        }
        self.field.insert(Vec2::new(1,0),'.');
        self.field.insert(Vec2::new(self.size.x-2,self.size.y-1),'.');
        


        self.time+=1; 
    }
 
    fn new(data:&[String],time:usize)->Self
    {
        let mut field = HashMap::new();
        let mut bliz  = vec![];
        let size = Vec2::new(data[0].len() as i64,data.len() as i64);
    
        for (py, line) in data.iter().enumerate()
        {
            for (px ,c) in line.chars().enumerate()
            {         
                //if c!='#' && c!='.'
                //{
                    let position = Vec2::new(px as i64,py as i64);
                    if c!='.' && c!='#'
                    {
                        bliz.push(Blizzard::new(position,c));
                    }
                    if c=='#' { field.insert(position,'#'); }
                         else { field.insert(position,'.'); }
                //}
            }
        }
        let mut rec = HashMap::new();
    
        Self {
            bliz,
            field,            
            size,
            time,
            rec,
            shot: vec![]
        }
    }

    #[allow(unused)]
    fn print(&self,xx:usize,yy:usize)
    {
        println!("Minute {}",self.time);

        for y in 0..=yy as i64
        {
            for x in 0..=xx as i64
            {
                let mut c = '.';
                for b in self.bliz.iter()
                {
                    if x==b.position.x && y==b.position.y
                    {
                        c = b.dir;
                    }
                }

                if x==0 || x==xx as i64 || y==0 || y==yy as i64 
                {
                    print!("#");
                }
                  else
                {
                    let cc =  *self.field.get(&Vec2::new(x,y)).unwrap_or(&'.');
                    print!("{}",cc);
                }
            }
            println!();
        }
    }

    
    #[allow(unused)]
    fn printh(&self,t:usize,f:HashMap<Vec2,char>)
    {
        let xx = self.size.x;
        let yy = self.size.y;        

        for y in 0..yy as i64
        {
            for x in 0..xx as i64
            {
                let cc =  *f.get(&Vec2::new(x,y)).unwrap_or(&'.');
                print!("{}",cc);
            }
            println!();
        }
    }
  
    fn compute1(&mut self,rounds:usize)->usize
    {    
        let least = 77100;//123*30+20;

        
        //73820
        self.shot.push(self.field.clone());

        for id in 0..least
        {                        
            self.update();
            self.shot.push(self.field.clone());
        }

        //self.printh(63000,self.shot[63000].clone());

        for i in 0..least
        {
          //  println!("rev {}",i);
         //   self.printh(i,self.shot[i].clone());
        }

        let mut memo = HashMap::new();
        self.dfs(&mut memo,Vec2::new(1,1),1)
    }

    fn dfs(&self,memo:&mut HashMap<(Vec2,usize),usize>,pos:Vec2,time:usize)->usize
    {
        let key = (pos,time);

        if memo.get(&key).is_some()
        {
            return *memo.get(&key).unwrap();
        }
        if pos.x==self.size.x-2 && pos.y==self.size.y-1
        {
            return time;
        }

        if pos.x==1 && pos.y==0
        {
            return usize::MAX;
        }

        if time+1>self.shot.len()
        {
            return usize::MAX;
        }

        if *self.shot[time].get(&pos).unwrap_or(&'.')!='.'
        {
            return usize::MAX;
        }
        let mut res = usize::MAX;
        res = res.min(self.dfs(memo,Vec2::new(pos.x+1,pos.y  ), time+1));
        res = res.min(self.dfs(memo,Vec2::new(pos.x  ,pos.y+1), time+1));
        res = res.min(self.dfs(memo,Vec2::new(pos.x-1,pos.y  ), time+1));
        res = res.min(self.dfs(memo,Vec2::new(pos.x  ,pos.y-1), time+1));
        res = res.min(self.dfs(memo,Vec2::new(pos.x  ,pos.y  ), time+1));
        memo.insert(key,res);
        res
    }

    fn compute2(&mut self)->usize
    {    
        //for id in 1..usize::MAX
        //{                        
          //  if !self.moving() { return id; }
        //}
        0
    }

}

fn part1(data:&[String],rounds:usize)->usize
{
    World::new(data,0).compute1(rounds)
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

/*
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
    assert_eq!(part1(&v,10),18);
}
 */

#[test]
fn test1()
{
    let v = vec![
        "#.######".to_string(),
        "#>>.<^<#".to_string(),
        "#.<..<<#".to_string(),
        "#>v.><>#".to_string(),
        "#<^v^^>#".to_string(),
        "######.#".to_string(),
    ];
    assert_eq!(part1(&v,10),18);
}

#[test]
fn test2()
{
    let v = vec![
        "#E######".to_string(),
        "#>>.<^<#".to_string(),
        "#.<..<<#".to_string(),
        "#>v.><>#".to_string(),
        "#<^v^^>#".to_string(),
        "######.#".to_string(),
    ];
    assert_eq!(part2(&v),20);
}
