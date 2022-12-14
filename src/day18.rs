use std::collections::{HashSet,HashMap};

#[derive(Eq, PartialEq, Debug, Clone,Hash)]
struct Voxel{
    x : i8,
    y : i8,
    z : i8,
}

impl Voxel {
    fn new(x:i8,y:i8,z:i8)->Self
    {
        Self 
        {
            x,y,z
        }
    }

    fn from_v(v:&Voxel)->Self
    {
        Self {
            x : v.x,
            y : v.y,
            z : v.z
        }
    }
}

struct Space{
    lavas   : HashSet<Voxel>,
    offs    : Vec<Vec<i8>>,
    visited : HashSet<Voxel>,
    counted : HashMap<Voxel,u8>
}

impl Space {
    fn new(data:&[String])->Self
    {
        Self 
        {
            lavas : data.iter().map(|line |
                                    {
                                        let tab : Vec<&str> = line.split(',').collect(); 
                                        let x = tab[0].parse::<i8>().unwrap();
                                        let y = tab[1].parse::<i8>().unwrap();
                                        let z = tab[2].parse::<i8>().unwrap();
                                        Voxel::new(x,y,z)                                                                                
                                    }
                                    ).collect::<HashSet<Voxel>>(),              
            offs  : vec![
                            vec![-1, 0, 0],
                            vec![ 1, 0, 0],
                            vec![ 0,-1, 0],
                            vec![ 0, 1, 0],
                            vec![ 0, 0,-1],
                            vec![ 0, 0, 1],
                        ],
            visited : HashSet::new(),
            counted : HashMap::new(),
        }
    }

    fn count(&self)->usize
    {
        let substract : usize = self.lavas.iter().map( |v| 
            self.offs.iter().map( |off|
                self.lavas.get(&Voxel::new(v.x + off[0],
                                           v.y + off[1],
                                           v.z + off[2])).is_some() as usize
            ).sum::<usize>()
        ).sum();
        
        6*self.lavas.len() - substract
    }    

    fn pos_ok(p:&Voxel)->bool
    {
         !(p.x < -1 || p.y < -1 || p.z < -1 ||
           p.x > 20 || p.y > 20 || p.z > 20)
    }

    fn flood(&mut self,p:Voxel,code:u8)->u16
    {       
        let mut stack = vec![(p,code)];
        let mut res = 0;

        while !stack.is_empty()
        {
            let (p,code) = stack.pop().unwrap();

            if !Self::pos_ok(&p) || self.visited.get(&p).is_some()
            {
            }
            else if self.lavas.get(&p).is_some()
            {
                let code_stored = *self.counted.get(&p).unwrap_or(&0);
                self.counted.insert(Voxel::from_v(&p), code | code_stored);
                res+=(code_stored & code==0) as u16;
            }
              else
            {
                self.visited.insert(Voxel::from_v(&p));
    
                stack.push((Voxel::new(p.x+1,p.y  ,p.z  ), 1<<0));
                stack.push((Voxel::new(p.x-1,p.y  ,p.z  ), 1<<1));
                stack.push((Voxel::new(p.x  ,p.y+1,p.z  ), 1<<2));
                stack.push((Voxel::new(p.x  ,p.y-1,p.z  ), 1<<3));
                stack.push((Voxel::new(p.x  ,p.y  ,p.z+1), 1<<4));
                stack.push((Voxel::new(p.x  ,p.y  ,p.z-1), 1<<5));
            }         
        }
        res

    }

    fn count2(&mut self)->usize
    {
        self.flood(Voxel::new(0,0,0),0) as usize
    }
}

pub fn part1(data:&[String])->usize
{
     Space::new(data).count()   
}

