use std::collections::HashMap;
use super::tools;

#[derive(Debug, PartialEq, Eq,PartialOrd, Ord, Hash)]
struct Cost
{
    ore_ore  : i32,
    clay_ore : i32,
    obs_ore  : i32,
    obs_clay : i32,
    geo_ore  : i32,
    geo_obs  : i32,    
}

impl Cost
{
    fn new( ore_ore  : i32,
            clay_ore : i32,
            obs_ore  : i32,
            obs_clay : i32,
            geo_ore  : i32,
            geo_obs  : i32,
    )->Self
    {
        Self { 
            ore_ore ,
            clay_ore,
            obs_ore ,
            obs_clay,
            geo_ore ,
            geo_obs ,
         }
    }
}





fn sol(hash:&mut HashMap<(i32,i32,i32,i32,i32,i32,i32,i32,i32),i32>,add:bool,cost:&Cost,time:i32,r_ore:i32,r_clay:i32,r_obs:i32,r_geo:i32,ore:i32,clay:i32,obs:i32,geo:i32)->i32
{
    if ore<0 || clay<0 || obs<0 || geo<0 { return 0; }
    //println!("add {},time {},r_ore {},r_clay {},r_obs {},r_geo {},ore {},clay {},obs {},geo {}",add,time,r_ore,r_clay,r_obs,r_geo,ore,clay,obs,geo);
    let key = (time,r_ore,r_clay,r_obs,r_geo,ore,clay,obs,geo);

    if time==24
    {
        if geo>7
        {
            println!("{:?} ",key);
        }
        let res = geo;
        return res;
    }

    

    //println!("{:?}",key);
    let hh = hash.get(&key);
    if hh.is_some()
    {
        return *hh.unwrap();
    }
    
    let ore  = if add { ore  + r_ore  } else { ore  };
    let clay = if add { clay + r_clay } else { clay };
    let obs  = if add { obs  + r_obs  } else { obs  };
    let geo  = if add { geo  + r_geo  } else { geo  };    

    
    // println!("{} = {} {} {} {}",time,ore,clay,obs,geo);
//    let mut                  res =         sol(hash,true ,cost, time+1, r_ore  , r_clay  , r_obs  , r_geo  , ore               , clay              , obs              , geo);

    let mut   res =         sol(hash,true ,cost, time+1, r_ore  , r_clay  , r_obs  , r_geo  , ore               , clay              , obs              , geo);
              res = res.max(sol(hash,false,cost, time+1, r_ore+1, r_clay  , r_obs  , r_geo  , ore-cost.ore_ore  , clay              , obs              , geo));
              res = res.max(sol(hash,false,cost, time+1, r_ore  , r_clay+1, r_obs  , r_geo  , ore-cost.clay_ore , clay              , obs              , geo));
              res = res.max(sol(hash,false,cost, time+1, r_ore  , r_clay  , r_obs+1, r_geo  , ore-cost.obs_ore  , clay-cost.obs_clay, obs              , geo));
              res = res.max(sol(hash,false,cost, time+1, r_ore  , r_clay  , r_obs  , r_geo+1, ore-cost.geo_ore  , clay              , obs-cost.geo_obs , geo));
/*
    
    if ore >=cost.ore_ore  { res = res.max(sol(hash,false,cost, time  , r_ore+1, r_clay  , r_obs  , r_geo  , ore-cost.ore_ore  , clay              , obs              , geo)); }
    
    if ore >=cost.clay_ore { res = res.max(sol(hash,false,cost, time  , r_ore  , r_clay+1, r_obs  , r_geo  , ore-cost.clay_ore , clay              , obs              , geo)); }

    if ore >=cost.obs_ore && 
       clay>=cost.obs_clay
                          { res = res.max(sol(hash,false,cost, time  , r_ore  , r_clay  , r_obs+1, r_geo  , ore-cost.obs_ore  , clay-cost.obs_clay, obs              , geo)); }
    if ore >=cost.geo_ore && 
       obs >=cost.geo_obs
                          { res = res.max(sol(hash,false,cost, time  , r_ore  , r_clay  , r_obs  , r_geo+1, ore-cost.geo_ore  , clay              , obs-cost.geo_obs , geo)); }
 */
    hash.insert(key,res);
    res
}

