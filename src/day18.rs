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
    vox : Vec<Voxel>,
    s   : HashSet<Voxel>,
    offs: Vec<Vec<i8>>,
    visited: HashSet<Voxel>,
    counted: HashMap<Voxel,u8>
}

impl Space {
    fn new(data:&[String])->Self
    {
        Self 
        {
            vox : data.iter().map(|line |
                {
                    let tab : Vec<&str> = line.split(',').collect(); 
                    let x = tab[0].parse::<i8>().unwrap();
                    let y = tab[1].parse::<i8>().unwrap();
                    let z = tab[2].parse::<i8>().unwrap();
                    Voxel::new(x,y,z)
                }
            ).collect::<Vec<Voxel>>(),              
            s    : HashSet::new(),
            offs : vec![
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

    fn count(&mut self)->usize
    {
        self.count_from_vec(&self.vox)
    }

    fn count_from_vec(&self,voxels:&Vec<Voxel>)->usize
    {
        let mut res = 6*voxels.len();
        let mut space = HashMap::new();

        for v in voxels.iter() 
        {
            space.insert(Voxel::new(v.x,v.y,v.z),true);
        }        

        for v in voxels.iter() 
        {
            for off in self.offs.iter()
            {
                let voxel = Voxel::new(v.x + off[0],
                                       v.y + off[1],
                                       v.z + off[2]);

                if space.get(&voxel).is_some() { res-=1; }
            }
        }
        res
    }    

    fn blow2(&mut self,start:&Voxel,code:u8)->usize
    {
        if start.x < -1 { return 0; }
        if start.y < -1 { return 0; }
        if start.z < -1 { return 0; }
        if start.x > 20 { return 0; }
        if start.y > 20 { return 0; }
        if start.z > 20 { return 0; }

        if self.visited.get(start).is_some()
        {
            return 0;
        }
        
        if self.s.get(start).is_some()
        {
            let code_stored = *self.counted.get(start).unwrap_or(&0);
            self.counted.insert(Voxel::from_v(start), code | code_stored);
            usize::from((code_stored & code)==0)
        }
          else
        {
            self.visited.insert(Voxel::from_v(start));

            self.blow2(&Voxel::new(start.x+1,start.y  ,start.z  ), 1<<0) + 
            self.blow2(&Voxel::new(start.x-1,start.y  ,start.z  ), 1<<1) + 
            self.blow2(&Voxel::new(start.x  ,start.y+1,start.z  ), 1<<2) + 
            self.blow2(&Voxel::new(start.x  ,start.y-1,start.z  ), 1<<3) + 
            self.blow2(&Voxel::new(start.x  ,start.y  ,start.z+1), 1<<4) + 
            self.blow2(&Voxel::new(start.x  ,start.y  ,start.z-1), 1<<5)
        }
    }

    fn count2(&mut self)->usize
    {
        for v in self.vox.iter() 
        {
            self.s.insert(Voxel::new(v.x,v.y,v.z));
        }        

        let start = Voxel::new(0,0,0);
        self.visited.clear();
        self.blow2(&start,0)
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

