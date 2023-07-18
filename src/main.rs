use std::collections::{HashMap, HashSet, BTreeMap};
use itertools::Itertools;
use rand::prelude::*;


#[derive(Clone,Default,Debug,Eq,PartialEq,Hash)]
struct State {
    id : usize,
    //color : usize,
    a : usize,
    b : usize,
    terminated : Option<usize>
    //terminated : Option<(usize,usize)>
}

impl State {
    fn new(id : usize, color : usize) -> State {
        State { a : 0, b : 0, id, terminated : None }
    }


    //5 coloring
    fn next(&self, neighbors : &[&State]) -> State {
        if self.terminated.is_some() {
            return self.clone();
        }

        let colors_of_higher : Vec<_> = neighbors.iter().filter(|v|v.id != 0).filter(|v|v.id > self.id).flat_map(|v| [v.a,v.b].into_iter()).collect();
        let colors_of_all : Vec<_> = neighbors.iter().filter(|v|v.id != 0).flat_map(|v| [v.a,v.b].into_iter()).collect();

        if !colors_of_all.contains(&self.a) {
            let mut s = self.clone();
            s.terminated = Some(s.a);
            return s;
        }
        if !colors_of_all.contains(&self.b) {
            let mut s = self.clone();
            s.terminated = Some(s.b);
            return s;
        }
        let a = [0,1,2,3,4].into_iter().filter(|c|!colors_of_higher.contains(c) ).next().unwrap();
        let b = [0,1,2,3,4].into_iter().filter(|c|!colors_of_all.contains(c) ).next().unwrap();

        State { a, b, id : self.id, terminated : None }
    }

    // 6 coloring
    /*fn next(&self, neighbors : &[&State]) -> State {
        if self.terminated.is_some() {
            return self.clone();
        }
        if neighbors.iter().filter(|v|v.id != 0).all(|v|(v.a,v.b) != (self.a,self.b)) {
            let mut s = self.clone();
            s.terminated = Some((s.a,s.b));
            assert!(s.a + s.b <= 2);
            return s;
        }
        let a_of_higher : Vec<_> = neighbors.iter().filter(|v|v.id != 0).filter(|v|v.id > self.id).map(|v| v.a).collect();
        let b_of_smaller : Vec<_> = neighbors.iter().filter(|v|v.id != 0).filter(|v|v.id < self.id).map(|v| v.b).collect();
        let a = (0..).filter(|c|!a_of_higher.contains(c)).next().unwrap();
        let b = (0..).filter(|c|!b_of_smaller.contains(c)).next().unwrap();
        State { a, b, color : self.color, id : self.id, terminated : None }
    }*/
    
}

#[derive(Clone)]
struct Process {
    external_state : State,
    internal_state : State
}



impl Process {
    fn new(id : usize, color : usize) -> Self {
        Self { external_state : Default::default(), internal_state : State::new(id, color) }
    }
    fn write(&mut self) {
        self.external_state = self.internal_state.clone();
    }
    fn read(&mut self, neighbors : &[&State]) {
        self.internal_state = self.external_state.next(neighbors);
    }
}


struct Scheduler {
    processes : HashMap<usize,Process>, 
    graph : HashMap<usize, Vec<usize>>,
    activation_history : Vec<Vec<usize>>,
    rng : ThreadRng
}

impl Scheduler {
    fn new(graph : &HashMap<usize, Vec<usize>>, colors : &HashMap<usize,usize>) -> Self {
        let processes = graph.keys().map(|&i|(i,Process::new(i,colors[&i]))).collect();
        Self { processes, graph : graph.clone(), rng : rand::thread_rng(), activation_history : vec![] }
    }
    fn tick(&mut self, custom_scheduling : Option<&[usize]>){
        let mut active = if let Some(scheduling) = custom_scheduling {
            scheduling.to_owned()
        } else {
            loop {
                let active : Vec<_> = self.processes.keys().filter(|p|self.rng.gen() && self.processes[p].external_state.terminated.is_none()).cloned().collect();
                if !active.is_empty() {
                    break active;
                }
            }
        };
        for &p in &active {
            self.processes.get_mut(&p).unwrap().write();
        }
        let before_update = self.processes.clone();
        for &p in &active {
            let neighbors : Vec<&State> = self.graph[&p].iter().map(|v|&before_update[v].external_state).collect();
            self.processes.get_mut(&p).unwrap().read(&neighbors);
        }
        active.sort();
        self.activation_history.push(active);
    }
    fn print_state(&self) {
        println!("active : {:?}",self.activation_history.last().unwrap());
        println!("states :");
        for (p,s) in self.processes.iter().sorted_by_key(|(&p,_)|p) {
            println!("{} : ext {:?}, int {:?}",p,s.external_state,s.internal_state);
        }
    }

