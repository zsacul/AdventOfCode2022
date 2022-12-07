use std::{collections::HashMap};

fn get_hash_table(data:&[String])->HashMap<String,usize>
{
    let mut path = vec!["/"];
    let mut hash = HashMap::new();
    
    for s in data
    {
        let tokens:Vec<&str> = s.split(' ').collect();

        match tokens[0]
        {
            "$" =>  {
                        match tokens[1]
                        {
                            "cd" =>  match tokens[2]
                                    {
                                        "/"  => { path = vec!["/"] },
                                        ".." => { path.pop();      },
                                        tok  => { path.push(tok);  }, 
                                    },
                            "ls" => {},
                            _    => {},
                        }
                    }
            "dir"=> {
                        let full_path = format!("{}/{}",path.join("/"),tokens[1]); 
                        *hash.entry(full_path).or_insert(0)+=0;
                    }
            size =>
                    {
                        let size      = size.parse::<usize>().unwrap();
                        let full_path = format!("{}/{}.[file]",path.join("/"),tokens[1]); 
                        *hash.entry(full_path).or_insert(0)+=size;

                        for i in 0..path.len()
                        {
                            let path = path[0..=i].join("/");
                            *hash.entry(path).or_insert(0)+=size;
                        }
                    }
        }
    }

    hash
}

pub fn part1(data:&[String])->usize
{
    get_hash_table(data).iter()
                        .map(
                            |(l,&v)| 
                            if l.contains('.') { 0 } else { v }
                        )
                        .filter(|&v| v<=100000)
                        .into_iter()
                        .sum()
}

pub fn part2(data:&[String])->usize
{
    let disk_size   = 70_000_000;
    let update_size = 30_000_000;
    let hash        = get_hash_table(data);
    let sum         = hash.get("/").unwrap();
    let free_space  = disk_size - sum;
    let need        = update_size - free_space;

    hash.iter()
        .map(
            |(l,&v)| 
            if l.contains('.') { 0 } else { v }
        )
        .filter(|&v| v>=need)
        .min()
        .unwrap_or(0)
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day7");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
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