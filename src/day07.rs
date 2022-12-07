use std::{collections::HashMap};

//fn get_table()

pub fn part1(data:&[String])->usize
{
    let mut hm = HashMap::new();
    let mut cd = "";
    let mut pp = vec!["/"];
    
    for s in data
    {
        let tokens:Vec<&str> = s.split(" ").collect();

        if tokens[0]=="$"
        {
            match tokens[1]
            {
                "cd"=>{ 
                        if tokens[2]=="/"
                        {
                            pp = vec!["/"];
                        }
                        else
                        if tokens[2]==".."
                        {
                            pp.pop();
                        }
                        else 
                        {
                            pp.push(tokens[2]); 
                        }                       
                    }
                    ,
                "ls"=>{},
                _   =>{},
            }
        }
        else 
        {
            if (tokens[0]=="dir")
            {
                let path = format!("{}//{}",pp.join("/"),tokens[1]); 
                *hm.entry(path).or_insert(0)+=0;
            }
              else 
            {
                let s:usize = tokens[0].parse().unwrap();
                let path = format!("{}//{}.f",pp.join("/"),tokens[1]); 
                *hm.entry(path).or_insert(0)+=s;

                for i in 0..pp.len()
                {
                    let path = pp[0..=i].join("/");
                    *hm.entry(path).or_insert(0)+=s;
                }
            }
        }
    }

     hm.iter()
        .map(
            |(l,&v)| 
            if l.contains(".") { 0 } else { v }
        )
        .filter(|&v| v<=100000)
        .into_iter()
        .sum()

}

pub fn part2(data:&[String])->usize
{
    let mut hm = HashMap::new();
    let mut cd = "";
    let mut pp = vec!["/"];
    
    for s in data
    {
        let tokens:Vec<&str> = s.split(" ").collect();

        if tokens[0]=="$"
        {
            match tokens[1]
            {
                "cd"=>{ 
                        if tokens[2]=="/"
                        {
                            pp = vec!["/"];
                        }
                        else
                        if tokens[2]==".."
                        {
                            pp.pop();
                        }
                        else 
                        {
                            pp.push(tokens[2]); 
                        }                       
                    }
                    ,
                "ls"=>{},
                _   =>{},
            }
        }
        else 
        {
            if (tokens[0]=="dir")
            {
                let path = format!("{}//{}",pp.join("/"),tokens[1]); 
                *hm.entry(path).or_insert(0)+=0;
            }
              else 
            {                
                let s:usize = tokens[0].parse().unwrap();
                let path = format!("{}//{}.f",pp.join("/"),tokens[1]); 
                *hm.entry(path).or_insert(0)+=s;

                for i in 0..pp.len()
                {
                    let path = pp[0..=i].join("/");
                    *hm.entry(path).or_insert(0)+=s;
                }
            }
        }
    }

    let free   = 70_000_000;
    let update_size = 30_000_000;

    let sum = hm.get("/").unwrap();
    let need = update_size-(free-sum);

    
     hm.iter()
        .map(
            |(l,&v)| 
            if l.contains(".") { 0 } else { v }
        )
        .filter(|&v| v>=need)
        .min()
        .unwrap_or(0)
    
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day7");
    println!("part1:{}",part1(&data));
    println!("part2:{}",part2(&data));
}

#[test]
fn test1()
{
    let v = vec![
        "$ cd /".to_string(),
        "$ ls".to_string(),
        "dir a".to_string(),
        "14848514 b.txt".to_string(),
        "8504156 c.dat".to_string(),
        "dir d".to_string(),
        "$ cd a".to_string(),
        "$ ls".to_string(),
        "dir e".to_string(),
        "29116 f".to_string(),
        "2557 g".to_string(),
        "62596 h.lst".to_string(),
        "$ cd e".to_string(),
        "$ ls".to_string(),
        "584 i".to_string(),
        "$ cd ..".to_string(),
        "$ cd ..".to_string(),
        "$ cd d".to_string(),
        "$ ls".to_string(),
        "4060174 j".to_string(),
        "8033020 d.log".to_string(),
        "5626152 d.ext".to_string(),
        "7214296 k".to_string()
    ];
//    assert_eq!(part1(&v),1);
    assert_eq!(part1(&v),95437);
}


#[test]
fn test2()
{
    let v = vec![
            "$ cd /".to_string(),
            "$ ls".to_string(),
            "dir a".to_string(),
            "14848514 b.txt".to_string(),
            "8504156 c.dat".to_string(),
            "dir d".to_string(),
            "$ cd a".to_string(),
            "$ ls".to_string(),
            "dir e".to_string(),
            "29116 f".to_string(),
            "2557 g".to_string(),
            "62596 h.lst".to_string(),
            "$ cd e".to_string(),
            "$ ls".to_string(),
            "584 i".to_string(),
            "$ cd ..".to_string(),
            "$ cd ..".to_string(),
            "$ cd d".to_string(),
            "$ ls".to_string(),
            "4060174 j".to_string(),
            "8033020 d.log".to_string(),
            "5626152 d.ext".to_string(),
            "7214296 k".to_string()
        ];
    assert_eq!(part2(&v),24933642);
}
