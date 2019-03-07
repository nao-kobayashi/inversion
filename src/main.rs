fn main() {
    let ppp = [3, 10, 1, 8, 5];
    let mut bit = BinaryIndexTree::new(&ppp);
    let mut ans = 0;
 
    for (i, p) in ppp.iter().enumerate() {
        bit.add(*p, 1);
        //自分自身が+1しているので引く数は（0originであれば）+1
        //つまり2つ目の要素であれば+2のはず。
        //2より小さい場合は自分より左の要素がでかかったということ
        //逆にindex + 1 より大きい数（マイナスになる数）は来ない。
        ans += i as i32 + 1 - bit.sum(*p);
        //println!("index:{} {}", i, ans);
    } 
 
    println!("{}", ans);
}

//see details
//https://ikatakos.com/pot/programming_algorithm/dynamic_programming/inversion
struct BinaryIndexTree {
    size: i32,
    tree: Vec<i32>,
}

impl BinaryIndexTree {
    pub fn new(target: &[i32]) -> Self {
        //配列のインデックスを合わせるために+1、スライスが半開なのでさらに+1
        let size = target.iter().map(|n| *n).max().unwrap() + 2;
        let tree = (0..size).map(|_n| 0).collect::<Vec<i32>>();
        Self { size, tree }
    }

    pub fn sum(&self, mut i: i32) -> i32 {
        // println!("{:?}", self.tree);
        let mut s = 0;
        while i > 0 {
            s += self.tree.get(i as usize).unwrap();
            i -= i & -i;
        }
        s
    }

    pub fn add(&mut self, mut i: i32, x: i32) {
        let v: &mut Vec<i32> = self.tree.as_mut();
        while i < self.size {
            let element = v.get_mut(i as usize).unwrap();
            *element += x;
            i += i & -i;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BinaryIndexTree;

    #[test]
    fn test1() {
        let a = vec![3, 1, 5, 4, 2];
        let b = vec![1, 2, 3, 4, 5, 6];
        let c = vec![7, 6, 5, 4, 3, 2, 1];
        let d = vec![19, 11, 10, 7, 8, 9, 17, 18, 20, 4, 3, 15, 16, 1, 5, 14, 6, 2, 13, 12];

        assert_eq!(5, calc(&a));
        assert_eq!(0, calc(&b));
        assert_eq!(21, calc(&c));
        assert_eq!(114, calc(&d));
    }

    fn calc(ppp: &[i32]) -> i32 {
        let mut bit = BinaryIndexTree::new(ppp);
        let mut ans = 0;
    
        for (i, p) in ppp.iter().enumerate() {
            bit.add(*p, 1);
            ans += i as i32 + 1 - bit.sum(*p);
        } 
    
        println!("{}", ans);
        ans
    }
}
