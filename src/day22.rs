use super::vec2::Vec2;

#[derive(Hash, Eq, PartialEq, Debug,  Clone)]
struct World
{
    field : Vec<Vec<char>>,
    path  : String,
    size  : Vec2,
    start : Vec2,
    pos   : Vec2,
    dir   : u8,
}

impl World {
    fn new(data:&[String])->Self
    {
        let mut size = Vec2::newu(data[0].len(), data.len()-2);       
        let mut start = Vec2::zero();
    
        for (py, line) in data.iter().enumerate()
        {
            if py<size.y as usize
            {                
                for (px ,c) in line.chars().enumerate()
                {                    
                    size.x = size.x.max(px as i64 + 1);

                    if c=='.' 
                    {
                        if start==Vec2::zero() 
                        {
                            start = Vec2::new(px as i64,py as i64);
                        }
                    }
                }
            }
        }
    
        let mut field = vec![vec![' ';size.x as usize];size.y as usize];

        for (py, line) in data.iter().enumerate()
        {
            if py<size.y as usize
            {
                println!("{}",line);
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
            pos:start,
            dir:0
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
        match self.dir 
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

    fn print(&self)
    {
        for line in self.field.iter()
        {
            for x in line 
            {
                print!("{}",x);
            }
            println!();
        }
    }

    fn get(&self,p:Vec2)->char
    {
        if !self.pos_okv(&p) {return ' ';}
        self.field[p.y as usize][p.x as usize]
    }

    fn get_path(p:String)->Vec<String>
    {
        let p = p.replace('R',"R ");
        let p = p.replace('L',"L ");

        let tab : Vec<String> = p.split(' ')
                                 .map(|s| s.to_string())
                                 .collect(); 

        println!("[{:?}]",tab);
        tab
    }

    fn warp(&self,n:Vec2)->Vec2
    {
        let v = Vec2::zero();
        let offs = self.move_op();
        let mut p = n;
        let mut prev = p;

        p = p.addv(offs);

        while self.get(p)!=' '
        {
            prev = p;
            p = p.addv(offs)
        }

        prev
    }

    fn next_pos(&self)->Option<Vec2>
    {
        let n = self.pos.addv(self.moved());
        
        if self.get(n)!=' ' && self.get(n)!='#' 
        {
            return Some(n);
        }
        
        if self.get(n)==' '
        {
            let w = self.warp(n);

            if self.get(w)!=' ' && self.get(w)!='#' 
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
        println!("for:{}",n);

        for _ in 0..n 
        {
            let np = self.next_pos();
            if np.is_none()
            {
                break;
            }
            let nv = np.unwrap();

            //if self.pos_okv(&nv)
            {
                self.pos = nv;
                self.field[self.pos.y as usize][self.pos.x as usize] = self.mark();
            }

            println!("{},{} d{}",self.pos.x,self.pos.y,self.dir);
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

        for m in moves.iter() {

            match m.chars().last().unwrap()
            {
                'L' => { self.forwards(&m[..m.len()-1]); println!("<");  self.left() },
                'R' => { self.forwards(&m[..m.len()-1]); println!(">");  self.right()},
                 _  => { self.forwards(&m[..         ]); },
            }
            self.field[self.pos.y as usize][self.pos.x as usize] = self.mark();
        }

        //self.print();
        //println!{"{},{} rot:{}",self.pos.x,self.pos.y,self.dir};
        //println!{"{}",self.path};
        self.get_final_code()
    }
}

//> 157142
//  164014

pub fn part1(data:&[String])->i64
{
    World::new(data).compute()
    //compute(&World::new(data),usize::MAX,usize::MAX)
}

pub fn part2(data:&[String])->i32
{
   // let hills = World::new(data);
    0
/*
    tools::get_2d_iter(0,hills.size.x as usize,
                       0,hills.size.y as usize)
                       .iter()
                       .map(|(y,x)|
                       {
                           if hills.val(*x,*y)==0
                           {
                               compute(&hills,*x,*y)
                           }
                               else 
                           {
                               i32::MAX
                           }
                       }
                       )
                       .min()
                       .unwrap()
                        */
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

#[test]
fn test2()
{
    let v = vec![
    ];
    assert_eq!(part2(&v),5031);
}