fn compute(data:&[String],days:usize)->usize
{    
    //let y = data.join(" ");
    let mut res = 0;

    for line in data
    {
        let s        = &line[..];
        let id       = tools::str_get_between(s, "Blueprint ",":").trim().parse::<usize>().unwrap();
        let ore      = tools::str_get_between(s, "Each ore robot costs","ore.").trim().parse::<usize>().unwrap();
        let clay     = tools::str_get_between(s, "Each clay robot costs","ore").trim().parse::<usize>().unwrap();

        let s_obs    = tools::str_get_between(s, "Each obsidian robot costs "," clay.");
        let tab_ore  = tools::split_to_usize(s_obs,"ore and");       
        let obs_ore  = tab_ore[0];
        let obs_clay = tab_ore[1];
      
        let s_geo    = tools::str_get_between(s, "Each geode robot costs "," obsidian.");        
        let tab_geo  = tools::split_to_usize(s_geo,"ore and");       
        let geo_ore  = tab_geo[0];
        let geo_obs  = tab_geo[1];

        let cost = Cost::new(ore as i32,
                             clay as i32,
                             obs_ore as i32,
                             obs_clay as i32,
                             geo_ore as i32,
                             geo_obs as i32);

        //println!("{:#?}",cost);

        let mut hash = HashMap::new();
        let r = sol(&mut hash,true,&cost,1,1,0,0,0,0,0,0,0);

        println!("id={} sol={}",id,r);
        res+=id*(r as usize);
        //println!("id={} ore={} clay={} obs={}/{} geo={}/{}",id,ore,clay,obs_ore,obs_clay,geo_ore,geo_obs);
    }

    res
    /*
    let mut dist = vec![vec![9999;f.size.x as usize];f.size.y as usize];
    let       fs = if ssx!=usize::MAX
                   {
                       Vec2::newu(ssx,ssy)
                   }
                     else
                   {
                       Vec2::new(f.start.x,f.start.y)
                   };

    dist[fs.y as usize][fs.x as usize] = 0;

    let mut queue : VecDeque<Node> = VecDeque::new(); 
    queue.push_back( Node::new(dist[fs.y as usize][fs.x as usize],fs) );

    while !queue.is_empty() 
    {
        let node = queue.pop_front().unwrap();

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
                        queue.push_back(Node::new(node.distance+1,des) );
                    }    
            }
        }
    }
    
    dist[f.end.y as usize][f.end.x as usize]
     */
}

pub fn part1(data:&[String])->usize
{
    compute(data,24)
}

pub fn part2(data:&[String])->usize
{
    0
    /*
    let hills = Hills::new(data);

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
    println!("Day 19");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "Blueprint 1:  Each ore robot costs 4 ore.  Each clay robot costs 2 ore.  Each obsidian robot costs 3 ore and 14 clay.  Each geode robot costs 2 ore and 7 obsidian.".to_string(),
        "Blueprint 2:  Each ore robot costs 2 ore.  Each clay robot costs 3 ore.  Each obsidian robot costs 3 ore and 8 clay.  Each geode robot costs 3 ore and 12 obsidian.".to_string(),
            ];
    assert_eq!(part1(&v),33);
}

#[test]
fn test2()
{
    let v = vec![
        "Blueprint 1:  Each ore robot costs 4 ore.  Each clay robot costs 2 ore.  Each obsidian robot costs 3 ore and 14 clay.  Each geode robot costs 2 ore and 7 obsidian.".to_string(),
        "Blueprint 2:  Each ore robot costs 2 ore.  Each clay robot costs 3 ore.  Each obsidian robot costs 3 ore and 8 clay.  Each geode robot costs 3 ore and 12 obsidian.".to_string(),
            ];
    assert_eq!(part2(&v),29);
}