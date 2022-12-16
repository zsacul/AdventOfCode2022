use std::collections::HashMap;
use super::vec2::Vec2;
use super::tools;

#[derive(Eq, PartialEq, Debug, Clone)]
struct Valve
{
    name   : String,
    opened : bool,
    flow   : i64,
    tunels : Vec<String>
}

impl Valve 
{
    fn new(name:String,flow:i64,tunels:Vec<String>)->Self
    {
        Self
        {
            name,
            opened:false,
            flow,
            tunels
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct World
{
    field  : HashMap<Vec2,char>,
    b_dist : HashMap<Vec2,i64>,
    bits   : HashMap<String,i64>,
    v      : Vec<Valve>,
}

impl World {
    fn new()->Self
    {
        Self
        {
            field  : HashMap::new(),
            b_dist : HashMap::new(),
            bits   : HashMap::new(),
            v      : vec![],
        }
    }



    fn load(&mut self,data:&[String])
    {
        let mut id = 0usize;
        for line in data 
        {
            //Valve FF has flow rate=0; tunnels lead to valves EE, GG
            let src  = tools::str_get_between(&line[..],"Valve "," has flow");
            let rate  = tools::i64_get_between(&line[..],"flow rate=","; tunnel");

            let mut tab = vec![];

            if line.find("to valves").is_some()
            {
                let des  = tools::str_get_between(&line[..],"to valves ","");
                tab = des.split(", ").map(|s| s.to_string()).collect::<Vec<String>>();
            }
              else
            {
                let des  = tools::str_get_between(&line[..],"to valve ","");
                tab = vec![des.to_string()];
            }
            
            
            let v = Valve::new(src.to_string(),rate,tab);            
            self.v.push(v);
            self.bits.insert(src.to_string(), 1<<id);
            id+=1;
            //println!("{} {} [{:?}]",src,rate,tab);
        }
    }

    #[allow(unused)]
    fn print(&self,x0:usize,x1:usize,y0:usize,y1:usize)
    {
        //for y in y0..=y1 {
        //for x in x0..=x1 { print!("{}",self.val(x as i64,y as i64)); }
          //                 println!();
        //}
    }

    fn simulate(&mut self,time:usize,opended:usize,act:String,mut flow: i64,mut total:i64)->i64
    {
        total+=flow;
        
        println!("time:{} opened:{} total:{} act:{}",time,opended,total,act);
        
        if time==30 
        {
            return total;
        }

        0
    }

    fn count(&self,y_pos:i64)->i64
    {
        0
    }


}

pub fn part1(data:&[String])->i64
{
    let mut w = World::new();
    w.load(data);
    w.simulate(0,0,"AA".to_string(),0,0)
    //let mut w = World::new();
    //w.load(data);
    //w.count()
    //0
}

pub fn part2(data:&[String])->i64
{
    let mut w = World::new();
    w.load(data);
    0

//    let mut w = World::new();
//    w.load(data);
//    let (x,y) = w.find_valid(range);
//    x*4000000 + y
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day 16");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = 
    vec![
        "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB".to_string(),
        "Valve BB has flow rate=13; tunnels lead to valves CC, AA".to_string(),
        "Valve CC has flow rate=2; tunnels lead to valves DD, BB".to_string(),
        "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE".to_string(),
        "Valve EE has flow rate=3; tunnels lead to valves FF, DD".to_string(),
        "Valve FF has flow rate=0; tunnels lead to valves EE, GG".to_string(),
        "Valve GG has flow rate=0; tunnels lead to valves FF, HH".to_string(),
        "Valve HH has flow rate=22; tunnel leads to valve GG".to_string(),
        "Valve II has flow rate=0; tunnels lead to valves AA, JJ".to_string(),
        "Valve JJ has flow rate=21; tunnel leads to valve II".to_string()
    ];
    assert_eq!(part1(&v),1651);
}

#[test]
fn test2()
{
    let v = 
    vec![
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB".to_string(),
            "Valve BB has flow rate=13; tunnels lead to valves CC, AA".to_string(),
            "Valve CC has flow rate=2; tunnels lead to valves DD, BB".to_string(),
            "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE".to_string(),
            "Valve EE has flow rate=3; tunnels lead to valves FF, DD".to_string(),
            "Valve FF has flow rate=0; tunnels lead to valves EE, GG".to_string(),
            "Valve GG has flow rate=0; tunnels lead to valves FF, HH".to_string(),
            "Valve HH has flow rate=22; tunnel leads to valve GG".to_string(),
            "Valve II has flow rate=0; tunnels lead to valves AA, JJ".to_string(),
            "Valve JJ has flow rate=21; tunnel leads to valve II".to_string()
       ];
    assert_eq!(part2(&v),56000011);
}