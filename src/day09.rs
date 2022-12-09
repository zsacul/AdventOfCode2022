use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Vec2{
    x: i64,
    y: i64,
}

impl Vec2 {
    fn new(x:i64,y:i64)->Vec2 {
        Self {
            x,
            y
        }
    }
}
struct Range{
    head : Vec2,
    tail : Vec2,
    pos  : HashSet<(i64,i64)>,
}

impl Range {
    fn new()->Self {
        Self {
            head: Vec2::new(0,0),
            tail: Vec2::new(0,0),
            pos : HashSet::new(),
        }
    }

    fn move_tail(&mut self)
    {
        let dx = self.head.x-self.tail.x;
        let dy = self.head.y-self.tail.y;

        if dx==0 || dy==0
        {
            if dx>0 { self.tail.x+=1 }
            if dx<0 { self.tail.x-=1 }
            if dy>0 { self.tail.y+=1 }
            if dy<0 { self.tail.y-=1 }
        }
        else
        {
            if dx>0 { self.tail.x+=1 }
            if dx<0 { self.tail.x-=1 }
            if dy>0 { self.tail.y+=1 }
            if dy<0 { self.tail.y-=1 }
        }
        
    }

    fn touches(&self)->bool
    {
        let dx = self.head.x-self.tail.x;
        let dy = self.head.y-self.tail.y;
        !(dx.abs()>1 || dy.abs() >1)
//        dx.abs() + dy.abs() <=1
    }

    fn moveme(&mut self,c:char,moves:usize)
    {
        //let movesp = [(-1, 0),( 1, 0),( 0,-1),( 0, 1)];
        self.pos.insert( (self.tail.x,self.tail.y) );
        
        println!("{} {}",c,moves);

        for i in 0..moves
        {
            match  c  {
                'L' => { self.head.x-=1; },
                'R' => { self.head.x+=1; },
                'U' => { self.head.y-=1; },
                'D' => { self.head.y+=1; },
                _   => { panic!("wrong code")},
            }
        }

        while (!self.touches())
        {
            self.move_tail();            
            self.pos.insert( (self.tail.x,self.tail.y) );
        }
           
        //while (!self.touches()) {
             
        //}
        
    }

    //<6636
    //1856
    fn movestr(&mut self,s:&str)
    {
//        let positions = [( 0, 0),( 1,-1),( 0,-1),( 0, 1),(-1, 1),( 1, 0),(-1, 0)];
        let tab   : Vec<&str> =  s.split(' ').collect(); 
        let dir = tab[0].chars().next().unwrap();
        let moves = tab[1].parse::<usize>().unwrap();
        self.moveme(dir,moves);

    }

    fn print(&self)
    {
        for y in -10..5 {
            for x in -4..15 {
                let v = self.pos.contains(&(x,y));

                if v 
                {
                    print!("#");
                }
                else
                {
                    print!(".");
                }
                
            }
            println!("");
            
        }
    }

    fn count(&self)->usize
    {
        self.pos.len()
    }
}



pub fn part1(data:&[String])->i32
{
    let mut r = Range::new();    
    for s in data.iter() {
        r.movestr(&s[..]);  
    } 
    r.print();
    r.count() as i32

      //         .filter(|r| r.contains())
        //       .count() as i32
}

pub fn part2(data:&[String])->i32
{
    let mut r = Range::new();    
    data.iter().map(|s| r.movestr(&s[..]));
    
    r.count() as i32
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
    ];
    assert_eq!(part2(&v),4);
}
