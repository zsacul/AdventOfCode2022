use std::collections::HashMap;
use super::tools;

//part1: 1199
//Elapsed: 147.59601 secs
//51 sec
//part2: 3510
//Elapsed: 1019.62305 secs

//part1: 1199
//part2: 3510
//Elapsed: 353.51602 secs

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
    const ORE  : u8 = 1<<1;
    const CLAY : u8 = 1<<2;
    const OBSI : u8 = 1<<3;
    const GEO  : u8 = 1<<4;
    
    fn new( ore_ore  : i32,
            clay_ore : i32,
            obs_ore  : i32,
            obs_clay : i32,
            geo_ore  : i32,
            geo_obs  : i32 )->Self
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

type State = (u8,u16,u16,u16,u16,i32,i32,i32,i32);

struct World
{
    cost       : Cost,
    time_limit : u8,
    hash       : HashMap<State,i32>,
}

impl World
{
    fn new(line:&str,time_limit:u8)->Self
    {
        Self
        {
            cost : Self::get_cost(line),
            time_limit,
            hash : HashMap::new(),
        }
    }

    fn get_cost(s:&str)->Cost
    {
        let _id      = tools::usize_get_between(s, "Blueprint ",":");
        let ore      = tools::usize_get_between(s, "Each ore robot costs "," ore.");
        let clay     = tools::usize_get_between(s, "Each clay robot costs "," ore");

        let s_obs    = tools::str_get_between(s, "Each obsidian robot costs "," clay.");
        let tab_ore  = tools::split_to_usize(s_obs,"ore and");       
        let obs_ore  = tab_ore[0];
        let obs_clay = tab_ore[1];
      
        let s_geo    = tools::str_get_between(s, "Each geode robot costs "," obsidian.");        
        let tab_geo  = tools::split_to_usize(s_geo,"ore and");       
        let geo_ore  = tab_geo[0];
        let geo_obs  = tab_geo[1];

         Cost::new(ore      as i32,
                   clay     as i32,
                   obs_ore  as i32,
                   obs_clay as i32,
                   geo_ore  as i32,
                   geo_obs  as i32)
    }

    #[allow(clippy::too_many_arguments)]
    fn sol(&mut self,time:u8,r_ore:u16,r_clay:u16,r_obs:u16,r_geo:u16,ore:i32,clay:i32,obs:i32,geo:i32,buy:u8)->i32
    {
        let mut ore_cost = 0;
        let mut cla_cost = 0;
        let mut obs_cost = 0;
        
        if buy & Cost::ORE!=0 {
            ore_cost+=self.cost.ore_ore
        }
        if buy & Cost::CLAY!=0 {
            ore_cost+=self.cost.clay_ore;
        }
        if buy & Cost::OBSI!=0 {
            ore_cost+=self.cost.obs_ore;
            cla_cost+=self.cost.obs_clay;
        }
        if buy & Cost::GEO!=0 {
            ore_cost+=self.cost.geo_ore;
            obs_cost+=self.cost.geo_obs;
        }

        if ore_cost>ore  {  return 0;  }
        if cla_cost>clay {  return 0;  }
        if obs_cost>obs  {  return 0;  }

        let ore  = ore  + r_ore  as i32;
        let clay = clay + r_clay as i32;
        let obs  = obs  + r_obs  as i32;
        let geo  = geo  + r_geo  as i32;
        
        if time==self.time_limit 
        { 
            return geo; 
        }
        
        let r_ore  = r_ore  + (buy & Cost::ORE !=0) as u16;
        let r_clay = r_clay + (buy & Cost::CLAY!=0) as u16;
        let r_obs  = r_obs  + (buy & Cost::OBSI!=0) as u16;
        let r_geo  = r_geo  + (buy & Cost::GEO !=0) as u16;
        
        let ore  = ore  - ore_cost;
        let clay = clay - cla_cost;
        let obs  = obs  - obs_cost;
        
        let key = (time,r_ore,r_clay,r_obs,r_geo,ore,clay,obs,geo);
       
        if let Some(res) = self.hash.get(&key) { return *res; }

        let mut res = 0;

        if ore>=self.cost.geo_ore && obs>=self.cost.geo_obs
        {
            res = res.max( self.sol(time+1, r_ore, r_clay, r_obs, r_geo, ore, clay, obs, geo,Cost::GEO ) );
        }
        else if ore>=self.cost.obs_ore && clay>=self.cost.obs_clay
        {
            res = res.max( self.sol(time+1, r_ore, r_clay, r_obs, r_geo, ore, clay, obs, geo,Cost::OBSI) );
        }
        else if ore>=self.cost.clay_ore
        {
            res = res.max( self.sol(time+1, r_ore, r_clay, r_obs, r_geo, ore, clay, obs, geo,Cost::CLAY) );
        }
        
        if ore>=self.cost.ore_ore
        {
            res = res.max(  self.sol(time+1, r_ore, r_clay, r_obs, r_geo, ore, clay, obs, geo,Cost::ORE ) );                
        }        
        
            res = res.max(  self.sol(time+1, r_ore, r_clay, r_obs, r_geo, ore, clay, obs, geo,         0) );
        

        


        //let mut res =         self.sol(time+1, r_ore, r_clay, r_obs, r_geo, ore, clay, obs, geo,Cost::GEO );
          //      res = res.max(self.sol(time+1, r_ore, r_clay, r_obs, r_geo, ore, clay, obs, geo,Cost::OBSI));
            //    res = res.max(self.sol(time+1, r_ore, r_clay, r_obs, r_geo, ore, clay, obs, geo,Cost::CLAY));
              //  res = res.max(self.sol(time+1, r_ore, r_clay, r_obs, r_geo, ore, clay, obs, geo,Cost::ORE ));
                //res = res.max(self.sol(time+1, r_ore, r_clay, r_obs, r_geo, ore, clay, obs, geo,0         ));

        self.hash.insert(key,res);
        res
    }
}

