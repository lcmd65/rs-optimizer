import dfs_module

graph = {
    "A": ["B", "C"],
    "B": ["D", "E"],
    "C": ["F"],
    "D": [],
    "E": [],
    "F": [],
}

start_node = "A"
result = dfs_module.dfs(graph, start_node)
print("DFS Traversal:", result)
