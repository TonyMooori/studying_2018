use std::hash::{Hash,Hasher};
use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;

fn calc_hash<T:Hash>(t:&T)->u64{
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

struct HM<T:Hash+Eq,K>{
    list : Vec<(u64,Vec<(T,K)>)>
}

impl<T:Hash+Eq+Debug,K:Debug> HM<T,K>{
    pub fn new() -> HM<T,K>{
        HM{
            list:Vec::new() 
        }
    }

    pub fn push(&mut self,key:T,val:K){
        let h = calc_hash(&key);

        let index = self.search_index(h);
        if index < self.list.len() && self.list[index].0 == h{
            let v = &mut self.list[index].1;
            for i in 0..v.len(){
                if v[i].0 == key{
                    v[i].1 = val;
                    return;
                }
            }
            v.push((key,val));
        }else{
            self.add_new_key(key,val,h,index);
        }

    }

    fn add_new_key(&mut self,key:T,val:K,h:u64,index:usize){
        let mut temp = Vec::new();
        temp.push((key,val));

        self.list.insert(index,(h,temp));
    }
    pub fn search_index(&self,h:u64)->usize{
        let mut left = 0;
        let mut right = self.list.len();

        while left < right{
            let middle = (left+right)/2;

            if self.list[middle].0 == h{
                return middle;
            }else if self.list[middle].0 > h{
                right = middle;
            }else{
                left = middle+1;
            }
        }

        left
    }

    pub fn get(&self,key:T)->Option<&K>{
        let h = calc_hash(&key);
        let index = self.search_index(h);
        
        if index < self.list.len() && self.list[index].0 == h{
            let v = &self.list[index].1;
            
            for i in 0..v.len(){
                if v[i].0 == key{
                    return Some(&v[i].1);
                }
            }

            None
        }else{
            None
        }
    }
}

fn main(){
    let mut hm = HM::new();

    hm.push("a",1);
    hm.push("b",2);
    hm.push("c",3);
    hm.push("d",4);
    hm.push("e",5);
    hm.push("f",6);
    hm.push("f",7);

    for s in vec!["a","b","c","d","e","f","g"]{
        match hm.get(s.clone()){
            Some(v) => println!("hash[{}] = {}",s,v),
            None => println!("hash[{}] is None",s),
        }
    }
    
}
