use super::tools;
use std::collections::VecDeque;

#[derive(Clone, Debug, Copy)]
struct Operation
{
    left : i128,
    oper : char,
    right : i128,
    acc : i128,
}

impl Operation 
{
    fn new(l:&str)->Self
    {
        let ll = l.replace("old","-1");
        
        let ll : Vec<&str> = ll.split(' ').collect(); 
        Self 
        {
            left  : ll[0].parse().unwrap(), 
            oper  : ll[1].chars().next().unwrap(),
            right : ll[2].parse().unwrap(), 
            acc   : 0
        }
    }

    fn des(&self)->String
    {
        format!("{} {} {}",self.left,self.oper,self.right)
    }

    fn div(&mut self)
    {
        self.acc = self.acc/3;
    }

    fn calc(&mut self)->i128
    {
        let mut l = self.left;
        let mut r = self.right;
        if self.left ==-1 { l = self.acc; }
        if self.right==-1 { r = self.acc; }

        //println!("calc:{}{}{}",self.left,self.oper,self.right);

        match self.oper {
            '+' => { self.acc = l + r },
            '-' => { self.acc = l - r },
            '*' => { self.acc = l * r },
            '/' => { self.acc = l / r },
            _   => panic!("rr"),            
        }
      
        self.acc = self.acc % (2*3*5*7*11*13*17*19*23);
    }
}


#[derive(Clone, Debug)]
struct Monkey{
    id: usize,
    items : VecDeque<i128>,
    operation: Operation,
    test_div : i128,
    test_t: usize,
    test_f: usize,
    throws: usize
}

impl Monkey 
{
    fn new(ids:&str,items:&str,operation:&str,test:&str,test_true:&str,test_false:&str)->Self
    {
        let id    =     tools::usize_get_between(ids,"Monkey " ,":");
        let i    =      items.split("  Starting items: ").last().unwrap(); 
        let o    =      operation.split("Operation: new = ").last().unwrap(); 
        let t1   =      test.split("Test: divisible by ").last().unwrap(); 
        let t2   =      test_true.split("If true: throw to monkey ").last().unwrap(); 
        let t3   =      test_false.split("If false: throw to monkey ").last().unwrap(); 
        //let left  : Vec<&str> = tab[0].split('-').collect(); 
        //let right : Vec<&str> = tab[1].split('-').collect(); 

        println!("_{}",i);
        let it: VecDeque<i128> = i.split(", ")
                            .map(|i| i.parse::<i128>().unwrap())
                            .collect::<VecDeque<i128>>();

        Self 
        {
            id,
            items: it,
            operation: Operation::new(o),
            test_div: t1.parse::<i128>().unwrap(),
            test_t:   t2.parse::<usize>().unwrap(),
            test_f:   t3.parse::<usize>().unwrap(),
            throws: 0,
        }
    }

    fn go(&self)->usize
    {
        if self.operation.acc % self.test_div==0
        {
            self.test_t
        }
        else {
            self.test_f
        }
    }

    fn throw(&mut self,division3:bool)->(Option<usize>,i128)
    {
        if self.items.is_empty() {return (None,0); }

        let item = self.items.pop_front().unwrap();

        //println!("ITEM{}",item);

        self.operation.acc = item;
        self.operation.calc();

        if division3 
        {
            self.operation.div();
        }

        self.throws+=1;
        (Some(self.go()),self.operation.acc)
    }

    fn push(&mut self,item:i128)
    {
        self.items.push_back(item);
    }

    fn print(&self)
    {
        println!();
        println!("    Monkey {}:",self.id);
        println!("  Starting items: {:?}",self.items);
        println!("  Operation: {}"      ,self.operation.des());
        println!("  Test: divisible by {}"         ,self.test_div );
        println!("    If true: throw to monkey {}" ,self.test_t);
        println!("    If false: throw to monkey {}",self.test_f);
    }

    fn is_empty(&self)->bool
    {
        self.items.is_empty()
    }
}

fn print_monkeys(monkeys:&Vec<Monkey>,round:usize)
{
    println!("After round {}",round);

    for m in monkeys
    {
        println!("Monkey {}: {:?}",m.id,m.items);
    }
}

pub fn calc(data:&[String],rounds:usize,division3:bool)->i128
{
    let monkeys = data.split(|s| s.is_empty()).collect::<Vec<&[String]>>();

    let mut mm = monkeys.iter()
                   .map(|&m| Monkey::new(&m[0][..],
                        &m[1][..],
                                        &m[2][..],
                                        &m[3][..],
                                        &m[4][..],
                                        &m[5][..])).collect::<Vec<Monkey>>();

    for m in mm.iter_mut()
    {
        m.print();
    }

    for round in 0..rounds
    {
        for id in 0..mm.len()
        {
            let mut temp = vec![];

            while !mm[id].is_empty()
            {
                temp.push(mm[id].throw(division3));
            }

            for i in 0..temp.len() 
            {
                let (to,item) = temp[i];
                if to!=None
                {
          //          println!("monkey {} throws at {} value {}",id,to.unwrap(),item);
                    mm[to.unwrap()].push(item);
                }
            }
        };

        //print_monkeys(&mm,round+1);

//        println!("round:{} {:?}",round, mm.iter().map(|m| m.throws as i128 ).collect::<Vec<i128>>());
     
     }    

     let mut counted = mm.iter().map(|m| m.throws as i128 ).collect::<Vec<i128>>();
     counted.sort_unstable();
     
     
     let res = counted[counted.len()-1] *counted[counted.len()-2];
     println!("{:?}",counted);
     println!("{:?}",res);
     res
           
}




pub fn part1(data:&[String])->i128
{
    calc(data, 20 , true)
}

pub fn part2(data:&[String])->i128
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
