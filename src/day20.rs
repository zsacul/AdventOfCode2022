use super::cycliclist::CyclicList;
use std::collections::HashSet;


fn code(n:i128,pos:usize)->i128
{
    let bit = 1i128 << 63; 
    //return n;
    if n<0 
    {
        (-n) | ((pos as i128)<<64) as i128 | bit
    }
      else 
    {
        (n ) | ((pos as i128)<<64) as i128
    }
    
}

fn decode(n:i128)->i128
{
    //return n;
    let bit = 1i128 << 63; 
    let mask = (1i128<<63)-1;
    if (n & bit)==bit
    {
        -(n&mask)
    }
      else
    {
        n&mask
    }
}

pub fn solve1(data:&[String],moves:usize)->i128
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

    
    println!("dec {}",decode(92135424));
    println!("zer {}",zero_id);
    


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

    
}


pub fn solve2(data:&[String],moves:usize)->i128
{
    let mut table = CyclicList::new();

    let mut moves = vec![];

    let mut hh = HashSet::new();

    let mut id = 1;
    let mut zero_id = -999;

 

    for line in data.iter() 
    {
        let v = line.parse::<i128>().unwrap()*811589153;



        let cc = code(v,id);

        table.push_right(cc);       
           hh.insert(cc);
        moves.push(cc);

        if v==0 { zero_id = cc; }
        id+=1;
    }



    for _ in 0..10 
    {
        for &d in moves.iter()
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
                    let nn = (n)%(table.len() as i128);
                    for _ in 0..nn as usize {table.right();}
                }
                else
                {
                    let nn = (-n)%(table.len() as i128);
                    for _ in 0..nn as usize {table.left();}
                }
                
                table.push_right(org);
                //println!("after {}",d);
            // table.print();
            }

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

}




pub fn solve(data:&[String])
{
    println!("Day 20");
    println!("part1: {}",solve1(data,3));
    println!("part2: {}",solve2(data,3));    
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
