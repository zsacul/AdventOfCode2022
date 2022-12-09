use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Vec2{
    x: i64,
    y: i64,
}

impl Vec2 {
    fn new(x:i64,y:i64)->Vec2 {
        Self { x, y }
    }
}

#[derive(Clone)]
struct Range{
    head : Vec2,
    tail : Vec2,
    pos  : HashSet<(i64,i64)>,
}

impl Range {
    fn new()->Self {
        Self {
            head : Vec2::new(0,0),
            tail : Vec2::new(0,0),
            pos  : HashSet::new(),
        }
    }

    fn move_tail(&mut self)
    {
        self.tail.x+=(self.head.x - self.tail.x).signum();
        self.tail.y+=(self.head.y - self.tail.y).signum();
        
        self.pos.insert( (self.tail.x,self.tail.y) );
    }

    fn touches(&self)->bool
    {
        (self.head.x-self.tail.x).abs()<=1 && 
        (self.head.y-self.tail.y).abs()<=1
    }

    fn movestr(&mut self,s:&str)
    {
        let (dir,moves) = Self::get_moves(s);
        self.move_head(dir,moves);
    }

    fn get_moves(s:&str)->(char,usize)
    {
        let tab : Vec<&str> = s.split(' ').collect(); 
        let dir   = tab[0].chars().next().unwrap();
        let moves = tab[1].parse::<usize>().unwrap();
        (dir,moves)
    }

    fn move_head(&mut self,c:char,moves:usize)
    {
        for _i in 0..moves
        {
            match  c  {
                'L' => { self.head.x-=1;      },
                'R' => { self.head.x+=1;      },
                'U' => { self.head.y-=1;      },
                'D' => { self.head.y+=1;      },
                _   => { panic!("wrong code") },
            }
        }

        let v = self.head;
        self.set_head_pos(&v);
    }

    fn set_head_pos(&mut self,p:&Vec2)
    {
        self.head.x = p.x;
        self.head.y = p.y;
    }

    fn follow(&mut self)->bool
    {
        self.pos.insert( (self.tail.x,self.tail.y) );
        let moved = !self.touches();

        if moved
        {
            self.move_tail();            
        }  
        moved            
    }

    #[allow(unused)]
    fn print(&self)
    {
        for y in -30..35 {
        for x in -24..25 {
                let v = self.pos.contains(&(x,y));

                if v { print!("#"); }
                else { print!("."); }
            }
            println!();
        }
    }

    fn count(&self)->usize
    {
        self.pos.len()
    }
}

pub fn part1(data:&[String])->usize
{
    let mut r = Range::new();

    for s in data.iter() 
    {
        r.movestr(&s[..]);  
        while r.follow() {};
    } 
   
    r.count()
}

#[allow(unused)]
fn debug_print(r:&Vec<Range>)
{
    let n = r.len();
    let s = 16;
    for y in -s..s 
    {
        for x in -s..s 
        {
            let mut was = false;

            for q in 0..n
            {
                for (q, rq) in r.iter().enumerate().take(n)
                {
                    if rq.head.x==x && rq.head.y==y
                    {
                        was = true;
                        if q==0 { print!("H");    }
                           else { print!("{}",q); }
                        break;
                    }    
                }                
            }
            if !was {
                print!(".");
            }
        }
        println!();
    }        
}

pub fn part2(data:&[String])->usize
{
    let n = 9;
    let mut r = vec![Range::new(); n];

    for s in data.iter() 
    {
        let (move_des,count) = Range::get_moves(&s[..]);  

        for _ in 0..count
        {
            r[0].move_head(move_des,1);
            r[0].follow();

            for i in 1..n
            {
                let tail_pos = r[i-1].tail;
                r[i].set_head_pos(&tail_pos);
                r[i].follow();
            }
        }     
    }
    
    r.last().unwrap().count()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day9");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "R 4".to_string(),
        "U 4".to_string(),
        "L 3".to_string(),
        "D 1".to_string(),
        "R 4".to_string(),
        "D 1".to_string(),
        "L 5".to_string(),
        "R 2".to_string(),
    ];
    assert_eq!(part1(&v),13);
}

#[test]
fn test2()
{
    let v = vec![
        "R 5".to_string(),
        "U 8".to_string(),
        "L 8".to_string(),
        "D 3".to_string(),
        "R 17".to_string(),
        "D 10".to_string(),
        "L 25".to_string(),
        "U 20".to_string(),
    ];
    assert_eq!(part2(&v),36);
}

#[test]
fn test2_2()
{
    let v = vec![
        "R 4".to_string(),
        "U 4".to_string(),
        "L 3".to_string(),
        "D 1".to_string(),
        "R 4".to_string(),
        "D 1".to_string(),
        "L 5".to_string(),
        "R 2".to_string(),
    ];
    assert_eq!(part2(&v),1);
}

#[test]
fn test2_3()
{
    let v = vec![
        "R 5".to_string(),
        "U 8".to_string(),
        "L 8".to_string(),
        "D 3".to_string(),
        "R 17".to_string(),
        "D 10".to_string(),
        "L 25".to_string(),
        "U 20".to_string(),
    ];
    assert_eq!(part2(&v),36);
}
