use std::cmp::Ordering;

#[derive(Debug)]
enum Element {
    Empty,
    Value(i32),
    Table(Vec<String>),
}

fn matching_bracket(line:String)->usize
{
    let mut opened_brackets = 1;

    if line.starts_with('[')
    {
        let bracket_start = line.find('[').unwrap();

        for i in bracket_start+1..line.len() 
        {
            let c = line.chars().nth(i).unwrap();
            
            if c=='[' { opened_brackets+=1;                 }
            if c==']' { opened_brackets-=1;
                        if opened_brackets==0 { return i; } }
        }   
    }
    0
}

fn get_table(s:&str)->Vec<String>
{
    let mut tab = vec![];
    let mut s = s;

    while s.find(',').is_some()
    {
        let coma = s.find(',').unwrap();

        if s.starts_with('[')
        {
            let bracket_end = matching_bracket(s.to_string());          

            tab.push(s[0..bracket_end+1].to_string());
            s = &s[bracket_end+1..];

            if s.find(',').unwrap_or(999)==0
            {
                s = &s[1..];
            }
        }
          else
        {
            let m = s[..coma].to_string();
            tab.push(m);
            s = &s[coma+1..];
        }
    }

    if !s.is_empty()
    {
        tab.push(s.to_string());
    }

    tab
}

fn get_type(s:&str)->Element
{
    let line = s;

    if line.is_empty()   
    {  
                            Element::Empty
    }
    else if line.starts_with('[')
    {
        let bracket_end = matching_bracket(line.to_string());
        let vv = if bracket_end>=line.len()-1 { get_table(&line[1..bracket_end]) }
                                         else { get_table(line)                  };

        if vv.is_empty() {  Element::Empty     }
                    else {  Element::Table(vv) }
    }
    else if s.find(',').is_none()
    {
        Element::Value(s.parse::<i32>().unwrap())
    }
        else
    {
        let vv = get_table(line);
        if vv.is_empty() { Element::Empty      }
                    else { Element::Table(vv)  }
    }
}

fn compare2(a:&str,b:&str)->Ordering
{
    match compare(a,b)
    {
        -1 => Ordering::Greater,
         1 => Ordering::Less, 
         _ => Ordering::Equal,    
    }
}

//<1
//=0
//>-1
fn compare(str1:&str,str2:&str)->i32
{
    match (get_type(str1),get_type(str2))
    {
        (Element::Empty         ,Element::Empty    ) =>  {                0 },
        (Element::Empty         ,             _    ) =>  {                1 },
        (             _         ,Element::Empty    ) =>  {               -1 },
        (Element::Value(v1)     ,Element::Value(v2)) =>  { (v2-v1).signum() },
        (Element::Table(t1)     ,Element::Table(t2)) => 
        {
            for i in 0..t1.len().min(t2.len())
            {
                let r = compare(&t1[i][..], &t2[i][..]);
                if r!=0          { return  r; }
            }          
            if t1.len()<t2.len() { return  1; }
            if t1.len()>t2.len() { return -1; }
            0
        },  
        (Element::Table(t1)     ,Element::Value(_s2)) => 
        {
            let r = compare(&t1[0][..],str2);
            if r!=0              { return  r; }
            if t1.len()>1        { return -1; }
            0
        },
        (Element::Value(_s1)    ,Element::Table(t2) ) => 
        {
            let r = compare(str1,&t2[0][..]);
            if r!=0              { return  r; }
            if t2.len()>1        { return  1; }
            0
        }
    }
}

fn compute(data:&[String])->usize
{       
    (0..data.len()).step_by(3)
                   .enumerate()
                   .map(|(id,i)| if compare(&data[i][..],&data[i+1][..]) >= 0 { id + 1 } 
                                                                         else {      0 } )
                   .sum()
}

pub fn part1(data:&[String])->usize
{
    compute(data)
}

pub fn part2(data:&[String])->usize
{
    let mut data2 = vec![];
    for d in data 
    {
        if !d.is_empty() { data2.push(&d[..]) }        
    }

    data2.push("[[2]]");
    data2.push("[[6]]");
    
    data2.sort_by(|a,b| compare2(a,b));

    (data2.iter().position(|s| s==&"[[2]]").unwrap() as usize+1) *
    (data2.iter().position(|s| s==&"[[6]]").unwrap() as usize+1)
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day 13");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
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
    assert_eq!(part2(&v),140);
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


#[test]
fn test_9()
{
    let v = 
    vec![
        "[[[],10,[[6],[]],3,1]]".to_string(),
        "[[6,8,[9,2],7,2],[[10,[4],[6,2,9,2],[8,8,9]],8,[4],[[0,2]]]]".to_string(),
        ];
         
    assert_eq!(part1(&v),1);
}

#[test]
fn test_10()
{
    let v = 
    vec![
    "[[10,[[2,3,9,10],[8,4,3,8]],2,2,[8,8,2,[2,0,9]]],[[[2,0,7,4,2]],9,[[3,8,7,3]],3,9]]".to_string(),
    "[[1],[[]],[1,[]]]".to_string(),
    ];
         
    assert_eq!(part1(&v),0);
}


#[test]
fn test_table_1()
{
    assert_eq!(get_table("[1],[2,3,4]"),vec!["[1]".to_string(),"[2,3,4]".to_string()]);
}

#[test]
fn test_table_2()
{
    assert_eq!(get_table("[1],4"),vec!["[1]".to_string(),"4".to_string()]);
}

#[test]
fn test_table_3()
{
    assert_eq!(get_table("[1,4,5],4,6"),vec!["[1,4,5]".to_string(),"4".to_string(),"6".to_string()]);
}


#[test]
fn test_add_1() {
    assert_eq!(compare("[1]" , "[2]") ,  1);
    assert_eq!(compare("[2]" , "[1]") , -1);
    assert_eq!(compare("[10]", "[9]") , -1);
    assert_eq!(compare("[1]" ,"[10]") ,  1);
}

#[test]
fn test_add_2() {
    assert_eq!(compare("[[1]]","[[2]]"),  1);
    assert_eq!(compare("[[2]]","[[1]]"), -1);
}

#[test]
fn test_add_3() {
    assert_eq!(compare("[[1,2]]","[[2]]"  ), 1);
    assert_eq!(compare("[[1]]"  ,"[[1,2]]"), 1);
    assert_eq!(compare("[[1,2]]","[[2,1]]"), 1);
}
 
#[test]
fn test_add_4() {
    assert_eq!(compare("[1,1,3,1,1]","[1,1,5,1,1]"),1);
}

#[test]
fn test_add_5() {
    assert_eq!(compare("[[1],[2,3,4]]","[[1],4]"),1);
}

#[test]
fn test_add_6() {
    assert_eq!(compare("[9]","[[8,7,6]]"),-1); 
}

#[test]
fn test_add_7() {
    assert_eq!(compare("[[4,4],4,4]","[[4,4],4,4,4]"),1);
}

#[test]
fn test_add_8() {
    assert_eq!(compare("[7,7,7,7]","[7,7,7]"),-1);
}

#[test]
fn test_add_9() {
    assert_eq!(compare("[]","[3]"),1);
}

#[test]
fn test_add_10() {
    assert_eq!(compare("[[[]]]","[[]]"),-1);
}
