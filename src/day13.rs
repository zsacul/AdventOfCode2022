#[derive(Debug)]
enum Element {
    Empty,
    Value(i32),
    List(String),
    Table(Vec<String>),
}

fn get(s:&str)->Element
{
    let line = s;

    if line.len()==0 
    {
        return Element::Empty;
    }
      else 
    {
        if s.chars().nth(0).unwrap()=='['
        {
            let bracket_start = line.find('[').unwrap();
            let mut bracket_end = 0;
            let mut opened_brackets = 1;
    
            //find matching bracket
            for i in bracket_start+1..line.len() {
                let c = line.chars().nth(i).unwrap();
                if c=='[' {
                    opened_brackets+=1;
                }
                if c==']' {
                    opened_brackets-=1;
                    if opened_brackets==0 {
                        bracket_end = i;
                        break;
                    }
                }
            }   
            let m = &line[bracket_start+1..bracket_end].to_string();
            return Element::List(m.to_string());
            //ns.push_str(&eval().to_string()[..]);
        }
        else
        {

            if s.find(',')==None
            {
                return Element::Value(s.parse::<i32>().unwrap());
            }
              else
            {
                let tab : Vec<String> = s.split(',')
                .map(|s| s.to_string())
                .collect(); 
    
                dbg!(tab.clone());
                    //let mut v = vec![];
                return Element::Table(tab);
            }
//            if s.chars().nth(0).unwrap()=='['

  //          dbg!(s);
            
        }
    }

    //return Element::Error;
}


fn eval(s:&str)->String
{
    let mut line = s.to_string();//(*l).clone();

    //eval and replace all equations in brackets first
    while line.find('[')!=None
    {
        let bracket_start = line.find('[').unwrap();
        let mut bracket_end = 0;
        let mut opened_brackets = 1;

        //find matching bracket
        for i in bracket_start+1..line.len() {
            let c = line.chars().nth(i).unwrap();
            if c=='[' {
                opened_brackets+=1;
            }
            if c==']' {
                opened_brackets-=1;
                if opened_brackets==0 {
                    bracket_end = i;
                    break;
                }
            }
        }

        let mut ns = line[..bracket_start].to_string();        
        ns.push_str(&eval(&line[bracket_start+1..bracket_end].to_string()).to_string()[..]);
        ns.push_str("*");
        ns.push_str(&line[bracket_end+1..].to_string());

        line = ns;
    }
    line

}

fn compare(d:usize,str1:&str,str2:&str)->i32
{
    println!(">{}< ? >{}<",str1,str2);


    //if d>5 {
      //  return -999;
    //}

    if str1==str2 { return 0; }

    let e1 = get(str1);
    let e2 = get(str2);

    let pair = (e1,e2);

    //if e1==e2 { return 0; }

    match pair
    {
        (Element::Empty,_) => { println!(">emptyl"); return  1; },
        (_ ,Element::Empty) => { println!(">emptyr"); return -1; },
        (Element::List(s1) ,Element::List(s2) ) => 
        { 
            println!(">list");
            return compare(d+1,&s1[..],&s2[..])
        },
        (Element::Value(v1),Element::Value(v2)) =>   
        { 
            let rr = (v2-v1).signum(); 
            dbg!(rr);
            return rr; 
        },
        (Element::List(s1) ,Element::Value(_v2)) => { return compare(d+1 ,&s1[..],str2)},
        (Element::Value(_v1),Element::List(s2)  ) => { return compare(d+1,   str1,&s2[..])},
        (Element::Table(t1),Element::Table(t2)) => 
        {
            println!("t1 {} t2 {}",t1.len(),t2.len());

            if t1.len()<t2.len() { return -1; }
            if t2.len()>t1.len() { return  1; }
            for i in 0..t1.len() {
                let r = compare(d+1,&t1[i][..], &t2[i][..]);
                if r!=0 {
                    println!("diff {}",r);
                    return r;
                }
                println!("same {}",i);
            }
            return 0;
        },
        //_ => {-999}
        
        (Element::Table(t1),_) => {
            //for v in t1 
            //{
            //    let res = compare(   &v[..],str2);
            //    if res!=0 {return res;}
            //}
            println!("elo1");
            return 0;
        }
        (_,Element::Table(t2)) => {
            println!("elo2");
            //for v in t2 
            //{
            //    let res = compare(  str1, &v[..]);
            //    if res!=0 {return res;}
            //}
            return 0;
        }
        
    }

/*
    if e1==Element::Empty { return  1; }
    if e2==Element::Empty { return -1; }

    if e1==Element::Value(v1) && e2==Element::Value(v2)
    {
        if v1==v2 { return  0;}
        if v1 >v2 { return  1;}
        if v1 <v2 { return -1;}
    }


    println!("{} = {:?}",s1,e1);
    println!("{} = {:?}",s2,e2);
     */    
    //println!("{}",s2);
    //println!("{}",e2);
    //println!();
    
    //0
}