fn solve_single(s:&str,time:u8)->usize
{
    World::new(s,time).sol(1,1,0,0,0,0,0,0,0,0) as usize
}

fn compute(data:&[String])->usize
{    
    data.iter()
        .enumerate()
        .map( |(id,line)| (id+1)*solve_single(&line[..],24) )
        .sum()
}

fn compute2(s:&str)->usize
{
    solve_single(s,32)
}

pub fn part1(data:&[String])->usize
{
    compute(data)
}

pub fn part2(data:&[String])->usize
{
    if data.len()==1 
    {
        compute2(&data[0][..])
    }
      else
    {
        compute2(&data[0][..])*
        compute2(&data[1][..])*
        compute2(&data[2][..])  
    }
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

//big test
#[test]
fn test2_1()
{
    let v = vec![
                "Blueprint 1:  Each ore robot costs 4 ore.  Each clay robot costs 2 ore.  Each obsidian robot costs 3 ore and 14 clay.  Each geode robot costs 2 ore and 7 obsidian.".to_string(),
            ];
    assert_eq!(part2(&v),56);
}

//big test
#[test]
fn test2_2()
{
    let v = vec![
                "Blueprint 1:  Each ore robot costs 4 ore.  Each clay robot costs 2 ore.  Each obsidian robot costs 3 ore and 14 clay.  Each geode robot costs 2 ore and 7 obsidian.".to_string(),
                "Blueprint 2:  Each ore robot costs 2 ore.  Each clay robot costs 3 ore.  Each obsidian robot costs 3 ore and 8 clay.  Each geode robot costs 3 ore and 12 obsidian.".to_string(),
            ];
            assert_eq!(part2(&v),62);
        }
        

#[test]
fn small_01()
{        
    let s = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 7 clay. Each geode robot costs 2 ore and 19 obsidian.";
    assert_eq!(solve_single(s,24),1);
}

#[test]
fn small_02()
{        
    let s = "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 20 clay. Each geode robot costs 4 ore and 18 obsidian.";
    assert_eq!(solve_single(s,24),0);
}

#[test]
fn small_03()
{        
    let s = "Blueprint 3: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 20 clay. Each geode robot costs 2 ore and 10 obsidian.";
    assert_eq!(solve_single(s,24),0);
}

#[test]
fn small_04()
{        
    let s = "Blueprint 4: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 19 clay. Each geode robot costs 2 ore and 12 obsidian.";
    assert_eq!(solve_single(s,24),0);
}

#[test]
fn small_05()
{        
    let s = "Blueprint 5: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 20 clay. Each geode robot costs 3 ore and 14 obsidian.";
    assert_eq!(solve_single(s,24),0);
}

#[test]
fn small_06()
{        
    let s = "Blueprint 6: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 15 clay. Each geode robot costs 3 ore and 7 obsidian.";
    assert_eq!(solve_single(s,24),3);
}

#[test]
fn small_07()
{        
    let s = "Blueprint 7: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 19 clay. Each geode robot costs 2 ore and 20 obsidian.";
    assert_eq!(solve_single(s,24),0);
}

#[test]
fn small_08()
{        
    let s = "Blueprint 8: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 13 clay. Each geode robot costs 2 ore and 20 obsidian.";
    assert_eq!(solve_single(s,24),3);
}

//big test
#[test]
fn small_09()
{        
    let s = "Blueprint 9: Each ore robot costs 2 ore. Each clay robot costs 2 ore. Each obsidian robot costs 2 ore and 8 clay. Each geode robot costs 2 ore and 14 obsidian.";
    assert_eq!(solve_single(s,24),13);
}

#[test]
fn small_10()
{        
    let s = "Blueprint 10: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 11 clay. Each geode robot costs 3 ore and 14 obsidian.";
    assert_eq!(solve_single(s,24),0);
}

#[test]
fn small_11()
{        
    let s = "Blueprint 11: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 5 clay. Each geode robot costs 4 ore and 8 obsidian.";
    assert_eq!(solve_single(s,24),13);
}

#[test]
fn small_12()
{        
    let s = "Blueprint 12: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 16 clay. Each geode robot costs 2 ore and 18 obsidian.";
    assert_eq!(solve_single(s,24),0);
}

#[test]
fn small_13()
{        
    let s = "Blueprint 13: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 11 clay. Each geode robot costs 2 ore and 10 obsidian.";
    assert_eq!(solve_single(s,24),4);
}

#[test]
fn small_14()
{        
    let s = "Blueprint 14: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 14 clay. Each geode robot costs 3 ore and 17 obsidian.";
    assert_eq!(solve_single(s,24),0);
}

#[test]
fn small_15()
{        
    let s = "Blueprint 15: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 19 clay. Each geode robot costs 3 ore and 17 obsidian.";
    assert_eq!(solve_single(s,24),0);
}

#[test]
fn small_16()
{        
    let s = "Blueprint 16: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 20 clay. Each geode robot costs 2 ore and 17 obsidian.";
    assert_eq!(solve_single(s,24),1);
}

#[test]
fn small_17()
{        
    let s = "Blueprint 17: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 4 ore and 8 obsidian.";
    assert_eq!(solve_single(s,24),1);
}

#[test]
fn small_18()
{        
    let s = "Blueprint 18: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 9 clay. Each geode robot costs 3 ore and 9 obsidian.";
    assert_eq!(solve_single(s,24),15);
}

#[test]
fn small_19()
{        
    let s = "Blueprint 19: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 10 clay. Each geode robot costs 3 ore and 14 obsidian.";
    assert_eq!(solve_single(s,24),1);
}

#[test]
fn small_20()
{        
    let s = "Blueprint 20: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 13 clay. Each geode robot costs 3 ore and 12 obsidian.";
    assert_eq!(solve_single(s,24),3);
}

#[test]
fn small_21()
{        
    let s = "Blueprint 21: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 4 ore and 15 clay. Each geode robot costs 4 ore and 9 obsidian.";
    assert_eq!(solve_single(s,24),2);
}

#[test]
fn small_22()
{        
    let s = "Blueprint 22: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 20 clay. Each geode robot costs 2 ore and 12 obsidian.";
    assert_eq!(solve_single(s,24),1);
}

#[test]
fn small_23()
{        
    let s = "Blueprint 23: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 4 ore and 19 clay. Each geode robot costs 4 ore and 12 obsidian.";
    assert_eq!(solve_single(s,24),0);
}

#[test]
fn small_24()
{        
    let s = "Blueprint 24: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 15 clay. Each geode robot costs 3 ore and 8 obsidian.";
    assert_eq!(solve_single(s,24),1);
}

#[test]
fn small_25()
{        
    let s = "Blueprint 25: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 11 clay. Each geode robot costs 2 ore and 16 obsidian.";
    assert_eq!(solve_single(s,24),5);
}

#[test]
fn small_26()
{        
    let s = "Blueprint 26: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 17 clay. Each geode robot costs 3 ore and 7 obsidian.";
    assert_eq!(solve_single(s,24),2);
}

#[test]
fn small_27()
{        
    let s = "Blueprint 27: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 7 clay. Each geode robot costs 3 ore and 20 obsidian.";
    assert_eq!(solve_single(s,24),1);
}

#[test]
fn small_28()
{        
    let s = "Blueprint 28: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 10 clay. Each geode robot costs 2 ore and 10 obsidian.";
    assert_eq!(solve_single(s,24),5);
}

#[test]
fn small_29()
{        
    let s = "Blueprint 29: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 17 clay. Each geode robot costs 2 ore and 13 obsidian.";
    assert_eq!(solve_single(s,24),0);
}

#[test]
fn small_30()
{        
    let s = "Blueprint 30: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 4 ore and 20 clay. Each geode robot costs 4 ore and 8 obsidian.";    
    assert_eq!(solve_single(s,24),1);
}
