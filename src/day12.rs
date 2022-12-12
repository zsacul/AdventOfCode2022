use std::collections::{BinaryHeap};
use super::vec2::Vec2;

#[derive(Debug, PartialEq, Eq,PartialOrd, Ord)]
struct Node
{
    distance : i32,
    p        : Vec2,
}

impl Node
{
    fn new(distance:i32,p:Vec2)->Self
    {
        Self { distance , p }
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
        let size = Vec2::newu(data[0].len(), data.len());       
        let mut start = Vec2::new(0,0);
        let mut end   = Vec2::new(0,0);
        let mut field = vec![vec![0;size.x as usize];size.y as usize];

        for (py, line) in data.iter().enumerate()
        {
            for (px ,c) in line.chars().enumerate()
            {
                field[py][px] = match c 
                                {
                                    'S' => { start = Vec2::newu(px,py);   0                    },
                                    'E' => { end   = Vec2::newu(px,py);   (b'z' - b'a') as i32 },
                                     _  => {                              c as i32 - 'a' as i32},
                                }
            }
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

    fn pos_okv(&self,p: &Vec2)->bool
    {
        self.pos_ok(p.x as i32,p.y as i32)
    }

    fn val(&self,x:usize,y:usize)->i32
    {
        self.field[y][x]
    }

    fn valv(&self,p:&Vec2)->i32
    {
        self.field[p.y as usize][p.x as usize]
    }
}

fn compute(f:&Hills,ssx:usize,ssy:usize)->i32
{    
    let mut dist = vec![vec![9999;f.size.x as usize];f.size.y as usize];
    let fs =    if ssx!=99999 && ssy!=99999 
                {
                    Vec2::newu(ssx,ssy)
                }
                  else
                {
                    Vec2::new(f.start.x,f.start.y)
                };

    dist[fs.y as usize][fs.x as usize] = 0;

    let mut queue : BinaryHeap<Node> = BinaryHeap::new(); 
    queue.push( Node::new(dist[fs.y as usize][fs.x as usize],fs) );

    while !queue.is_empty() 
    {
        let node = queue.pop().unwrap();

        for des in node.p.around4()
        {            
            if f.pos_okv(&des) 
            {
                let (dx,dy) = (des.x as usize,des.y as usize);
                let diff    = f.valv(&des) - f.valv(&node.p);
                let end     = des==f.end;

                if ((!end && diff<=1) || 
                    ( end && f.valv(&node.p)=='z' as i32 - 'a' as i32)) &&
                    dist[dy][dx] > node.distance + 1
                    {
                        dist[des.y as usize][des.x as usize] = node.distance + 1;
                        queue.push(Node::new(node.distance+1,des) );
                    }    
            }
        }
    }
    
    dist[f.end.y as usize][f.end.x as usize]
}

pub fn part1(data:&[String])->i32
{
    compute(&Hills::new(data),99999,99999)
}

pub fn part2(data:&[String])->i32
{
    let hills = Hills::new(data);
    let mut minv = 1000000;

    for yyy in 0..hills.size.y as usize
    {
        for xxx in 0..hills.size.x as usize
        {
            if hills.val(xxx,yyy)==0
            {
                minv = i32::min(minv,compute(&hills,xxx,yyy));
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
