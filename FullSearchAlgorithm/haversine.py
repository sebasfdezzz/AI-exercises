import math
from geopy.geocoders import Nominatim

geolocator = Nominatim(user_agent="city_coordinates")

def get_coordinates(city):
    location = geolocator.geocode(city)
    if location:
        return location.latitude, location.longitude
    else:
        return None

def haversine(lat1, lon1, lat2, lon2):

    dLat = (lat2 - lat1) * math.pi / 180.0
    dLon = (lon2 - lon1) * math.pi / 180.0
 
    lat1 = (lat1) * math.pi / 180.0
    lat2 = (lat2) * math.pi / 180.0
 
    a = (pow(math.sin(dLat / 2), 2) +
         pow(math.sin(dLon / 2), 2) *
             math.cos(lat1) * math.cos(lat2));
    rad = 6371
    c = 2 * math.asin(math.sqrt(a))
    return rad * c

def haversine_distance(origin, destination):
    origin_coords = get_coordinates(origin)
    dest_coords = get_coordinates(destination)

    if not origin_coords or not dest_coords:
        return "Coordenadas no encontradas"

    return haversine(origin_coords[0], origin_coords[1], dest_coords[0], dest_coords[1])


org = "Cozumel"
dest = "Acapulco"
distance = haversine_distance(org, dest)

print(f"La distancia Haversine entre {org} y {dest} es de {distance} kilometros.")