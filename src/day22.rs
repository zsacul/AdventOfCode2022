use std::collections::HashMap;
use super::vec2::Vec2;

#[derive( Eq, PartialEq, Debug,  Clone)]
struct World
{
    field    : Vec<Vec<char>>,
    path     : String,
    size     : Vec2,
    start    : Vec2,
    pos      : Vec2,
    dir      : u8,
    teleport : HashMap<Vec2,(Vec2,u8)>,
    n        : usize,
    part2    : bool,
}

impl World {
    
    fn get_point(&self,x:usize,y:usize)->Vec2
    {
        Vec2::new( (x*self.n) as i64, (y*self.n) as i64)
    }
 
    //0 => Vec2::new( 1, 0),
    //1 => Vec2::new( 0, 1),
    //2 => Vec2::new(-1, 0),
    //3 => Vec2::new( 0,-1),
    fn get_d1(&self,c:char)->u8
    {
        match c
        {
            'r' => { 2 },
            'd' => { 3 },
            'l' => { 0 },
            'u' => { 1 },
            _ => { panic!("wrong code"); },
        }
    }

    fn get_line(&self,x0:usize,y0:usize,x1:usize,y1:usize,c:char)->(Vec2,Vec2)
    {
        let mut s = self.get_point(x0,y0);
        let mut e = self.get_point(x1,y1);

        if c=='l'
        {
            if s.y>e.y
            {
                s = s.add(-1,-1);
                e = e.add(-1,-1);  
            }
              else
            {
                s = s.add(-1,0);
                e = e.add(-1,0);  
            }
        }
        else if c=='u'
        {
            if s.x>e.x
            {
                s = s.add(-1,-1);
                e = e.add(-1,-1);
            }
              else
            {
                s = s.add(0,-1);
                e = e.add(0,-1);
            }
        }
        else if c=='d'
        {
            if s.x>e.x
            {
                s = s.add(-1,-1);
                e = e.add(-1,-1);
            }
        }
        else if c=='r'
        {
            if s.y>e.y
            {
                s = s.add(0,-1);
                e = e.add(0,-1);   
            }
        }
        else
        {
            panic!("wrong code");
        }
        (s,e)
    }

    #[allow(clippy::too_many_arguments)]
    fn draw(&mut self,sx0:usize,sy0:usize,sx1:usize,sy1:usize,c1:char,ex0:usize,ey0:usize,ex1:usize,ey1:usize,c2:char)
    {
        let (sp1,ep1) = self.get_line(sx0,sy0,sx1,sy1,c1);
        let (sp2,ep2) = self.get_line(ex0,ey0,ex1,ey1,c2);
      
        let delta_p1 = ep1.subv(sp1).signum();
        let delta_p2 = ep2.subv(sp2).signum();
    
        let n = sp1.distance2(&ep1);
 
        let mut pp1 = sp1;
        let mut pp2 = sp2;

        for _ in 0..n
        {
            self.teleport.insert(pp1, (pp2,self.get_d1(c2)) );
            self.teleport.insert(pp2, (pp1,self.get_d1(c1)) );

            pp1 = pp1.addv(delta_p1);
            pp2 = pp2.addv(delta_p2);
        }    
    }

    fn prepare_teleport(&mut self)
    {   
        self.draw(1,0,2,0,'u',0,3,0,4,'l'); //5
        self.draw(1,1,1,0,'l',0,2,0,3,'l'); //3
        self.draw(2,0,3,0,'u',0,4,1,4,'d'); //6
        self.draw(3,0,3,1,'r',2,3,2,2,'r'); //7
        self.draw(1,1,1,2,'l',0,2,1,2,'u'); //2
        self.draw(2,1,2,2,'r',2,1,3,1,'d'); //4
        self.draw(1,3,2,3,'d',1,3,1,4,'r'); //1
    }

