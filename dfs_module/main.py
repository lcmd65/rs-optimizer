import time
import memory_profiler
import dfs_module  # For the Rust-backed DFS

# Python DFS function
def dfs_python(graph, start):
    visited = set()
    stack = [start]
    result = []

    while stack:
        node = stack.pop()
        if node not in visited:
            visited.add(node)
            result.append(node)
            stack.extend(reversed(graph.get(node, [])))

    return result

# Function to compare output, time, and memory
def compare_dfs():
    # Define the graph for the tests
    graph = {
        "A": ["B", "C"],
        "B": ["D", "E"],
        "C": ["F"],
        "D": [],
        "E": [],
        "F": [],
    }

    start_node = "A"

    # 1. Compare Output
    print("Running Python DFS...")
    python_result = dfs_python(graph, start_node)
    print("Python DFS Output:", python_result)

    print("Running Rust DFS...")
    rust_result = dfs_module.dfs(graph, start_node)
    print("Rust DFS Output:", rust_result)

    # Ensure the outputs are the same
    assert python_result == rust_result, "The outputs are not the same!"

    # 2. Measure Time
    # Python DFS Time
    start_time = time.time()
    dfs_python(graph, start_node)
    python_time = time.time() - start_time
    print(f"Python DFS Execution Time: {python_time:.6f} seconds")

    # Rust DFS Time
    start_time = time.time()
    dfs_module.dfs(graph, start_node)
    rust_time = time.time() - start_time
    print(f"Rust DFS Execution Time: {rust_time:.6f} seconds")

    # 3. Measure Memory Usage
    # Memory profiling for Python DFS
    print("Measuring Python DFS memory usage...")
    @memory_profiler.profile
    def python_memory():
        dfs_python(graph, start_node)

    python_memory()

    # Memory profiling for Rust DFS (Note: Python wrapper for Rust should work the same as a Python function)
    print("Measuring Rust DFS memory usage...")
    @memory_profiler.profile
    def rust_memory():
        dfs_module.dfs(graph, start_node)

    rust_memory()

if __name__ == "__main__":
    compare_dfs()