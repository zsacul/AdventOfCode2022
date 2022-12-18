use std::collections::HashMap;

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
    s   : HashMap<Voxel,bool>,
    offs: Vec<Vec<i32>>,
    visited: HashMap<Voxel,bool>,
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
                    let x = tab[0].parse::<i32>().unwrap();
                    let y = tab[1].parse::<i32>().unwrap();
                    let z = tab[2].parse::<i32>().unwrap();
                    Voxel::new(x,y,z)
                }
            ).collect::<Vec<Voxel>>(),              
            s    : HashMap::new(),
            offs : vec![
                            vec![-1, 0, 0],
                            vec![ 1, 0, 0],
                            vec![ 0,-1, 0],
                            vec![ 0, 1, 0],
                            vec![ 0, 0,-1],
                            vec![ 0, 0, 1],
                       ],
            visited : HashMap::new(),
            counted : HashMap::new(),
        }
    }

    fn count(&mut self)->i32 
    {
        self.count_from_vec(&self.vox)
/*
        let mut res = self.vox.len() as i32*6;


        for v in self.vox.iter() 
        {
            self.s.insert(Voxel::new(v.x,v.y,v.z),true);
        }        

        for v in self.vox.iter() 
        {
            for offset in self.offs.iter()
            {
                let voxel = Voxel::new(v.x + offset[0],
                                       v.y + offset[1],
                                       v.z + offset[2]);

                if self.s.get(&voxel).is_some()
                {
                    res-=1;
                }
            }
            
        }
        res
         */
    }


    fn count_from_vec(&self,voxels:&Vec<Voxel>)->i32 
    {
        let mut res = voxels.len() as i32*6;
        let mut ss = HashMap::new();

        for v in voxels.iter() 
        {
            ss.insert(Voxel::new(v.x,v.y,v.z),true);
        }        

        for v in voxels.iter() 
        {
            for off in self.offs.iter()
            {
                let voxel = Voxel::new(v.x+off[0],
                                       v.y+off[1],
                                       v.z+off[2]);

                if ss.get(&voxel).is_some()
                {
                    res-=1;
                }
            }
            
        }
        res
    }    
/*
    fn blow(&mut self,start:&Voxel)->i32
    {
        if start.x < -1 { return i32::MAX; }
        if start.y < -1 { return i32::MAX; }
        if start.z < -1 { return i32::MAX; }
        if start.x > 21 { return i32::MAX; }
        if start.y > 21 { return i32::MAX; }
        if start.z > 21 { return i32::MAX; }

        if self.visited.get(&start).is_some()
        {
            return 0;
        }
        self.visited.insert(Voxel::new(start.x,start.y,start.z),true);

        if self.s.get(&start).is_some()
        {
            return 1;
        }

        let mut oo = vec![];
        {
            oo = self.offs.clone();
        }

        let mut res = 0;

        for f in oo
        {
            let nv = Voxel::new(start.x + f[0],
                                start.y + f[1],
                                start.z + f[2]);

                                let r = self.blow(&nv);
            if r==i32::MAX { return i32::MAX; }
            res = res.max(r);
        }
        
        res
        
    }
 */
    fn blow2(&mut self,start:&Voxel,code:u8)->i32
    {
        if start.x < 0  { return 0; }
        if start.y < 0  { return 0; }
        if start.z < 0  { return 0; }
        if start.x > 20 { return 0; }
        if start.y > 20 { return 0; }
        if start.z > 20 { return 0; }

        //println!("v:{:?}",start);

        if self.visited.get(&start).is_some()
        {
            return 0;
        }


        let code_stored = *self.counted.get(start).unwrap_or(&0);
        

        
        if self.s.get(&start).is_some()
        {
            if (code_stored & code)==0
            {
                self.counted.insert(Voxel::from_v(start), code | code_stored);
              //  println!("{:?} {}", start,code | code_stored);
                return 1;
            }
            else
            {
                return 0;
            }
            
        }

        self.visited.insert(Voxel::from_v(start),true);

        self.blow2(&Voxel::new(start.x+1,start.y  ,start.z  ), 1<<0) + 
        self.blow2(&Voxel::new(start.x-1,start.y  ,start.z  ), 1<<1) + 
        self.blow2(&Voxel::new(start.x  ,start.y+1,start.z  ), 1<<2) + 
        self.blow2(&Voxel::new(start.x  ,start.y-1,start.z  ), 1<<3) + 
        self.blow2(&Voxel::new(start.x  ,start.y  ,start.z+1), 1<<4) + 
        self.blow2(&Voxel::new(start.x  ,start.y  ,start.z-1), 1<<5)
    }

    fn count2(&mut self)->i32 
    {
        for v in self.vox.iter() 
        {
            self.s.insert(Voxel::new(v.x,v.y,v.z),true);
        }        

/*
        let mut pot = HashMap::new();
        
        for v in self.vox.iter() 
        {
            for off in self.offs.iter()
            {
                let voxel = Voxel::new(v.x + off[0],
                                       v.y + off[1],
                                       v.z + off[2]);

                if self.s.get(&voxel).is_none()
                {
                    pot.insert(voxel,false);
                }
            }
        }
*/
        let start = Voxel::new(0,0,0);
        self.visited.clear();
        return self.blow2(&start,0);

        
        //<12576
        //<3252
        //>1424
        /*
        let mut vv = vec![];

        for xx in 0..21
        {
            for yy in 0..21
            {
                for zz in 0..21
                {
                    self.visited.clear();
                    let v = Voxel::new(xx,yy,zz);
                    let r = self.blow(&v);
        
                    if r!=i32::MAX
                    {
                        //println!("trapped {:?}",v);
                        vv.push(Voxel::from_v(&v));
                    }
        
                }
            }
        }        
/*
        for v in pot.keys()
        {           
            self.visited.clear();
            let r = self.blow(v);

            if r!=i32::MAX
            {
                println!("trapped {:?}",v);
                vv.push(Voxel::from_v(v));
            }
        }
*/
        self.count() - self.count_from_vec(&vv)   
         */
    }    
    
}


pub fn part1(data:&[String])->i32
{
     Space::new(data).count()   
}

pub fn part2(data:&[String])->i32
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
