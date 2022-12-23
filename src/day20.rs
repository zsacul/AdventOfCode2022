use super::cycliclist::CyclicList;

fn encode(n:i128,pos:usize)->i128
{
    const MINUS_BIT : i128 = 1i128 << 63; 

    if n<0 
    {
        (-n) | ((pos as i128)<<64) as i128 | MINUS_BIT
    }
      else 
    {
        ( n) | ((pos as i128)<<64) as i128
    }
}

fn decode(n:i128)->i128
{
    const MINUS_BIT : i128 =  1i128 << 63; 
    const MASK      : i128 = (1i128 << 63) - 1;
    
    if (n & MINUS_BIT)==MINUS_BIT
    {
        -(n & MASK)
    }
      else
    {
          n & MASK
    }
}

pub fn compute(data:&[String],multipler:i128,times:usize)->i128
{
    let mut list = CyclicList::new();

    let mut moves = vec![];
    let mut zero_id = 0;

    for (id,line) in data.iter().enumerate()
    {   
        let v    = line.parse::<i128>().unwrap();
        let code = encode(v*multipler,id);

        list.push_right(code);
        moves.push(code);

        if v==0 { zero_id = code; }        
    }

    for _ in 0..times 
    {
        for &d in moves.iter()
        {
            list.move_right_till_value(d);

            let org = list.pop().unwrap();
            let n   = decode(org);
            let m   = n.abs() % (list.len() as i128);

            if n>=0
            {
                for _ in 0..m as usize { list.right(); }
            }
              else
            {
                for _ in 0..m as usize { list.left();  }
            }
            
            list.push_right(org);
        }
    }

    list.move_right_till_value(zero_id);
        
    for _ in 0..1000 { list.right(); }
    let v1 = decode(list.peek().unwrap()); 
    
    for _ in 0..1000 { list.right(); }
    let v2 = decode(list.peek().unwrap());
    
    for _ in 0..1000 { list.right(); }
    let v3 = decode(list.peek().unwrap());
        
    v1+v2+v3
}

fn solve1(data:&[String])->usize
{
    compute(data,1,1) as usize
}

fn solve2(data:&[String])->usize
{
    compute(data,811589153,10) as usize
}

#[allow(unused)]
fn solve(data:&[String])
{
    println!("Day 20");
    println!("part1: {}",solve1(data));
    println!("part2: {}",solve2(data));    
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
    assert_eq!(solve1(&v),3);
}

#[test]
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
    assert_eq!(solve2(&v),1623178306);
}



#[test]
fn testdec()
{
    assert_eq!(decode(encode(    7,  30)),    7);
    assert_eq!(decode(encode( 1023,  30)), 1023);
    assert_eq!(decode(encode(-1023,  30)),-1023);
    assert_eq!(decode(encode(  774,3420)),  774);
    assert_eq!(decode(encode(-7007,  30)),-7007);
}
