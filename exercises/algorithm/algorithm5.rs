/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/

//
use std::collections::VecDeque;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>, 
    //临接链表，存储每个节点的邻接节点
}

impl Graph {
    // Create a new graph with n vertices
    // 邻接矩阵
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }

    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest); 
        self.adj[dest].push(src); 
        // 无向图，两个节点互为邻接节点 相当于把两个节点都加入到对方的邻接节点列表中 在图上表示两个节点之间有一条边
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        
        let mut visit_order = vec![]; // 存储访问顺序
        let mut queue = VecDeque::new(); // 创建一个队列来进行 BFS
        let mut visited = vec![false; self.adj.len()]; // 记录访问过的节点 ， 布尔数组，长度为节点个数，初始化为false

        queue.push_back(start); // 将起始节点加入队列
        visited[start] = true; // 标记起始节点为已访问

        while let Some(node) = queue.pop_front() { // 当队列不为空时
            visit_order.push(node); // 记录访问的节点

            // 遍历所有邻居
            for &neighbor in &self.adj[node] {// 这里要用引用，否则会导致所有权问题 S
                if !visited[neighbor] { // 如果邻居未被访问
                    visited[neighbor] = true; // 标记为已访问
                    queue.push_back(neighbor); // 将邻居加入队列
                }
            }
        }

        visit_order // 返回访问顺序
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}

