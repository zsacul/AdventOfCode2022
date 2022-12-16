use std::{collections::HashMap};
use super::tools;

#[derive(Eq, PartialEq, Debug, Clone)]
struct Valve
{
    name   : String,
    opened : bool,
    flow   : usize,
    tunels : Vec<String>
}

impl Valve 
{
    fn new(name:String,flow:usize,tunels:Vec<String>)->Self
    {
        Self
        {
            name,
            opened:false,
            flow,
            tunels,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct World
{
    bits   : HashMap<String,usize>,
    v      : HashMap<String,Valve>,
    golden : usize,
    full   : usize,
    //best   : usize,
    time_lim: usize,
}

impl World {
    fn new(t:usize)->Self
    {
        Self
        {
            bits   : HashMap::new(),
            v      : HashMap::new(),
            golden : 0,
            full   : 0,
            time_lim : t,
        }
    }

    fn load(&mut self,data:&[String])
    {
        let mut id = 0usize;
        for line in data 
        {
            //Valve FF has flow rate=0; tunnels lead to valves EE, GG
            let src  = tools::str_get_between(&line[..],"Valve "," has flow");
            let rate  = tools::usize_get_between(&line[..],"flow rate=","; tunnel");

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
            
            let ptt = tab.clone();
            
            let v = Valve::new(src.to_string(),rate,tab);            
            self.v.insert(src.to_string(), v);
            self.bits.insert(src.to_string(), 1<<id);
            
            if rate>0
            {
                self.golden|= 1<<id;
                self.full  += rate;
            }

            id+=1;
          //  println!("{} {} [{:?}]",src,rate,ptt);
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
    //

    fn simulate(&self,memory:&mut HashMap<(usize,usize,String),usize>,time:usize,opended:usize,act:String,flow: usize,total:usize)->usize
    {
        
        let key = (time,opended,act.clone());
        let v   = *memory.get(&key).unwrap_or(&0);
        let nnn = act.clone();
        
        if total+flow*(31-time)<v 
        {
            return v;
        }

        //total+=flow;
        //println!("time:{} opened:{} flow:{} total:{} act:{}",time,opended,flow,total,act);

        let res = 
            if time==29
            {
                total + flow
            }
              else
            {
                let new_flow = self.v.get(&act).unwrap().flow;

                if opended==self.golden
                {
                    self.simulate(memory,time+1,opended ,act,flow ,total+flow)
                }
                else
                {
                    let bit = *self.bits.get(&act).unwrap();
                   
                    let mut best = usize::MIN;
                    let nn = act.clone();

                    if (opended & bit==0) && new_flow>0
                    {            
                        best = self.simulate(memory,time+1,opended | bit,act,flow + new_flow,total+flow)
                    }

                    for exit in self.v.get(&nn).unwrap().tunels.iter() 
                    {
                        let r = self.simulate(memory,time+1,opended,exit.clone(),flow,total+flow);
                        if r > best 
                        {
                            best = r;
                        }
                    }
                    best
                
                }
            };

        if res>v 
        {
         //   println!("time:{} opened:{} flow:{} total:{} act:{}",time+1,opended,flow,total,nnn);
            memory.insert(key,res);
        }
        
        res
    }


    fn simulate2(&self,memory:&mut HashMap<(usize,usize,String),usize>,time:usize,opended:usize,act1:String,act2:String,flow: usize,total:usize)->usize
    {
        
        let full = if act1<act2 { 
            format!("{}x{}",act1.clone(),act2.clone())
        }
          else
        {
            format!("{}x{}",act2.clone(),act1.clone())
        };

        let key = (time,opended,full.clone());
        let v   = *memory.get(&key).unwrap_or(&0);

        let ac1 = act1.clone();
        let ac2 = act2.clone();
            
        
        if total+flow*(self.time_lim+1-time)<v
        {
            return v;
        }

        //if total+flow*(self.time_lim+1-time)<self.best
        //{
          //  return v;
        //}

        //total+=flow;
        //println!("time:{} opened:{} flow:{} total:{} act:{}",time,opended,flow,total,act);

        let res = 
            if time==self.time_lim-1
            {
                total + flow
            }
              else
            {
                if opended==self.golden
                {
                    self.simulate2(memory,time+1,opended ,act1,act2,flow ,total+flow)
                }
                else
                {
                    let bit1 = *self.bits.get(&act1).unwrap();
                    let bit2 = *self.bits.get(&act2).unwrap();
                   
                    let mut best = usize::MIN;
                    let nn1 = act1.clone();
                    let nn2 = act2.clone();
                    let new_flow1 = self.v.get(&act1).unwrap().flow;
                    let new_flow2 = self.v.get(&act2).unwrap().flow;

                    if bit1!=bit2 &&
                       (opended & bit1==0) && new_flow1>0 && 
                       (opended & bit2==0) && new_flow2>0
                    {
                        let r = self.simulate2(memory,time+1,opended | bit1 | bit2,act1.clone(),act2.clone(),flow + new_flow1 + new_flow2,total+flow);
                        best = best.max(r);
                    }

                    let t1 = &self.v.get(&nn1).unwrap().tunels;
                    let t2 = &self.v.get(&nn2).unwrap().tunels;

                    if (opended & bit1==0) && new_flow1>0
                    {            
                        //let a1 = act1.clone();
                        //let a2 = act2.clone();

                        for e2 in t2
                        {
                            let r = self.simulate2(memory,time+1,opended | bit1,act1.clone(),e2.to_string(),flow + new_flow1,total+flow);
                            best = best.max(r);
                        }

                        //let r = self.simulate2(memory,time+1,opended | bit1,a1,a2,flow + new_flow1,total+flow);
                        //best = best.max(r);
                    }

                    if (opended & bit2==0) && new_flow2>0
                    {            
                        //let a1 = act1.clone();
                        //let a2 = act2.clone();

                        for e1 in t1
                        {
                            let r = self.simulate2(memory,time+1,opended | bit2,e1.clone(),act2.to_string(),flow + new_flow2,total+flow);
                            best = best.max(r);
                        }


                        //let r = self.simulate2(memory,time+1,opended | bit2,a1,a2,flow + new_flow2,total+flow);
                        //best = best.max(r);
                    }




                    for e1 in t1
                    {
                        for e2 in t2
                        {
                            if (*e1!=ac1 && *e2!=ac2) && (*e2!=ac1 && *e1!=ac2)
                            {
                                let r = self.simulate2(memory,time+1,opended,e1.to_string(),e2.to_string(),flow,total+flow);
                                best = best.max(r);
                            }
                        }
                    }
                    best
                
                }
            };

        if res>v 
        {
            if time==self.time_lim-1
            {
                //println!("time:{} opened:{} flow:{} total:{} key:{} ",time+1,opended,flow,total,full);
            }
           
            memory.insert(key,res);
        }
        
        res
    }


}

pub fn part1(data:&[String])->usize
{
    let mut w = World::new(30);
    w.load(data);
    let mut memory = HashMap::new();
    let res = w.simulate(&mut memory,0,0,"AA".to_string(),0,0);


    //for (k,v) in memory {
      //  if k.0>=29 {
        //    println!("{:?} = {}",k,v);
        //}
    //}

    //println!("{:#?}",memory);
    res
    //let mut w = World::new();
    //w.load(data);
    //w.count()
    //0
}

pub fn part2(data:&[String],limit:usize)->usize
{
    let mut w = World::new(limit);
    w.load(data);
    let mut memory2 = HashMap::new();
    let res = w.simulate2(&mut memory2,0,0,"AA".to_string(),"AA".to_string(),0,0);

    res

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
    println!("part2: {}",part2(data,26));
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
    assert_eq!(part2(&v,26),1707);
}


#[test]
fn testp2_1()
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
    assert_eq!(part2(&v,1),0);
}


#[test]
fn testp2_2()
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
    assert_eq!(part2(&v,2),0);
}


#[test]
fn testp2_3()
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
    assert_eq!(part2(&v,3),20);
}


#[test]
fn testp2_4()
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
    assert_eq!(part2(&v,4),61);
}


#[test]
fn testp2_5()
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
    assert_eq!(part2(&v,5),102);
}


#[test]
fn testp2_6()
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
    assert_eq!(part2(&v,6),143);
}


#[test]
fn testp2_7()
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
    assert_eq!(part2(&v,7),143+41);
}



#[test]
fn testp2_8()
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
    assert_eq!(part2(&v,8),143+41);
}
