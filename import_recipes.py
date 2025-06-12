import json
import sqlite3

# Leer archivo JSON
with open("recipes.json", "r") as f:
    recipes = json.load(f)

# Conectar a la base de datos
conn = sqlite3.connect("recipes.db")
cur = conn.cursor()

# Insertar recetas en la tabla
for recipe in recipes:
    title = recipe["title"]
    instructions = recipe["instructions"]
    cur.execute("INSERT INTO recipes (title, description) VALUES (?, ?)", (title, instructions))

conn.commit()
conn.close()
print("Recetas importadas con éxito.")

