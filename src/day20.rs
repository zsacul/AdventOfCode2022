use super::cycliclist::CyclicList;
use std::collections::HashSet;


fn code(n:i32,pos:usize)->i32
{
    //return n;
    if n<0 
    {
        (-n) | (pos<<18) as i32 | (1i32<<17)
    }
      else 
    {
        (n ) | (pos<<18) as i32
    }
    
}

fn decode(n:i32)->i32
{
    //return n;
    let bit = 1i32 << 17; 
    if (n & bit)==bit
    {
        -(n&65535)
    }
      else
    {
        n&65535
    }
}

pub fn solve1(data:&[String],moves:usize)->i32
{
    let mut table = CyclicList::new();

    let mut moves = vec![];

    let mut hh = HashSet::new();

    let mut id = 1;
    let mut zero_id = -999;

 

    for line in data.iter() 
    {
        let v = line.parse::<i128>().unwrap();



        let cc = code(v,id);

        table.push_right(cc);       
           hh.insert(cc);
        moves.push(cc);

        if v==0 { zero_id = cc; }
        id+=1;
    }

    println!("min {} max {}",minv,maxv);
    
    println!("dec {}",decode(92135424));
    println!("zer {}",zero_id);
    

    let mut vv = moves.clone();
    vv.sort();
    vv.dedup();

    println!("{} {}",moves.len(),vv.len());


    //table.print();
    //>5268



    for d in moves 
    {
        table.move_right_till_value(d);

        //if d!=0 
        {
            //println!("before {}",d);
          //  table.print();
            let org = table.pop().unwrap();
            let n = decode(org);

            if n>=0
            {
                for _ in 0..n as usize {table.right();}
            }
            else
            {
                for _ in 0..-n as usize {table.left();}
            }
            
            table.push_right(org);
            //println!("after {}",d);
           // table.print();
        }

    }

    table.move_right_till_value(zero_id);
//    table.print();
    
    let mut s =0;
    
    for x1 in 0..1000 { table.right(); }
    let v1 = decode(table.peek().unwrap()); 
    
    for x2 in 0..1000 { table.right(); }
    let v2 = decode(table.peek().unwrap());
    
    for x3 in 0..1000 { table.right(); }
    let v3 = decode(table.peek().unwrap());
    

    println!("vvv {} {} {}",v1,v2,v3);
    v1+v2+v3

/*
    table.right();
    const SHOW_DEBUG : bool = false;

    for _m in 1..=moves {

        if SHOW_DEBUG 
        {
            table.print();
        }
        
        let curr_val = table.peek().unwrap();
        table.right();
        let p1 = table.pop().unwrap(); table.right();
        let p2 = table.pop().unwrap(); table.right();
        let p3 = table.pop().unwrap(); table.right();

        let mut dest_v = curr_val-1;
        if dest_v<1 { dest_v = max_label; }

        while dest_v==p1 || dest_v==p2 || dest_v==p3
        {
            dest_v-=1;
            if dest_v<1 { dest_v = max_label; }
        }      

        table.move_right_till_value(dest_v);
        
        table.push_right(p1);
        table.push_right(p2);
        table.push_right(p3);

        table.move_right_till_value(curr_val);
        table.right();
    }
    
    let mut res ="".to_string();

    table.move_right_till_value(1);
    table.right();
     
    while table.peek().unwrap()!=1 
    {      
        res.push_str(&table.peek().unwrap().to_string()[..]);
        table.right();
    }
     */

    
}


pub fn solve2(data:&[String],moves:usize)->i64
{
   
   0
}



pub fn solve(data:&[String])
{
    println!("Day 20");
    println!("part1: {}",solve1(data,3));
    //println!("part2: {}",solve2(data,3));    
}


#[test]
fn test1()
{
    let v = vec![
        "1".to_string(),
        "2".to_string(),
        "-3".to_string(),
        "3".to_string(),
        "-2".to_string(),
        "0".to_string(),
        "4".to_string(),        
    ];
    assert_eq!(solve1(&v,3),3);
}


fn test2()
{
    let v = vec![
        "1".to_string(),
        "2".to_string(),
        "-3".to_string(),
        "3".to_string(),
        "-2".to_string(),
        "0".to_string(),
        "4".to_string(),        
    ];
    assert_eq!(solve2(&v,3),1623178306);
}



#[test]
fn testdec()
{
    assert_eq!(decode(code(7,30)),7);
    assert_eq!(decode(code(1023,30)),1023);
    assert_eq!(decode(code(-1023,30)),-1023);
    assert_eq!(decode(code(774,3420)),774);
    assert_eq!(decode(code(-7007,30)),-7007);
}
