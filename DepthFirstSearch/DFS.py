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

def DFS(orig, dest):
  if orig == dest:
    return dest
  
  checked = [orig]
  resting = G.nodes()
  resting.remove(orig)

  while resting:
    



