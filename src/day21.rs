use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug, Clone)]
enum Type
{
    Value(i64),
    Operation(String,char,String)
}

#[derive(Eq, PartialEq,  Debug, Clone)]
struct World
{    
    tree : HashMap<String,Type>
}

impl World
{
    fn new(data:&[String])->Self 
    {
        Self {               
            tree : data.iter()
                       .map(|line|
                            {
                                let tab : Vec<&str> = line.split(": ").collect(); 
                                (tab[0].to_string(),Self::new_node(tab[1]))
                            }
                       ).collect::<HashMap<String,Type>>()
        }
    }

    fn new_node(line:&str)->Type 
    {
        let t : Vec<&str> = line.split(' ').collect(); 

        if t.len()==1 
        {
            Type::Value(t[0].parse::<i64>().unwrap())
        }
          else if t.len()==3 
        {            
            Type::Operation(t[0].to_string(),t[1].chars().next().unwrap(),t[2].to_string())
        }
          else
        {
            panic!("error: {:?}",t);
        }
    }

    fn get_node(&self,name:String)->&Type
    {
        self.tree.get(&name).unwrap()
    }

    fn eval_node(&self,node:&Type)->i64
    {
        match node 
        {
            Type::Value(n)           => *n  ,
            Type::Operation(l,'+',r) => self.eval_node(self.get_node(l.to_string())) + self.eval_node(self.get_node(r.to_string())),
            Type::Operation(l,'-',r) => self.eval_node(self.get_node(l.to_string())) - self.eval_node(self.get_node(r.to_string())),
            Type::Operation(l,'/',r) => self.eval_node(self.get_node(l.to_string())) / self.eval_node(self.get_node(r.to_string())),
            Type::Operation(l,'*',r) => self.eval_node(self.get_node(l.to_string())) * self.eval_node(self.get_node(r.to_string())),
            _                        => panic!("wrong value"),
        }
    }

    fn eval(&self)->i64
    {
        self.eval_node(self.get_node("root".to_string()))        
    }

    fn check(&mut self,val:i64,left:&Type,right:&Type)->i64
    {
        *self.tree.get_mut("humn").unwrap() = Type::Value(val);
        self.eval_node(right) - self.eval_node(left)
    }

    fn eval2(&mut self)->i64    
    {
        let rtree = self.clone();
        let (left, right) = if let Some(Type::Operation(l,_,r)) = rtree.tree.get(&"root".to_string()) {
                                (l,r)
                            } else {
                                panic!("unexpected type")
                            };
        
        let  left_node = rtree.get_node( left.to_string());
        let right_node = rtree.get_node(right.to_string());

        let mut limits = (i64::MIN,i64::MAX);

        loop
        {
            let guess = (limits.0 + limits.1)/2;
            
            match self.check(guess,left_node,right_node).signum()
            {
                 1 => { limits.0 = guess; },
                -1 => { limits.1 = guess; },
                 0 => { return     guess; },
                 _ => panic!("e"),
            }
        }
    }
}

fn solve1(data:&[String])->i64
{
    World::new(data).eval()
}

fn solve2(data:&[String])->i64
{
    World::new(data).eval2()    
}

pub fn solve(data:&[String])
{
    println!("Day 21");
    println!("part1: {}",solve1(data));
    println!("part2: {}",solve2(data));    
}

#[test]
fn test1()
{
    let v = vec![
        "root: pppw + sjmn".to_string(),
        "dbpl: 5".to_string(),
        "cczh: sllz + lgvd".to_string(),
        "zczc: 2".to_string(),
        "ptdq: humn - dvpt".to_string(),
        "dvpt: 3".to_string(),
        "lfqf: 4".to_string(),
        "humn: 5".to_string(),
        "ljgn: 2".to_string(),
        "sjmn: drzm * dbpl".to_string(),
        "sllz: 4".to_string(),
        "pppw: cczh / lfqf".to_string(),
        "lgvd: ljgn * ptdq".to_string(),
        "drzm: hmdt - zczc".to_string(),
        "hmdt: 32".to_string(),
    ];
    assert_eq!(solve1(&v),152);
}

#[test]
fn test2()
{
    let v = vec![
        "root: pppw + sjmn".to_string(),
        "dbpl: 5".to_string(),
        "cczh: sllz + lgvd".to_string(),
        "zczc: 2".to_string(),
        "ptdq: humn - dvpt".to_string(),
        "dvpt: 3".to_string(),
        "lfqf: 4".to_string(),
        "humn: 5".to_string(),
        "ljgn: 2".to_string(),
        "sjmn: drzm * dbpl".to_string(),
        "sllz: 4".to_string(),
        "pppw: cczh / lfqf".to_string(),
        "lgvd: ljgn * ptdq".to_string(),
        "drzm: hmdt - zczc".to_string(),
        "hmdt: 32".to_string(),
    ];
    assert_eq!(solve2(&v),301);
}
