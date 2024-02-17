import networkx as nx
import matplotlib.pyplot as plt
import random

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

def DFS(node, dest, visited=None):
    if visited is None:
        visited = set()

    visited.add(node)

    if node == dest:
        return [node]

    if G.neighbors(node):
        for child in G.neighbors(node):
            if child not in visited:
                path = DFS(child, dest, visited)
                if path:
                    return [node] + path
    return None


def DFS_limited(node, dest, limit, depth=0, visited=None):
    if visited is None:
        visited = set()

    visited.add(node)

    if node == dest:
        return [node]

    if depth >= limit:
        return None  # Depth limit exceeded

    if node in G:  # Check if node exists in the graph
        for child in G[node]:
            weight = G[node][child]['weight']  # Access weight of the edge
            if child not in visited and weight <= limit - depth:
                path = DFS_limited(child, dest, limit, depth + weight, visited)
                if path:
                    return [node] + path
    return None


def DFS_rnd(node, dest, visited=None):
    if visited is None:
        visited = set()

    visited.add(node)

    if node == dest:
        return [node]

    children = [child for child in G.neighbors(node) if child not in visited]
    if children:
        random_child = random.choice(children)
        path = DFS_rnd(random_child, dest, visited)
        if path:
            return [node] + path
    return None


def DFS_limited_rnd(node, dest, limit, depth=0, visited=None):
    if visited is None:
        visited = set()

    visited.add(node)

    if node == dest:
        return [node]

    if depth >= limit:
        return None
    children = [child for child in G.neighbors(node) if child not in visited]
    if children:
            child = random.choice(children)
            weight = G[node][child]['weight']
            if child not in visited and weight <= limit - depth:
                path = DFS_limited(child, dest, limit, depth + weight, visited)
                if path:
                    return [node] + path
    return None


print(DFS('Cozumel','Acapulco'))
print(DFS_rnd('Cozumel','Acapulco'))
print(DFS_limited('Cozumel','Acapulco',60))
print(DFS_limited_rnd('Cozumel','Acapulco',900))
    



