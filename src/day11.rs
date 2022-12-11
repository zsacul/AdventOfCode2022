use super::tools;
use std::collections::VecDeque;

#[derive(Clone, Debug, Copy)]
struct Operation
{
    left  : i64,
    oper  : char,
    right : i64,
    acc   : i64,
}

impl Operation 
{
    fn new(line:&str)->Self
    {
        let line = line.replace("old","-1");
        let tokens : Vec<&str> = line.split(' ').collect(); 
        Self 
        {
            left  : tokens[0].parse().unwrap(), 
            oper  : tokens[1].chars().next().unwrap(),
            right : tokens[2].parse().unwrap(), 
            acc   : 0
        }
    }

    fn description(&self)->String
    {
        format!("{} {} {}",self.left,self.oper,self.right)
    }

    fn div(&mut self)
    {
        self.acc /= 3;
    }

    fn calc(&mut self)
    {
        let l = if self.left ==-1 { self.acc } else { self.left  };
        let r = if self.right==-1 { self.acc } else { self.right };

        match self.oper 
        {
            '+' => { self.acc = l + r },
            '-' => { self.acc = l - r },
            '*' => { self.acc = l * r },
            '/' => { self.acc = l / r },
            _   => panic!("unknown code"),            
        }

        self.acc %= 2*3*5*7*11*13*17*19*23;
    }
}

#[derive(Clone, Debug)]
struct Monkey{
    id         : usize,
    items      : VecDeque<i64>,
    operation  : Operation,
    test_div   : i64,
    test_true  : usize,
    test_false : usize,
    throws     : usize
}

impl Monkey 
{
    fn new(ids:&str,items:&str,operation:&str,test:&str,test_truerue:&str,test_falsealse:&str)->Self
    {
        let items: VecDeque<i64> = items.split("  Starting items: ")
                                        .last()
                                        .unwrap()        
                                        .split(", ")
                                        .map(|i| i.parse::<i64>().unwrap())
                                        .collect::<VecDeque<i64>>();

        Self 
        {
            id         : tools::usize_get_between(ids,"Monkey " ,":"),
            items,
            operation  : Operation::new(tools::str_get_between(operation,"Operation: new = ","")),
            test_div   : tools::i64_get_between(test,"Test: divisible by "        ,""),
            test_true  : tools::usize_get_between(test_truerue ,"If true: throw to monkey "  ,""),
            test_false : tools::usize_get_between(test_falsealse,"If false: throw to monkey " ,""),
            throws     : 0,
        }
    }

    fn go(&self)->usize
    {
        if self.operation.acc % self.test_div==0
        {
            self.test_true
        }
        else 
        {
            self.test_false
        }
    }

    fn throw(&mut self,division3:bool)->(usize,i64)
    {
        let item = self.items.pop_front().unwrap();

        self.operation.acc = item;
        self.operation.calc();

        if division3 
        {
            self.operation.div();
        }

        self.throws+=1;
        (self.go(),self.operation.acc)
    }

    fn push(&mut self,item:i64)
    {
        self.items.push_back(item);
    }

    #[allow(unused)]
    fn print(&self)
    {
        println!();
        println!("    Monkey {}:"                  ,self.id);
        println!("  Starting items: {:?}"          ,self.items);
        println!("  Operation: {}"                 ,self.operation.description());
        println!("  Test: divisible by {}"         ,self.test_div );
        println!("    If true: throw to monkey {}" ,self.test_true);
        println!("    If false: throw to monkey {}",self.test_false);
    }

    fn is_empty(&self)->bool
    {
        self.items.is_empty()
    }
}

#[allow(unused)]
fn print_monkeys(monkeys:&Vec<Monkey>,round:usize)
{
    println!("After round {}",round);

    for m in monkeys
    {
        println!("Monkey {}: {:?}",m.id,m.items);
    }
}

pub fn calc(data:&[String],rounds:usize,division3:bool)->i64
{
    let monkeys = data.split(|s| s.is_empty()).collect::<Vec<&[String]>>();

    let mut monkeys = monkeys.iter()
                             .map(|&m| Monkey::new(&m[0][..],
                                                   &m[1][..],
                                                   &m[2][..],
                                                   &m[3][..],
                                                   &m[4][..],
                                                   &m[5][..])).collect::<Vec<Monkey>>();

    //for m in mm.iter_mut() { m.print(); }

    for _round in 0..rounds
    {
        for id in 0..monkeys.len()
        {
            let mut list = vec![];

            while !monkeys[id].is_empty()
            {
                 list.push(monkeys[id].throw(division3));
            }

            for (to,item) in list
            {
                monkeys[to].push(item);                
            }
        };
     }    

     let mut counted = monkeys.iter().map(|m| m.throws as i64).collect::<Vec<i64>>();
     counted.sort_unstable();
     
     counted[counted.len()-1]*counted[counted.len()-2]
}

pub fn part1(data:&[String])->i64
{
    calc(data, 20    , true)
}

pub fn part2(data:&[String])->i64
{
    calc(data, 10000 , false)
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day 11");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
"        Monkey 0:".to_string(),
"        Starting items: 79, 98".to_string(),
"        Operation: new = old * 19".to_string(),
"        Test: divisible by 23".to_string(),
"          If true: throw to monkey 2".to_string(),
"          If false: throw to monkey 3".to_string(),
"".to_string(),
"      Monkey 1:".to_string(),
"        Starting items: 54, 65, 75, 74".to_string(),
"        Operation: new = old + 6".to_string(),
"        Test: divisible by 19".to_string(),
"          If true: throw to monkey 2".to_string(),
"          If false: throw to monkey 0".to_string(),
"".to_string(),
"      Monkey 2:".to_string(),
"        Starting items: 79, 60, 97".to_string(),
"        Operation: new = old * old".to_string(),
"        Test: divisible by 13".to_string(),
"          If true: throw to monkey 1".to_string(),
"          If false: throw to monkey 3".to_string(),
"".to_string(),
"      Monkey 3:".to_string(),
"        Starting items: 74".to_string(),
"        Operation: new = old + 3".to_string(),
"        Test: divisible by 17".to_string(),
"          If true: throw to monkey 0".to_string(),
"          If false: throw to monkey 1".to_string(),
    ];
    assert_eq!(part1(&v),10605);
}

#[test]
fn test2()
{
    let v = vec![
"        Monkey 0:".to_string(),
"        Starting items: 79, 98".to_string(),
"        Operation: new = old * 19".to_string(),
"        Test: divisible by 23".to_string(),
"          If true: throw to monkey 2".to_string(),
"          If false: throw to monkey 3".to_string(),
"".to_string(),
"      Monkey 1:".to_string(),
"        Starting items: 54, 65, 75, 74".to_string(),
"        Operation: new = old + 6".to_string(),
"        Test: divisible by 19".to_string(),
"          If true: throw to monkey 2".to_string(),
"          If false: throw to monkey 0".to_string(),
"".to_string(),
"      Monkey 2:".to_string(),
"        Starting items: 79, 60, 97".to_string(),
"        Operation: new = old * old".to_string(),
"        Test: divisible by 13".to_string(),
"          If true: throw to monkey 1".to_string(),
"          If false: throw to monkey 3".to_string(),
"".to_string(),
"      Monkey 3:".to_string(),
"        Starting items: 74".to_string(),
"        Operation: new = old + 3".to_string(),
"        Test: divisible by 17".to_string(),
"          If true: throw to monkey 0".to_string(),
"          If false: throw to monkey 1".to_string(),
    ];
    assert_eq!(part2(&v),2713310158);
}