    fn new(data:&[String],part2:bool)->Self
    {
        let mut size  = Vec2::newu(data[0].len(), data.len()-2);       
        let mut start = Vec2::zero();
        let mut count = 0usize;
    
        for (py, line) in data.iter().enumerate()
        {
            if py<size.y as usize
            {                
                for (px ,c) in line.chars().enumerate()
                {                    
                    size.x = size.x.max(px as i64 + 1);

                    if c!=' ' { count+=1; }
                    
                    if c=='.' && start==Vec2::zero() 
                    {
                        start = Vec2::new(px as i64,py as i64);
                    }
                }
            }
        }
    
        let mut field = vec![vec![' ';size.x as usize];size.y as usize];

        for (py, line) in data.iter().enumerate()
        {
            if py<size.y as usize
            {
                for (px ,c) in line.chars().enumerate()
                {                    
                    field[py][px] = c;
                    size.x = size.x.max(px as i64 + 1)
                }
            }
        }
    
        let path = data[(size.y+2-1) as usize].clone();

        Self {
            field,
            path,
            size,
            start,
            pos     : start,
            dir     : 0,
            teleport: HashMap::new(),
            n       : f64::sqrt(count as f64/6.0f64) as usize,
            part2
        }
    }

    fn right(&mut self)
    {
        self.dir = (self.dir+1)%4;
    }

    fn left(&mut self)
    {
        self.dir = (self.dir+3)%4;
    }
    
    //0 right
    //1 down
    //2 left
    //3 up
    fn mark(&self)->char
    {
        match self.dir 
        {
            0 => '>',
            1 => 'v',
            2 => '<',
            3 => '^',
            _ => {panic!("wrong rotation"); },
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

    fn get(&self,p:Vec2)->char
    {
        if !self.pos_okv(&p) { return ' '; }
        self.field[p.y as usize][p.x as usize]
    }

    fn get_path(p:String)->Vec<String>
    {
         p.replace('R',"R ")
          .replace('L',"L ")       
          .split(' ')
          .map(|s| s.to_string())
          .collect()
    }

    fn wrap1(&self,n:Vec2)->(Vec2,u8)
    {        
        let     offs = self.move_op();
        let mut prev = n;
        let mut p    = n.addv(offs);

        while self.get(p)!=' '
        {
            prev = p;
            p    = p.addv(offs);
        }

        (prev,self.dir)
    }

    fn wrap2(&self,n:Vec2)->(Vec2,u8)
    {        
        let o = self.teleport.get(&n);

        if o.is_none()
        {
            println!("{} {}",n.x,n.y);
            println!("{} {}",n.x/self.n as i64,n.y/self.n as i64);
            println!("{} {}",n.x%self.n as i64,n.y%self.n as i64);
            panic!("no teleport!")
        }
        let (pos,dir) =  *o.unwrap();
        (pos.addv(self.movedir(dir)),dir)
    }

    fn next_pos(&mut self)->Option<(Vec2,u8)>
    {
        let n = self.pos.addv(self.moved());
        
        if self.get(n)!=' ' && self.get(n)!='#' 
        {
            return Some((n,self.dir));
        }
        
        if self.get(n)==' '
        {
            let w = if !self.part2 { self.wrap1(n) } else { self.wrap2(n) };

            if self.get(w.0)!=' ' && self.get(w.0)!='#' 
            {                
                return Some(w);
            }
              else 
            {
                return None;
            }
        }
        None
    }

    fn forward(&mut self,n:usize)
    {
        for _ in 0..n
        {
            let np = self.next_pos();
            if np.is_none() { break; }
            let nv = np.unwrap();
           
            self.pos = nv.0;
            self.dir = nv.1;
            self.field[self.pos.y as usize][self.pos.x as usize] = self.mark();
        }
    }

    fn forwards(&mut self,s:&str)
    {
        let n = s.parse::<usize>().unwrap();
        self.forward(n);
    }

    fn compute(&mut self)->i64
    {    
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
    }
}

pub fn part1(data:&[String])->i64
{
    World::new(data,false).compute()
}

pub fn part2(data:&[String])->i64
{
   let mut w = World::new(data,true);
   w.prepare_teleport();
   w.compute()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day 22");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "        ...#".to_string(),
        "        .#..".to_string(),
        "        #...".to_string(),
        "        ....".to_string(),
        "...#.......#".to_string(),
        "........#...".to_string(),
        "..#....#....".to_string(),
        "..........#.".to_string(),
        "        ...#....".to_string(),
        "        .....#..".to_string(),
        "        .#......".to_string(),
        "        ......#.".to_string(),
        "".to_string(),
        "10R5L5R10L4R5L5".to_string(),
    ];
    assert_eq!(part1(&v),6032);
}
