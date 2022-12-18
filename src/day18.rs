use std::collections::HashMap;
use super::tools;

#[derive(Eq, PartialEq,  Debug, Clone,Hash)]
struct Voxel{
    x : i32,
    y : i32,
    z : i32,
}

impl Voxel {
    fn new(x:i32,y:i32,z:i32)->Self
    {
        Self {
            x,y,z
        }
    }
}

struct Space{
    vox : Vec<Voxel>,
    s   : HashMap<Voxel,bool>
}

impl Space {
    fn new(data:&[String])->Self
    {
        Self 
        {
            vox : data.iter().map(|line |
                {
                    let tab : Vec<&str> = line.split(',').collect(); 
                    let x = tab[0].parse::<i32>().unwrap();
                    let y = tab[1].parse::<i32>().unwrap();
                    let z = tab[2].parse::<i32>().unwrap();
                    Voxel::new(x,y,z)
                }
            ).collect::<Vec<Voxel>>(),              
            s: HashMap::new(),
        }
    }

    fn count(&mut self)->i32 
    {
        let mut res = self.vox.len() as i32*6;

        let offs = vec![
            vec![-1, 0, 0],
            vec![ 1, 0, 0],
            vec![ 0,-1, 0],
            vec![ 0, 1, 0],
            vec![ 0, 0,-1],
            vec![ 0, 0, 1],
        ];

        for v in self.vox.iter() 
        {
            self.s.insert(Voxel::new(v.x,v.y,v.z),true);
        }        

        for v in self.vox.iter() 
        {
            for off in offs.iter()
            {
                let voxel = Voxel::new(v.x+off[0],
                                       v.y+off[1],
                                       v.z+off[2]);

                if self.s.get(&voxel).is_some()
                {
                    res-=1;
                }
            }
            
        }
        res
    }
    
}


pub fn part1(data:&[String])->i32
{
     Space::new(data).count()
    
    
}

pub fn part2(data:&[String])->i32
{
    0
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day 18");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "2,2,2".to_string(),
        "1,2,2".to_string(),
        "3,2,2".to_string(),
        "2,1,2".to_string(),
        "2,3,2".to_string(),
        "2,2,1".to_string(),
        "2,2,3".to_string(),
        "2,2,4".to_string(),
        "2,2,6".to_string(),
        "1,2,5".to_string(),
        "3,2,5".to_string(),
        "2,1,5".to_string(),
        "2,3,5".to_string(),
    ];
    assert_eq!(part1(&v),64);
}

#[test]
fn test1_1()
{
    let v = vec![
        "2,0,0".to_string(),
        "1,0,0".to_string(),
    ];
    assert_eq!(part1(&v),10);
}

#[test]
fn test2()
{
    let v = vec![
    ];
    assert_eq!(part2(&v),4);
}
