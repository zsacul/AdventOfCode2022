use std::collections::{HashMap,BinaryHeap};


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Node
{
    distance : i32,
           x : i32,
           y : i32,
}

impl Node
{
    fn new(distance:i32,x:i32,y:i32)->Self
    {
        Self { distance,x,y }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Vec2{
    x: i32,
    y: i32,
}

impl Vec2 {
    fn new(x:i32,y:i32)->Vec2 {
        Self { x, y }
    }
}

#[derive(Hash, Eq, PartialEq, Debug,  Clone)]
struct Hills
{
    field: Vec<Vec<i32>>,
    size : Vec2,
    start: Vec2,
    end  : Vec2,    
}

impl Hills {
    fn new(data:&[String])->Self
    {
        let size = Vec2::new(data[0].len() as i32, data.len() as i32);
        let mut y=0;
        let mut start = Vec2::new(0,0);
        let mut end   = Vec2::new(0,0);
        let mut field = vec![vec![0;size.x as usize];size.y as usize];

        for line in data.iter().enumerate()
        {
            for x in line.1.chars().into_iter().enumerate()
            {
                let (px,py) = (x.0 as usize,line.0 as usize);
                let c = x.1;
                               
                if c=='S'
                {
                    start = Vec2::new(px as i32,py as i32);                    
                    field[py][px] = 'a' as i32 - 'a' as i32;
                }
                else if c=='E'
                {
                    end = Vec2::new(px as i32,py as i32);
                    field[py][px] = 'z' as i32 - 'w' as i32;
                }
                else 
                {
                    field[py][px] = c as i32 - 'a' as i32;
                }
            }
            y+=1;
        }
    
        Self {
            field,
            size,
            start,
            end
        }
    
    }

    fn pos_ok(&self,x:i32,y:i32)->bool
    {
          !(x<0 || y<0 || x>=self.size.x as i32 || y>=self.size.y as i32)
    }

    fn val(&self,x:i32,y:i32)->i32
    {
        self.field[y as usize][x as usize]
    }

}



fn compute(f:&mut Hills,ssx:i32,ssy:i32)->i32
{    
    let mut dist = vec![vec![9999;f.size.x as usize];f.size.y as usize];
    
    if ssx!=-1 && ssy!=-1 {
        f.start.x = ssx;
        f.start.y = ssy;
        f.field[f.start.y as usize][f.start.x as usize] = 0;
        //println!("{} x {}",ssy,ssx);
    }
    dist[f.start.y as usize][f.start.x as usize] = 0;

    let mut queue : BinaryHeap<Node> = BinaryHeap::new(); 
    queue.push( Node::new(dist[f.start.y as usize][f.start.x as usize],f.start.x,f.start.y) );

    while !queue.is_empty() 
    {
        let node = queue.pop().unwrap();
        let (distance,px,py) = (node.distance,node.x,node.y);

        let neigh = vec![(px+1,py  ),
                                          (px  ,py+1),
                                          (px-1,py  ),
                                          (px  ,py-1)];

        for (sx,sy) in neigh
        {
            if f.pos_ok(sx,sy) 
            {
                let diff = f.val(sx ,sy) - f.val(px,py);

                let end = sx==f.end.x && sy==f.end.y;

                if (!end && diff<=1) || 
                   ( end && f.val(px,py)=='z' as i32 - 'a' as i32) 
                {    
                    if dist[sy as usize][sx as usize] > (distance as i32 + 1i32)
                    {
                        let distance = distance + 1;
                        dist[sy as usize][sx as usize] = distance;
                        queue.push(Node::new(distance,sx,sy) );
                    }    
                        
                }
            }
        }
    }
    
    dist[f.end.y as usize][f.end.x as usize]
}

pub fn part1(data:&[String])->i32
{
    let mut f = Hills::new(data);
    compute(&mut f,-1,-1)
}

pub fn part2(data:&[String])->i32
{
    let f = Hills::new(data);
    let xx = f.size.x;
    let yy = f.size.y;
    let mut minv = 1000000;

    for yyy in 0..yy
    {
        for xxx in 0..xx 
        {
            if f.val(xxx,yyy)==0
            {
                let mut f2 = Hills::new(data);
                let res = compute(&mut f2,xxx,yyy);
                //println!("{} ",res);
                

                if res<minv
                {
                    minv = res;
                }
            }
        }
    }
    minv

}



#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day 12");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "Sabqponm".to_string(),
        "abcryxxl".to_string(),
        "accszExk".to_string(),
        "acctuvwj".to_string(),
        "abdefghi".to_string(),
    ];
    assert_eq!(part1(&v),31);
}

#[test]
fn test2()
{
    let v = vec![
        "Sabqponm".to_string(),
        "abcryxxl".to_string(),
        "accszExk".to_string(),
        "acctuvwj".to_string(),
        "abdefghi".to_string(),
    ];
    assert_eq!(part2(&v),29);
}


//v..v<<<<
//>v.vv<<^
//.>vv>E^^
//..v>>>^^
//..>>>>>^
//
//
//...v<<<<
//...vv<<^
//...v>E^^
//.>v>>>^^
//>^>>>>>^