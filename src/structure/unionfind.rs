use cargo_snippet::snippet;

#[snippet("unionfind")]
#[derive(Debug)]
struct Unionfind {
    node: Vec<usize>,
    rank: Vec<usize>,
}

#[snippet("unionfind")]
impl Unionfind {
    fn new(number: usize) -> Unionfind {
        Unionfind {
            node: (0..number).collect::<Vec<_>>(),
            rank: vec![0; number],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.node[x] == x {
            return x;
        }

        self.node[x] = self.find(self.node[x]);
        self.node[x]
    }

    fn check(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn unite(&mut self, x: usize, y: usize) {
        let x_par = self.find(x);
        let y_par = self.find(y);

        if x_par == y_par {
            return;
        }

        if self.rank[x_par] < self.rank[y_par] {
            self.node[x_par] = y_par;
        } else {
            self.node[y_par] = x_par;
            if self.rank[x_par] == self.rank[y_par] {
                self.rank[x_par] += 1;
            }
        }
    }
}
