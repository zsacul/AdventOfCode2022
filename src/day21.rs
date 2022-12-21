use std::collections::HashMap;

#[derive(Eq, PartialEq,  Debug, Clone)]
enum Type
{
    None,
    Value(i64),
    Operation(String,char,String)
}

#[derive(Eq, PartialEq,  Debug, Clone)]
struct Node
{    
    tree : HashMap<String,Type>
}

impl Node
{
    fn new(s:&[String])->Self {
        
        let mut tree = HashMap::new();

        for line in s
        {
            let t : Vec<&str> = line.split(": ").collect(); 
            
            let name = t[0];
            let v = Self::new_node(&t[1][..]);
            
            tree.insert(name.to_string(), v);
        }

        Self
        {
            tree            
        }
    }

    fn new_node(line:&str)->Type {

        let t : Vec<&str> = line.split(" ").collect(); 

        if t.len()==1 
        {
            Type::Value(t[0].parse::<i64>().unwrap())
        }
          else if t.len()==3 
        {            
            Type::Operation(t[0].to_string(),t[1].chars().nth(0).unwrap(),t[2].to_string())
        }
          else
        {
            panic!("error: {:?}",t);
        }
    }

    fn get_node(&self,name:String)->&Type
    {
        let res = self.tree.get(&name);
        if res==None
        {
            &Type::None
        }
          else
        {
            res.unwrap()
        }
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

    fn check(&mut self,val:i64,l:&Type,r:&Type)->bool
    {
        *self.tree.get_mut("humn").unwrap() = Type::Value(val);
        
        
        let res = self.eval_node(l)-self.eval_node(r);
        if res==0 { return true;}
        println!("{} {}",val,res);
        false
        

    }

    //humn: 4672
    fn eval2(&mut self)->i64    
    {
        let mut ll="".to_string();
        let mut rr="".to_string();
        {
            match self.clone().get_node("root".to_string()).clone()
            {
                Type::Operation(l,_,r) => 
                { ll=l; rr=r; },
                _ => {}
            }
        }

        let mut guess = 0;
        //self.tree.insert("humn".to_string(), Type::Value(guess));
        let bb = self.clone();
        let ln = bb.get_node(ll.to_string());
        let rn = bb.get_node(rr.to_string());
              
        guess = 3910938071000;
        loop
        {
            guess+=1;
            if self.check(guess,ln,rn) { return guess; } 
        }
        

        

        //self.eval_node(self.get_node(l.to_string())) + self.eval_node(self.get_node(r.to_string()))

    }

}


fn solve1(data:&[String])->i64
{
    let n = Node::new(data);
    n.eval()

}

fn solve2(data:&[String])->i64
{
    let mut n = Node::new(data);
    n.eval2()
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
