# Importar las librerías para hacer el grafo y para graficarlo
import networkx as nx
import matplotlib.pyplot as plt
# Se define la estructura de un grafo direccinado vacía
G = nx.DiGraph()
# Se añaden los nodos que conformarán el grafo
G.add_nodes_from(['Cozumel','Ciudad del Carmen','Angel Albino Corzo','Minatitlan','Veracruz',
                  'Acapulco', 'Puebla', 'CDMX', 'Morelia', 'Uruapan',
                  'San Luis Potosi', 'Tampico','Piedras Negras', 'Culiacan', 'Abraham Gonzalez',
                  'Nogales', 'Guaymas', 'Loreto', 'Manuel Marquez de Leon', 'Los Cabos'
                  ])
# Revisamos que los nodos hayan sido añadidos
print(G.nodes())
# Se expresan las relaciones entre los nodos y el coste de transición

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

def beam(start, goal, n):
    state = start
    queue = {}
    dist = {}
    visited = []
    path = {start: [start]}
    
    while True:
        if state == goal:
            return path[state]
        else:
            previous_weight = 0
            try:
                previous_weight = dist[state]
            except:
                previous_weight = 0
            for child in G.successors(state):
                if child not in visited:
                    dist[child] = previous_weight + G.edges[state, child]['weight']
                    queue[child] = dist[child]
                    path[child] = path[state] + [child]  

            queue = dict(list(sorted(queue.items(), key=lambda key_val: key_val[1]))[0:n])
            for key, value in queue.items():
                visited.append(key)

        if not queue:
            return 'Failure'

        state = list(sorted(queue.items(), key=lambda key_val: key_val[1]))[0][0]
        del queue[state]

print(beam('Cozumel', 'Loreto', 5))
print(beam('Culiacan', 'Abraham Gonzalez', 5))

start_node = input("Inicio: ")
goal_node = input("Final: ")
k = int(input("K: "))

print(beam(start_node, goal_node, k))
