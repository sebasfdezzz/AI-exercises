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

def createHeuristicTable2(goal):
  h={}
  for node in G.nodes():
    h[node] = random.randint(20,50)
  h[goal] = 0
  return h

def min_value_star(q, tentatives):
  temp =float('infinity')
  min_val=None
  for node in q:
    if tentatives[node] < temp:
      temp = tentatives[node]
      min_val = node
  return min_val

def astar(orig, dest):
  h = createHeuristicTable2(dest)

  path = []

  to_check_queue = [orig]
  dist = {orig: 0}
  prev = {orig: None}
  tentatives = {orig: 0 + 1.3*h[orig]}

  while to_check_queue:
    current_node = min_value_star(to_check_queue,tentatives)
    to_check_queue.remove(current_node)

    if current_node == dest:
      while current_node is not None:
        path.append(current_node)
        current_node = prev[current_node]
      return path[::-1], dist[dest]

    for child in G.neighbors(current_node):
      temp_dist = dist[current_node] + G.get_edge_data(current_node, child).get('weight', 0)
      if temp_dist < dist.get(child,float('infinity')) or child not in dist:
        dist[child] = temp_dist
        prev[child] = current_node
        tentatives[child] = dist[current_node] + (1.3*h[current_node])
        to_check_queue.append(child)
  return "Path not found"

org = input("Origen: ")
destino = input("Destino: ")
print(astar(org,destino))