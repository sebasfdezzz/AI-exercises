import networkx as nx
import matplotlib.pyplot as plt

G = nx.Graph() #DiGrpah es direccionada
G.add_nodes_from(['Cozumel','Ciudad del Carmen','Angel Albino Corzo','Minatitlan','Veracruz','Acapulco','Puebla','CDMX','Morelia','Uruapan','San Luis Potosi','Tampico','Piedras Negras','Culiacan','Abraham Gonzalez','Nogales','Guaymas','Loreto','Manuel Marquez de Leon','Los Cabos'])
print(G.nodes())

G.add_edges_from([
('Cozumel', 'Tampico', {'weight': 1}),
('Ciudad del Carmen', 'Cozumel', {'weight': 1}),
('Angel Albino Corzo', 'Ciudad del Carmen', {'weight': 1}),
('Angel Albino Corzo', 'Minatitlan', {'weight': 1}),
('Angel Albino Corzo', 'Acapulco', {'weight': 1}),
('Minatitlan', 'Ciudad del Carmen', {'weight': 1}),
('Minatitlan', 'Veracruz', {'weight': 1}),
('Acapulco', 'Morelia', {'weight': 1}),
('Acapulco', 'Uruapan', {'weight': 1}),
('Puebla', 'Veracruz', {'weight': 1}),
('Puebla', 'Acapulco', {'weight': 1}),
('CDMX', 'Acapulco', {'weight': 1}),
('Morelia', 'CDMX', {'weight': 1}),
('Morelia', 'San Luis Potosi', {'weight': 1}),
('Uruapan', 'Culiacan', {'weight': 1}),
('San Luis Potosi', 'Abraham Gonzalez', {'weight': 1}),
('Tampico', 'Puebla', {'weight': 1}),
('Tampico', 'CDMX', {'weight': 1}),
('Tampico', 'San Luis Potosi', {'weight': 1}),
('Piedras Negras', 'San Luis Potosi', {'weight': 1}),
('Piedras Negras', 'Tampico', {'weight': 1}),
('Culiacan', 'Piedras Negras', {'weight': 1}),
('Culiacan', 'Guaymas', {'weight': 1}),
('Abraham Gonzalez', 'Piedras Negras', {'weight': 1}),
('Nogales', 'Abraham Gonzalez', {'weight': 1}),
('Guaymas', 'Abraham Gonzalez', {'weight': 1}),
('Guaymas', 'Nogales', {'weight': 1}),
('Guaymas', 'Loreto', {'weight': 1}),
('Loreto', 'Manuel Marquez de Leon', {'weight': 1}),
('Manuel Marquez de Leon', 'Culiacan', {'weight': 1}),
('Manuel Marquez de Leon', 'Los Cabos', {'weight': 1}),
])

# Se grafica el grafo
plt.figure(3,figsize=(32,12))
pos = nx.spring_layout(G, seed=1)
nx.draw_networkx_nodes(G, pos, node_color = 'blue', node_size = 5000)
nx.draw_networkx_labels(G, pos, font_size=10, font_family="sans-serif", font_color='white')
nx.draw_networkx_edges(
G, pos, edgelist=G.edges, width=6, alpha=1, edge_color="black", style="dashed"
)
edge_labels = nx.get_edge_attributes(G, "weight")
nx.draw_networkx_edge_labels(G, pos, edge_labels, font_size=20, font_color='red')
ax = plt.gca()
ax.margins(0.08)
plt.axis("off")
plt.tight_layout()
plt.show()
nx.dfs_successors(G)


#BFS
def BFS(orig, dest):
  if orig == dest:
      return dest

  frontier = [orig]
  reached = [orig]

  while frontier:
    node = frontier.pop(0)
    for child in G.neighbors(node):
      if child not in reached:
        reached.append(child)
        if child == dest:
          return dest
        frontier.append(child)
  return None

#print(BFS('Culiacan','Cozumel'))

def BFS2(graph, dest):
  orig = orig = next(iter(graph.nodes())) 
  if orig == dest:
      return dest

  frontier = [orig]
  reached = [orig]

  while frontier:
    node = frontier.pop(0)
    for child in graph.neighbors(node):
      if child not in reached:
        reached.append(child)
        if child == dest:
          return dest
        frontier.append(child)
  return None

#print(BFS2(G,'Cozumel'))

org = input("Origen: ")
destino = input("Destino: ")

def BFSpath(orig, dest):
  path = []
  parents={}

  G_low = nx.relabel_nodes(G, lambda x: x.lower())
  orig = orig.lower()
  dest = dest.lower()

  if orig == dest:
      return [dest.title()]

  frontier = [orig]
  reached = [orig]

  while frontier:
    node = frontier.pop(0)
    for child in G_low.neighbors(node):
      if child not in reached:
        reached.append(child)
        if child not in parents:
          parents[child] = node
        if child == dest:
          path.append(child.title())
          parent = node
          while parent:
            path.append(parent.title())
            parent = parents.get(parent,None)
          return path[::-1]
        frontier.append(child)
  return "Camino o nodos inexistentes"
  
camino = "Camino o nodos inexistentes"
try:
  camino = BFSpath(org,destino)
except:
  camino = "Camino o nodos inexistentes"

print(camino)

