use super::vec2::Vec2;
use super::tools;

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

    //0 right
    //1 down
    //2 left
    //3 up

    fn right(&mut self)
    {
        self.dir = (self.dir+1)%4;
    }

    fn left(&mut self)
    {
        self.dir = (self.dir+3)%4;
    }

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
            1 => Vec2::new( 0,-1),
            2 => Vec2::new(-1, 0),
            3 => Vec2::new( 0, 1),
            _ => {panic!("wrong rotation"); },
        }
    }

    fn get_final_code(&self)->i32
    {
        (1000*(self.pos.y+1) + (self.pos.x+1) + self.dir as i64) as i32
    }

    fn pos_ok(&self,x:i32,y:i32)->bool
    {
        !(x<0 || y<0 || x>=self.size.x as i32 || y>=self.size.y as i32)
    }

    fn pos_okv(&self,p: &Vec2)->bool
    {
        self.pos_ok(p.x as i32,p.y as i32)
    }

    fn val(&self,x:usize,y:usize)->char
    {
        self.field[y][x]
    }


    fn get_path(p:String)->Vec<String>
    {
        
        let p = p.replace('R'," R");
        let p = p.replace('L'," L");
        //let r =  p.split(' ').collect();
        let tab : Vec<String> = p.split(' ')
                                 .map(|s| s.to_string())
                                 .collect(); 

        println!("[{:?}]",tab);
        tab
    }

    fn forward(&mut self,n:usize)
    {
        //self.field
        for _ in 0..n 
        {
            self.field[pos.y as usize][pos.x as usize] = self.mark();
        }
    }

    fn compute(&mut self)->i32
    {    
        let moves = Self::get_path(self.path.clone());

        for m in moves.iter() {

            match m.chars().next().unwrap() 
            {
                'L' => self.dir+=1,
                'R' => self.dir-=1,
                 _  => {},
            }
        }

     // println!{"{:?}",self.field};
        println!{"{}",self.path};
        self.get_final_code()
    }
}



pub fn part1(data:&[String])->i32
{
    World::new(data).compute()
    //compute(&World::new(data),usize::MAX,usize::MAX)
}

pub fn part2(data:&[String])->i32
{
    let hills = World::new(data);
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
    assert_eq!(part2(&v),29);
}
