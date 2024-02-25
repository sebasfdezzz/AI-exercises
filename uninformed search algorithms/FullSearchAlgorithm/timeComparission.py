import time

methods = {
    "BFS": bfs,
    "Dijkstra": dijkstra,
    "DFS": dfs,
    "DFS_limite": dfs_limited,
    "DFS_iter": dfs_iter
}

def method_comparission(G, orig, dest):
  exec_time = {}
  res = {}
  for nombre, func in methods.items():
    start = time.time()
    res[nombre] = func(G,orig,dest)
    end = time.time()

    exec_time[nombre] = end - start

  fastest = min(exec_time, key=exec_time.get)
  return fastest, res[fastest], exec_time[fastest]

best_method, path, time = method_comparission(None, None, None)
print("El metodo mas rapido fue",best_method,"con un tiempo de ejecucion de",time,"y regreso el path",path)