    fn is_all_done(&self) -> bool {
        self.processes.values().all(|p|p.external_state.terminated.is_some())
    }

    fn state(&self) -> (Vec<State>,Vec<State>) {
        (self.processes.iter().sorted_by_key(|(&p,_)|p).map(|(_,p)|p.external_state.clone()).collect(),
        self.processes.iter().sorted_by_key(|(&p,_)|p).map(|(_,p)|p.internal_state.clone()).collect())
    }
}

fn main() {
    /*let mut graph = HashMap::new();
    graph.insert(2, vec![5,3]);
    graph.insert(3, vec![2,1]);
    graph.insert(1, vec![3,4]);
    graph.insert(4, vec![1,5]);
    graph.insert(5, vec![4,2]);


    let scheduling = vec![vec![1, 3, 5], vec![1, 4, 5], vec![2, 4], vec![2, 3, 4], vec![1, 3, 5], vec![2], vec![2, 3], vec![3], vec![2], vec![2, 3], vec![3]];
    let colors : HashMap<_,_> = vec![(2, 3), (1, 4), (5, 4), (3, 6), (4, 3)].into_iter().collect();
    let mut s = Scheduler::new(&graph, &colors);
    for active in scheduling {
        s.tick(Some(&active));
        s.print_state();
        println!();
    }
    return;*/

    let mut count = 0;
    let mut max_steps = 0;

    'outer : loop{
        count += 1;
        if count % 10000 == 0 {
            println!("count: {}, max steps: {}",count,max_steps);
        }
        let mut graph = HashMap::new();
        let mut v : Vec<_> = (1..=5).collect();
        v.shuffle(&mut rand::thread_rng());
        for i in 0..v.len() {
            graph.insert(v[i],vec![v[(i+1)%v.len()],v[(i+v.len()-1)%v.len()]]);
        }

        let mut colors = HashMap::new();
        colors.insert(v[0],*[1,2,3,4,5,6].choose(&mut rand::thread_rng()).unwrap());
        for i in 1..v.len()-1 {
            let from : Vec<_> = [1,2,3,4,5,6].into_iter().filter(|&x|x != colors[&v[i-1]]).collect();
            colors.insert(v[i],*from.choose(&mut rand::thread_rng()).unwrap());
        }
        let from : Vec<_> = [1,2,3,4,5,6].into_iter().filter(|&x|x != colors[&v[0]] && x != colors[&v[v.len()-2]]).collect();
        colors.insert(v[v.len()-1],*from.choose(&mut rand::thread_rng()).unwrap());


        let mut s = Scheduler::new(&graph, &colors);
        let mut reached_states = HashSet::new();
        for i in 0..100 {
            s.tick(None);
            //s.print_state();
            //println!();
            if s.is_all_done() {
                for i in 0..v.len() {
                    assert!(s.processes[&v[i]].external_state.terminated.unwrap() < 5);
                    let neighbors = &graph[&v[i]];
                    for u in neighbors {
                        assert!(s.processes[&v[i]].external_state.terminated != s.processes[u].external_state.terminated );
                    }
                }
                //println!("Cycle : {:?}",v);
                //s.print_state();
                //println!("Done in {} steps",i);
                if i > max_steps {
                    max_steps = i;
                }
                break;
            }

            if !reached_states.insert(s.state()) {
                println!("Cycle : {:?}",v);
                println!("Colors : {:?}",colors);
                println!("Found a counterexample\nState : {:?}\nHistory : {:?}\n",s.state(),s.activation_history);
                break 'outer;
            }
        }
        if !s.is_all_done() {
            println!("Cycle : {:?}",v);
            println!("Colors : {:?}",colors);
            println!("Did not terminate\nState : {:?}\nHistory : {:?}\n",s.state(),s.activation_history);
            break 'outer;
        }
    }


}
