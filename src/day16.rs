use std::collections::HashMap;
use super::tools;

#[derive(Eq, PartialEq, Debug, Clone)]
struct Valve
{
    name   : String,
    flow   : u16,
    tunels : Vec<String>,
    tunels3: Vec<u8>
}

impl Valve 
{
    fn new(name:String,flow:u16,tunels:Vec<String>,tunels3:Vec<u8>)->Self
    {
        Self
        {
            name,
            flow,
            tunels,
            tunels3,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct World
{
    ids      : HashMap<String,usize>,
    v        : HashMap<String,Valve>,
    v3       : Vec<Valve>,
    golden   : usize,
    full     : usize,
    time_lim : u8,
}

impl World {
    fn new(t:u8)->Self
    {
        Self
        {
            ids      : HashMap::new(),
            v        : HashMap::new(),
            v3       : vec![],
            golden   : 0,
            full     : 0,
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
            let rate = tools::usize_get_between(&line[..],"flow rate=","; tunnel");
            let mut tab = vec![];

            if line.find("to valves").is_some()
            {
                let des = tools::str_get_between(&line[..],"to valves ","");
                tab = des.split(", ").map(|s| s.to_string()).collect::<Vec<String>>();
            }
              else
            {
                let des = tools::str_get_between(&line[..],"to valve ","");
                tab = vec![des.to_string()];
            }
                       
            let v = Valve::new(src.to_string(),rate as u16,tab,vec![]);            
            self.v.insert(src.to_string(), v.clone());
            self.v3.push(v);
            //self.bits.insert(src.to_string(), 1<<id);
            self.ids.insert( src.to_string() , id);
            
            if rate>0
            {
                self.golden|= 1<<id;
                self.full  += rate;
            }

            id+=1;
          //  println!("{} {} [{:?}]",src,rate,ptt);
        }

        //println!("{:#?}",self.bits);

        let _ooo:Vec<_> = self.v3.iter_mut().map(|e| 
                            {
                                for s in e.tunels.iter() 
                                {
                                    let id = *self.ids.get(s).unwrap_or(&0);
                                    if id>255
                                    {
                                        println!("error:{}",id);
                                    }
                                    e.tunels3.push(id as u8);
                                }
                            }            
                        ).collect::<Vec<_>>();

                        //println!("des {:#?}",self.v3);

                        //println!("full {}",self.full);
                        //println!("golden {}",self.golden);

    }


    fn simulate(&self,memory:&mut HashMap<(u8,usize,String),usize>,time:u8,opended:usize,act:String,flow: u8,total:usize)->usize
    {
        
        let key = (time,opended,act.clone());
        let v   = *memory.get(&key).unwrap_or(&0);
        let _nnn = act.clone();
        
        if total + (flow as usize)*(31-time as usize)<v 
        {
            return v;
        }

        let res = 
            if time==29
            {
                total + flow as usize
            }
              else
            {
                let new_flow = self.v.get(&act).unwrap().flow;

                if opended==self.golden
                {
                    self.simulate(memory,time+1,opended ,act,flow ,total + flow as usize)
                }
                else
                {
                    let bit = 1<<(*self.ids.get(&act).unwrap() as usize);
                   
                    let mut best = usize::MIN;
                    let nn = act.clone();

                    if (opended & bit==0) && new_flow>0
                    {            
                        best = self.simulate(memory,time+1,opended | bit,act,flow + new_flow as u8,total+flow as usize)
                    }

                    for exit in self.v.get(&nn).unwrap().tunels.iter() 
                    {
                        let r = self.simulate(memory,time+1,opended,exit.clone(),flow,total+flow as usize);
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
         //   println!("time:{} limi:{} opened:{} flow:{} total:{} act:{}",time+1,opended,flow,total,nnn);
            memory.insert(key,res);
        }
        
        res
    }

    fn simulate4(&self,memory:&mut HashMap<(u8,u8,u8,u16,usize),u16>,
                 time:u8,opended:usize,act1:u8,act2:u8,flow:u16,total:u16,left:u16)->u16
    {
        let add = ((self.time_lim as i64 -time as i64 +1) as u16)*flow;

        let total = total + add;
        let left  = left - flow;

        let (u1,u2) = if act1>act2 { (act1,act2) } else { (act2,act1) };
        // let full = ((act1)<<4) + act2;// ((act1.max(act2))*16 + act1.min(act2)) as u8;
        let key  = (time,u1,u2,total,opended);

        if memory.get(&key).is_some()
        {
          return *memory.get(&key).unwrap();
        }

        let _v = *memory.get(&key).unwrap_or(&0) as u16;
        let rec_key = (255u8,255u8,255u8,65535,65535);
        let rec = *memory.get(&rec_key).unwrap_or(&0);

        if (total + ((self.time_lim-time+1) as u16)*left)<rec
        {
            //memory.insert(key,0);
            return 0;
        }

        let res: u16 = 
        if time>=self.time_lim
        {
            total//- ((self.time_lim-time) as u16)*flow
        }
        else
        {
            if opended==self.golden
            {
                self.simulate4(memory,time+1,opended ,act1,act2,0 ,total,left)
            }
            else
            {
                let bit1 = 1usize<<(act1 as usize);
                let bit2 = 1usize<<(act2 as usize);
                
                let new_flow1 = self.v3[act1 as usize].flow;
                let new_flow2 = self.v3[act2 as usize].flow;
                
                let t1 = &self.v3[act1 as usize].tunels3;
                let t2 = &self.v3[act2 as usize].tunels3;
                let mut best = u16::MIN;

                if bit1!=bit2 && new_flow1>0 && new_flow2>0 && 
                    ((opended & bit1)==0) && 
                    ((opended & bit2)==0) 
                {
                    let r = self.simulate4(memory,time+1,opended | bit1 | bit2, act1, act2, new_flow1 + new_flow2,total,left);
                    best = best.max(r);
                }
                
                if new_flow1>0 && ((opended & bit1)==0) 
                {            
                    for e2 in t2
                    {
                        let r = self.simulate4(memory,time+1,opended | bit1    ,act1,*e2  , new_flow1,total,left);
                        best = best.max(r);
                    }
                }
                else
                if new_flow2>0 && ((opended & bit2)==0)
                {            
                    for e1 in t1
                    {
                        let r = self.simulate4(memory,time+1,opended | bit2     ,*e1  ,act2, new_flow2,total,left);
                        best = best.max(r);
                    }
                }
                 
                {
                    for e1 in t1
                    {
                        if *e1 !=act1 {
                            for e2 in t2
                            {
                                if *e2 !=act1 && !(*e1==act2  && *e2==act1)                                
                                {
                                    //if time==2 { println!("{} time:{} opened:{} flow:{} total:{} key:{} ",best,time+1,opended,flow,total,full); }
                                    let r = self.simulate4(memory,time+1,opended,*e1 ,*e2 ,0,total,left);
                                    best = best.max(r);
                                }
                            }
                        }
                    }
                }
                best
            }
        };

    //  if res>v 
    // {
        if time>=self.time_lim
        {
            //let rec_key = (255,255,255,65535,65535);
            //let rec = *memory.get(&rec_key).unwrap_or(&0);

            if res>rec
            {
                memory.insert(rec_key,res);
                println!("res: {} ",res);
                println!("{} time:{} limit:{} opened:{} flow:{} total:{} ",res, time,self.time_lim,opended,flow,total);
                //println!("time:{} ",now.to_string());
                //println!("time:{} opened:{} flow:{} total:{} key:{} ",time+1,opended,flow,total,full);
            }
        }

    //HashMap<(usize,usize,usize),usize>

    //println!("{:?}={}",key,res);
    memory.insert(key,res);

    res
}



}


//part2: 2304
//Elapsed: 667.74005 secs

pub fn part1(data:&[String])->usize
{
    let mut w = World::new(30);
    w.load(data);
    let mut memory = HashMap::new();
    let res = w.simulate(&mut memory,0,0,"AA".to_string(),0,0);

    res
}

pub fn part2(data:&[String],limit:u8)->u16
{
    let mut w = World::new(limit);
    w.load(data);
    let mut memory2 = HashMap::new();
//    let res = w.simulate2(&mut memory2,0,0,"AA".to_string(),"AA".to_string(),0,0);

    let start_id = *w.ids.get("AA").unwrap() as u8;
    let res = w.simulate4(&mut memory2,1,0,start_id,start_id,0,0,w.full as u16);

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
    assert_eq!(part2(&v,26),1707+0);
}


#[test]
fn p2_1()
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
fn p2_2()
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
fn p2big()
{
    let v = 
    vec![
        "Valve GJ has flow rate=14; tunnels lead to valves UV, AO, MM, UD, GM".to_string(),
        "Valve HE has flow rate=0; tunnels lead to valves QE, SV".to_string(),
        "Valve ET has flow rate=0; tunnels lead to valves LU, SB".to_string(),
        "Valve SG has flow rate=0; tunnels lead to valves FF, SB".to_string(),
        "Valve LC has flow rate=0; tunnels lead to valves QJ, GM".to_string(),
        "Valve EE has flow rate=13; tunnels lead to valves RE, BR".to_string(),
        "Valve AA has flow rate=0; tunnels lead to valves QC, ZR, NT, JG, FO".to_string(),
        "Valve TF has flow rate=0; tunnels lead to valves LU, MM".to_string(),
        "Valve GO has flow rate=0; tunnels lead to valves LB, AH".to_string(),
        "Valve QE has flow rate=24; tunnels lead to valves LG, HE".to_string(),
        "Valve MI has flow rate=0; tunnels lead to valves KU, FF".to_string(),
        "Valve BR has flow rate=0; tunnels lead to valves HY, EE".to_string(),
        "Valve UV has flow rate=0; tunnels lead to valves GP, GJ".to_string(),
        "Valve EH has flow rate=0; tunnels lead to valves UU, FF".to_string(),
        "Valve WK has flow rate=0; tunnels lead to valves HY, EL".to_string(),
        "Valve NT has flow rate=0; tunnels lead to valves FF, AA".to_string(),
        "Valve KI has flow rate=0; tunnels lead to valves OQ, AO".to_string(),
        "Valve AH has flow rate=22; tunnels lead to valves GO, RE".to_string(),
        "Valve EL has flow rate=0; tunnels lead to valves WK, SQ".to_string(),
        "Valve GP has flow rate=0; tunnels lead to valves SB, UV".to_string(),
        "Valve GM has flow rate=0; tunnels lead to valves LC, GJ".to_string(),
        "Valve LU has flow rate=9; tunnels lead to valves UU, DW, TF, ET, ML".to_string(),
        "Valve LB has flow rate=0; tunnels lead to valves GO, VI".to_string(),
        "Valve QC has flow rate=0; tunnels lead to valves ML, AA".to_string(),
        "Valve JJ has flow rate=0; tunnels lead to valves QJ, DV".to_string(),
        "Valve MM has flow rate=0; tunnels lead to valves TF, GJ".to_string(),
        "Valve VI has flow rate=18; tunnel leads to valve LB".to_string(),
        "Valve NV has flow rate=0; tunnels lead to valves SB, KU".to_string(),
        "Valve VT has flow rate=0; tunnels lead to valves HY, JG".to_string(),
        "Valve RE has flow rate=0; tunnels lead to valves AH, EE".to_string(),
        "Valve FO has flow rate=0; tunnels lead to valves SB, AA".to_string(),
        "Valve DV has flow rate=10; tunnels lead to valves JH, UD, JJ".to_string(),
        "Valve SQ has flow rate=12; tunnels lead to valves EL, QA".to_string(),
        "Valve OQ has flow rate=23; tunnels lead to valves KI, IV, JS".to_string(),
        "Valve FF has flow rate=3; tunnels lead to valves EU, NT, SG, MI, EH".to_string(),
        "Valve IV has flow rate=0; tunnels lead to valves LG, OQ".to_string(),
        "Valve HY has flow rate=8; tunnels lead to valves VT, BR, WK".to_string(),
        "Valve ML has flow rate=0; tunnels lead to valves LU, QC".to_string(),
        "Valve JS has flow rate=0; tunnels lead to valves EM, OQ".to_string(),
        "Valve KU has flow rate=5; tunnels lead to valves MI, VL, NV, HU, DW".to_string(),
        "Valve QA has flow rate=0; tunnels lead to valves OS, SQ".to_string(),
        "Valve EU has flow rate=0; tunnels lead to valves FF, OS".to_string(),
        "Valve SV has flow rate=0; tunnels lead to valves QJ, HE".to_string(),
        "Valve JG has flow rate=0; tunnels lead to valves AA, VT".to_string(),
        "Valve DW has flow rate=0; tunnels lead to valves LU, KU".to_string(),
        "Valve UD has flow rate=0; tunnels lead to valves DV, GJ".to_string(),
        "Valve QJ has flow rate=17; tunnels lead to valves JJ, SV, LC, EM, YA".to_string(),
        "Valve HU has flow rate=0; tunnels lead to valves JH, KU".to_string(),
        "Valve ZR has flow rate=0; tunnels lead to valves AA, VL".to_string(),
        "Valve YA has flow rate=0; tunnels lead to valves QJ, OS".to_string(),
        "Valve JH has flow rate=0; tunnels lead to valves HU, DV".to_string(),
        "Valve OS has flow rate=15; tunnels lead to valves EU, YA, QA".to_string(),
        "Valve LG has flow rate=0; tunnels lead to valves QE, IV".to_string(),
        "Valve SB has flow rate=4; tunnels lead to valves FO, SG, NV, GP, ET".to_string(),
        "Valve UU has flow rate=0; tunnels lead to valves EH, LU".to_string(),
        "Valve VL has flow rate=0; tunnels lead to valves ZR, KU".to_string(),
        "Valve AO has flow rate=0; tunnels lead to valves GJ, KI".to_string(),
        "Valve EM has flow rate=0; tunnels lead to valves QJ, JS".to_string(),
        ];
    assert_eq!(part2(&v,10),196);
}


#[test]
fn p1big()
{
    let v = 
    vec![
        "Valve GJ has flow rate=14; tunnels lead to valves UV, AO, MM, UD, GM".to_string(),
        "Valve HE has flow rate=0; tunnels lead to valves QE, SV".to_string(),
        "Valve ET has flow rate=0; tunnels lead to valves LU, SB".to_string(),
        "Valve SG has flow rate=0; tunnels lead to valves FF, SB".to_string(),
        "Valve LC has flow rate=0; tunnels lead to valves QJ, GM".to_string(),
        "Valve EE has flow rate=13; tunnels lead to valves RE, BR".to_string(),
        "Valve AA has flow rate=0; tunnels lead to valves QC, ZR, NT, JG, FO".to_string(),
        "Valve TF has flow rate=0; tunnels lead to valves LU, MM".to_string(),
        "Valve GO has flow rate=0; tunnels lead to valves LB, AH".to_string(),
        "Valve QE has flow rate=24; tunnels lead to valves LG, HE".to_string(),
        "Valve MI has flow rate=0; tunnels lead to valves KU, FF".to_string(),
        "Valve BR has flow rate=0; tunnels lead to valves HY, EE".to_string(),
        "Valve UV has flow rate=0; tunnels lead to valves GP, GJ".to_string(),
        "Valve EH has flow rate=0; tunnels lead to valves UU, FF".to_string(),
        "Valve WK has flow rate=0; tunnels lead to valves HY, EL".to_string(),
        "Valve NT has flow rate=0; tunnels lead to valves FF, AA".to_string(),
        "Valve KI has flow rate=0; tunnels lead to valves OQ, AO".to_string(),
        "Valve AH has flow rate=22; tunnels lead to valves GO, RE".to_string(),
        "Valve EL has flow rate=0; tunnels lead to valves WK, SQ".to_string(),
        "Valve GP has flow rate=0; tunnels lead to valves SB, UV".to_string(),
        "Valve GM has flow rate=0; tunnels lead to valves LC, GJ".to_string(),
        "Valve LU has flow rate=9; tunnels lead to valves UU, DW, TF, ET, ML".to_string(),
        "Valve LB has flow rate=0; tunnels lead to valves GO, VI".to_string(),
        "Valve QC has flow rate=0; tunnels lead to valves ML, AA".to_string(),
        "Valve JJ has flow rate=0; tunnels lead to valves QJ, DV".to_string(),
        "Valve MM has flow rate=0; tunnels lead to valves TF, GJ".to_string(),
        "Valve VI has flow rate=18; tunnel leads to valve LB".to_string(),
        "Valve NV has flow rate=0; tunnels lead to valves SB, KU".to_string(),
        "Valve VT has flow rate=0; tunnels lead to valves HY, JG".to_string(),
        "Valve RE has flow rate=0; tunnels lead to valves AH, EE".to_string(),
        "Valve FO has flow rate=0; tunnels lead to valves SB, AA".to_string(),
        "Valve DV has flow rate=10; tunnels lead to valves JH, UD, JJ".to_string(),
        "Valve SQ has flow rate=12; tunnels lead to valves EL, QA".to_string(),
        "Valve OQ has flow rate=23; tunnels lead to valves KI, IV, JS".to_string(),
        "Valve FF has flow rate=3; tunnels lead to valves EU, NT, SG, MI, EH".to_string(),
        "Valve IV has flow rate=0; tunnels lead to valves LG, OQ".to_string(),
        "Valve HY has flow rate=8; tunnels lead to valves VT, BR, WK".to_string(),
        "Valve ML has flow rate=0; tunnels lead to valves LU, QC".to_string(),
        "Valve JS has flow rate=0; tunnels lead to valves EM, OQ".to_string(),
        "Valve KU has flow rate=5; tunnels lead to valves MI, VL, NV, HU, DW".to_string(),
        "Valve QA has flow rate=0; tunnels lead to valves OS, SQ".to_string(),
        "Valve EU has flow rate=0; tunnels lead to valves FF, OS".to_string(),
        "Valve SV has flow rate=0; tunnels lead to valves QJ, HE".to_string(),
        "Valve JG has flow rate=0; tunnels lead to valves AA, VT".to_string(),
        "Valve DW has flow rate=0; tunnels lead to valves LU, KU".to_string(),
        "Valve UD has flow rate=0; tunnels lead to valves DV, GJ".to_string(),
        "Valve QJ has flow rate=17; tunnels lead to valves JJ, SV, LC, EM, YA".to_string(),
        "Valve HU has flow rate=0; tunnels lead to valves JH, KU".to_string(),
        "Valve ZR has flow rate=0; tunnels lead to valves AA, VL".to_string(),
        "Valve YA has flow rate=0; tunnels lead to valves QJ, OS".to_string(),
        "Valve JH has flow rate=0; tunnels lead to valves HU, DV".to_string(),
        "Valve OS has flow rate=15; tunnels lead to valves EU, YA, QA".to_string(),
        "Valve LG has flow rate=0; tunnels lead to valves QE, IV".to_string(),
        "Valve SB has flow rate=4; tunnels lead to valves FO, SG, NV, GP, ET".to_string(),
        "Valve UU has flow rate=0; tunnels lead to valves EH, LU".to_string(),
        "Valve VL has flow rate=0; tunnels lead to valves ZR, KU".to_string(),
        "Valve AO has flow rate=0; tunnels lead to valves GJ, KI".to_string(),
        "Valve EM has flow rate=0; tunnels lead to valves QJ, JS".to_string(),
        ];
    assert_eq!(part2(&v,5),17);
}

#[test]
fn p0big()
{
    let v = 
    vec![
        "Valve GJ has flow rate=14; tunnels lead to valves UV, AO, MM, UD, GM".to_string(),
        "Valve HE has flow rate=0; tunnels lead to valves QE, SV".to_string(),
        "Valve ET has flow rate=0; tunnels lead to valves LU, SB".to_string(),
        "Valve SG has flow rate=0; tunnels lead to valves FF, SB".to_string(),
        "Valve LC has flow rate=0; tunnels lead to valves QJ, GM".to_string(),
        "Valve EE has flow rate=13; tunnels lead to valves RE, BR".to_string(),
        "Valve AA has flow rate=0; tunnels lead to valves QC, ZR, NT, JG, FO".to_string(),
        "Valve TF has flow rate=0; tunnels lead to valves LU, MM".to_string(),
        "Valve GO has flow rate=0; tunnels lead to valves LB, AH".to_string(),
        "Valve QE has flow rate=24; tunnels lead to valves LG, HE".to_string(),
        "Valve MI has flow rate=0; tunnels lead to valves KU, FF".to_string(),
        "Valve BR has flow rate=0; tunnels lead to valves HY, EE".to_string(),
        "Valve UV has flow rate=0; tunnels lead to valves GP, GJ".to_string(),
        "Valve EH has flow rate=0; tunnels lead to valves UU, FF".to_string(),
        "Valve WK has flow rate=0; tunnels lead to valves HY, EL".to_string(),
        "Valve NT has flow rate=0; tunnels lead to valves FF, AA".to_string(),
        "Valve KI has flow rate=0; tunnels lead to valves OQ, AO".to_string(),
        "Valve AH has flow rate=22; tunnels lead to valves GO, RE".to_string(),
        "Valve EL has flow rate=0; tunnels lead to valves WK, SQ".to_string(),
        "Valve GP has flow rate=0; tunnels lead to valves SB, UV".to_string(),
        "Valve GM has flow rate=0; tunnels lead to valves LC, GJ".to_string(),
        "Valve LU has flow rate=9; tunnels lead to valves UU, DW, TF, ET, ML".to_string(),
        "Valve LB has flow rate=0; tunnels lead to valves GO, VI".to_string(),
        "Valve QC has flow rate=0; tunnels lead to valves ML, AA".to_string(),
        "Valve JJ has flow rate=0; tunnels lead to valves QJ, DV".to_string(),
        "Valve MM has flow rate=0; tunnels lead to valves TF, GJ".to_string(),
        "Valve VI has flow rate=18; tunnel leads to valve LB".to_string(),
        "Valve NV has flow rate=0; tunnels lead to valves SB, KU".to_string(),
        "Valve VT has flow rate=0; tunnels lead to valves HY, JG".to_string(),
        "Valve RE has flow rate=0; tunnels lead to valves AH, EE".to_string(),
        "Valve FO has flow rate=0; tunnels lead to valves SB, AA".to_string(),
        "Valve DV has flow rate=10; tunnels lead to valves JH, UD, JJ".to_string(),
        "Valve SQ has flow rate=12; tunnels lead to valves EL, QA".to_string(),
        "Valve OQ has flow rate=23; tunnels lead to valves KI, IV, JS".to_string(),
        "Valve FF has flow rate=3; tunnels lead to valves EU, NT, SG, MI, EH".to_string(),
        "Valve IV has flow rate=0; tunnels lead to valves LG, OQ".to_string(),
        "Valve HY has flow rate=8; tunnels lead to valves VT, BR, WK".to_string(),
        "Valve ML has flow rate=0; tunnels lead to valves LU, QC".to_string(),
        "Valve JS has flow rate=0; tunnels lead to valves EM, OQ".to_string(),
        "Valve KU has flow rate=5; tunnels lead to valves MI, VL, NV, HU, DW".to_string(),
        "Valve QA has flow rate=0; tunnels lead to valves OS, SQ".to_string(),
        "Valve EU has flow rate=0; tunnels lead to valves FF, OS".to_string(),
        "Valve SV has flow rate=0; tunnels lead to valves QJ, HE".to_string(),
        "Valve JG has flow rate=0; tunnels lead to valves AA, VT".to_string(),
        "Valve DW has flow rate=0; tunnels lead to valves LU, KU".to_string(),
        "Valve UD has flow rate=0; tunnels lead to valves DV, GJ".to_string(),
        "Valve QJ has flow rate=17; tunnels lead to valves JJ, SV, LC, EM, YA".to_string(),
        "Valve HU has flow rate=0; tunnels lead to valves JH, KU".to_string(),
        "Valve ZR has flow rate=0; tunnels lead to valves AA, VL".to_string(),
        "Valve YA has flow rate=0; tunnels lead to valves QJ, OS".to_string(),
        "Valve JH has flow rate=0; tunnels lead to valves HU, DV".to_string(),
        "Valve OS has flow rate=15; tunnels lead to valves EU, YA, QA".to_string(),
        "Valve LG has flow rate=0; tunnels lead to valves QE, IV".to_string(),
        "Valve SB has flow rate=4; tunnels lead to valves FO, SG, NV, GP, ET".to_string(),
        "Valve UU has flow rate=0; tunnels lead to valves EH, LU".to_string(),
        "Valve VL has flow rate=0; tunnels lead to valves ZR, KU".to_string(),
        "Valve AO has flow rate=0; tunnels lead to valves GJ, KI".to_string(),
        "Valve EM has flow rate=0; tunnels lead to valves QJ, JS".to_string(),
        ];
    assert_eq!(part2(&v,4),7);
}