pub fn part2(data:&[String])->usize
{
    Space::new(data).count2()
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
        assert_eq!(part2(&v),58);
    }
    
    #[test]
    fn test1_2()
    {
        let v = vec![   
            "1,1,1".to_string(),
            "2,1,1".to_string(),
            "3,1,1".to_string(),
            "4,1,1".to_string(),
            "5,1,1".to_string(),
            "6,1,1".to_string(),
            "1,2,1".to_string(),
            "2,2,1".to_string(),
            "3,2,1".to_string(),
            "4,2,1".to_string(),
            "5,2,1".to_string(),
            "6,2,1".to_string(),
            "1,3,1".to_string(),
            "2,3,1".to_string(),
            "3,3,1".to_string(),
            "4,3,1".to_string(),
            "5,3,1".to_string(),
            "6,3,1".to_string(),
            "1,1,2".to_string(),
            "2,1,2".to_string(),
            "3,1,2".to_string(),
            "4,1,2".to_string(),
            "5,1,2".to_string(),
            "6,1,2".to_string(),
            "1,2,2".to_string(),
            "6,2,2".to_string(),
            "1,3,2".to_string(),
            "2,3,2".to_string(),
            "3,3,2".to_string(),
            "4,3,2".to_string(),
            "5,3,2".to_string(),
            "6,3,2".to_string(),
            "1,1,3".to_string(),
            "2,1,3".to_string(),
            "3,1,3".to_string(),
            "4,1,3".to_string(),
            "5,1,3".to_string(),
            "6,1,3".to_string(),
            "1,2,3".to_string(),
            "2,2,3".to_string(),
            "3,2,3".to_string(),
            "4,2,3".to_string(),
            "5,2,3".to_string(),
            "6,2,3".to_string(),
            "1,3,3".to_string(),
            "2,3,3".to_string(),
            "3,3,3".to_string(),
            "4,3,3".to_string(),
            "5,3,3".to_string(),
            "6,3,3".to_string(),
        ];
        assert_eq!(part1(&v),108);
}

    
#[test]
fn test2_2()
{
    let v = vec![   
    "1,1,1".to_string(),
    "2,1,1".to_string(),
    "3,1,1".to_string(),
    "4,1,1".to_string(),
    "5,1,1".to_string(),
    "6,1,1".to_string(),
    "1,2,1".to_string(),
    "2,2,1".to_string(),
    "3,2,1".to_string(),
    "4,2,1".to_string(),
    "5,2,1".to_string(),
    "6,2,1".to_string(),
    "1,3,1".to_string(),
    "2,3,1".to_string(),
    "3,3,1".to_string(),
    "4,3,1".to_string(),
    "5,3,1".to_string(),
    "6,3,1".to_string(),
    "1,1,2".to_string(),
    "2,1,2".to_string(),
    "3,1,2".to_string(),
    "4,1,2".to_string(),
    "5,1,2".to_string(),
    "6,1,2".to_string(),
    "1,2,2".to_string(),
    "6,2,2".to_string(),
    "1,3,2".to_string(),
    "2,3,2".to_string(),
    "3,3,2".to_string(),
    "4,3,2".to_string(),
    "5,3,2".to_string(),
    "6,3,2".to_string(),
    "1,1,3".to_string(),
    "2,1,3".to_string(),
    "3,1,3".to_string(),
    "4,1,3".to_string(),
    "5,1,3".to_string(),
    "6,1,3".to_string(),
    "1,2,3".to_string(),
    "2,2,3".to_string(),
    "3,2,3".to_string(),
    "4,2,3".to_string(),
    "5,2,3".to_string(),
    "6,2,3".to_string(),
    "1,3,3".to_string(),
    "2,3,3".to_string(),
    "3,3,3".to_string(),
    "4,3,3".to_string(),
    "5,3,3".to_string(),
    "6,3,3".to_string(),
    ];
    assert_eq!(part2(&v),90);
}


#[test]
fn test2_3()
{
    let v = vec![   
    "3,3,3".to_string(),
    ];
    assert_eq!(part2(&v),6);
}

#[test]
fn test2_4()
{
    let v = vec![   
        "3,3,3".to_string(),
        "4,3,3".to_string(),
    ];
    assert_eq!(part2(&v),10);
}

#[test]
fn test2_5()
{
    let v = vec![   
        "3,3,3".to_string(),
        "4,3,3".to_string(),
        "5,3,3".to_string(),
    ];
    assert_eq!(part2(&v),14);
}

#[test]
fn test2_6()
{
    let v = vec![   
        "0,1,1".to_string(),
        "2,1,1".to_string(),
        "1,0,1".to_string(),
        "1,2,1".to_string(),
        "1,1,0".to_string(),
        "1,1,2".to_string(),
    ];
    assert_eq!(part2(&v),30);
}