fn compute(data:&[String])->usize
{   
    let mut res = 0;
    for i in (0..data.len()).step_by(3)
    {
        let result = compare(0,&data[i][..],&data[i+1][..]);
        println!("{} {} = {}",&data[i][..],&data[i+1][..],result);
        if result > 0
        {
            res+=i/3+1;
        }
    }
    res
}

pub fn part1(data:&[String])->usize
{
    compute(data)
}

pub fn part2(data:&[String])->usize
{
    0
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day 13");
    println!("part1: {}",compute(data));
    println!("part2: {}",compute(data));
}

#[test]
fn test11full()
{
    let v = 
    vec![
        "[1,1,3,1,1]".to_string(),
        "[1,1,5,1,1]".to_string(),
        "".to_string(),
        "[[1],[2,3,4]]".to_string(),
        "[[1],4]".to_string(),
        "".to_string(),
        "[9]".to_string(),
        "[[8,7,6]]".to_string(),
        "".to_string(),
        "[[4,4],4,4]".to_string(),
        "[[4,4],4,4,4]".to_string(),
        "".to_string(),
        "[7,7,7,7]".to_string(),
        "[7,7,7]".to_string(),
        "".to_string(),
        "[]".to_string(),
        "[3]".to_string(),
        "".to_string(),
        "[[[]]]".to_string(),
        "[[]]".to_string(),
        "".to_string(),
        "[1,[2,[3,[4,[5,6,7]]]],8,9]".to_string(),
        "[1,[2,[3,[4,[5,6,0]]]],8,9]".to_string(),
        ];
         
    assert_eq!(part1(&v),13);
}

#[test]
fn test22()
{
    let v =
        vec![
            "[1,1,3,1,1]".to_string(),
            "[1,1,5,1,1]".to_string(),
            "".to_string(),
            "[[1],[2,3,4]]".to_string(),
            "[[1],4]".to_string(),
            "".to_string(),
            "[9]".to_string(),
            "[[8,7,6]]".to_string(),
            "".to_string(),
            "[[4,4],4,4]".to_string(),
            "[[4,4],4,4,4]".to_string(),
            "".to_string(),
            "[7,7,7,7]".to_string(),
            "[7,7,7]".to_string(),
            "".to_string(),
            "[]".to_string(),
            "[3]".to_string(),
            "".to_string(),
            "[[[]]]".to_string(),
            "[[]]".to_string(),
            "".to_string(),
            "[1,[2,[3,[4,[5,6,7]]]],8,9]".to_string(),
            "[1,[2,[3,[4,[5,6,0]]]],8,9]".to_string(),
            ];
    assert_eq!(part2(&v),70);
}

#[test]
fn test_1()
{
    let v = 
    vec![
        "[1,1,3,1,1]".to_string(),
        "[1,1,5,1,1]".to_string(),
        "".to_string(),
        ];
         
    assert_eq!(part1(&v),1);
}

#[test]
fn test_2()
{
    let v = 
    vec![
        "[[1],[2,3,4]]".to_string(),
        "[[1],4]".to_string(),
        "".to_string(),
        ];
         
    assert_eq!(part1(&v),1);
}

#[test]
fn test_3()
{
    let v = 
    vec![
        "[9]".to_string(),
        "[[8,7,6]]".to_string(),
        "".to_string(),
        ];
         
    assert_eq!(part1(&v),0);
}

#[test]
fn test_4()
{
    let v = 
    vec![
        "[[4,4],4,4]".to_string(),
        "[[4,4],4,4,4]".to_string(),
        "".to_string(),
        ];
         
    assert_eq!(part1(&v),1);
}

#[test]
fn test_5()
{
    let v = 
    vec![
        "[7,7,7,7]".to_string(),
        "[7,7,7]".to_string(),
        "".to_string(),
        ];
         
    assert_eq!(part1(&v),0);
}

#[test]
fn test_6()
{
    let v = 
    vec![
        "[]".to_string(),
        "[3]".to_string(),
        "".to_string(),
        ];
         
    assert_eq!(part1(&v),1);
}

#[test]
fn test_7()
{
    let v = 
    vec![
        "[[[]]]".to_string(),
        "[[]]".to_string(),
        "".to_string(),
        ];
         
    assert_eq!(part1(&v),0);
}

#[test]
fn test_8()
{
    let v = 
    vec![
        "[1,[2,[3,[4,[5,6,7]]]],8,9]".to_string(),
        "[1,[2,[3,[4,[5,6,0]]]],8,9]".to_string(),
        ];
         
    assert_eq!(part1(&v),0);
}