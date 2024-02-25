import networkx as nx
import matplotlib.pyplot as plt

G = nx.Graph() #DiGrpah es direccionada
G.add_nodes_from(['Cozumel','Ciudad del Carmen','Angel Albino Corzo','Minatitlan','Veracruz','Acapulco','Puebla','CDMX','Morelia','Uruapan','San Luis Potosi','Tampico','Piedras Negras','Culiacan','Abraham Gonzalez','Nogales','Guaymas','Loreto','Manuel Marquez de Leon','Los Cabos'])
print(G.nodes())

G.add_edges_from([
('Cozumel', 'Tampico', {'weight': 17}),
('Ciudad del Carmen', 'Cozumel', {'weight': 15}),
('Angel Albino Corzo', 'Ciudad del Carmen', {'weight': 11}),
('Angel Albino Corzo', 'Minatitlan', {'weight': 15}),
('Angel Albino Corzo', 'Acapulco', {'weight': 1}),
('Minatitlan', 'Ciudad del Carmen', {'weight': 16}),
('Minatitlan', 'Veracruz', {'weight': 18}),
('Acapulco', 'Morelia', {'weight': 14}),
('Acapulco', 'Uruapan', {'weight': 14}),
('Puebla', 'Veracruz', {'weight': 12}),
('Puebla', 'Acapulco', {'weight': 19}),
('CDMX', 'Acapulco', {'weight': 4}),
('Morelia', 'CDMX', {'weight': 15}),
('Morelia', 'San Luis Potosi', {'weight': 11}),
('Uruapan', 'Culiacan', {'weight': 15}),
('San Luis Potosi', 'Abraham Gonzalez', {'weight': 15}),
('Tampico', 'Puebla', {'weight': 7}),
('Tampico', 'CDMX', {'weight': 8}),
('Tampico', 'San Luis Potosi', {'weight': 11}),
('Piedras Negras', 'San Luis Potosi', {'weight': 11}),
('Piedras Negras', 'Tampico', {'weight': 16}),
('Culiacan', 'Piedras Negras', {'weight': 8}),
('Culiacan', 'Guaymas', {'weight': 16}),
('Abraham Gonzalez', 'Piedras Negras', {'weight': 15}),
('Nogales', 'Abraham Gonzalez', {'weight': 6}),
('Guaymas', 'Abraham Gonzalez', {'weight': 18}),
('Guaymas', 'Nogales', {'weight': 5}),
('Guaymas', 'Loreto', {'weight': 13}),
('Loreto', 'Manuel Marquez de Leon', {'weight': 15}),
('Manuel Marquez de Leon', 'Culiacan', {'weight': 14}),
('Manuel Marquez de Leon', 'Los Cabos', {'weight': 10}),
])

def min_value(q,dist):
  temp =float('infinity')
  min_val=None
  for node in q:
    if dist[node] < temp:
      temp = dist[node]
      min_val = node
  return min_val

org = input("Origen: ")
destino = input("Destino: ")

def new_djkstra_paths(orig, dest):

    G_low = nx.relabel_nodes(G, lambda x: x.lower())
    orig = orig.lower()
    dest = dest.lower()

    queue = list(G_low.nodes())
    dist = {node: float('infinity') for node in G_low.nodes()}
    prev = {node: None for node in G_low.nodes()}
    path = {node: [float('infinity'), []] for node in G_low.nodes()}

    dist[orig] = 0
    path[orig][0] = 0
    path[orig][1] = [orig.title()]

    while queue:
        u = min_value(queue, dist)
        queue.remove(u)
        if u == dest:
            break
        for child in G_low.neighbors(u):
            temp_dist = dist[u] + G_low.get_edge_data(u, child).get('weight', 0)
            if temp_dist < dist[child]:
                dist[child] = temp_dist
                prev[child] = u

    node = dest
    while node:
        path[dest][1].append(node.title())
        node = prev[node]
    path[dest][1] = path[dest][1][::-1]
    path[dest][0] = dist[dest]

    return dist[dest], path[dest][1]

print(new_djkstra_paths(org,destino))