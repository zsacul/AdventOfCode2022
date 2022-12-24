use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
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

        self.field.insert(Vec2::new(1,-1),'#');
        self.field.insert(Vec2::new(self.size.x-2,self.size.y),'#');

        


        //self.time+=1; 
    }
 
    fn new(data:&[String])->Self
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
            rec,
            shot: vec![]
        }
    }

    #[allow(unused)]
    fn print(&self,xx:usize,yy:usize)
    {
        //println!("Minute {}",self.time);

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

        for y in -2..yy as i64
        {
            for x in 0..xx as i64
            {
                let cc =  *f.get(&Vec2::new(x,y)).unwrap_or(&'.');
                print!("{}",cc);
            }
            println!();
        }
    }
  
    fn compute1(&mut self)->usize
    {    
        let least = ((self.size.x-2)*(self.size.y-2)) as usize;
        //3710

        //return self.bfs(Vec2::new(1,1),1);
        //return 0;
        //73820

       // self.shot.push(self.field.clone());

        for id in 0..least
        {                        
            self.update();
            self.shot.push(self.field.clone());
        }

        //self.printh(0,self.shot[0].clone());
        //self.printh(1,self.shot[1].clone());
        self.printh(least-1,self.shot[least-1].clone());
        //self.printh(least  ,self.shot[least  ].clone());

        let s = Vec2::new(1,0);
        let e = Vec2::new(self.size.x-2,self.size.y-1);
        
        let mut memo = HashMap::new();
        self.dfs(&mut memo,s,e,1,1000)+1               
    }


    fn compute2(&mut self)->usize
    {    
        let least = ((self.size.x-2)*(self.size.y-2)) as usize;

        for id in 0..least
        {                        
            self.update();
            self.shot.push(self.field.clone());
        }

        //self.printh(0,self.shot[0].clone());
        //self.printh(1,self.shot[1].clone());
        self.printh(least-1,self.shot[least-1].clone());

        let s = Vec2::new(1,0);
        let e = Vec2::new(self.size.x-2,self.size.y-1);
        //self.printh(least  ,self.shot[least  ].clone());
        
        let mut memo = HashMap::new();
        let t1 = self.dfs(&mut memo,s,e,1      ,1000)+1;
        memo.clear();
        let t2 = self.dfs(&mut memo,e,s,1+t1   ,1000)+1-t1;
        memo.clear();
        let t3 = self.dfs(&mut memo,s,e,1+t1+t2,1000)+1-t1-t2;

        println!("1 {}",t1);
        println!("2 {}",t2);
        println!("3 {}",t3);
        t1+t2+t3
        //t2
    }


    fn dfs(&self,memo:&mut HashMap<(Vec2,usize),usize>,pos:Vec2,goal:Vec2,time:usize,lim:usize)->usize
    {
        let key = (pos,time);

        if memo.get(&key).is_some()
        {
            return *memo.get(&key).unwrap();
        }
        if pos.x==goal.x && pos.y==goal.y
        {
            return time;
        }

        //if pos.x==1 && pos.y==0
        //{
          //  return usize::MAX;
        //}
        
        if time+1>lim
        {
            return usize::MAX;
        }

        if *self.shot[time%self.shot.len()].get(&pos).unwrap_or(&'.')!='.'
        {
            return usize::MAX;
        }
        let mut res = usize::MAX;
        res = res.min(self.dfs(memo,Vec2::new(pos.x+1,pos.y  ),goal, time+1,lim));
        res = res.min(self.dfs(memo,Vec2::new(pos.x  ,pos.y+1),goal, time+1,lim));
        res = res.min(self.dfs(memo,Vec2::new(pos.x-1,pos.y  ),goal, time+1,lim));
        res = res.min(self.dfs(memo,Vec2::new(pos.x  ,pos.y-1),goal, time+1,lim));
        res = res.min(self.dfs(memo,Vec2::new(pos.x  ,pos.y  ),goal, time+1,lim));
        memo.insert(key,res);
        res
    }

}

fn part1(data:&[String])->usize
{
    World::new(data).compute1()
}

fn part2(data:&[String])->usize
{
    World::new(data).compute2()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day 24");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
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
    assert_eq!(part1(&v),18);
}

#[test]
fn test2()
{
    let v = vec![
        "#.######".to_string(),
        "#>>.<^<#".to_string(),
        "#.<..<<#".to_string(),
        "#>v.><>#".to_string(),
        "#<^v^^>#".to_string(),
        "######.#".to_string(),
    ];
    assert_eq!(part2(&v),54);
}
